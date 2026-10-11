namespace Educator.Core.Profiles;

/// <summary>A child's local profile (SUB000102). No password: every profile is open to everyone.</summary>
public sealed record Profile
{
    public const int MaxNameLength = 20;

    public required Guid Id { get; init; }

    public required string Name { get; init; }

    public required Grade Grade { get; init; }

    /// <summary>Id from <c>characters.json</c>; becomes the avatar configuration in lot 3.</summary>
    public required string CharacterId { get; init; }

    public required int Points { get; init; }

    public DateTimeOffset CreatedAt { get; init; }

    public DateTimeOffset UpdatedAt { get; init; }

    public required DateTimeOffset LastUsedAt { get; init; }

    /// <summary>Key of the case-insensitive uniqueness of names.</summary>
    public static string NormalizeName(string name)
    {
        ArgumentNullException.ThrowIfNull(name);
        return CleanName(name).ToUpperInvariant();
    }

    /// <summary>Name as stored: trimmed, inner spaces collapsed.</summary>
    public static string CleanName(string name)
    {
        ArgumentNullException.ThrowIfNull(name);
        return string.Join(' ', name.Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries));
    }
}
