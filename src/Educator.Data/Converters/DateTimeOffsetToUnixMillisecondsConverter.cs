using Microsoft.EntityFrameworkCore.Storage.ValueConversion;

namespace Educator.Data.Converters;

/// <summary>
/// Stores a <see cref="DateTimeOffset"/> as UTC Unix milliseconds (<c>INTEGER</c>).
/// SQLite has no native date type: this keeps values sortable and comparable in SQL.
/// The offset is not preserved — every value is read back in UTC.
/// </summary>
public sealed class DateTimeOffsetToUnixMillisecondsConverter : ValueConverter<DateTimeOffset, long>
{
    public DateTimeOffsetToUnixMillisecondsConverter()
        : base(
            value => value.ToUnixTimeMilliseconds(),
            value => DateTimeOffset.FromUnixTimeMilliseconds(value))
    {
    }
}
