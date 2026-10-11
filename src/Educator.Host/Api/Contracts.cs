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

/// <summary>A generated addition. The expected answer is not sent.</summary>
public sealed record AdditionExerciseResponse
{
    public required Guid Id { get; init; }

    public required int Left { get; init; }

    public required int Right { get; init; }

    /// <summary>Written form, e.g. <c>3 + 4</c>.</summary>
    public required string Statement { get; init; }
}

public sealed record AnswerRequest
{
    public required int Answer { get; init; }
}

public sealed record AnswerResponse
{
    public required bool IsCorrect { get; init; }

    public required int ExpectedAnswer { get; init; }
}
