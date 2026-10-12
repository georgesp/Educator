import type * as Types from "./types";

// Types exchanged with the application core (Tauri commands), generated from
// src-tauri/src/contracts.rs into ./types.ts by `npm run api:generate` — never written by hand.
export type StatusResponse = Types.StatusResponse;
export type StartupStatus = Types.StartupStatus;
export type Grade = Types.Grade;
export type AnswerMode = Types.AnswerMode;
export type Profile = Types.ProfileResponse;
export type UpdateProfile = Types.UpdateProfileRequest;
export type Character = Types.CharacterResponse;
export type Skills = Types.SkillsResponse;
export type Skill = Types.SkillResponse;
export type Series = Types.SeriesResponse;
export type SkillFamily = Types.SkillFamily;
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
export type CommandError = Types.CommandError;
export type Lessons = Types.LessonsResponse;
export type LessonSummary = Types.LessonSummaryResponse;
export type Lesson = Types.LessonResponse;
export type Slide = Types.SlideResponse;
export type Animation = Types.AnimationResponse;
export type Chain = Types.ChainResponse;
export type ChainMode = Types.ChainMode;
export type Jump = Types.JumpResponse;
export type QuizQuestion = Types.QuizQuestionResponse;
export type QuizAnswer = Types.QuizAnswerResponse;

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
