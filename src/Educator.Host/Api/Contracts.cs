using Educator.Core;
using Educator.Core.Characters;
using Educator.Core.Exercises;
using Educator.Core.Skills;

namespace Educator.Host.Api;

// DTOs of the UI API. Every property is `required` so that the OpenAPI schema (and the generated
// TypeScript types) mark it as always present.

/// <summary><c>GET /api/ui/status</c>.</summary>
public sealed record StatusResponse
{
    public required string Version { get; init; }

    public required DatabaseStatusResponse Database { get; init; }

    public required SpeechStatusResponse Speech { get; init; }
}

public sealed record DatabaseStatusResponse
{
    /// <summary>True once the database is created and migrated.</summary>
    public required bool IsReady { get; init; }
}

public sealed record SpeechStatusResponse
{
    public required bool IsAvailable { get; init; }

    /// <summary>Voice name, when available (e.g. <c>fr_FR-siwis-medium</c>).</summary>
    public required string? Voice { get; init; }

    /// <summary>Why the voice is unavailable (English, diagnostics only).</summary>
    public required string? Problem { get; init; }
}

// ── Skills (SUB000106) ───────────────────────────────────────────────────────────────────────────

/// <summary><c>GET /api/ui/skills</c>: the skills of a grade, sorted by period.</summary>
public sealed record SkillsResponse
{
    /// <summary>Grade asked for.</summary>
    public required Grade Grade { get; init; }

    /// <summary>
    /// Grade of the returned skills: the asked one, or CE1 when the asked grade has no skill yet
    /// (« Les exercices de ta classe arrivent bientôt »).
    /// </summary>
    public required Grade SkillsGrade { get; init; }

    /// <summary>School period of today (1 to 5), from <c>content/calendar/periods.json</c>.</summary>
    public required int CurrentPeriod { get; init; }

    public required IReadOnlyList<SkillResponse> Skills { get; init; }
}

public sealed record SkillResponse
{
    public required string Id { get; init; }

    public required string Title { get; init; }

    public required string Instruction { get; init; }

    public required int Period { get; init; }

    /// <summary>True for the skills of the current period (« C'est ce que tu travailles en ce moment »).</summary>
    public required bool IsCurrentPeriod { get; init; }

    public required IReadOnlyList<AnswerMode> AnswerModes { get; init; }
}

// ── Characters (SUB000101) ───────────────────────────────────────────────────────────────────────

public sealed record CharacterResponse
{
    public required string Id { get; init; }

    public required string Name { get; init; }

    public required CharacterKind Kind { get; init; }
}

// ── Profiles (SUB000102) ─────────────────────────────────────────────────────────────────────────

public sealed record ProfileResponse
{
    public required Guid Id { get; init; }

    public required string Name { get; init; }

    public required Grade Grade { get; init; }

    public required string CharacterId { get; init; }

    public required int Points { get; init; }

    public required DateTimeOffset LastUsedAt { get; init; }
}

public sealed record CreateProfileRequest
{
    public required string Name { get; init; }

    public required Grade Grade { get; init; }

    /// <summary>Character shown on the creation screen (<c>GET /api/ui/characters/draw</c>); drawn at random when absent.</summary>
    public string? CharacterId { get; init; }
}

/// <summary>Only the given fields change.</summary>
public sealed record UpdateProfileRequest
{
    public string? Name { get; init; }

    public Grade? Grade { get; init; }

    public string? CharacterId { get; init; }
}

// ── Series (SUB000107, SUB000110) ────────────────────────────────────────────────────────────────

public sealed record StartSeriesRequest
{
    public required Guid ProfileId { get; init; }

    public required string SkillId { get; init; }

    public required AnswerMode AnswerMode { get; init; }

    /// <summary>Fixed seed to replay a series (tests, debugging); random when absent.</summary>
    public int? Seed { get; init; }
}

/// <summary>A new series. The expected answers are not sent.</summary>
public sealed record SeriesResponse
{
    public required Guid SeriesId { get; init; }

    public required int Seed { get; init; }

    public required string SkillId { get; init; }

    public required string Instruction { get; init; }

    public required AnswerMode AnswerMode { get; init; }

    public required IReadOnlyList<ExerciseResponse> Exercises { get; init; }
}

/// <summary>
/// One exercise: <c>Left Operator Right = Result</c> with the asked number (<see cref="Hidden"/>) set to null.
/// </summary>
public sealed record ExerciseResponse
{
    public required int Index { get; init; }

    public required int? Left { get; init; }

    /// <summary><c>+</c> or <c>−</c> (U+2212).</summary>
    public required string Operator { get; init; }

    public required int? Right { get; init; }

    public required int? Result { get; init; }

    public required HiddenPart Hidden { get; init; }

    /// <summary>Written form with <c>?</c> for the asked number, e.g. <c>7 + ? = 12</c>.</summary>
    public required string Prompt { get; init; }

    /// <summary>French question to read aloud (send it to <c>POST /api/ui/speech</c>).</summary>
    public required string SpokenPrompt { get; init; }

    /// <summary>Four proposals in QCM mode, otherwise null.</summary>
    public required IReadOnlyList<int>? Choices { get; init; }
}

public sealed record SeriesAnswerRequest
{
    public required int Index { get; init; }

    public required int Answer { get; init; }
}

public sealed record SeriesAnswerResponse
{
    public required bool Correct { get; init; }

    public required int Expected { get; init; }

    /// <summary>Complete equality shown and read after a mistake, e.g. <c>7 + 5 = 12</c>.</summary>
    public required string Explanation { get; init; }

    /// <summary>Set when this answer ended the series: points awarded.</summary>
    public required SeriesSummaryResponse? Summary { get; init; }
}

public sealed record SeriesSummaryResponse
{
    public required int CorrectCount { get; init; }

    public required int ExerciseCount { get; init; }

    public required int PointsEarned { get; init; }

    /// <summary>New total of the profile.</summary>
    public required int ProfilePoints { get; init; }
}

// ── Speech (SUB000111) ───────────────────────────────────────────────────────────────────────────

public sealed record SpeechRequest
{
    /// <summary>French text; numbers and calculation signs are read as words (« 7 + 5 » → « sept plus cinq »).</summary>
    public required string Text { get; init; }

    /// <summary>Speaking rate, 0.5 to 1.5; slightly slow by default for young children.</summary>
    public double? Speed { get; init; }
}
