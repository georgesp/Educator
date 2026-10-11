namespace Educator.Data;

/// <summary>Time helpers aligned with the database precision.</summary>
internal static class TimeProviderExtensions
{
    /// <summary>
    /// Current UTC time truncated to the millisecond — the precision of the <c>INTEGER</c>
    /// (Unix milliseconds) columns, so values read back compare equal to the ones written.
    /// </summary>
    public static DateTimeOffset GetUtcNowMilliseconds(this TimeProvider timeProvider)
        => DateTimeOffset.FromUnixTimeMilliseconds(timeProvider.GetUtcNow().ToUnixTimeMilliseconds());
}
