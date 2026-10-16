//! The exchange format between the computer and the iPad (SUB027606): JSON messages, versioned by
//! [`PROTOCOL_VERSION`], described by the schemas of `crates/educator-sync/schemas/`. Only what the
//! merge needs: no address, no device name of a child. Conversions to the types of
//! `educator-core::sync` check every value: a malformed message is an error, never a panic.

use educator_core::reviews::{ReviewItem, ReviewOrigin};
use educator_core::rewards::accessories::WornAccessory;
use educator_core::rewards::daily_goal::GoalDay;
use educator_core::sessions::{AnswerRecord, SessionRecord, SessionType};
use educator_core::skills::{AnswerMode, ExerciseFormat, Subject};
use educator_core::sync::*;
use educator_core::Grade;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use educator_core::sync::PROTOCOL_VERSION;

/// A value refused while reading a message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatError(pub String);

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Malformed message: {}", self.0)
    }
}

impl std::error::Error for FormatError {}

type Checked<T> = Result<T, FormatError>;

fn bad(message: impl Into<String>) -> FormatError {
    FormatError(message.into())
}

fn clock(text: &str) -> Checked<Hlc> {
    Hlc::parse(text).ok_or_else(|| bad(format!("clock '{text}'")))
}

fn grade(text: &str) -> Checked<Grade> {
    text.parse().map_err(|_| bad(format!("grade '{text}'")))
}

// ----- Messages -------------------------------------------------------------------------------

/// `POST /v1/pair`: the secret of the QR code (or the short code) for a lasting token.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PairRequest {
    pub protocol: u32,
    pub device_id: Uuid,
    pub secret: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PairResponse {
    pub protocol: u32,
    pub device_id: Uuid,
    pub token: String,
}

/// Without the secret: it never reaches the logs.
impl std::fmt::Debug for PairRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PairRequest")
            .field("protocol", &self.protocol)
            .finish_non_exhaustive()
    }
}

/// Without the token: it never reaches the logs.
impl std::fmt::Debug for PairResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PairResponse")
            .field("protocol", &self.protocol)
            .finish_non_exhaustive()
    }
}

/// `POST /v1/hello`: before a synchronisation, the profiles of the computer (for the question
/// « Est-ce le même enfant ? »).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HelloRequest {
    pub protocol: u32,
    pub device_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HelloResponse {
    pub protocol: u32,
    pub device_id: Uuid,
    /// Changes when the computer restores a backup: the iPad then exchanges everything again.
    pub epoch: String,
    pub profiles: Vec<WireProfile>,
}

/// `POST /v1/push`: a batch of the iPad; `lastSeq`: its cursor once applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushRequest {
    pub protocol: u32,
    pub device_id: Uuid,
    pub last_seq: i64,
    pub batch: WireBatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushResponse {
    pub protocol: u32,
    /// The cursor applied: the iPad saves it.
    pub acknowledged: i64,
}

/// `POST /v1/pull`: the changes of the computer after `after`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    pub protocol: u32,
    pub device_id: Uuid,
    pub after: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullResponse {
    pub protocol: u32,
    pub last_seq: i64,
    pub more: bool,
    pub batch: WireBatch,
}

/// Why a request was refused. `code`: what the iPad shows to the parent (`unauthorized`,
/// `incompatible`, `invalid`, `expired`, `tooMany`, `unavailable`); `message`: English, logs only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub protocol: u32,
    pub code: String,
    pub message: String,
}

// ----- Batch ----------------------------------------------------------------------------------

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireBatch {
    #[serde(default)]
    pub profiles: Vec<WireProfile>,
    #[serde(default)]
    pub removals: Vec<WireRemoval>,
    #[serde(default)]
    pub sessions: Vec<WireSession>,
    #[serde(default)]
    pub lesson_progress: Vec<WireLessonProgress>,
    #[serde(default)]
    pub badges: Vec<WireBadge>,
    #[serde(default)]
    pub accessories: Vec<WireAccessory>,
    #[serde(default)]
    pub goal_days: Vec<WireGoalDay>,
    #[serde(default)]
    pub reviews: Vec<WireReview>,
    #[serde(default)]
    pub settings: Vec<WireSetting>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireAccessoryWorn {
    pub id: String,
    pub color: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireProfileClocks {
    pub name: String,
    pub grade: String,
    pub avatar: String,
    pub placement_seconds: String,
    pub exam_seconds: String,
    pub last_used_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireProfile {
    pub id: Uuid,
    pub name: String,
    pub grade: String,
    pub character_id: String,
    pub accessories: Vec<WireAccessoryWorn>,
    pub placement_seconds: u32,
    pub exam_seconds: u32,
    pub created_at: i64,
    pub last_used_at: i64,
    pub clocks: WireProfileClocks,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireRemoval {
    pub profile_id: Uuid,
    pub clock: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged_into: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireAnswer {
    pub id: Uuid,
    pub index: u32,
    pub skill_id: String,
    pub prompt: String,
    pub expected: String,
    pub given: String,
    pub is_correct: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u32>,
    pub answered_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireSession {
    pub id: Uuid,
    pub profile_id: Uuid,
    #[serde(rename = "type")]
    pub session_type: String,
    pub subject: String,
    pub grade: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skill_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lesson_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    pub seed: u32,
    pub question_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_limit_seconds: Option<u32>,
    pub started_at: i64,
    pub ended_at: i64,
    pub abandoned: bool,
    pub correct_count: u32,
    pub points_earned: i32,
    pub clock: String,
    pub answers: Vec<WireAnswer>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireLessonProgress {
    pub profile_id: Uuid,
    pub lesson_id: String,
    /// Absent: the lesson was finished.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slide: Option<u32>,
    pub clock: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireBadge {
    pub profile_id: Uuid,
    pub badge_id: String,
    pub earned_at: i64,
    pub clock: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireAccessory {
    pub profile_id: Uuid,
    pub accessory_id: String,
    pub unlocked_at: i64,
    pub clock: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireGoalDay {
    pub profile_id: Uuid,
    pub day: i64,
    pub reached_at: i64,
    pub bonus_points: i32,
    pub clock: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireReviewOrigin {
    pub seed: u32,
    pub answer_mode: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    pub index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireReviewItem {
    pub id: Uuid,
    pub prompt: String,
    pub expected: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub word: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<WireReviewOrigin>,
    pub step: u32,
    pub due_day: i64,
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireReview {
    pub profile_id: Uuid,
    pub skill_id: String,
    pub key: String,
    /// Absent: the item left the queue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<WireReviewItem>,
    pub clock: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireSetting {
    pub profile_id: Uuid,
    pub key: String,
    pub value: String,
    pub clock: String,
}

// ----- Conversions ----------------------------------------------------------------------------

impl From<&ProfileState> for WireProfile {
    fn from(state: &ProfileState) -> Self {
        Self {
            id: state.id,
            name: state.name.clone(),
            grade: state.grade.as_str().to_owned(),
            character_id: state.character_id.clone(),
            accessories: state
                .accessories
                .iter()
                .map(|a| WireAccessoryWorn {
                    id: a.accessory_id.clone(),
                    color: a.color_id.clone(),
                })
                .collect(),
            placement_seconds: state.placement_seconds,
            exam_seconds: state.exam_seconds,
            created_at: state.created_at,
            last_used_at: state.last_used_at,
            clocks: WireProfileClocks {
                name: state.clocks.name.encode(),
                grade: state.clocks.grade.encode(),
                avatar: state.clocks.avatar.encode(),
                placement_seconds: state.clocks.placement_seconds.encode(),
                exam_seconds: state.clocks.exam_seconds.encode(),
                last_used_at: state.clocks.last_used_at.encode(),
            },
        }
    }
}

impl TryFrom<&WireProfile> for ProfileState {
    type Error = FormatError;

    fn try_from(wire: &WireProfile) -> Checked<Self> {
        let name = educator_core::profiles::Profile::clean_name(&wire.name);
        if name.is_empty() || name.encode_utf16().count() > educator_core::profiles::MAX_NAME_LENGTH
        {
            return Err(bad("profile name"));
        }
        Ok(Self {
            id: wire.id,
            name,
            grade: grade(&wire.grade)?,
            character_id: wire.character_id.clone(),
            accessories: wire
                .accessories
                .iter()
                .map(|a| WornAccessory::new(a.id.clone(), a.color.clone()))
                .collect(),
            placement_seconds: wire.placement_seconds,
            exam_seconds: wire.exam_seconds,
            created_at: wire.created_at,
            last_used_at: wire.last_used_at,
            clocks: ProfileClocks {
                name: clock(&wire.clocks.name)?,
                grade: clock(&wire.clocks.grade)?,
                avatar: clock(&wire.clocks.avatar)?,
                placement_seconds: clock(&wire.clocks.placement_seconds)?,
                exam_seconds: clock(&wire.clocks.exam_seconds)?,
                last_used_at: clock(&wire.clocks.last_used_at)?,
            },
        })
    }
}

fn wire_session(change: &SessionChange) -> WireSession {
    let s = &change.session;
    WireSession {
        id: s.id,
        profile_id: s.profile_id,
        session_type: s.session_type.as_str().to_owned(),
        subject: s.subject.as_str().to_owned(),
        grade: s.grade.as_str().to_owned(),
        skill_id: s.skill_id.clone(),
        lesson_id: s.lesson_id.clone(),
        answer_mode: s.answer_mode.map(|m| m.as_str().to_owned()),
        format: s.format.map(|f| f.name().to_owned()),
        seed: s.seed,
        question_count: s.question_count,
        time_limit_seconds: s.time_limit_seconds,
        started_at: s.started_at,
        ended_at: s.ended_at.unwrap_or(s.started_at),
        abandoned: s.abandoned,
        correct_count: s.correct_count,
        points_earned: s.points_earned,
        clock: change.clock.encode(),
        answers: change
            .answers
            .iter()
            .map(|a| WireAnswer {
                id: a.id,
                index: a.index,
                skill_id: a.skill_id.clone(),
                prompt: a.prompt.clone(),
                expected: a.expected.clone(),
                given: a.given.clone(),
                is_correct: a.is_correct,
                duration_ms: a.duration_ms,
                answered_at: a.answered_at,
            })
            .collect(),
    }
}

fn session_change(wire: &WireSession) -> Checked<SessionChange> {
    let session = SessionRecord {
        id: wire.id,
        profile_id: wire.profile_id,
        session_type: wire
            .session_type
            .parse::<SessionType>()
            .map_err(|_| bad(format!("session type '{}'", wire.session_type)))?,
        subject: Subject::from_name(&wire.subject)
            .ok_or_else(|| bad(format!("subject '{}'", wire.subject)))?,
        grade: grade(&wire.grade)?,
        skill_id: wire.skill_id.clone(),
        lesson_id: wire.lesson_id.clone(),
        answer_mode: wire
            .answer_mode
            .as_deref()
            .map(|m| AnswerMode::from_name(m).ok_or_else(|| bad(format!("answer mode '{m}'"))))
            .transpose()?,
        format: wire
            .format
            .as_deref()
            .map(|f| ExerciseFormat::from_name(f).ok_or_else(|| bad(format!("format '{f}'"))))
            .transpose()?,
        seed: wire.seed,
        question_count: wire.question_count,
        time_limit_seconds: wire.time_limit_seconds,
        started_at: wire.started_at,
        ended_at: Some(wire.ended_at),
        abandoned: wire.abandoned,
        correct_count: wire.correct_count,
        points_earned: if wire.points_earned < 0 {
            return Err(bad("negative points"));
        } else {
            wire.points_earned
        },
    };
    Ok(SessionChange {
        answers: wire
            .answers
            .iter()
            .map(|a| AnswerRecord {
                id: a.id,
                session_id: wire.id,
                index: a.index,
                skill_id: a.skill_id.clone(),
                prompt: a.prompt.clone(),
                expected: a.expected.clone(),
                given: a.given.clone(),
                is_correct: a.is_correct,
                duration_ms: a.duration_ms,
                answered_at: a.answered_at,
            })
            .collect(),
        session,
        clock: clock(&wire.clock)?,
    })
}

fn wire_review(change: &ReviewChange) -> WireReview {
    WireReview {
        profile_id: change.profile_id,
        skill_id: change.skill_id.clone(),
        key: change.key.clone(),
        item: change.item.as_ref().map(|item| WireReviewItem {
            id: item.id,
            prompt: item.prompt.clone(),
            expected: item.expected.clone(),
            word: item.word.clone(),
            origin: item.origin.map(|o| WireReviewOrigin {
                seed: o.seed,
                answer_mode: o.answer_mode.as_str().to_owned(),
                format: o.format.map(|f| f.name().to_owned()),
                index: o.index,
            }),
            step: item.step,
            due_day: item.due_day,
            created_at: item.created_at,
        }),
        clock: change.clock.encode(),
    }
}

fn review_change(wire: &WireReview) -> Checked<ReviewChange> {
    let item = match &wire.item {
        None => None,
        Some(item) => Some(ReviewItem {
            id: item.id,
            profile_id: wire.profile_id,
            skill_id: wire.skill_id.clone(),
            key: wire.key.clone(),
            prompt: item.prompt.clone(),
            expected: item.expected.clone(),
            word: item.word.clone(),
            origin: item
                .origin
                .as_ref()
                .map(|o| {
                    Ok::<_, FormatError>(ReviewOrigin {
                        seed: o.seed,
                        answer_mode: AnswerMode::from_name(&o.answer_mode)
                            .ok_or_else(|| bad(format!("answer mode '{}'", o.answer_mode)))?,
                        format: o
                            .format
                            .as_deref()
                            .map(|f| {
                                ExerciseFormat::from_name(f)
                                    .ok_or_else(|| bad(format!("format '{f}'")))
                            })
                            .transpose()?,
                        index: o.index,
                    })
                })
                .transpose()?,
            step: item.step,
            due_day: item.due_day,
            created_at: item.created_at,
            updated_at: 0,
        }),
    };
    if wire.skill_id.contains('/') {
        return Err(bad("skill id"));
    }
    Ok(ReviewChange {
        profile_id: wire.profile_id,
        skill_id: wire.skill_id.clone(),
        key: wire.key.clone(),
        item,
        clock: clock(&wire.clock)?,
    })
}

impl From<&ChangeBatch> for WireBatch {
    fn from(batch: &ChangeBatch) -> Self {
        Self {
            profiles: batch.profiles.iter().map(WireProfile::from).collect(),
            removals: batch
                .removals
                .iter()
                .map(|r| WireRemoval {
                    profile_id: r.profile_id,
                    clock: r.clock.encode(),
                    merged_into: r.merged_into,
                })
                .collect(),
            sessions: batch.sessions.iter().map(wire_session).collect(),
            lesson_progress: batch
                .lesson_progress
                .iter()
                .map(|c| WireLessonProgress {
                    profile_id: c.profile_id,
                    lesson_id: c.lesson_id.clone(),
                    slide: c.slide,
                    clock: c.clock.encode(),
                })
                .collect(),
            badges: batch
                .badges
                .iter()
                .map(|c| WireBadge {
                    profile_id: c.profile_id,
                    badge_id: c.badge_id.clone(),
                    earned_at: c.earned_at,
                    clock: c.clock.encode(),
                })
                .collect(),
            accessories: batch
                .accessories
                .iter()
                .map(|c| WireAccessory {
                    profile_id: c.profile_id,
                    accessory_id: c.accessory_id.clone(),
                    unlocked_at: c.unlocked_at,
                    clock: c.clock.encode(),
                })
                .collect(),
            goal_days: batch
                .goal_days
                .iter()
                .map(|c| WireGoalDay {
                    profile_id: c.profile_id,
                    day: c.day.day,
                    reached_at: c.day.reached_at,
                    bonus_points: c.day.bonus_points,
                    clock: c.clock.encode(),
                })
                .collect(),
            reviews: batch.reviews.iter().map(wire_review).collect(),
            settings: batch
                .settings
                .iter()
                .map(|c| WireSetting {
                    profile_id: c.profile_id,
                    key: c.key.clone(),
                    value: c.value.clone(),
                    clock: c.clock.encode(),
                })
                .collect(),
        }
    }
}

impl TryFrom<&WireBatch> for ChangeBatch {
    type Error = FormatError;

    fn try_from(wire: &WireBatch) -> Checked<Self> {
        let no_slash = |part: &str, what: &str| {
            if part.contains('/') || part.is_empty() {
                Err(bad(what.to_owned()))
            } else {
                Ok(())
            }
        };
        for c in &wire.lesson_progress {
            no_slash(&c.lesson_id, "lesson id")?;
        }
        for c in &wire.badges {
            no_slash(&c.badge_id, "badge id")?;
        }
        for c in &wire.accessories {
            no_slash(&c.accessory_id, "accessory id")?;
        }
        for c in &wire.settings {
            no_slash(&c.key, "setting key")?;
        }
        Ok(ChangeBatch {
            profiles: wire
                .profiles
                .iter()
                .map(ProfileState::try_from)
                .collect::<Checked<_>>()?,
            removals: wire
                .removals
                .iter()
                .map(|r| {
                    Ok(ProfileRemoval {
                        profile_id: r.profile_id,
                        clock: clock(&r.clock)?,
                        merged_into: r.merged_into,
                    })
                })
                .collect::<Checked<_>>()?,
            sessions: wire
                .sessions
                .iter()
                .map(session_change)
                .collect::<Checked<_>>()?,
            lesson_progress: wire
                .lesson_progress
                .iter()
                .map(|c| {
                    Ok(LessonProgressChange {
                        profile_id: c.profile_id,
                        lesson_id: c.lesson_id.clone(),
                        slide: c.slide,
                        clock: clock(&c.clock)?,
                    })
                })
                .collect::<Checked<_>>()?,
            badges: wire
                .badges
                .iter()
                .map(|c| {
                    Ok(BadgeChange {
                        profile_id: c.profile_id,
                        badge_id: c.badge_id.clone(),
                        earned_at: c.earned_at,
                        clock: clock(&c.clock)?,
                    })
                })
                .collect::<Checked<_>>()?,
            accessories: wire
                .accessories
                .iter()
                .map(|c| {
                    Ok(AccessoryChange {
                        profile_id: c.profile_id,
                        accessory_id: c.accessory_id.clone(),
                        unlocked_at: c.unlocked_at,
                        clock: clock(&c.clock)?,
                    })
                })
                .collect::<Checked<_>>()?,
            goal_days: wire
                .goal_days
                .iter()
                .map(|c| {
                    if c.bonus_points < 0 {
                        return Err(bad("negative bonus"));
                    }
                    Ok(GoalDayChange {
                        profile_id: c.profile_id,
                        day: GoalDay {
                            day: c.day,
                            reached_at: c.reached_at,
                            bonus_points: c.bonus_points,
                        },
                        clock: clock(&c.clock)?,
                    })
                })
                .collect::<Checked<_>>()?,
            reviews: wire
                .reviews
                .iter()
                .map(review_change)
                .collect::<Checked<_>>()?,
            settings: wire
                .settings
                .iter()
                .map(|c| {
                    Ok(SettingChange {
                        profile_id: c.profile_id,
                        key: c.key.clone(),
                        value: c.value.clone(),
                        clock: clock(&c.clock)?,
                    })
                })
                .collect::<Checked<_>>()?,
        })
    }
}

/// Reads a message, checking its protocol version first: a message of another version is refused
/// as `incompatible`, whatever it contains.
pub fn read_message<T: for<'de> Deserialize<'de>>(body: &[u8]) -> Result<T, MessageError> {
    let value: serde_json::Value =
        serde_json::from_slice(body).map_err(|e| MessageError::Malformed(e.to_string()))?;
    match value.get("protocol").and_then(serde_json::Value::as_u64) {
        Some(version) if version == u64::from(PROTOCOL_VERSION) => {}
        Some(version) => return Err(MessageError::Incompatible(version)),
        None => return Err(MessageError::Malformed("no protocol version".to_owned())),
    }
    serde_json::from_value(value).map_err(|e| MessageError::Malformed(e.to_string()))
}

/// A message that cannot be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageError {
    Malformed(String),
    /// Another version of the protocol.
    Incompatible(u64),
}

impl std::fmt::Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageError::Malformed(m) => write!(f, "Malformed message: {m}"),
            MessageError::Incompatible(v) => write!(
                f,
                "Protocol version {v} (this application speaks {PROTOCOL_VERSION})."
            ),
        }
    }
}

impl std::error::Error for MessageError {}

/// The profile states of a hello, checked.
pub fn profile_states(wire: &[WireProfile]) -> Checked<Vec<ProfileState>> {
    wire.iter().map(ProfileState::try_from).collect()
}

/// For the logs: never a name.
pub fn describe(batch: &WireBatch) -> String {
    match ChangeBatch::try_from(batch) {
        Ok(batch) => batch.to_string(),
        Err(e) => e.to_string(),
    }
}
