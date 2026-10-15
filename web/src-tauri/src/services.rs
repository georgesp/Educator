//! The application core assembled at start-up (content, database, services, voice) and the
//! operations called by the commands. Thin adapters over the `educator-*` crates: no business rule
//! here. Kept free of Tauri types so that it is tested directly (`tests` below).

use crate::contracts::*;
use educator_content::{EducatorContent, WordBank};
use educator_core::adaptive::{advise, AdaptiveRules};
use educator_core::characters::CharacterCatalog;
use educator_core::clock::Clock;
use educator_core::exercises::exam::TIMED_DURATIONS as TIMED_CHOICES;
use educator_core::exercises::exam::{Listening, DEFAULT_TIMED_DURATION, TIMED_DURATIONS};
use educator_core::exercises::Series;
use educator_core::exercises::SeriesSummary;
use educator_core::exercises::{multiplication_table, SeriesService, TABLE_LAST_FACTOR};
use educator_core::lessons::LessonCatalog;
use educator_core::mascot::{Expression, MascotService};
use educator_core::parent_gate::ParentGateKeeper;
use educator_core::parent_profile::ParentProfileService;
use educator_core::profile_settings::SESSION_MINUTES;
use educator_core::profile_settings::{ProfileSettingsService, VoiceSpeed};
use educator_core::profiles::ProfileService;
use educator_core::random::RandomSource;
use educator_core::reviews::ReviewQueue;
use educator_core::rewards::accessories::WornAccessory;
use educator_core::rewards::daily_goal::GOAL_CHOICES;
use educator_core::rewards::{DateOf, RewardsDependencies, RewardsService};
use educator_core::sessions::{close_open_sessions, SessionStore};
use educator_core::settings::AppSettingsStore;
use educator_core::skills::{Date, SchoolCalendar, SkillCatalog};
use educator_core::speech::{self, SpeechPreparer, SpeechSynthesizer};
use educator_core::spelling::SpokenWord;
use educator_core::statistics::{profile_statistics, LocalCalendar};
use educator_core::Error;
use educator_data::{
    Database, SqliteAppSettingsStore, SqliteProfileSettingsStore, SqliteProfileStore,
    SqliteReviewStore, SqliteRewardStore, SqliteSessionStore,
};
use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

/// Default speaking rate: slightly slow for children who are learning to read.
pub const CHILD_SPEECH_SPEED: f64 = speech::speed::INSTRUCTIONS;

pub const MAX_SPEECH_LENGTH: usize = 300;

/// Where the application finds its files.
#[derive(Debug, Clone)]
pub struct Locations {
    pub content_directory: PathBuf,
    pub database_path: PathBuf,
}

pub struct Services {
    skills: Arc<dyn SkillCatalog>,
    words: WordBank,
    sessions: Arc<dyn SessionStore>,
    mascot: Option<MascotService>,
    lessons: Arc<dyn LessonCatalog>,
    characters: Arc<dyn CharacterCatalog>,
    calendar: SchoolCalendar,
    profiles: ProfileService,
    series: SeriesService,
    speech: Arc<dyn SpeechSynthesizer>,
    /// The words of the series read ahead of the child (SUB026705).
    preparer: SpeechPreparer,
    /// Local date of the computer (school period of today).
    today: Box<dyn Fn() -> Date + Send + Sync>,
    /// Local date of an instant (days of the statistics, goal of the day, reviews).
    local_date: DateOf,
    /// The parents' gate waiting for its answer (SUB001510).
    parent_gates: ParentGateKeeper,
    /// The parents' card of the profile choice.
    parent_profile: ParentProfileService,
    /// Lot 3: levels, stars, badges, accessories, goal of the day.
    rewards: RewardsService,
    reviews: Arc<ReviewQueue>,
    settings: Arc<ProfileSettingsService>,
    adaptive: AdaptiveRules,
    themes: Vec<educator_core::profile_settings::Theme>,
    database: Arc<Database>,
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
        local_date: DateOf,
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
        let database = Arc::new(database);
        let store = Arc::new(SqliteProfileStore::new(Arc::clone(&database)));
        let sessions: Arc<dyn SessionStore> =
            Arc::new(SqliteSessionStore::new(Arc::clone(&database)));
        let settings: Arc<dyn AppSettingsStore> =
            Arc::new(SqliteAppSettingsStore::new(Arc::clone(&database)));
        let profile_settings = Arc::new(ProfileSettingsService::new(
            Arc::new(SqliteProfileSettingsStore::new(Arc::clone(&database))),
            content.rewards.goal,
            content.themes.iter().map(|t| t.id.clone()).collect(),
        ));
        let reviews = Arc::new(ReviewQueue::new(
            Arc::new(SqliteReviewStore::new(Arc::clone(&database))),
            content.reviews.clone(),
            Arc::clone(&clock),
            Arc::clone(&local_date),
        ));
        // A series does not survive the application: what was left open is closed as abandoned.
        let closed =
            close_open_sessions(sessions.as_ref(), clock.now_ms()).map_err(|e| e.to_string())?;
        if closed > 0 {
            log::info!("{closed} session(s) left open closed as abandoned.");
        }

        let skills: Arc<dyn SkillCatalog> = Arc::new(content.skills);
        let characters: Arc<dyn CharacterCatalog> = Arc::new(content.characters);
        let lessons: Arc<dyn LessonCatalog> = Arc::new(content.lessons);
        let rewards = RewardsService::new(
            content.rewards.clone(),
            content.badges,
            content.accessories,
            RewardsDependencies {
                skills: Arc::clone(&skills),
                characters: Arc::clone(&characters),
                profiles: store.clone(),
                sessions: Arc::clone(&sessions),
                store: Arc::new(SqliteRewardStore::new(Arc::clone(&database))),
                settings: Arc::clone(&profile_settings),
                clock: Arc::clone(&clock),
                date_of: Arc::clone(&local_date),
            },
        );
        Ok(Services {
            rewards,
            reviews: Arc::clone(&reviews),
            settings: profile_settings,
            adaptive: content.adaptive,
            themes: content.themes,
            database,
            profiles: ProfileService::new(
                store.clone(),
                Arc::clone(&characters),
                Arc::clone(&random),
                Arc::clone(&clock),
            ),
            mascot: content
                .mascot
                .map(|m| MascotService::new(m, Arc::clone(&random))),
            parent_gates: ParentGateKeeper::new(Arc::clone(&random), Arc::clone(&clock)),
            parent_profile: ParentProfileService::new(settings, Arc::clone(&characters)),
            series: SeriesService::new(
                Arc::clone(&skills),
                store,
                Arc::clone(&sessions),
                content.points,
                random,
                clock,
            )
            .with_listening(listening_of(speech.as_ref()))
            .with_reviews(reviews),
            local_date,
            skills,
            words: content.words,
            sessions,
            lessons,
            characters,
            calendar: content.calendar,
            preparer: SpeechPreparer::new(Arc::clone(&speech)),
            speech,
            today,
        })
    }

    /// Reads the state of the voice again (after its warm-up): without it, the exams ask no
    /// question where the word is only heard (SUB026704).
    pub fn refresh_listening(&self) {
        self.series
            .set_listening(listening_of(self.speech.as_ref()));
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

    /// The skills of a subject offered to a grade, with the mastery of the profile when given.
    pub fn skills(
        &self,
        grade: Grade,
        subject: Subject,
        profile_id: Option<&str>,
    ) -> Result<SkillsResponse, CommandError> {
        let current_period = self.calendar.period_of((self.today)());
        let (skills_grade, offered) = self
            .skills
            .offered_or_fallback(subject.into(), grade.into());
        let (masteries, stars) = match profile_id {
            Some(id) => {
                let id = profile_id_of(id)?;
                (
                    self.sessions.masteries(id).map_err(unexpected)?,
                    self.rewards.best_stars(id).map_err(unexpected)?,
                )
            }
            None => (Vec::new(), Default::default()),
        };
        let mastery = |skill_id: &str| {
            masteries
                .iter()
                .find(|m| m.skill_id == skill_id)
                .map(|m| m.score)
        };
        Ok(SkillsResponse {
            subject,
            grade,
            skills_grade: skills_grade.into(),
            current_period,
            skills: offered
                .iter()
                .map(|o| {
                    SkillResponse::offered(
                        o,
                        current_period,
                        mastery(&o.skill.id),
                        stars.skill(&o.skill.id),
                    )
                })
                .collect(),
        })
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
        let profile_id = profile_id_of(id)?;
        let mut profile = self
            .profiles
            .update(
                profile_id,
                changes.name.as_deref(),
                changes.grade.map(Into::into),
                changes.character_id.as_deref(),
            )
            .map_err(profile_error)?;
        if changes.placement_seconds.is_some() || changes.exam_seconds.is_some() {
            profile = self
                .profiles
                .update_timers(profile_id, changes.placement_seconds, changes.exam_seconds)
                .map_err(profile_error)?;
        }
        Ok((&profile).into())
    }

    pub fn select_profile(&self, id: &str) -> Result<ProfileResponse, CommandError> {
        Ok((&self
            .profiles
            .select(profile_id_of(id)?)
            .map_err(profile_error)?)
            .into())
    }

    pub fn reroll_character(&self, id: &str) -> Result<ProfileResponse, CommandError> {
        Ok((&self
            .profiles
            .reroll_character(profile_id_of(id)?)
            .map_err(profile_error)?)
            .into())
    }

    pub fn delete_profile(&self, id: &str) -> Result<(), CommandError> {
        let profile = profile_id_of(id)?;
        self.profiles.delete(profile).map_err(profile_error)?;
        self.series.forget_profile(profile);
        Ok(())
    }

    pub fn start_series(
        &self,
        request: StartSeriesRequest,
    ) -> Result<SeriesResponse, CommandError> {
        let profile = profile_id_of(&request.profile_id)?;
        let series = self
            .series
            .start(
                profile,
                &request.skill_id,
                request.answer_mode.into(),
                request.format.map(Into::into),
                request.seed,
            )
            .map_err(start_error)?;
        Ok(self.prepared(&series))
    }

    /// `duration_ms`: time taken to answer, measured by the front (SUB001401).
    pub fn answer_series(
        &self,
        series_id: &str,
        index: u32,
        answer: AnswerPayload,
        duration_ms: Option<u32>,
    ) -> Result<SeriesAnswerResponse, CommandError> {
        let outcome = self
            .series
            .answer(
                series_id_of(series_id)?,
                index as usize,
                &answer.into(),
                duration_ms,
            )
            .map_err(|e| series_error(e, "Answer refused"))?;
        if outcome.summary.is_some() {
            self.preparer.release(series_id_of(series_id)?);
        }
        Ok(SeriesAnswerResponse {
            correct: outcome.correct,
            expected: AnswerPayload::from(&outcome.expected),
            expected_label: outcome.expected_label,
            explanation: outcome.explanation,
            help: outcome.help.as_ref().map(Into::into),
            correction: outcome.correction.as_ref().map(Into::into),
            summary: outcome
                .summary
                .as_ref()
                .map(|s| self.summary_with_rewards(s)),
        })
    }

    /// An exam of a subject (SUB001414) for the profile's grade, up to the current period.
    pub fn start_exam(
        &self,
        profile_id: &str,
        subject: Subject,
        seed: Option<u32>,
    ) -> Result<SeriesResponse, CommandError> {
        let current_period = self.calendar.period_of((self.today)());
        let series = self
            .series
            .start_exam(
                profile_id_of(profile_id)?,
                subject.into(),
                current_period,
                seed,
            )
            .map_err(start_error)?;
        Ok(self.prepared(&series))
    }

    /// The level assessment (done, to do) of each subject for a profile.
    pub fn placement_status(
        &self,
        profile_id: &str,
    ) -> Result<PlacementStatusResponse, CommandError> {
        let id = profile_id_of(profile_id)?;
        let profile = self.series.profile(id).map_err(profile_error)?;
        let current_period = self.calendar.period_of((self.today)());
        let mut subjects = Vec::new();
        for subject in [Subject::Math, Subject::Spelling] {
            subjects.push(PlacementSubjectResponse {
                subject,
                available: self
                    .series
                    .placement_available(id, subject.into(), current_period)
                    .map_err(profile_error)?,
                done: self
                    .series
                    .placement_done(id, subject.into())
                    .map_err(profile_error)?,
            });
        }
        Ok(PlacementStatusResponse {
            subjects,
            question_seconds: profile.placement_seconds,
        })
    }

    /// The level assessment of a subject (once finished, it cannot be taken again).
    pub fn start_placement(
        &self,
        profile_id: &str,
        subject: Subject,
        seed: Option<u32>,
    ) -> Result<SeriesResponse, CommandError> {
        let current_period = self.calendar.period_of((self.today)());
        let series = self
            .series
            .start_placement(
                profile_id_of(profile_id)?,
                subject.into(),
                current_period,
                seed,
            )
            .map_err(start_error)?;
        Ok(self.prepared(&series))
    }

    /// What the timed challenge offers to a profile: the mastered skills and the best scores.
    pub fn timed_challenge(
        &self,
        profile_id: &str,
        subject: Subject,
    ) -> Result<TimedChallengeResponse, CommandError> {
        let profile = profile_id_of(profile_id)?;
        let skills = self
            .series
            .timed_skills(profile, subject.into())
            .map_err(profile_error)?;
        let mut durations = Vec::new();
        for seconds in TIMED_DURATIONS {
            durations.push(TimedDurationResponse {
                seconds,
                best: self
                    .sessions
                    .best_timed_score(profile, subject.into(), seconds)
                    .map_err(unexpected)?,
            });
        }
        Ok(TimedChallengeResponse {
            subject,
            skill_count: u32::try_from(skills.len()).unwrap_or(u32::MAX),
            durations,
            default_seconds: DEFAULT_TIMED_DURATION,
        })
    }

    /// A timed challenge of `seconds` (SUB001415), ended by `finish_series`.
    pub fn start_timed(
        &self,
        profile_id: &str,
        subject: Subject,
        seconds: u32,
        seed: Option<u32>,
    ) -> Result<SeriesResponse, CommandError> {
        let series = self
            .series
            .start_timed(profile_id_of(profile_id)?, subject.into(), seconds, seed)
            .map_err(start_error)?;
        Ok(self.prepared(&series))
    }

    /// The end of a timed challenge: the questions answered count, without penalty.
    pub fn finish_series(&self, series_id: &str) -> Result<SeriesSummaryResponse, CommandError> {
        let id = series_id_of(series_id)?;
        let summary = self
            .series
            .finish(id)
            .map_err(|e| series_error(e, "Series already over"))?;
        self.preparer.release(id);
        Ok(self.summary_with_rewards(&summary))
    }

    /// The mascot and her pictures; `None` when the content has no valid mascot.
    pub fn mascot(&self) -> Option<MascotResponse> {
        let mascot = self.mascot.as_ref()?.mascot();
        Some(MascotResponse {
            name: mascot.name.clone(),
            pictures: Expression::ALL
                .into_iter()
                .filter_map(|e| {
                    mascot.pictures.get(&e).map(|svg| MascotPictureResponse {
                        expression: e.into(),
                        svg: svg.clone(),
                    })
                })
                .collect(),
        })
    }

    /// A phrase of the mascot for a situation (`title`: the lesson, at its start).
    pub fn mascot_phrase(
        &self,
        situation: MascotSituation,
        title: Option<&str>,
    ) -> Option<MascotPhraseResponse> {
        self.mascot
            .as_ref()?
            .phrase(situation.into(), title)
            .map(|p| MascotPhraseResponse {
                text: p.text,
                expression: p.expression.into(),
            })
    }

    /// The summary with the title and the lesson of each skill.
    fn summary(&self, summary: &SeriesSummary) -> SeriesSummaryResponse {
        SeriesSummaryResponse::new(summary, |skill_id| {
            let title = self
                .skills
                .find(skill_id)
                .map(|s| s.title.clone())
                .unwrap_or_else(|| skill_id.to_owned());
            let lesson = self
                .lessons
                .all()
                .iter()
                .find(|l| l.skill_ids.iter().any(|id| id == skill_id))
                .map(|l| l.id.clone());
            (title, lesson)
        })
    }

    /// The summary and what the session brought (lot 3): stars, level, goal of the day, badges,
    /// accessories. A failure of the rewards never hides the summary.
    fn summary_with_rewards(&self, summary: &SeriesSummary) -> SeriesSummaryResponse {
        let mut response = self.summary(summary);
        let Some(profile_id) = self
            .sessions
            .session(summary.series_id)
            .ok()
            .flatten()
            .map(|s| s.profile_id)
        else {
            return response;
        };
        match self.rewards.after_session(profile_id, summary) {
            Ok(rewards) => {
                response.profile_points = rewards.profile_points;
                response.rewards = Some(SessionRewardsResponse::new(
                    &rewards,
                    self.rewards.accessory_catalog(),
                    self.rewards.badge_list(),
                ));
            }
            Err(e) => log::error!("Rewards of session {}: {e}", summary.series_id),
        }
        response
    }

    /// The speed of the voice chosen by a profile (normal when unknown).
    fn voice_speed(&self, profile_id: Uuid) -> VoiceSpeed {
        self.settings
            .get(profile_id)
            .map(|s| s.voice_speed)
            .unwrap_or_default()
    }

    /// The series as sent to the front, its words being prepared in the background: each word,
    /// then its sentence (SUB026705), at the speed chosen by the child.
    fn prepared(&self, series: &Series) -> SeriesResponse {
        let voice = self.voice_speed(series.profile_id);
        self.preparer.prepare(
            series.id,
            adjusted(speech::readings_ahead(&series.exercises, 0), voice),
        );
        series.into()
    }

    /// The word of a spelling exercise in progress (`example`: its sentence; `slow`: « Plus
    /// lentement »), prepared ahead when it can be (SUB026705). The word itself never goes to the
    /// front before the answer.
    pub fn speak_series_word(
        &self,
        series_id: &str,
        index: u32,
        example: bool,
        slow: bool,
    ) -> Result<Vec<u8>, CommandError> {
        let id = series_id_of(series_id)?;
        let exercise = self
            .series
            .exercise(id, index as usize)
            .map_err(|e| series_error(e, "Unknown exercise"))?;
        let word = exercise.word.ok_or_else(|| {
            CommandError::new(400, "No word", "This exercise has no word to read.")
        })?;
        let voice = self
            .sessions
            .session(id)
            .ok()
            .flatten()
            .map(|s| self.voice_speed(s.profile_id))
            .unwrap_or_default();
        let mut reading = word_speech(&word, example, slow)?;
        reading.speed = voice.apply(reading.speed);
        let wav = self.preparer.read(id, &reading).map_err(speech_error)?;
        // The child goes on: the following words are prepared in turn.
        if let Ok(ahead) = self.series.readings_ahead(id, index as usize + 1) {
            self.preparer.prepare(id, adjusted(ahead, voice));
        }
        Ok(wav)
    }

    /// A word of the bank (lessons), or its example sentence, or the word read slowly.
    pub fn speak_word(
        &self,
        word: &str,
        example: bool,
        slow: bool,
        voice: VoiceSpeed,
    ) -> Result<Vec<u8>, CommandError> {
        let found = self.words.find(word).ok_or_else(|| {
            CommandError::new(
                404,
                "Unknown word",
                format!("No word '{word}' in the bank."),
            )
        })?;
        let reading = word_speech(&SpokenWord::of(found, None), example, slow)?;
        self.speech
            .synthesize(&reading.text, voice.apply(reading.speed))
            .map_err(speech_error)
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
        let id = series_id_of(series_id)?;
        // The words not read yet are not prepared any more.
        self.preparer.release(id);
        let summary = self
            .series
            .abandon(id)
            .map_err(|e| series_error(e, "Series already over"))?;
        Ok(self.summary_with_rewards(&summary))
    }

    /// The lessons of a grade (SUB001410). They follow the exercises: a grade without skills yet
    /// gets the CE1 lessons, like it gets the CE1 skills, so that « Je m'entraîne » always finds the
    /// skill of the lesson; a grade with skills but no lesson yet gets an empty list.
    pub fn lessons(
        &self,
        grade: Grade,
        subject: Subject,
        profile_id: Option<&str>,
    ) -> Result<LessonsResponse, CommandError> {
        let stars = match profile_id {
            Some(id) => self
                .rewards
                .best_stars(profile_id_of(id)?)
                .map_err(unexpected)?,
            None => Default::default(),
        };
        let current_period = self.calendar.period_of((self.today)());
        let (lessons_grade, _) = self
            .skills
            .offered_or_fallback(subject.into(), grade.into());
        let lessons = self.lessons.for_grade(subject.into(), lessons_grade);
        Ok(LessonsResponse {
            subject,
            grade,
            lessons_grade: lessons_grade.into(),
            current_period,
            lessons: lessons
                .iter()
                .map(|l| {
                    let main_skill = l.skill_ids.first().and_then(|id| self.skills.find(id));
                    LessonSummaryResponse::new(l, current_period, main_skill, stars.lesson(&l.id))
                })
                .collect(),
        })
    }

    /// A lesson, with the slide where the profile left it when `profile_id` is given.
    pub fn lesson(
        &self,
        id: &str,
        profile_id: Option<&str>,
    ) -> Result<LessonResponse, CommandError> {
        let lesson = self.lessons.find(id).ok_or_else(|| unknown_lesson(id))?;
        let mut response = LessonResponse::from(lesson);
        if let Some(profile) = profile_id {
            response.resume_slide = self
                .sessions
                .lesson_slide(profile_id_of(profile)?, id)
                .map_err(unexpected)?
                .filter(|slide| (*slide as usize) < lesson.slides.len());
        }
        Ok(response)
    }

    /// Keeps the slide where the profile is in a lesson (`None`: the lesson is finished).
    pub fn save_lesson_progress(
        &self,
        profile_id: &str,
        lesson_id: &str,
        slide: Option<u32>,
    ) -> Result<(), CommandError> {
        self.lessons
            .find(lesson_id)
            .ok_or_else(|| unknown_lesson(lesson_id))?;
        self.sessions
            .save_lesson_slide(profile_id_of(profile_id)?, lesson_id, slide)
            .map_err(unexpected)
    }

    /// The end-of-lesson quiz (SUB001411), answered with `answer_series` and recorded.
    pub fn start_lesson_quiz(
        &self,
        profile_id: &str,
        lesson_id: &str,
    ) -> Result<SeriesResponse, CommandError> {
        let lesson = self
            .lessons
            .find(lesson_id)
            .ok_or_else(|| unknown_lesson(lesson_id))?;
        let series = self
            .series
            .start_lesson_quiz(profile_id_of(profile_id)?, lesson)
            .map_err(start_error)?;
        Ok(self.prepared(&series))
    }

    /// A new « adult » gate (SUB001510); it replaces the one waiting.
    pub fn parent_gate(&self) -> ParentGateResponse {
        let (id, gate) = self.parent_gates.open();
        ParentGateResponse {
            id: id.to_string().to_uppercase(),
            left: gate.left,
            right: gate.right,
        }
    }

    /// Whether `answer` opens the gate `id`. The gate is used up either way.
    pub fn check_parent_gate(&self, id: &str, answer: u32) -> bool {
        self.parent_gates.check(id, answer)
    }

    /// The parents' card, created with default values the first time.
    pub fn parent_profile(&self) -> Result<ParentProfileResponse, CommandError> {
        let profile = self.parent_profile.get().map_err(profile_error)?;
        Ok((&profile).into())
    }

    pub fn update_parent_profile(
        &self,
        changes: UpdateParentProfileRequest,
    ) -> Result<ParentProfileResponse, CommandError> {
        let profile = self
            .parent_profile
            .update(changes.name.as_deref(), changes.character_id.as_deref())
            .map_err(profile_error)?;
        Ok((&profile).into())
    }

    /// Statistics of a profile for the parents' space (SUB001511).
    pub fn profile_statistics(
        &self,
        profile_id: &str,
    ) -> Result<ProfileStatisticsResponse, CommandError> {
        let profile = self
            .profiles
            .get(profile_id_of(profile_id)?)
            .map_err(profile_error)?;
        let sessions = self
            .sessions
            .profile_sessions(profile.id)
            .map_err(unexpected)?;
        let answers = self
            .sessions
            .profile_answers(profile.id)
            .map_err(unexpected)?;
        let statistics = profile_statistics(
            &sessions,
            &answers,
            &LocalCalendar {
                today: (self.today)(),
                date_of: &*self.local_date,
            },
        );
        Ok(ProfileStatisticsResponse::new(
            profile_id.to_owned(),
            &statistics,
            &|id| {
                self.skills
                    .find(id)
                    .map(|s| (s.title.clone(), s.grade.into()))
            },
        ))
    }

    // ── Lot 3 ────────────────────────────────────────────────────────────────────────────

    /// Level, goal of the day, badges and reviews due of a profile (the menu's panel).
    pub fn profile_overview(
        &self,
        profile_id: &str,
    ) -> Result<ProfileOverviewResponse, CommandError> {
        let id = profile_id_of(profile_id)?;
        let overview = self.rewards.overview(id).map_err(profile_error)?;
        let mut reviews = Vec::new();
        for subject in [Subject::Math, Subject::Spelling] {
            let in_subject = |skill_id: &str| {
                self.skills
                    .find(skill_id)
                    .is_some_and(|s| s.subject == subject.into())
            };
            let due = self.reviews.due(id, &in_subject).map_err(unexpected)?;
            reviews.push(ReviewCountResponse {
                subject,
                due: u32::try_from(due.len()).unwrap_or(u32::MAX),
            });
        }
        Ok(ProfileOverviewResponse::new(&overview, reviews))
    }

    /// Every badge, earned or not (« Mes badges », SUB001503).
    pub fn badges(&self, profile_id: &str) -> Result<Vec<BadgeResponse>, CommandError> {
        Ok(self
            .rewards
            .badges(profile_id_of(profile_id)?)
            .map_err(profile_error)?
            .iter()
            .map(Into::into)
            .collect())
    }

    /// Every accessory, unlocked or not (SUB001505).
    pub fn accessories(&self, profile_id: &str) -> Result<Vec<AccessoryResponse>, CommandError> {
        let states = self
            .rewards
            .accessories(profile_id_of(profile_id)?)
            .map_err(profile_error)?;
        Ok(states
            .iter()
            .map(|s| {
                AccessoryResponse::of_state(
                    s,
                    self.rewards.accessory_catalog(),
                    self.rewards.badge_list(),
                )
            })
            .collect())
    }

    /// Dresses the avatar of a profile; saved at once.
    pub fn update_avatar(
        &self,
        profile_id: &str,
        request: UpdateAvatarRequest,
    ) -> Result<ProfileResponse, CommandError> {
        let worn: Vec<WornAccessory> = request.accessories.iter().map(Into::into).collect();
        let profile = self
            .rewards
            .set_avatar(
                profile_id_of(profile_id)?,
                request.character_id.as_deref(),
                &worn,
            )
            .map_err(request_error)?;
        Ok((&profile).into())
    }

    /// An avatar as one SVG (the character and its accessories).
    pub fn avatar_svg(
        &self,
        character_id: &str,
        accessories: &[WornAccessoryResponse],
    ) -> Result<String, CommandError> {
        let worn: Vec<WornAccessory> = accessories.iter().map(Into::into).collect();
        self.rewards.avatar_svg(character_id, &worn).ok_or_else(|| {
            CommandError::new(
                404,
                "Unknown character",
                format!("No character with id {character_id}."),
            )
        })
    }

    /// « Je te conseille… » (SUB001508) for the exercises of a subject; `None` when everything
    /// up to the next period is mastered.
    pub fn advice(
        &self,
        profile_id: &str,
        subject: Subject,
    ) -> Result<Option<AdviceResponse>, CommandError> {
        let id = profile_id_of(profile_id)?;
        let profile = self.profiles.get(id).map_err(profile_error)?;
        let current_period = self.calendar.period_of((self.today)());
        let (_, offered) = self
            .skills
            .offered_or_fallback(subject.into(), profile.grade);
        let masteries = self.sessions.masteries(id).map_err(unexpected)?;
        Ok(
            advise(&self.adaptive, &offered, current_period, &masteries).map(|a| {
                AdviceResponse::new(&a, |skill_id| {
                    self.skills
                        .find(skill_id)
                        .map(|s| s.title.clone())
                        .unwrap_or_else(|| skill_id.to_owned())
                })
            }),
        )
    }

    /// « Mes révisions » of a subject (SUB001509).
    pub fn start_review(
        &self,
        profile_id: &str,
        subject: Subject,
        seed: Option<u32>,
    ) -> Result<SeriesResponse, CommandError> {
        let series = self
            .series
            .start_review(profile_id_of(profile_id)?, subject.into(), seed)
            .map_err(start_error)?;
        Ok(self.prepared(&series))
    }

    /// The settings of a child (SUB001512).
    pub fn profile_settings(
        &self,
        profile_id: &str,
    ) -> Result<ProfileSettingsResponse, CommandError> {
        let id = profile_id_of(profile_id)?;
        self.profiles.get(id).map_err(profile_error)?;
        Ok((&self.settings.get(id).map_err(unexpected)?).into())
    }

    pub fn update_profile_settings(
        &self,
        profile_id: &str,
        changes: UpdateProfileSettingsRequest,
    ) -> Result<ProfileSettingsResponse, CommandError> {
        let id = profile_id_of(profile_id)?;
        self.profiles.get(id).map_err(profile_error)?;
        let settings = self
            .settings
            .update(id, &changes.pairs())
            .map_err(request_error)?;
        Ok((&settings).into())
    }

    /// What the settings offer: the themes, the goals, the durations.
    pub fn settings_options(&self) -> SettingsOptionsResponse {
        SettingsOptionsResponse {
            themes: self.themes.iter().map(Into::into).collect(),
            goal_choices: GOAL_CHOICES.to_vec(),
            timed_choices: std::iter::once(0).chain(TIMED_CHOICES).collect(),
            session_choices: SESSION_MINUTES.to_vec(),
        }
    }

    /// Writes every profile to a backup file (SUB001512).
    pub fn export_backup(&self, path: &str) -> Result<BackupResponse, CommandError> {
        let text = educator_data::backup::export(&self.database).map_err(unexpected)?;
        std::fs::write(path, text)
            .map_err(|e| CommandError::new(500, "Backup not written", format!("{path}: {e}")))?;
        let profiles = self.profiles.list().map_err(unexpected)?.len();
        log::info!("Backup of {profiles} profile(s) written to {path}.");
        Ok(BackupResponse::new(profiles, path))
    }

    /// Replaces every profile by those of a backup file (a copy of the database is kept first).
    pub fn import_backup(&self, path: &str) -> Result<BackupResponse, CommandError> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| CommandError::new(400, "Backup not read", format!("{path}: {e}")))?;
        for profile in self.profiles.list().map_err(unexpected)? {
            self.series.forget_profile(profile.id);
        }
        let report =
            educator_data::backup::restore(&self.database, &text).map_err(request_error)?;
        log::info!(
            "Backup {path} restored: {} profile(s); previous database kept in {}.",
            report.profiles,
            report.safety_copy.display()
        );
        Ok(BackupResponse::new(report.profiles, path))
    }

    /// WAV of a French text; numbers and calculation signs are read as words (« 7 + 5 » → « sept plus cinq »).
    /// `voice`: the speed chosen by the child, applied to the instructions' one.
    pub fn speak(
        &self,
        text: &str,
        speed: Option<f64>,
        voice: VoiceSpeed,
    ) -> Result<Vec<u8>, CommandError> {
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

        self.speech
            .synthesize(&text, voice.apply(speed))
            .map_err(speech_error)
    }
}

/// Readings at the speed chosen by the child.
fn adjusted(readings: Vec<speech::Reading>, voice: VoiceSpeed) -> Vec<speech::Reading> {
    readings
        .into_iter()
        .map(|mut r| {
            r.speed = voice.apply(r.speed);
            r
        })
        .collect()
}

fn listening_of(speech: &dyn SpeechSynthesizer) -> Listening {
    if speech.status().is_available {
        Listening::Available
    } else {
        Listening::Unavailable
    }
}

/// What the voice reads for a word, or 404 when it has no sentence.
fn word_speech(
    word: &SpokenWord,
    example: bool,
    slow: bool,
) -> Result<speech::Reading, CommandError> {
    word.speech(example, slow).ok_or_else(|| {
        CommandError::new(
            404,
            "No example",
            format!("No example sentence for '{}'.", word.word),
        )
    })
}

fn speech_error(e: Error) -> CommandError {
    match e {
        Error::SpeechUnavailable(message) => CommandError::new(503, "Speech unavailable", message),
        other => unexpected(other),
    }
}

fn parse_id(id: &str, title: &str, what: &str) -> Result<Uuid, CommandError> {
    Uuid::parse_str(id)
        .map_err(|_| CommandError::new(404, title, format!("No {what} with id {id}.")))
}

fn profile_id_of(id: &str) -> Result<Uuid, CommandError> {
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

/// A request the core refuses (an accessory still locked, an unknown setting, a bad backup file).
fn request_error(error: Error) -> CommandError {
    match error {
        Error::InvalidRequest(message) => CommandError::new(400, "Invalid request", message),
        other => profile_error(other),
    }
}

fn start_error(error: Error) -> CommandError {
    match error {
        Error::SeriesRequest(message) => CommandError::new(400, "Invalid series request", message),
        other => profile_error(other),
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
        fixture_with_voice(Arc::new(UnavailableSpeechSynthesizer::new("tests")))
    }

    fn fixture_with_voice(speech: Arc<dyn SpeechSynthesizer>) -> Fixture {
        let dir = tempfile::tempdir().unwrap();
        let locations = Locations {
            content_directory: Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content"),
            database_path: dir.path().join("educator.db"),
        };
        let services = Services::start(
            &locations,
            speech,
            Arc::new(ScriptedRandom::new([3])),
            Arc::new(ManualClock::new(unix_ms(2026, 10, 11, 8, 0, 0))),
            Box::new(|| Date::new(2026, 10, 11)),
            Arc::new(educator_core::statistics::utc_date_of),
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
        let skills = fixture()
            .services
            .skills(Grade::CP, Subject::Math, None)
            .unwrap();

        assert_eq!(Grade::CP, skills.skills_grade);
        assert_eq!(1, skills.current_period);
        assert!(skills.skills.windows(2).all(|w| w[0].period <= w[1].period));
        assert!(skills.skills.iter().any(|s| s.is_current_period));
    }

    #[test]
    fn ce1_gets_the_cp_revisions_after_its_own_first_period() {
        let skills = fixture()
            .services
            .skills(Grade::CE1, Subject::Math, None)
            .unwrap();
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
        let skills = fixture()
            .services
            .skills(Grade::CM2, Subject::Math, None)
            .unwrap();

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
                format: None,
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
                    placement_seconds: None,
                    exam_seconds: None,
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
        let skills = f
            .services
            .skills(updated.grade, Subject::Math, None)
            .unwrap();
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

    /// SUB001510: the id sent to the front opens its gate once (the rules: educator-core).
    #[test]
    fn the_parents_gate_round_trips_through_the_front() {
        let f = fixture();
        let gate = f.services.parent_gate();
        assert!((12..=99).contains(&gate.left) && (3..=9).contains(&gate.right));
        assert_eq!(gate.id.to_uppercase(), gate.id);

        assert!(f
            .services
            .check_parent_gate(&gate.id, gate.left * gate.right));
        assert!(!f
            .services
            .check_parent_gate(&gate.id, gate.left * gate.right));
    }

    /// The parents' card: default values, then a change kept in the database.
    #[test]
    fn the_parents_card_is_created_then_edited() {
        let f = fixture();

        let card = f.services.parent_profile().unwrap();
        assert_eq!(
            ("Parents", "ours"),
            (card.name.as_str(), card.character_id.as_str())
        );

        let card = f
            .services
            .update_parent_profile(UpdateParentProfileRequest {
                name: Some("Papa".to_owned()),
                character_id: Some("lion".to_owned()),
            })
            .unwrap();
        assert_eq!(
            ("Papa", "lion"),
            (card.name.as_str(), card.character_id.as_str())
        );
        assert_eq!("Papa", f.services.parent_profile().unwrap().name);
        // The card is not a child: the profile list stays empty.
        assert!(f.services.list_profiles().unwrap().is_empty());

        let error = f
            .services
            .update_parent_profile(UpdateParentProfileRequest {
                name: Some(" ".to_owned()),
                character_id: None,
            })
            .unwrap_err();
        assert_eq!((400, Some("name")), (error.status, error.field.as_deref()));
    }

    /// SUB001511: a profile without any session, then after a series.
    #[test]
    fn the_statistics_of_a_profile_follow_its_series() {
        let f = fixture();
        let lea = create(&f.services, "Léa");

        let empty = f.services.profile_statistics(&lea.id).unwrap();
        assert_eq!(lea.id, empty.profile_id);
        assert_eq!(0, empty.summary.session_count);
        assert_eq!(None, empty.summary.success_rate);
        assert_eq!(14, empty.days.len());
        assert_eq!("2026-10-11", empty.days.last().unwrap().date);
        assert_eq!(8, empty.weeks.len());
        assert_eq!("2026-10-05", empty.weeks.last().unwrap().start);
        assert!(empty.skills.is_empty() && empty.tests.is_empty());
        assert!(empty.frequent_mistakes.is_empty() && empty.to_review.is_empty());

        let series = f
            .services
            .start_series(StartSeriesRequest {
                profile_id: lea.id.clone(),
                skill_id: "cp-add-sum-10".to_owned(),
                answer_mode: AnswerMode::Keyboard,
                format: None,
                seed: Some(42),
            })
            .unwrap();
        for exercise in &series.exercises {
            f.services
                .answer_series(&series.series_id, exercise.index, number(-1), None)
                .unwrap();
        }

        let statistics = f.services.profile_statistics(&lea.id).unwrap();
        assert_eq!(1, statistics.summary.session_count);
        assert_eq!(20, statistics.summary.points_earned);
        assert_eq!(Some(0), statistics.summary.success_rate);
        assert_eq!(1, statistics.days.last().unwrap().session_count);
        let skill = &statistics.skills[0];
        assert_eq!("cp-add-sum-10", skill.skill_id);
        assert_ne!(skill.skill_id, skill.title);
        assert_eq!(Some(Grade::CP), skill.grade);
        assert_eq!(
            (1, 0, 10),
            (
                skill.completed_count,
                skill.abandoned_count,
                skill.answer_count
            )
        );
        assert_eq!(MasteryStateName::ToReview, skill.state);
        assert_eq!(
            vec![20],
            skill
                .history
                .iter()
                .map(|p| p.points_earned)
                .collect::<Vec<_>>()
        );
        assert_eq!(vec!["cp-add-sum-10".to_owned()], statistics.to_review);
        let mistake = &statistics.frequent_mistakes[0];
        assert_eq!("-1", mistake.usual_answer);
        assert_eq!(skill.title, mistake.skill_title);

        let unknown = f
            .services
            .profile_statistics("0192F6B4-0000-7000-8000-000000000000")
            .unwrap_err();
        assert_eq!(404, unknown.status);
        assert_eq!(
            404,
            f.services.profile_statistics("nope").unwrap_err().status
        );
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
                format: None,
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
                .answer_series(&series.series_id, exercise.index, number(-1), None)
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
                format: None,
                seed: None,
            })
        };

        assert_eq!(400, start("unknown").unwrap_err().status);
        let series = start("cp-add-sum-10").unwrap();
        f.services
            .answer_series(&series.series_id, 0, number(1), None)
            .unwrap();
        assert_eq!(
            409,
            f.services
                .answer_series(&series.series_id, 0, number(1), None)
                .unwrap_err()
                .status
        );
        let summary = f.services.abandon_series(&series.series_id).unwrap();
        assert_eq!(10, summary.exercise_count);
        assert_eq!(
            404,
            f.services
                .answer_series(&series.series_id, 1, number(1), None)
                .unwrap_err()
                .status
        );
    }

    /// SUB001406, SUB001405: a dictation never sends the word, corrects letter by letter and reads
    /// the word with the voice.
    #[test]
    fn a_dictation_hides_the_word_and_corrects_letter_by_letter() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        let series = f
            .services
            .start_series(StartSeriesRequest {
                profile_id: lea.id.clone(),
                skill_id: "ce1-son-o-au-eau".to_owned(),
                answer_mode: AnswerMode::Keyboard,
                format: Some(SpellingFormat::Dictation),
                seed: Some(5),
            })
            .unwrap();

        assert_eq!(Subject::Spelling, series.subject);
        let exercise = &series.exercises[0];
        assert!(matches!(exercise.input, InputResponse::Spelling { .. }));
        assert_eq!("Dictée", exercise.prompt);
        assert!(exercise.spoken_prompt.is_empty());
        let word = exercise.word.as_ref().unwrap();
        assert_eq!((None, None), (word.before.clone(), word.after.clone()));

        // The voice is unavailable in these tests.
        assert_eq!(
            503,
            f.services
                .speak_series_word(&series.series_id, 0, false, false)
                .unwrap_err()
                .status
        );

        let outcome = f
            .services
            .answer_series(
                &series.series_id,
                0,
                AnswerPayload::Text {
                    value: "zz".to_owned(),
                },
                Some(3200),
            )
            .unwrap();
        assert!(!outcome.correct);
        let correction = outcome.correction.unwrap();
        assert!(!correction.correct);
        assert!(correction
            .letters
            .iter()
            .any(|l| l.kind != LetterKindName::Right));
        let AnswerPayload::Text { value: expected } = &outcome.expected else {
            panic!("a word is expected");
        };
        // The word was never in the series sent to the front.
        let json = serde_json::to_string(&series).unwrap();
        assert!(!json.contains(&format!("\"{expected}\"")), "{json}");

        // Any written answer gets its letter-by-letter correction.
        let right = f
            .services
            .answer_series(
                &series.series_id,
                1,
                AnswerPayload::Text {
                    value: "x".to_owned(),
                },
                None,
            )
            .unwrap();
        assert!(right.correction.is_some());
    }

    /// A voice that answers the text and the speed it was asked, and counts the requests.
    #[derive(Default)]
    struct EchoVoice(std::sync::Mutex<Vec<(String, f64)>>);

    impl SpeechSynthesizer for EchoVoice {
        fn status(&self) -> educator_core::speech::SpeechEngineStatus {
            educator_core::speech::SpeechEngineStatus::available("echo")
        }

        fn synthesize(&self, text: &str, speed: f64) -> educator_core::Result<Vec<u8>> {
            self.0.lock().unwrap().push((text.to_owned(), speed));
            Ok(format!("{speed}|{text}").into_bytes())
        }

        fn warm_up(&self, _speed: f64) {}
    }

    /// SUB026705, SUB026706, SUB026702: the words of a dictation are prepared (each word, then its
    /// sentence, at 0.85), a noun is read with its article, served without a new synthesis; « Plus
    /// lentement » reads it at 0.5.
    #[test]
    fn the_words_of_a_dictation_are_prepared_and_read_with_their_article() {
        let voice = Arc::new(EchoVoice::default());
        let f = fixture_with_voice(voice.clone());
        let lea = create(&f.services, "Léa");
        let series = f
            .services
            .start_series(StartSeriesRequest {
                profile_id: lea.id.clone(),
                skill_id: "ce1-son-o-au-eau".to_owned(),
                answer_mode: AnswerMode::Keyboard,
                format: Some(SpellingFormat::Dictation),
                seed: Some(5),
            })
            .unwrap();
        let expected = 2 * series.exercises.len();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while voice.0.lock().unwrap().len() < expected {
            assert!(std::time::Instant::now() < deadline, "not prepared");
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        let asked = voice.0.lock().unwrap().clone();
        assert!(asked
            .iter()
            .all(|(_, speed)| *speed == speech::speed::WORDS));
        // Word, sentence, word, sentence…: a noun with its article and a period.
        let first = &asked[0].0;
        assert!(
            first.starts_with("un ") || first.starts_with("une "),
            "{first}"
        );
        assert!(first.ends_with('.'), "{first}");
        let word = f
            .services
            .speak_series_word(&series.series_id, 0, false, false)
            .unwrap();
        assert_eq!(format!("0.85|{first}").into_bytes(), word);
        f.services
            .speak_series_word(&series.series_id, 0, true, false)
            .unwrap();
        assert_eq!(expected, voice.0.lock().unwrap().len(), "served prepared");
        // The article is shown before the box; the word itself is not sent.
        let article = series.exercises[0].word.as_ref().unwrap().article.clone();
        assert_eq!(Some(first.split(' ').next().unwrap().to_owned()), article);

        let slow = f
            .services
            .speak_series_word(&series.series_id, 0, false, true)
            .unwrap();
        assert_eq!(format!("0.5|{first}").into_bytes(), slow);

        f.services.abandon_series(&series.series_id).unwrap();
    }

    #[test]
    fn spelling_skills_have_their_kinds_of_exercise_and_the_mastery_follows_the_series() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        let skills = f
            .services
            .skills(Grade::CP, Subject::Spelling, Some(&lea.id))
            .unwrap();
        assert_eq!(Subject::Spelling, skills.subject);
        assert!(skills
            .skills
            .iter()
            .all(|s| s.family == SkillFamily::Spelling));
        let ou = skills.skills.iter().find(|s| s.id == "cp-son-ou").unwrap();
        assert_eq!(
            vec![
                SpellingFormat::Dictation,
                SpellingFormat::MissingLetters,
                SpellingFormat::ChooseSpelling
            ],
            ou.spelling_formats
        );
        assert_eq!(None, ou.mastery);

        let series = f
            .services
            .start_series(StartSeriesRequest {
                profile_id: lea.id.clone(),
                skill_id: "cp-son-ou".to_owned(),
                answer_mode: AnswerMode::Choice,
                format: Some(SpellingFormat::ChooseSpelling),
                seed: Some(8),
            })
            .unwrap();
        for exercise in &series.exercises {
            assert_eq!(InputResponse::Pick, exercise.input);
            assert_eq!(3, exercise.choices.as_ref().unwrap().len());
        }
        // Wrong format for the mode.
        assert_eq!(
            400,
            f.services
                .start_series(StartSeriesRequest {
                    profile_id: lea.id.clone(),
                    skill_id: "cp-mots-outils".to_owned(),
                    answer_mode: AnswerMode::Keyboard,
                    format: Some(SpellingFormat::MissingLetters),
                    seed: None,
                })
                .unwrap_err()
                .status
        );
        let summary = f.services.abandon_series(&series.series_id).unwrap();
        assert!(summary.skills.is_empty());

        let series = f
            .services
            .start_series(StartSeriesRequest {
                profile_id: lea.id.clone(),
                skill_id: "cp-son-ou".to_owned(),
                answer_mode: AnswerMode::Choice,
                format: Some(SpellingFormat::ChooseSpelling),
                seed: Some(8),
            })
            .unwrap();
        let mut summary = None;
        for exercise in &series.exercises {
            let choice = exercise.choices.as_ref().unwrap()[0].answer.clone();
            summary = f
                .services
                .answer_series(&series.series_id, exercise.index, choice, Some(1000))
                .unwrap()
                .summary;
        }
        let summary = summary.unwrap();
        assert_eq!(1, summary.skills.len());
        assert_eq!("cp-son-ou", summary.skills[0].skill_id);
        assert_eq!(10, summary.skills[0].question_count);
        let mastery = f
            .services
            .skills(Grade::CP, Subject::Spelling, Some(&lea.id))
            .unwrap()
            .skills
            .into_iter()
            .find(|s| s.id == "cp-son-ou")
            .unwrap()
            .mastery;
        assert_eq!(
            Some(((summary.correct_count * 100 + 5) / 10).min(100) as u8),
            mastery
        );
    }

    #[test]
    fn a_session_left_open_is_abandoned_at_the_next_start() {
        let dir = tempfile::tempdir().unwrap();
        let locations = Locations {
            content_directory: Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content"),
            database_path: dir.path().join("educator.db"),
        };
        let start = || {
            Services::start(
                &locations,
                Arc::new(UnavailableSpeechSynthesizer::new("tests")),
                Arc::new(ScriptedRandom::new([3])),
                Arc::new(ManualClock::new(unix_ms(2026, 10, 11, 8, 0, 0))),
                Box::new(|| Date::new(2026, 10, 11)),
                Arc::new(educator_core::statistics::utc_date_of),
            )
            .unwrap()
        };
        let services = start();
        let lea = create(&services, "Léa");
        let series = services
            .start_series(StartSeriesRequest {
                profile_id: lea.id.clone(),
                skill_id: "cp-add-sum-10".to_owned(),
                answer_mode: AnswerMode::Keyboard,
                format: None,
                seed: Some(1),
            })
            .unwrap();
        services
            .answer_series(&series.series_id, 0, number(-1), None)
            .unwrap();
        drop(services);

        let services = start();
        let id = Uuid::parse_str(&series.series_id).unwrap();
        let session = services.sessions.session(id).unwrap().unwrap();
        assert!(session.abandoned);
        assert!(session.ended_at.is_some());
        assert_eq!(1, services.sessions.answers(id).unwrap().len());
    }

    /// SUB001414: 15 questions of the grade, the results by skill with their lesson.
    #[test]
    fn an_exam_gives_the_results_by_skill() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        for subject in [Subject::Math, Subject::Spelling] {
            let exam = f.services.start_exam(&lea.id, subject, Some(4)).unwrap();
            assert_eq!(SessionType::Exam, exam.session_type);
            assert_eq!(15, exam.exercises.len());
            let mut summary = None;
            for exercise in &exam.exercises {
                summary = f
                    .services
                    .answer_series(&exam.series_id, exercise.index, number(-1), None)
                    .unwrap()
                    .summary;
            }
            let summary = summary.unwrap();
            assert_eq!(15, summary.exercise_count);
            assert!(!summary.skills.is_empty());
            assert!(summary.skills.iter().all(|s| !s.strong));
            assert_eq!(
                15,
                summary.skills.iter().map(|s| s.question_count).sum::<u32>()
            );
            // Period 1 (11 October): the CP skills of period 1, which have their lessons.
            assert!(summary.skills.iter().all(|s| s.skill_id.starts_with("cp-")));
            assert!(summary.skills.iter().any(|s| s.lesson_id.is_some()));
        }
    }

    /// SUB001418: one level assessment per subject, valid once finished, a timer on every question.
    #[test]
    fn the_level_assessment_is_offered_once_per_subject_and_counts_when_finished() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        let state = |services: &Services| {
            services
                .placement_status(&lea.id)
                .unwrap()
                .subjects
                .iter()
                .map(|s| (s.subject, s.available, s.done))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            20,
            f.services
                .placement_status(&lea.id)
                .unwrap()
                .question_seconds
        );
        assert_eq!(
            vec![
                (Subject::Math, true, false),
                (Subject::Spelling, true, false)
            ],
            state(&f.services)
        );

        let placement = f
            .services
            .start_placement(&lea.id, Subject::Math, Some(1))
            .unwrap();
        assert_eq!(SessionType::Placement, placement.session_type);
        assert_eq!(Some(20), placement.question_seconds);
        assert!(!placement.exercises.is_empty() && placement.exercises.len() <= 30);
        // Abandoned: still to do.
        f.services.abandon_series(&placement.series_id).unwrap();
        assert_eq!(
            vec![
                (Subject::Math, true, false),
                (Subject::Spelling, true, false)
            ],
            state(&f.services)
        );

        // Every question out of time: the assessment is over, wrong answers count.
        let placement = f
            .services
            .start_placement(&lea.id, Subject::Math, Some(2))
            .unwrap();
        let mut summary = None;
        for exercise in &placement.exercises {
            let outcome = f
                .services
                .answer_series(
                    &placement.series_id,
                    exercise.index,
                    AnswerPayload::TimedOut,
                    Some(20_000),
                )
                .unwrap();
            assert!(!outcome.correct);
            summary = outcome.summary;
        }
        let summary = summary.unwrap();
        assert_eq!((0, 0), (summary.correct_count, summary.points_earned));
        assert_eq!(
            vec![
                (Subject::Math, false, true),
                (Subject::Spelling, true, false)
            ],
            state(&f.services)
        );
        assert_eq!(
            400,
            f.services
                .start_placement(&lea.id, Subject::Math, Some(3))
                .unwrap_err()
                .status
        );
        // It feeds the statistics of the parents' space.
        let statistics = f.services.profile_statistics(&lea.id).unwrap();
        assert!(statistics.summary.answer_count > 0);
        assert!(statistics
            .tests
            .iter()
            .any(|t| t.session_type == SessionType::Placement));
    }

    /// SUB001418: the timers of the assessment and of the exam are options of the profile.
    #[test]
    fn the_timers_of_a_profile_are_options_that_the_exam_follows() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        assert_eq!((20, 0), (lea.placement_seconds, lea.exam_seconds));
        let exam = f
            .services
            .start_exam(&lea.id, Subject::Math, Some(4))
            .unwrap();
        assert_eq!(None, exam.question_seconds);

        let updated = f
            .services
            .update_profile(
                &lea.id,
                UpdateProfileRequest {
                    placement_seconds: Some(45),
                    exam_seconds: Some(30),
                    ..Default::default()
                },
            )
            .unwrap();

        assert_eq!((45, 30), (updated.placement_seconds, updated.exam_seconds));
        let exam = f
            .services
            .start_exam(&lea.id, Subject::Math, Some(4))
            .unwrap();
        assert_eq!(Some(30), exam.question_seconds);
        let error = f
            .services
            .update_profile(
                &lea.id,
                UpdateProfileRequest {
                    exam_seconds: Some(25),
                    ..Default::default()
                },
            )
            .unwrap_err();
        assert_eq!(
            (400, Some("examSeconds".to_owned())),
            (error.status, error.field)
        );
    }

    /// SUB001415: offered once a skill is mastered, best score per duration.
    #[test]
    fn the_timed_challenge_opens_with_a_mastered_skill() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        let challenge = f.services.timed_challenge(&lea.id, Subject::Math).unwrap();
        assert_eq!(0, challenge.skill_count);
        assert_eq!(
            vec![60, 120, 180],
            challenge
                .durations
                .iter()
                .map(|d| d.seconds)
                .collect::<Vec<_>>()
        );
        assert_eq!(120, challenge.default_seconds);
        assert_eq!(
            400,
            f.services
                .start_timed(&lea.id, Subject::Math, 120, Some(1))
                .unwrap_err()
                .status
        );

        // A series all right: the right answers are found by another profile trying the same series.
        let tom = create(&f.services, "Tom");
        let series = f
            .services
            .start_series(StartSeriesRequest {
                profile_id: tom.id.clone(),
                skill_id: "cp-add-sum-10".to_owned(),
                answer_mode: AnswerMode::Choice,
                format: None,
                seed: Some(2),
            })
            .unwrap();
        let replay = f
            .services
            .start_series(StartSeriesRequest {
                profile_id: lea.id.clone(),
                skill_id: "cp-add-sum-10".to_owned(),
                answer_mode: AnswerMode::Choice,
                format: None,
                seed: Some(2),
            })
            .unwrap();
        for exercise in &series.exercises {
            let expected = f
                .services
                .answer_series(&series.series_id, exercise.index, number(-1), None)
                .unwrap()
                .expected;
            f.services
                .answer_series(&replay.series_id, exercise.index, expected, None)
                .unwrap();
        }

        let challenge = f.services.timed_challenge(&lea.id, Subject::Math).unwrap();
        assert_eq!(1, challenge.skill_count);
        let timed = f
            .services
            .start_timed(&lea.id, Subject::Math, 60, Some(1))
            .unwrap();
        assert_eq!(Some(60), timed.time_limit_seconds);
        f.services
            .answer_series(&timed.series_id, 0, number(-1), None)
            .unwrap();
        let summary = f.services.finish_series(&timed.series_id).unwrap();
        assert_eq!((0, 1), (summary.correct_count, summary.exercise_count));
        let challenge = f.services.timed_challenge(&lea.id, Subject::Math).unwrap();
        assert_eq!(Some(0), challenge.durations[0].best);
        assert_eq!(None, challenge.durations[1].best);
    }

    /// SUB001417: the mascot, her four expressions, a phrase per situation.
    #[test]
    fn the_mascot_speaks_with_the_expression_of_the_situation() {
        let f = fixture();
        let mascot = f.services.mascot().unwrap();
        assert_eq!("Plume", mascot.name);
        assert_eq!(4, mascot.pictures.len());
        assert!(mascot.pictures.iter().all(|p| p.svg.contains("<svg")));

        let start = f
            .services
            .mascot_phrase(MascotSituation::LessonStart, Some("Le son [o]"))
            .unwrap();
        assert!(start.text.contains("Le son [o]"), "{}", start.text);
        assert_eq!(MascotExpression::Happy, start.expression);
        let party = f
            .services
            .mascot_phrase(MascotSituation::SeriesPerfect, None)
            .unwrap();
        assert_eq!(MascotExpression::Party, party.expression);
    }

    #[test]
    fn speech_checks_the_request_and_reports_a_missing_voice() {
        let f = fixture();

        assert_eq!(
            400,
            f.services
                .speak("", None, VoiceSpeed::Normal)
                .unwrap_err()
                .status
        );
        assert_eq!(
            400,
            f.services
                .speak("Bonjour", Some(3.0), VoiceSpeed::Normal)
                .unwrap_err()
                .status
        );
        let error = f
            .services
            .speak("7 + 5", None, VoiceSpeed::Normal)
            .unwrap_err();
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
                        format: None,
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
                            None,
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
                format: None,
                seed: Some(11),
            })
            .unwrap();
        let first = &series.exercises[0];
        let wrong = f
            .services
            .answer_series(
                &series.series_id,
                0,
                AnswerPayload::Number { value: -5 },
                None,
            )
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
        let lessons = fixture()
            .services
            .lessons(Grade::CE1, Subject::Math, None)
            .unwrap();

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
        let lessons = fixture()
            .services
            .lessons(Grade::CM1, Subject::Math, None)
            .unwrap();

        assert_eq!(Grade::CM1, lessons.grade);
        assert_eq!(Grade::CE1, lessons.lessons_grade);
        assert!(!lessons.lessons.is_empty());
    }

    /// CP has its own skills, and its own lessons: no CE1 lesson, whose « Je m'entraîne » would
    /// lead to a skill the CP profile is not offered.
    #[test]
    fn cp_gets_its_own_lessons() {
        let f = fixture();
        let lessons = f.services.lessons(Grade::CP, Subject::Math, None).unwrap();

        assert_eq!(Grade::CP, lessons.lessons_grade);
        assert_eq!(11, lessons.lessons.len());
        for summary in &lessons.lessons {
            assert!(summary.id.starts_with("cp-"), "{}", summary.id);
            assert_eq!(
                Grade::CP,
                f.services.lesson(&summary.id, None).unwrap().grade
            );
        }
    }

    #[test]
    fn the_skill_of_every_lesson_is_offered_where_the_lesson_is() {
        let f = fixture();
        for grade in [Grade::CP, Grade::CE1, Grade::CE2, Grade::CM1, Grade::CM2] {
            let skills = f.services.skills(grade, Subject::Math, None).unwrap();
            for summary in f
                .services
                .lessons(grade, Subject::Math, None)
                .unwrap()
                .lessons
            {
                let lesson = f.services.lesson(&summary.id, None).unwrap();
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
        let lesson = f.services.lesson("ce1-add-9-19-29", None).unwrap();

        assert_eq!(vec!["ce1-add-9-19-29".to_owned()], lesson.skill_ids);
        assert_eq!(10, lesson.slides.len());
        assert_eq!(2, lesson.slides.iter().filter(|s| s.interactive).count());
        let AnimationResponse::TensUnits {
            value,
            hundreds,
            tens,
            units,
            missing,
            steps,
            ..
        } = &lesson.slides[2].animation
        else {
            panic!("slide 3 is base ten blocks");
        };
        assert_eq!(
            (19, 0, 1, 9, 1),
            (*value, *hundreds, *tens, *units, *missing)
        );
        assert_eq!(1, steps.len());
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
        assert_eq!(404, f.services.lesson("nothing", None).unwrap_err().status);
    }

    /// SUB000137: the written subtraction comes with its columns computed by the core, one layout per
    /// technique; the checking equation with its family.
    #[test]
    fn written_operations_and_equations_come_computed() {
        let f = fixture();
        let lesson = f.services.lesson("ce1-sub-2-digits", None).unwrap();

        let AnimationResponse::ColumnOperation {
            operator,
            result,
            layouts,
            computed_columns,
            interactive,
            ..
        } = &lesson.slides[7].animation
        else {
            panic!("slide 8 is a written operation");
        };
        assert_eq!(
            (LessonOperator::Subtract, 46, 2, false),
            (*operator, *result, *computed_columns, *interactive)
        );
        let methods: Vec<Option<SubtractionMethod>> = layouts.iter().map(|l| l.method).collect();
        assert_eq!(
            vec![
                Some(SubtractionMethod::Cassage),
                Some(SubtractionMethod::Compensation)
            ],
            methods
        );
        assert_eq!(vec![14, 8], layouts[0].columns[0].terms);
        assert_eq!(6, layouts[0].columns[1].rewrites[0].value);
        assert!(layouts[1].columns[0].plus_ten);
        assert_eq!(1, layouts[1].columns[1].carry);

        let AnimationResponse::Equation {
            missing_value,
            equalities,
            choices,
            ..
        } = &lesson.slides[9].animation
        else {
            panic!("slide 10 is an equation");
        };
        assert_eq!(Some(74), *missing_value);
        assert_eq!(4, equalities.len());
        assert_eq!(vec![64, 74, 84], *choices);

        let json = serde_json::to_value(&lesson).unwrap();
        assert_eq!("columnOperation", json["slides"][7]["animation"]["type"]);
        assert_eq!(
            "Cassage",
            json["slides"][7]["animation"]["layouts"][0]["method"]
        );
        assert_eq!("equation", json["slides"][9]["animation"]["type"]);
        assert_eq!("Result", json["slides"][9]["animation"]["missing"]);
    }

    /// SUB001411: the quiz is a series of the lesson, recorded, with points; a word is typed.
    #[test]
    fn the_quiz_is_recorded_with_its_points_and_explanations() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        let quiz = f
            .services
            .start_lesson_quiz(&lea.id, "ce1-add-9-19-29")
            .unwrap();
        assert_eq!(SessionType::LessonQuiz, quiz.session_type);
        assert_eq!(4, quiz.exercises.len());
        assert_eq!(InputResponse::Number { slots: 1 }, quiz.exercises[0].input);

        let right = f
            .services
            .answer_series(&quiz.series_id, 0, number(37), Some(4000))
            .unwrap();
        assert!(right.correct);
        assert!(right.explanation.contains("28 + 10 = 38"));
        let wrong = f
            .services
            .answer_series(&quiz.series_id, 1, number(66), None)
            .unwrap();
        assert!(!wrong.correct);
        assert_eq!(number(65), wrong.expected);
        let mut summary = None;
        for index in 2..4 {
            summary = f
                .services
                .answer_series(&quiz.series_id, index, number(0), None)
                .unwrap()
                .summary;
        }
        let summary = summary.unwrap();
        assert_eq!((1, 4), (summary.correct_count, summary.exercise_count));
        assert_eq!(10 + 20, summary.points_earned);
        // Fewer than half right: « Revoir la leçon » first.
        assert!(summary.review_lesson && !summary.new_record);
        assert_eq!(
            Some("ce1-add-9-19-29"),
            summary.skills[0].lesson_id.as_deref()
        );
        let session = f
            .services
            .sessions
            .session(Uuid::parse_str(&quiz.series_id).unwrap())
            .unwrap()
            .unwrap();
        assert_eq!(Some("ce1-add-9-19-29"), session.lesson_id.as_deref());

        // A spelling quiz: the word to write is read like a dictation and corrected letter by letter.
        let spelling = f
            .services
            .start_lesson_quiz(&lea.id, "ce1-son-o-au-eau")
            .unwrap();
        let typed = spelling
            .exercises
            .iter()
            .find(|e| matches!(e.input, InputResponse::Spelling { .. }))
            .unwrap();
        assert!(typed.word.is_some());
        let outcome = f
            .services
            .answer_series(
                &spelling.series_id,
                typed.index,
                AnswerPayload::Text {
                    value: "velo".to_owned(),
                },
                None,
            )
            .unwrap();
        assert!(!outcome.correct);
        assert!(!outcome.correction.unwrap().correct);
        assert_eq!(
            404,
            f.services
                .start_lesson_quiz(&lea.id, "nothing")
                .unwrap_err()
                .status
        );
    }

    /// SUB001410: a lesson left in the middle starts again at its slide.
    #[test]
    fn a_lesson_left_in_the_middle_resumes_at_its_slide() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        assert_eq!(
            None,
            f.services
                .lesson("cp-son-ou", Some(&lea.id))
                .unwrap()
                .resume_slide
        );
        f.services
            .save_lesson_progress(&lea.id, "cp-son-ou", Some(4))
            .unwrap();
        assert_eq!(
            Some(4),
            f.services
                .lesson("cp-son-ou", Some(&lea.id))
                .unwrap()
                .resume_slide
        );
        assert_eq!(
            None,
            f.services.lesson("cp-son-ou", None).unwrap().resume_slide
        );
        f.services
            .save_lesson_progress(&lea.id, "cp-son-ou", None)
            .unwrap();
        assert_eq!(
            None,
            f.services
                .lesson("cp-son-ou", Some(&lea.id))
                .unwrap()
                .resume_slide
        );
        assert_eq!(
            404,
            f.services
                .save_lesson_progress(&lea.id, "nothing", Some(1))
                .unwrap_err()
                .status
        );
    }

    #[test]
    fn spelling_lessons_are_listed_by_subject() {
        let f = fixture();
        let lessons = f
            .services
            .lessons(Grade::CP, Subject::Spelling, None)
            .unwrap();
        assert_eq!(Subject::Spelling, lessons.subject);
        assert!(lessons.lessons.iter().any(|l| l.id == "cp-son-ou"));
        assert!(lessons.lessons.iter().all(|l| l.id.starts_with("cp-")));
        let math = f.services.lessons(Grade::CP, Subject::Math, None).unwrap();
        assert!(math.lessons.iter().all(|l| !l.id.contains("son")));
    }

    #[test]
    fn every_lesson_text_read_aloud_is_accepted_by_the_voice() {
        assert_eq!(educator_content::MAX_SPOKEN_LENGTH, MAX_SPEECH_LENGTH);
        let f = fixture();
        let mut summaries = Vec::new();
        for grade in [Grade::CP, Grade::CE1] {
            for subject in [Subject::Math, Subject::Spelling] {
                summaries.extend(f.services.lessons(grade, subject, None).unwrap().lessons);
            }
        }
        for summary in summaries {
            let lesson = f.services.lesson(&summary.id, None).unwrap();
            let texts = lesson
                .slides
                .iter()
                .map(|s| s.speech.clone())
                .chain(lesson.quiz.iter().map(|q| q.spoken.clone()));
            for text in texts {
                // The voice is unavailable in the tests: 503 means the text itself was accepted.
                assert_eq!(
                    503,
                    f.services
                        .speak(&text, None, VoiceSpeed::Normal)
                        .unwrap_err()
                        .status,
                    "{text}"
                );
            }
        }
    }

    // ── Lot 3 ────────────────────────────────────────────────────────────────────────────

    /// A series of `skill_id` answered wrong to the end (the core keeps the answers secret).
    fn wrong_series(services: &Services, profile: &str, skill_id: &str) -> SeriesSummaryResponse {
        let series = services
            .start_series(StartSeriesRequest {
                profile_id: profile.to_owned(),
                skill_id: skill_id.to_owned(),
                answer_mode: AnswerMode::Keyboard,
                format: None,
                seed: Some(1),
            })
            .unwrap();
        let mut summary = None;
        for index in 0..series.exercises.len() {
            summary = services
                .answer_series(&series.series_id, index as u32, number(-1), None)
                .unwrap()
                .summary;
        }
        summary.unwrap()
    }

    #[test]
    fn a_finished_series_brings_its_stars_its_badge_and_its_accessories() {
        let f = fixture();
        let lea = create(&f.services, "Léa");

        let summary = wrong_series(&f.services, &lea.id, "cp-add-sum-10");

        let rewards = summary.rewards.expect("rewards");
        assert_eq!(Some(1), rewards.stars, "a series finished: one star");
        assert_eq!((1, 1), (rewards.goal.done, rewards.goal.target.min(1)));
        assert!(rewards.new_badges.iter().any(|b| b.id == "premier-tour"));
        let unlocked: Vec<&str> = rewards
            .new_accessories
            .iter()
            .map(|a| a.id.as_str())
            .collect();
        assert!(unlocked.contains(&"casquette") && unlocked.contains(&"echarpe"));
        assert!(
            !unlocked.contains(&"medaille"),
            "the medal comes with the badge of 10 series"
        );
        let skills = f
            .services
            .skills(Grade::CP, Subject::Math, Some(&lea.id))
            .unwrap();
        assert_eq!(
            1,
            skills
                .skills
                .iter()
                .find(|s| s.id == "cp-add-sum-10")
                .unwrap()
                .stars
        );
        let overview = f.services.profile_overview(&lea.id).unwrap();
        assert_eq!(1, overview.level.level);
        assert_eq!(1, overview.badges_earned);
        assert!(overview.badges_total >= 20);
        assert!(
            overview.reviews.iter().all(|r| r.due == 0),
            "mistakes come back tomorrow"
        );
        let badges = f.services.badges(&lea.id).unwrap();
        assert!(badges.iter().any(|b| b.id == "premier-tour" && b.earned));
    }

    #[test]
    fn the_avatar_wears_what_is_unlocked_and_is_drawn_as_one_svg() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        let locked = f
            .services
            .update_avatar(
                &lea.id,
                UpdateAvatarRequest {
                    character_id: None,
                    accessories: vec![WornAccessoryResponse {
                        accessory_id: "couronne".to_owned(),
                        color_id: "or".to_owned(),
                    }],
                },
            )
            .unwrap_err();
        assert_eq!(400, locked.status);

        let cap = WornAccessoryResponse {
            accessory_id: "casquette".to_owned(),
            color_id: "bleu".to_owned(),
        };
        let dressed = f
            .services
            .update_avatar(
                &lea.id,
                UpdateAvatarRequest {
                    character_id: Some("lapin".to_owned()),
                    accessories: vec![cap.clone()],
                },
            )
            .unwrap();
        assert_eq!(
            ("lapin", vec![cap.clone()]),
            (dressed.character_id.as_str(), dressed.accessories.clone())
        );
        assert_eq!(
            vec![cap.clone()],
            f.services.list_profiles().unwrap()[0].accessories
        );
        let svg = f.services.avatar_svg("lapin", &[cap]).unwrap();
        assert!(svg.contains(r#"data-accessory="casquette""#) && svg.contains("--acc:#2F6FD0"));
        assert_eq!(
            404,
            f.services.avatar_svg("dragon", &[]).unwrap_err().status
        );
        let accessories = f.services.accessories(&lea.id).unwrap();
        assert!(accessories.len() >= 20);
        let crown = accessories.iter().find(|a| a.id == "couronne").unwrap();
        assert!(!crown.unlocked);
        assert_eq!(Some(20), crown.unlock.level, "the crown comes last");
        let cape = accessories.iter().find(|a| a.id == "cape").unwrap();
        assert!(cape.unlock.badge_title.is_some());
    }

    #[test]
    fn the_settings_of_a_child_are_checked_and_kept() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        let defaults = f.services.profile_settings(&lea.id).unwrap();
        assert_eq!(
            ("sport", 3, true),
            (defaults.theme.as_str(), defaults.daily_goal, defaults.sound)
        );

        let changed = f
            .services
            .update_profile_settings(
                &lea.id,
                UpdateProfileSettingsRequest {
                    theme: Some("cosmos".to_owned()),
                    daily_goal: Some(5),
                    text_size: Some(TextSizeName::Large),
                    ..Default::default()
                },
            )
            .unwrap();
        assert_eq!(
            ("cosmos", 5, TextSizeName::Large),
            (
                changed.theme.as_str(),
                changed.daily_goal,
                changed.text_size
            )
        );
        assert_eq!(5, f.services.profile_overview(&lea.id).unwrap().goal.target);
        for bad in [
            UpdateProfileSettingsRequest {
                theme: Some("jungle".to_owned()),
                ..Default::default()
            },
            UpdateProfileSettingsRequest {
                daily_goal: Some(50),
                ..Default::default()
            },
            UpdateProfileSettingsRequest {
                timed_seconds: Some(45),
                ..Default::default()
            },
        ] {
            assert_eq!(
                400,
                f.services
                    .update_profile_settings(&lea.id, bad)
                    .unwrap_err()
                    .status
            );
        }
        assert_eq!(
            "cosmos",
            f.services.profile_settings(&lea.id).unwrap().theme
        );
        let options = f.services.settings_options();
        assert!(options.themes.iter().any(|t| t.id == "cosmos"));
    }

    #[test]
    fn an_advice_is_given_for_the_current_period() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        let advice = f.services.advice(&lea.id, Subject::Math).unwrap().unwrap();
        assert_eq!(AdviceReasonName::Start, advice.reason);
        let skills = f.services.skills(Grade::CP, Subject::Math, None).unwrap();
        assert!(skills
            .skills
            .iter()
            .any(|s| s.id == advice.skill_id && s.is_current_period));
    }

    #[test]
    fn a_backup_written_then_read_gives_the_profiles_back() {
        let f = fixture();
        let lea = create(&f.services, "Léa");
        wrong_series(&f.services, &lea.id, "cp-add-sum-10");
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("2026-10-15_educator-sauvegarde.json");
        let path = path.to_str().unwrap();

        assert_eq!(1, f.services.export_backup(path).unwrap().profiles);
        f.services.delete_profile(&lea.id).unwrap();
        assert!(f.services.list_profiles().unwrap().is_empty());
        assert_eq!(1, f.services.import_backup(path).unwrap().profiles);

        let back = f.services.list_profiles().unwrap();
        assert_eq!(
            vec!["Léa".to_owned()],
            back.iter().map(|p| p.name.clone()).collect::<Vec<_>>()
        );
        assert_eq!(lea.id, back[0].id);
        assert_eq!(
            1,
            f.services
                .badges(&lea.id)
                .unwrap()
                .iter()
                .filter(|b| b.earned)
                .count()
        );
        let not_a_backup = dir.path().join("note.json");
        std::fs::write(&not_a_backup, "{}").unwrap();
        assert_eq!(
            400,
            f.services
                .import_backup(not_a_backup.to_str().unwrap())
                .unwrap_err()
                .status
        );
    }

    #[test]
    fn the_words_missed_come_back_the_next_day_in_my_reviews() {
        let dir = tempfile::tempdir().unwrap();
        let clock = Arc::new(ManualClock::new(unix_ms(2026, 10, 11, 8, 0, 0)));
        let services = Services::start(
            &Locations {
                content_directory: Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content"),
                database_path: dir.path().join("educator.db"),
            },
            Arc::new(UnavailableSpeechSynthesizer::new("tests")),
            Arc::new(ScriptedRandom::new([3])),
            clock.clone(),
            Box::new(|| Date::new(2026, 10, 11)),
            Arc::new(educator_core::statistics::utc_date_of),
        )
        .unwrap();
        let lea = create(&services, "Léa");
        let series = services
            .start_series(StartSeriesRequest {
                profile_id: lea.id.clone(),
                skill_id: "cp-son-ou".to_owned(),
                answer_mode: AnswerMode::Keyboard,
                format: Some(SpellingFormat::MissingLetters),
                seed: Some(4),
            })
            .unwrap();
        let mut words = Vec::new();
        for index in 0..3u32 {
            let outcome = services
                .answer_series(
                    &series.series_id,
                    index,
                    AnswerPayload::Text {
                        value: "zz".to_owned(),
                    },
                    None,
                )
                .unwrap();
            words.push(outcome.expected_label);
        }
        services.abandon_series(&series.series_id).unwrap();
        assert_eq!(
            400,
            services
                .start_review(&lea.id, Subject::Spelling, Some(1))
                .unwrap_err()
                .status
        );

        clock.advance(86_400_000);
        let due = services.profile_overview(&lea.id).unwrap().reviews;
        assert_eq!(
            3,
            due.iter()
                .find(|r| r.subject == Subject::Spelling)
                .unwrap()
                .due
        );
        let review = services
            .start_review(&lea.id, Subject::Spelling, Some(1))
            .unwrap();
        assert_eq!(
            (SessionType::Review, 3),
            (review.session_type, review.exercises.len())
        );
        let mut asked = Vec::new();
        for index in 0..3u32 {
            let outcome = services
                .answer_series(
                    &review.series_id,
                    index,
                    AnswerPayload::Text {
                        value: "zz".to_owned(),
                    },
                    None,
                )
                .unwrap();
            asked.push(outcome.expected_label);
        }
        asked.sort();
        words.sort();
        assert_eq!(words, asked, "the same words");
    }
}
