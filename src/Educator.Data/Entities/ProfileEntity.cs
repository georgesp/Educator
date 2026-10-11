using Educator.Core;

namespace Educator.Data.Entities;

/// <summary>Row of the <c>Profile</c> table (SUB000102).</summary>
public sealed class ProfileEntity : ITimestamped
{
    public Guid Id { get; set; }

    public required string Name { get; set; }

    /// <summary>Upper-case name, unique: two profiles cannot differ by case only.</summary>
    public required string NormalizedName { get; set; }

    public Grade Grade { get; set; }

    public required string CharacterId { get; set; }

    public int Points { get; set; }

    public DateTimeOffset CreatedAt { get; set; }

    public DateTimeOffset UpdatedAt { get; set; }

    public DateTimeOffset LastUsedAt { get; set; }
}
