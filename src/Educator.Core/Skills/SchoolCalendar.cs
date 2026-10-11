namespace Educator.Core.Skills;

/// <summary>
/// Splits the school year into 5 periods (rentrée → Toussaint → Noël → hiver → printemps → été).
/// The start dates are content (<c>content/calendar/periods.json</c>), chosen after the holidays of
/// every zone so that the result does not depend on the holiday zone. July and August belong to period 5.
/// </summary>
public sealed class SchoolCalendar
{
    private readonly IReadOnlyList<PeriodStart> starts;

    /// <param name="starts">One start per period 1 to 5, in school-year order (period 1 starts the year).</param>
    public SchoolCalendar(IReadOnlyList<PeriodStart> starts)
    {
        ArgumentNullException.ThrowIfNull(starts);
        if (starts.Count != Skill.LastPeriod
            || starts.Select((s, i) => s.Period == i + 1).Any(ok => !ok))
        {
            throw new ArgumentException("Exactly one start per period 1 to 5, in order.", nameof(starts));
        }

        var offsets = starts.Select(s => OffsetInSchoolYear(s.Month, s.Day, starts[0])).ToArray();
        if (offsets.Zip(offsets.Skip(1)).Any(pair => pair.First >= pair.Second))
        {
            throw new ArgumentException("Period starts must follow each other within the school year.", nameof(starts));
        }

        this.starts = starts;
    }

    /// <summary>Default dates: 1 Sept, 4 Nov, 5 Jan, 9 March, 4 May.</summary>
    public static SchoolCalendar Default { get; } = new(
    [
        new PeriodStart(1, 9, 1),
        new PeriodStart(2, 11, 4),
        new PeriodStart(3, 1, 5),
        new PeriodStart(4, 3, 9),
        new PeriodStart(5, 5, 4),
    ]);

    public IReadOnlyList<PeriodStart> Starts => starts;

    public int GetPeriod(DateOnly date)
    {
        var offset = OffsetInSchoolYear(date.Month, date.Day, starts[0]);
        var period = 1;
        foreach (var start in starts)
        {
            if (offset >= OffsetInSchoolYear(start.Month, start.Day, starts[0]))
            {
                period = start.Period;
            }
        }

        return period;
    }

    /// <summary>Days since the start of period 1, in a non-leap reference year.</summary>
    private static int OffsetInSchoolYear(int month, int day, PeriodStart yearStart)
    {
        const int referenceYear = 2001; // not a leap year: 29 Feb is counted as 1 March
        var dayOfYear = new DateOnly(referenceYear, month, month == 2 && day == 29 ? 28 : day).DayOfYear;
        var startDayOfYear = new DateOnly(referenceYear, yearStart.Month, yearStart.Day).DayOfYear;
        return (dayOfYear - startDayOfYear + 365) % 365;
    }
}

/// <summary>First day of a period (month and day, every year).</summary>
public sealed record PeriodStart(int Period, int Month, int Day);
