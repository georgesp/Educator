import type * as Types from "./types";

// Types exchanged with the application core (Tauri commands), generated from
// src-tauri/src/contracts.rs into ./types.ts by `npm run api:generate` — never written by hand.
export type StatusResponse = Types.StatusResponse;
export type StartupStatus = Types.StartupStatus;
export type Grade = Types.Grade;
export type Subject = Types.Subject;
export type SpellingFormat = Types.SpellingFormat;
export type SessionType = Types.SessionType;
export type AnswerMode = Types.AnswerMode;
export type Profile = Types.ProfileResponse;
export type UpdateProfile = Types.UpdateProfileRequest;
export type Character = Types.CharacterResponse;
export type Skills = Types.SkillsResponse;
export type Skill = Types.SkillResponse;
export type Series = Types.SeriesResponse;
export type SkillFamily = Types.SkillFamily;
export type SkillCategory = Types.SkillCategory;
export type Exercise = Types.ExerciseResponse;
export type Token = Types.TokenResponse;
export type Picture = Types.PictureResponse;
export type Input = Types.InputResponse;
export type Choice = Types.ChoiceResponse;
export type AnswerPayload = Types.AnswerPayload;
export type ComparisonSign = Types.ComparisonSign;
export type SeriesAnswer = Types.SeriesAnswerResponse;
export type MultiplicationTable = Types.MultiplicationTableResponse;
export type SeriesSummary = Types.SeriesSummaryResponse;
export type SkillResult = Types.SkillResultResponse;
export type TimedChallenge = Types.TimedChallengeResponse;
export type PlacementStatus = Types.PlacementStatusResponse;
export type PlacementSubject = Types.PlacementSubjectResponse;
export type Mascot = Types.MascotResponse;
export type MascotPhrase = Types.MascotPhraseResponse;
export type MascotSituation = Types.MascotSituation;
export type MascotExpression = Types.MascotExpression;
export type WordPrompt = Types.WordPromptResponse;
export type SpellingCorrection = Types.SpellingCorrectionResponse;
export type Letter = Types.LetterResponse;
export type LetterKind = Types.LetterKindName;
export type CommandError = Types.CommandError;
export type Lessons = Types.LessonsResponse;
export type LessonSummary = Types.LessonSummaryResponse;
export type Lesson = Types.LessonResponse;
export type Slide = Types.SlideResponse;
export type Animation = Types.AnimationResponse;
export type Chain = Types.ChainResponse;
export type ChainMode = Types.ChainMode;
export type Jump = Types.JumpResponse;
export type LessonOperator = Types.LessonOperator;
export type WrittenOperation = Types.WrittenOperationResponse;
export type WrittenColumn = Types.WrittenColumnResponse;
export type Equality = Types.EqualityResponse;
export type EquationTerm = Types.EquationTerm;
export type Blocks = Types.BlocksResponse;
export type BlockStep = Types.BlockStepResponse;
export type BlockStepName = Types.BlockStepName;
export type BlockTask = Types.BlockTask;
export type RankName = Types.RankName;
export type RankDigits = Types.RankDigitsResponse;
export type RankCount = Types.RankCountResponse;
export type Spellings = Types.SpellingsResponse;
export type SequenceMode = Types.SequenceMode;
export type GridPart = Types.GridPartResponse;
export type GridTask = Types.GridTask;
export type TableCell = Types.TableCellResponse;
export type TableHighlight = Types.TableHighlightResponse;
export type TableHighlightKind = Types.TableHighlightKind;
export type GroupMode = Types.GroupModeName;
export type RaceObjectName = Types.RaceObjectName;
export type Grouping = Types.GroupingResponse;
export type FrameSlot = Types.FrameSlotName;
export type FriendPair = Types.FriendPairResponse;
export type FractionStrip = Types.FractionStripResponse;
export type BarKind = Types.BarKindName;
export type BarValue = Types.BarValueResponse;
export type QuizQuestion = Types.QuizQuestionResponse;
export type SortedWord = Types.SortedWordResponse;
export type ParentGate = Types.ParentGateResponse;
export type ParentProfile = Types.ParentProfileResponse;
export type UpdateParentProfile = Types.UpdateParentProfileRequest;
export type ProfileStatistics = Types.ProfileStatisticsResponse;
export type SkillStatistics = Types.SkillStatisticsResponse;
export type SeriesPoint = Types.SeriesPointResponse;
export type TestResult = Types.TestResultResponse;
export type FrequentMistake = Types.FrequentMistakeResponse;
export type DayActivity = Types.DayActivityResponse;
export type WeekActivity = Types.WeekActivityResponse;
export type MasteryState = Types.MasteryStateName;
export type Trend = Types.TrendName;
export type Level = Types.LevelResponse;
export type DailyGoal = Types.DailyGoalResponse;
export type ProfileOverview = Types.ProfileOverviewResponse;
export type Badge = Types.BadgeResponse;
export type AccessorySlot = Types.AccessorySlotName;
export type WornAccessory = Types.WornAccessoryResponse;
export type Accessory = Types.AccessoryResponse;
export type AccessoryColor = Types.AccessoryColorResponse;
export type UpdateAvatar = Types.UpdateAvatarRequest;
export type SessionRewards = Types.SessionRewardsResponse;
export type Advice = Types.AdviceResponse;
export type AdviceReason = Types.AdviceReasonName;
export type VoiceSpeed = Types.VoiceSpeedName;
export type TextSize = Types.TextSizeName;
export type ProfileSettings = Types.ProfileSettingsResponse;
export type UpdateProfileSettings = Types.UpdateProfileSettingsRequest;
export type Theme = Types.ThemeResponse;
export type SettingsOptions = Types.SettingsOptionsResponse;
export type Backup = Types.BackupResponse;
export type SyncRole = Types.SyncRole;
export type SyncDevice = Types.SyncDeviceResponse;
export type SyncQuestion = Types.SyncQuestionResponse;
export type SyncStatus = Types.SyncStatusResponse;
export type SyncPairing = Types.SyncPairingResponse;
export type PairWithComputer = Types.PairWithComputerRequest;
export type SyncOutcome = Types.SyncOutcomeName;
export type SyncResult = Types.SyncResultResponse;

/**
 * Error returned by a command. `status` keeps the meaning of the former HTTP API (400 invalid input,
 * 404 unknown, 409 conflict, 503 unavailable); `detail` is a French sentence for the child on profile errors.
 */
export class ApiError extends Error {
  constructor(
    readonly status: number,
    readonly title: string,
    readonly detail?: string,
    readonly field?: string,
  ) {
    super(detail ? `${title}: ${detail}` : title);
    this.name = "ApiError";
  }
}

function isCommandError(value: unknown): value is CommandError {
  return typeof value === "object" && value !== null && "status" in value && "title" in value;
}

/** An ApiError from what a command rejected with (a CommandError, or a message from Tauri itself). */
export function toApiError(error: unknown): ApiError {
  if (isCommandError(error)) {
    return new ApiError(error.status, error.title, error.detail ?? undefined, error.field ?? undefined);
  }

  return new ApiError(500, "Unexpected error", error instanceof Error ? error.message : String(error));
}
