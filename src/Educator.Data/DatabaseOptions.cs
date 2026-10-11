namespace Educator.Data;

/// <summary>Location of the SQLite database file.</summary>
public sealed record DatabaseOptions
{
    public required string DatabasePath { get; init; }
}
