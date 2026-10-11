namespace Educator.Data.Entities;

/// <summary>Row of the <c>AppSetting</c> table: one application setting (key / value).</summary>
public sealed class AppSettingEntity : ITimestamped
{
    public required string Key { get; set; }

    public required string Value { get; set; }

    public DateTimeOffset CreatedAt { get; set; }

    public DateTimeOffset UpdatedAt { get; set; }
}
