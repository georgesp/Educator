//! The application core assembled at start-up (content, database, services, voice) and the
//! operations called by the commands. Thin adapters over the `educator-*` crates: no business rule
//! here. Kept free of Tauri types so that it is tested directly (`tests` below).

use crate::contracts::*;
use educator_content::EducatorContent;
use educator_core::characters::CharacterCatalog;
use educator_core::clock::Clock;
use educator_core::exercises::{multiplication_table, SeriesService, TABLE_LAST_FACTOR};
use educator_core::lessons::LessonCatalog;
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
    lessons: Arc<dyn LessonCatalog>,
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
            "Content loaded from {}: {} skills, {} lessons, {} characters.",
            content.directory.display(),
            content.skills.all().len(),
            content.lessons.all().len(),
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
        let lessons: Arc<dyn LessonCatalog> = Arc::new(content.lessons);
        Ok(Services {
            profiles: ProfileService::new(
                store.clone(),
                Arc::clone(&characters),
                Arc::clone(&random),
                Arc::clone(&clock),
            ),
            series: SeriesService::new(Arc::clone(&skills), store, content.points, random, clock),
            skills,
            lessons,
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
        answer: AnswerPayload,
    ) -> Result<SeriesAnswerResponse, CommandError> {
        let outcome = self
            .series
            .answer(series_id_of(series_id)?, index as usize, &answer.into())
            .map_err(|e| series_error(e, "Answer refused"))?;
        Ok(SeriesAnswerResponse {
            correct: outcome.correct,
            expected: AnswerPayload::from(&outcome.expected),
            expected_label: outcome.expected_label,
            explanation: outcome.explanation,
            help: outcome.help.as_ref().map(Into::into),
            summary: outcome.summary.map(Into::into),
        })
    }

    /// « Mes tables » (SUB000119): the table of a number from 0 to 10.
    pub fn multiplication_table(
        &self,
        number: i32,
    ) -> Result<MultiplicationTableResponse, CommandError> {
        let lines = multiplication_table(number).ok_or_else(|| {
            CommandError::new(
                400,
                "Unknown table",
                format!("No multiplication table of {number} (0 to {TABLE_LAST_FACTOR})."),
            )
        })?;
        Ok(MultiplicationTableResponse {
            number,
            lines: lines.iter().map(Into::into).collect(),
        })
    }

    pub fn abandon_series(&self, series_id: &str) -> Result<SeriesSummaryResponse, CommandError> {
        let summary = self
            .series
            .abandon(series_id_of(series_id)?)
            .map_err(|e| series_error(e, "Series already over"))?;
        Ok(summary.into())
    }

    /// The lessons of a grade (SUB001410). They follow the exercises: a grade without skills yet
    /// gets the CE1 lessons, like it gets the CE1 skills, so that « Je m'entraîne » always finds the
    /// skill of the lesson; a grade with skills but no lesson yet gets an empty list.
    pub fn lessons(&self, grade: Grade) -> LessonsResponse {
        let current_period = self.calendar.period_of((self.today)());
        let (lessons_grade, _) = self.skills.offered_or_fallback(Subject::Math, grade.into());
        let lessons = self.lessons.for_grade(Subject::Math, lessons_grade);
        LessonsResponse {
            grade,
            lessons_grade: lessons_grade.into(),
            current_period,
            lessons: lessons
                .iter()
                .map(|l| LessonSummaryResponse::new(l, current_period))
                .collect(),
        }
    }

    pub fn lesson(&self, id: &str) -> Result<LessonResponse, CommandError> {
        self.lessons
            .find(id)
            .map(Into::into)
            .ok_or_else(|| unknown_lesson(id))
    }

    /// Checks an answer of the end-of-lesson quiz (SUB001411).
    pub fn answer_lesson_quiz(
        &self,
        lesson_id: &str,
        index: u32,
        answer: AnswerPayload,
    ) -> Result<QuizAnswerResponse, CommandError> {
        let lesson = self
            .lessons
            .find(lesson_id)
            .ok_or_else(|| unknown_lesson(lesson_id))?;
        let outcome = lesson
            .check_quiz(index as usize, &answer.into())
            .ok_or_else(|| {
                CommandError::new(
                    404,
                    "Unknown question",
                    format!("No question {index} in lesson {lesson_id}."),
                )
            })?;
        Ok(QuizAnswerResponse {
            correct: outcome.correct,
            expected: AnswerPayload::from(&outcome.expected),
            expected_label: outcome.expected.label(),
            explanation: outcome.explanation,
        })
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

fn unknown_lesson(id: &str) -> CommandError {
    CommandError::new(404, "Unknown lesson", format!("No lesson with id {id}."))
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

    fn number(value: i32) -> AnswerPayload {
        AnswerPayload::Number { value }
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
            let slots = exercise
                .statement
                .iter()
                .filter(|t| matches!(t, TokenResponse::Slot { .. }))
                .count();
            assert_eq!(
                1, slots,
                "exactly one number is hidden: {}",
                exercise.prompt
            );
            assert_eq!(InputResponse::Pick, exercise.input);
            assert_eq!(4, exercise.choices.as_ref().unwrap().len());
            // Find the expected answer by trying the choices: the first answer is the one recorded.
            let probe = f
                .services
                .answer_series(&series.series_id, exercise.index, number(-1))
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
        f.services
            .answer_series(&series.series_id, 0, number(1))
            .unwrap();
        assert_eq!(
            409,
            f.services
                .answer_series(&series.series_id, 0, number(1))
                .unwrap_err()
                .status
        );
        let summary = f.services.abandon_series(&series.series_id).unwrap();
        assert_eq!(10, summary.exercise_count);
        assert_eq!(
            404,
            f.services
                .answer_series(&series.series_id, 1, number(1))
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

    #[test]
    fn every_ce1_family_hides_its_answer_and_checks_the_payload() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        for skill in [
            "ce1-num-compare",
            "ce1-num-frame-line",
            "ce1-num-read-write",
            "ce1-parity",
            "ce1-frac-compare-add",
            "ce1-problems-additive",
        ] {
            let modes = if skill == "ce1-num-compare" || skill == "ce1-parity" {
                vec![AnswerMode::Keyboard]
            } else {
                vec![AnswerMode::Keyboard, AnswerMode::Choice]
            };
            for mode in modes {
                let series = f
                    .services
                    .start_series(StartSeriesRequest {
                        profile_id: lea.id.clone(),
                        skill_id: skill.to_owned(),
                        answer_mode: mode,
                        seed: Some(3),
                    })
                    .unwrap();
                let json = serde_json::to_string(&series).unwrap();
                assert!(json.contains("\"kind\""), "{json}");
                for exercise in &series.exercises {
                    // A wrong answer of the wrong kind is simply wrong; the correction gives the right one.
                    let outcome = f
                        .services
                        .answer_series(
                            &series.series_id,
                            exercise.index,
                            AnswerPayload::YesNo { value: true },
                        )
                        .unwrap();
                    assert!(!outcome.expected_label.is_empty());
                    assert!(!outcome.explanation.is_empty());
                }
            }
        }
    }

    #[test]
    fn a_picked_choice_is_right_when_it_is_the_expected_answer() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        let series = f
            .services
            .start_series(StartSeriesRequest {
                profile_id: lea.id.clone(),
                skill_id: "ce1-num-read-write".to_owned(),
                answer_mode: AnswerMode::Choice,
                seed: Some(11),
            })
            .unwrap();
        let first = &series.exercises[0];
        let wrong = f
            .services
            .answer_series(&series.series_id, 0, AnswerPayload::Number { value: -5 })
            .unwrap();
        let right = first
            .choices
            .as_ref()
            .unwrap()
            .iter()
            .find(|c| c.answer == wrong.expected)
            .expect("the expected answer is one of the choices");
        assert_eq!(right.label, wrong.expected_label);
    }

    #[test]
    fn multiplication_tables() {
        let f = fixture();
        let table = f.services.multiplication_table(7).unwrap();
        assert_eq!(11, table.lines.len());
        assert_eq!(
            (7, 3, 21),
            (
                table.lines[3].left,
                table.lines[3].right,
                table.lines[3].result
            )
        );
        assert_eq!(400, f.services.multiplication_table(11).unwrap_err().status);
    }

    #[test]
    fn answer_payloads_use_the_kind_tag() {
        let payload: AnswerPayload =
            serde_json::from_str(r#"{ "kind": "fraction", "numerator": 2, "denominator": 4 }"#)
                .unwrap();
        assert_eq!(
            educator_core::exercises::Answer::Fraction(educator_core::exercises::Fraction::new(
                2, 4
            )),
            payload.into()
        );
        let sign: AnswerPayload =
            serde_json::from_str(r#"{ "kind": "comparison", "sign": "<" }"#).unwrap();
        assert_eq!(
            AnswerPayload::Comparison {
                sign: ComparisonSign::Less
            },
            sign
        );
    }

    #[test]
    fn lessons_of_ce1_list_the_lesson_on_adding_9_19_29() {
        let lessons = fixture().services.lessons(Grade::CE1);

        assert_eq!(Grade::CE1, lessons.lessons_grade);
        let lesson = lessons
            .lessons
            .iter()
            .find(|l| l.id == "ce1-add-9-19-29")
            .unwrap();
        assert_eq!(2, lesson.period);
        assert!(!lesson.is_current_period);
    }

    #[test]
    fn a_grade_without_skills_gets_the_ce1_lessons() {
        let lessons = fixture().services.lessons(Grade::CM1);

        assert_eq!(Grade::CM1, lessons.grade);
        assert_eq!(Grade::CE1, lessons.lessons_grade);
        assert!(!lessons.lessons.is_empty());
    }

    /// CP has its own skills: no CE1 lesson, whose « Je m'entraîne » would lead to a skill the CP
    /// profile is not offered.
    #[test]
    fn cp_gets_no_lesson_of_another_grade() {
        let lessons = fixture().services.lessons(Grade::CP);

        assert_eq!(Grade::CP, lessons.lessons_grade);
        assert!(lessons.lessons.is_empty());
    }

    #[test]
    fn the_skill_of_every_lesson_is_offered_where_the_lesson_is() {
        let f = fixture();
        for grade in [Grade::CP, Grade::CE1, Grade::CE2, Grade::CM1, Grade::CM2] {
            let skills = f.services.skills(grade);
            for summary in f.services.lessons(grade).lessons {
                let lesson = f.services.lesson(&summary.id).unwrap();
                assert!(
                    skills.skills.iter().any(|s| s.id == lesson.skill_ids[0]),
                    "{grade:?}: {}",
                    lesson.id
                );
            }
        }
    }

    #[test]
    fn a_lesson_comes_with_computed_animations_and_without_the_answers() {
        let f = fixture();
        let lesson = f.services.lesson("ce1-add-9-19-29").unwrap();

        assert_eq!(vec!["ce1-add-9-19-29".to_owned()], lesson.skill_ids);
        assert_eq!(10, lesson.slides.len());
        assert_eq!(2, lesson.slides.iter().filter(|s| s.interactive).count());
        assert_eq!(
            AnimationResponse::TensUnits {
                value: 19,
                hundreds: 0,
                tens: 1,
                units: 9,
                missing: 1
            },
            lesson.slides[2].animation
        );
        let AnimationResponse::NumberLine {
            jumps,
            hover_values,
            ..
        } = &lesson.slides[5].animation
        else {
            panic!("slide 6 is a number line");
        };
        assert!(*hover_values);
        assert_eq!(
            vec![
                JumpResponse {
                    delta: 20,
                    value: 77
                },
                JumpResponse {
                    delta: -1,
                    value: 76
                }
            ],
            *jumps
        );
        let json = serde_json::to_value(&lesson).unwrap();
        assert_eq!("numberLine", json["slides"][5]["animation"]["type"]);
        assert_eq!(true, json["slides"][5]["animation"]["hoverValues"]);
        assert!(json["quiz"][0].get("answer").is_none());
        assert_eq!(
            3,
            lesson.quiz[1].choices.as_ref().map(Vec::len).unwrap_or(0)
        );
        assert_eq!(404, f.services.lesson("nothing").unwrap_err().status);
    }

    #[test]
    fn the_quiz_is_checked_by_the_core_with_its_explanation() {
        let f = fixture();

        let right = f
            .services
            .answer_lesson_quiz("ce1-add-9-19-29", 0, number(37))
            .unwrap();
        assert!(right.correct);
        assert!(right.explanation.contains("28 + 10 = 38"));

        let wrong = f
            .services
            .answer_lesson_quiz("ce1-add-9-19-29", 1, number(66))
            .unwrap();
        assert!(!wrong.correct);
        assert_eq!(number(65), wrong.expected);
        assert_eq!("65", wrong.expected_label);

        let missing = f
            .services
            .answer_lesson_quiz("ce1-add-9-19-29", 9, number(1))
            .unwrap_err();
        assert_eq!(404, missing.status);
    }

    #[test]
    fn every_lesson_text_read_aloud_is_accepted_by_the_voice() {
        assert_eq!(educator_content::MAX_SPOKEN_LENGTH, MAX_SPEECH_LENGTH);
        let f = fixture();
        for summary in f.services.lessons(Grade::CE1).lessons {
            let lesson = f.services.lesson(&summary.id).unwrap();
            let texts = lesson
                .slides
                .iter()
                .map(|s| s.speech.clone())
                .chain(lesson.quiz.iter().map(|q| q.spoken.clone()));
            for text in texts {
                // The voice is unavailable in the tests: 503 means the text itself was accepted.
                assert_eq!(
                    503,
                    f.services.speak(&text, None).unwrap_err().status,
                    "{text}"
                );
            }
        }
    }
}
