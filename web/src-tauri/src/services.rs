//! The application core assembled at start-up (content, database, services, voice) and the
//! operations called by the commands. Thin adapters over the `educator-*` crates: no business rule
//! here. Kept free of Tauri types so that it is tested directly (`tests` below).

use crate::contracts::*;
use educator_content::EducatorContent;
use educator_core::characters::CharacterCatalog;
use educator_core::clock::Clock;
use educator_core::exercises::SeriesService;
use educator_core::profiles::ProfileService;
use educator_core::random::RandomSource;
use educator_core::skills::{Date, SchoolCalendar, SkillCatalog, Subject};
use educator_core::speech::{self, SpeechSynthesizer};
use educator_core::Error;
use educator_data::{Database, SqliteProfileStore};
use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

/// Default speaking rate: slightly slow for children who are learning to read.
pub const CHILD_SPEECH_SPEED: f64 = 0.9;

pub const MAX_SPEECH_LENGTH: usize = 300;

/// Where the application finds its files.
#[derive(Debug, Clone)]
pub struct Locations {
    pub content_directory: PathBuf,
    pub database_path: PathBuf,
}

pub struct Services {
    skills: Arc<dyn SkillCatalog>,
    characters: Arc<dyn CharacterCatalog>,
    calendar: SchoolCalendar,
    profiles: ProfileService,
    series: SeriesService,
    speech: Arc<dyn SpeechSynthesizer>,
    /// Local date of the computer (school period of today).
    today: Box<dyn Fn() -> Date + Send + Sync>,
}

impl fmt::Debug for Services {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Services").finish_non_exhaustive()
    }
}

impl Services {
    /// Loads the content (invalid files are logged and skipped) and opens the database.
    pub fn start(
        locations: &Locations,
        speech: Arc<dyn SpeechSynthesizer>,
        random: Arc<dyn RandomSource>,
        clock: Arc<dyn Clock>,
        today: Box<dyn Fn() -> Date + Send + Sync>,
    ) -> Result<Services, String> {
        let content = EducatorContent::load(&locations.content_directory);
        for problem in &content.problems {
            log::error!("Invalid content skipped: {problem}");
        }

        log::info!(
            "Content loaded from {}: {} skills, {} characters.",
            content.directory.display(),
            content.skills.all().len(),
            content.characters.all().len()
        );

        let (database, report) = Database::open(&locations.database_path, Arc::clone(&clock))
            .map_err(|e| format!("Database {}: {e}", locations.database_path.display()))?;
        log::info!(
            "Database ready: {} ({} migration(s) applied).",
            database.path().display(),
            report.applied_migrations.len()
        );
        let store = Arc::new(SqliteProfileStore::new(Arc::new(database)));

        let skills: Arc<dyn SkillCatalog> = Arc::new(content.skills);
        let characters: Arc<dyn CharacterCatalog> = Arc::new(content.characters);
        Ok(Services {
            profiles: ProfileService::new(
                store.clone(),
                Arc::clone(&characters),
                Arc::clone(&random),
                Arc::clone(&clock),
            ),
            series: SeriesService::new(Arc::clone(&skills), store, content.points, random, clock),
            skills,
            characters,
            calendar: content.calendar,
            speech,
            today,
        })
    }

    pub fn status(&self) -> StatusResponse {
        let speech = self.speech.status();
        StatusResponse {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            database: DatabaseStatusResponse { is_ready: true },
            speech: SpeechStatusResponse {
                is_available: speech.is_available,
                voice: speech.voice,
                problem: speech.problem,
            },
        }
    }

    pub fn skills(&self, grade: Grade) -> SkillsResponse {
        let current_period = self.calendar.period_of((self.today)());
        let (skills_grade, offered) = self.skills.offered_or_fallback(Subject::Math, grade.into());
        SkillsResponse {
            grade,
            skills_grade: skills_grade.into(),
            current_period,
            skills: offered
                .iter()
                .map(|o| SkillResponse::offered(o, current_period))
                .collect(),
        }
    }

    pub fn characters(&self) -> Vec<CharacterResponse> {
        self.characters.all().iter().map(Into::into).collect()
    }

    /// A character at random for the creation screen, different from `except`.
    pub fn draw_character(&self, except: Option<&str>) -> Result<CharacterResponse, CommandError> {
        Ok((&self.profiles.draw_character(except).map_err(unexpected)?).into())
    }

    pub fn character_svg(&self, id: &str) -> Result<String, CommandError> {
        self.characters.svg(id).map(str::to_owned).ok_or_else(|| {
            CommandError::new(
                404,
                "Unknown character",
                format!("No character with id {id}."),
            )
        })
    }

    pub fn list_profiles(&self) -> Result<Vec<ProfileResponse>, CommandError> {
        Ok(self
            .profiles
            .list()
            .map_err(profile_error)?
            .iter()
            .map(Into::into)
            .collect())
    }

    pub fn create_profile(
        &self,
        request: CreateProfileRequest,
    ) -> Result<ProfileResponse, CommandError> {
        let profile = self
            .profiles
            .create(
                &request.name,
                request.grade.into(),
                request.character_id.as_deref(),
            )
            .map_err(profile_error)?;
        Ok((&profile).into())
    }

    pub fn update_profile(
        &self,
        id: &str,
        changes: UpdateProfileRequest,
    ) -> Result<ProfileResponse, CommandError> {
        let profile = self
            .profiles
            .update(
                profile_id(id)?,
                changes.name.as_deref(),
                changes.grade.map(Into::into),
                changes.character_id.as_deref(),
            )
            .map_err(profile_error)?;
        Ok((&profile).into())
    }

    pub fn select_profile(&self, id: &str) -> Result<ProfileResponse, CommandError> {
        Ok((&self
            .profiles
            .select(profile_id(id)?)
            .map_err(profile_error)?)
            .into())
    }

    pub fn reroll_character(&self, id: &str) -> Result<ProfileResponse, CommandError> {
        Ok((&self
            .profiles
            .reroll_character(profile_id(id)?)
            .map_err(profile_error)?)
            .into())
    }

    pub fn delete_profile(&self, id: &str) -> Result<(), CommandError> {
        self.profiles.delete(profile_id(id)?).map_err(profile_error)
    }

    pub fn start_series(
        &self,
        request: StartSeriesRequest,
    ) -> Result<SeriesResponse, CommandError> {
        let profile = profile_id(&request.profile_id)?;
        let series = self
            .series
            .start(
                profile,
                &request.skill_id,
                request.answer_mode.into(),
                request.seed,
            )
            .map_err(|e| match e {
                Error::SeriesRequest(message) => {
                    CommandError::new(400, "Invalid series request", message)
                }
                other => profile_error(other),
            })?;
        Ok(SeriesResponse {
            series_id: series.id.to_string(),
            seed: series.seed,
            skill_id: series.skill.id.clone(),
            instruction: series.skill.instruction.clone(),
            answer_mode: series.answer_mode.into(),
            exercises: series.exercises.iter().map(Into::into).collect(),
        })
    }

    pub fn answer_series(
        &self,
        series_id: &str,
        index: u32,
        answer: i32,
    ) -> Result<SeriesAnswerResponse, CommandError> {
        let outcome = self
            .series
            .answer(series_id_of(series_id)?, index as usize, answer)
            .map_err(|e| series_error(e, "Answer refused"))?;
        Ok(SeriesAnswerResponse {
            correct: outcome.correct,
            expected: outcome.expected,
            explanation: outcome.explanation,
            summary: outcome.summary.map(Into::into),
        })
    }

    pub fn abandon_series(&self, series_id: &str) -> Result<SeriesSummaryResponse, CommandError> {
        let summary = self
            .series
            .abandon(series_id_of(series_id)?)
            .map_err(|e| series_error(e, "Series already over"))?;
        Ok(summary.into())
    }

    /// WAV of a French text; numbers and calculation signs are read as words (« 7 + 5 » → « sept plus cinq »).
    pub fn speak(&self, text: &str, speed: Option<f64>) -> Result<Vec<u8>, CommandError> {
        let text = speech::normalize(text);
        let speed = speed.unwrap_or(CHILD_SPEECH_SPEED);
        if text.is_empty()
            || text.encode_utf16().count() > MAX_SPEECH_LENGTH
            || !(speech::speed::MIN..=speech::speed::MAX).contains(&speed)
        {
            return Err(CommandError::new(
                400,
                "Invalid speech request",
                format!(
                    "Text of 1 to {MAX_SPEECH_LENGTH} characters, speed from {} to {}.",
                    speech::speed::MIN,
                    speech::speed::MAX
                ),
            ));
        }

        self.speech.synthesize(&text, speed).map_err(|e| match e {
            Error::SpeechUnavailable(message) => {
                CommandError::new(503, "Speech unavailable", message)
            }
            other => unexpected(other),
        })
    }
}

fn parse_id(id: &str, title: &str, what: &str) -> Result<Uuid, CommandError> {
    Uuid::parse_str(id)
        .map_err(|_| CommandError::new(404, title, format!("No {what} with id {id}.")))
}

fn profile_id(id: &str) -> Result<Uuid, CommandError> {
    parse_id(id, "Unknown profile", "profile")
}

fn series_id_of(id: &str) -> Result<Uuid, CommandError> {
    parse_id(id, "Unknown series", "series in progress")
}

fn profile_error(error: Error) -> CommandError {
    match error {
        Error::ProfileValidation { field, message } => CommandError {
            status: 400,
            title: "Invalid profile".to_owned(),
            detail: Some(message),
            field: Some(field.to_owned()),
        },
        Error::ProfileNotFound(_) => CommandError::new(404, "Unknown profile", error.to_string()),
        other => unexpected(other),
    }
}

fn series_error(error: Error, conflict_title: &str) -> CommandError {
    match error {
        Error::SeriesNotFound(_) | Error::ProfileNotFound(_) => {
            CommandError::new(404, "Unknown series", error.to_string())
        }
        Error::SeriesRequest(message) => CommandError::new(409, conflict_title, message),
        other => unexpected(other),
    }
}

fn unexpected(error: Error) -> CommandError {
    log::error!("Unexpected error: {error}");
    CommandError::new(500, "Unexpected error", error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use educator_core::clock::{unix_ms, ManualClock};
    use educator_core::random::ScriptedRandom;
    use educator_core::speech::UnavailableSpeechSynthesizer;
    use std::path::Path;

    struct Fixture {
        _dir: tempfile::TempDir,
        services: Services,
    }

    fn fixture() -> Fixture {
        let dir = tempfile::tempdir().unwrap();
        let locations = Locations {
            content_directory: Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content"),
            database_path: dir.path().join("educator.db"),
        };
        let services = Services::start(
            &locations,
            Arc::new(UnavailableSpeechSynthesizer::new("tests")),
            Arc::new(ScriptedRandom::new([3])),
            Arc::new(ManualClock::new(unix_ms(2026, 10, 11, 8, 0, 0))),
            Box::new(|| Date::new(2026, 10, 11)),
        )
        .unwrap();
        Fixture {
            _dir: dir,
            services,
        }
    }

    fn create(services: &Services, name: &str) -> ProfileResponse {
        services
            .create_profile(CreateProfileRequest {
                name: name.to_owned(),
                grade: Grade::CP,
                character_id: None,
            })
            .unwrap()
    }

    #[test]
    fn status_reports_the_database_and_the_voice() {
        let status = fixture().services.status();

        assert!(status.database.is_ready);
        assert!(!status.speech.is_available);
        assert_eq!(Some("tests".to_owned()), status.speech.problem);
    }

    #[test]
    fn skills_of_cp_are_sorted_with_the_current_period() {
        let skills = fixture().services.skills(Grade::CP);

        assert_eq!(Grade::CP, skills.skills_grade);
        assert_eq!(1, skills.current_period);
        assert!(skills.skills.windows(2).all(|w| w[0].period <= w[1].period));
        assert!(skills.skills.iter().any(|s| s.is_current_period));
    }

    #[test]
    fn ce1_gets_the_cp_revisions_after_its_own_first_period() {
        let skills = fixture().services.skills(Grade::CE1);
        let first: Vec<_> = skills.skills.iter().filter(|s| s.period == 1).collect();

        assert!(first.iter().any(|s| s.revision_from == Some(Grade::CP)));
        // Own skills first, then the revisions, which are never « en ce moment ».
        let revision_start = first
            .iter()
            .position(|s| s.revision_from.is_some())
            .unwrap();
        assert!(first[revision_start..]
            .iter()
            .all(|s| s.revision_from == Some(Grade::CP) && !s.is_current_period));
        assert!(first[..revision_start]
            .iter()
            .all(|s| s.revision_from.is_none()));
    }

    #[test]
    fn grades_without_skills_get_the_ce1_ones() {
        let skills = fixture().services.skills(Grade::CM2);

        assert_eq!(Grade::CM2, skills.grade);
        assert_eq!(Grade::CE1, skills.skills_grade);
        assert!(!skills.skills.is_empty());
    }

    #[test]
    fn characters_and_their_svg() {
        let f = fixture();
        let characters = f.services.characters();

        assert!(characters.len() >= 11);
        assert!(f.services.character_svg("lion").unwrap().contains("<svg"));
        assert_eq!(404, f.services.character_svg("dragon").unwrap_err().status);
        assert_ne!("lion", f.services.draw_character(Some("lion")).unwrap().id);
    }

    #[test]
    fn profile_errors_keep_the_french_message_and_the_field() {
        let f = fixture();
        create(&f.services, "Léa");

        let error = f
            .services
            .create_profile(CreateProfileRequest {
                name: "LÉA".to_owned(),
                grade: Grade::CE1,
                character_id: None,
            })
            .unwrap_err();

        assert_eq!(400, error.status);
        assert_eq!("Invalid profile", error.title);
        assert_eq!(
            Some("Ce prénom est déjà pris. Choisis-en un autre.".to_owned()),
            error.detail
        );
        assert_eq!(Some("name".to_owned()), error.field);
        assert_eq!(
            404,
            f.services
                .select_profile(&Uuid::now_v7().to_string())
                .unwrap_err()
                .status
        );
        assert_eq!(
            404,
            f.services.select_profile("not-a-guid").unwrap_err().status
        );
    }

    /// SUB000115: name, grade and character changed together, points kept, and the skills
    /// follow the new grade.
    #[test]
    fn a_profile_sheet_change_keeps_the_points_and_moves_the_skills() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        let series = f
            .services
            .start_series(StartSeriesRequest {
                profile_id: lea.id.clone(),
                skill_id: "cp-add-sum-10".to_owned(),
                answer_mode: AnswerMode::Keyboard,
                seed: Some(1),
            })
            .unwrap();
        let points = f
            .services
            .abandon_series(&series.series_id)
            .unwrap()
            .profile_points;

        let updated = f
            .services
            .update_profile(
                &lea.id,
                UpdateProfileRequest {
                    name: Some("Léonie".to_owned()),
                    grade: Some(Grade::CE1),
                    character_id: Some("renard".to_owned()),
                },
            )
            .unwrap();

        assert_eq!(
            ("Léonie", Grade::CE1, "renard", points),
            (
                updated.name.as_str(),
                updated.grade,
                updated.character_id.as_str(),
                updated.points
            )
        );
        let skills = f.services.skills(updated.grade);
        assert_eq!(Grade::CE1, skills.skills_grade);
        assert!(skills
            .skills
            .iter()
            .all(|s| s.id.starts_with("ce1-") || s.revision_from == Some(Grade::CP)));

        let refused = f
            .services
            .update_profile(
                &lea.id,
                UpdateProfileRequest {
                    character_id: Some("dragon".to_owned()),
                    ..Default::default()
                },
            )
            .unwrap_err();
        assert_eq!(
            (400, Some("characterId")),
            (refused.status, refused.field.as_deref())
        );
    }

    #[test]
    fn profiles_are_listed_updated_and_selected() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        assert_eq!("2026-10-11T08:00:00.000Z", lea.last_used_at);

        let updated = f
            .services
            .update_profile(
                &lea.id,
                UpdateProfileRequest {
                    grade: Some(Grade::CE1),
                    ..Default::default()
                },
            )
            .unwrap();
        f.services.select_profile(&lea.id).unwrap();

        assert_eq!(Grade::CE1, updated.grade);
        assert_eq!("Léa", updated.name);
        assert_eq!(
            vec![lea.id.clone()],
            f.services
                .list_profiles()
                .unwrap()
                .into_iter()
                .map(|p| p.id)
                .collect::<Vec<_>>()
        );
        f.services.delete_profile(&lea.id).unwrap();
        assert!(f.services.list_profiles().unwrap().is_empty());
    }

    #[test]
    fn a_whole_series_hides_the_answer_and_awards_the_points() {
        let f = fixture();
        let lea = create(&f.services, "Léa");

        let series = f
            .services
            .start_series(StartSeriesRequest {
                profile_id: lea.id.clone(),
                skill_id: "cp-add-sum-10".to_owned(),
                answer_mode: AnswerMode::Choice,
                seed: Some(42),
            })
            .unwrap();

        assert_eq!(10, series.exercises.len());
        let mut last = None;
        for exercise in &series.exercises {
            let hidden = [exercise.left, exercise.right, exercise.result]
                .iter()
                .filter(|v| v.is_none())
                .count();
            assert_eq!(
                1, hidden,
                "exactly one number is hidden: {}",
                exercise.prompt
            );
            assert_eq!(4, exercise.choices.as_ref().unwrap().len());
            // Find the expected answer by trying the choices: the first answer is the one recorded.
            let probe = f
                .services
                .answer_series(&series.series_id, exercise.index, -1)
                .unwrap();
            assert!(!probe.correct);
            last = probe.summary;
        }

        let summary = last.unwrap();
        assert_eq!(
            (0, 10, 20, 20),
            (
                summary.correct_count,
                summary.exercise_count,
                summary.points_earned,
                summary.profile_points
            )
        );
        assert_eq!(
            404,
            f.services
                .abandon_series(&series.series_id)
                .unwrap_err()
                .status
        );
    }

    #[test]
    fn series_errors_map_to_400_404_and_409() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        let start = |skill: &str| {
            f.services.start_series(StartSeriesRequest {
                profile_id: lea.id.clone(),
                skill_id: skill.to_owned(),
                answer_mode: AnswerMode::Keyboard,
                seed: None,
            })
        };

        assert_eq!(400, start("unknown").unwrap_err().status);
        let series = start("cp-add-sum-10").unwrap();
        f.services.answer_series(&series.series_id, 0, 1).unwrap();
        assert_eq!(
            409,
            f.services
                .answer_series(&series.series_id, 0, 1)
                .unwrap_err()
                .status
        );
        let summary = f.services.abandon_series(&series.series_id).unwrap();
        assert_eq!(10, summary.exercise_count);
        assert_eq!(
            404,
            f.services
                .answer_series(&series.series_id, 1, 1)
                .unwrap_err()
                .status
        );
    }

    #[test]
    fn speech_checks_the_request_and_reports_a_missing_voice() {
        let f = fixture();

        assert_eq!(400, f.services.speak("", None).unwrap_err().status);
        assert_eq!(
            400,
            f.services.speak("Bonjour", Some(3.0)).unwrap_err().status
        );
        let error = f.services.speak("7 + 5", None).unwrap_err();
        assert_eq!(503, error.status);
        assert_eq!(Some("tests".to_owned()), error.detail);
    }
}
