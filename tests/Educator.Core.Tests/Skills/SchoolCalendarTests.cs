using Educator.Core.Skills;

namespace Educator.Core.Tests.Skills;

[Trait("Category", "Unit")]
public sealed class SchoolCalendarTests
{
    [Theory]
    [InlineData(2026, 9, 1, 1)]
    [InlineData(2026, 10, 11, 1)]
    [InlineData(2026, 11, 3, 1)]
    [InlineData(2026, 11, 4, 2)]
    [InlineData(2026, 12, 31, 2)]
    [InlineData(2027, 1, 4, 2)]
    [InlineData(2027, 1, 5, 3)]
    [InlineData(2027, 3, 8, 3)]
    [InlineData(2027, 3, 9, 4)]
    [InlineData(2028, 2, 29, 3)]
    [InlineData(2027, 5, 4, 5)]
    [InlineData(2027, 7, 14, 5)]
    [InlineData(2027, 8, 31, 5)]
    public void DefaultCalendar(int year, int month, int day, int period)
        => Assert.Equal(period, SchoolCalendar.Default.GetPeriod(new DateOnly(year, month, day)));

    [Fact]
    public void PeriodsOutOfOrder_AreRefused()
        => Assert.Throws<ArgumentException>(() => new SchoolCalendar(
        [
            new PeriodStart(1, 9, 1),
            new PeriodStart(2, 1, 5),
            new PeriodStart(3, 11, 4),
            new PeriodStart(4, 3, 9),
            new PeriodStart(5, 5, 4),
        ]));
}
