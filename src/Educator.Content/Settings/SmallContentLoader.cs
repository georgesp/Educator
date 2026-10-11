using System.Text.Json;
using System.Text.Json.Serialization;
using Educator.Content.Skills;
using Educator.Core.Rewards;
using Educator.Core.Skills;

namespace Educator.Content.Settings;

/// <summary>
/// Loads the small content files: school periods (<c>content/calendar/periods.json</c>) and points scale
/// (<c>content/rewards/points.json</c>). An invalid file is reported and the built-in default is used.
/// </summary>
public static class SmallContentLoader
{
    public static ContentLoadResult<SchoolCalendar> LoadCalendar(string path)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(path);
        return Load(path, SchoolCalendar.Default, (CalendarFile file) =>
            new SchoolCalendar([.. file.Periods.Select(p => new PeriodStart(p.Period, p.Month, p.Day))]));
    }

    public static ContentLoadResult<PointsRules> LoadPoints(string path)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(path);
        return Load(path, PointsRules.Default, (PointsFile file) =>
        {
            if (file.PerCorrectAnswer < 0 || file.SeriesCompletedBonus < 0 || file.PerfectSeriesBonus < 0)
            {
                throw new ArgumentException("Points cannot be negative.");
            }

            return new PointsRules(file.PerCorrectAnswer, file.SeriesCompletedBonus, file.PerfectSeriesBonus);
        });
    }

    private static ContentLoadResult<T> Load<T, TFile>(string path, T fallback, Func<TFile, T> convert)
    {
        var name = Path.GetFileName(path);
        try
        {
            var file = JsonSerializer.Deserialize<TFile>(File.ReadAllText(path), SkillLoader.JsonOptions)
                ?? throw new JsonException("Empty file.");
            return new ContentLoadResult<T>(convert(file), []);
        }
        catch (Exception ex) when (ex is IOException or JsonException or ArgumentException or UnauthorizedAccessException)
        {
            return new ContentLoadResult<T>(fallback, [new ContentProblem(name, ex.Message)]);
        }
    }

    private sealed record CalendarFile(IReadOnlyList<PeriodFile> Periods)
    {
        [JsonPropertyName("$schema")]
        public string? Schema { get; init; }

        public string? Comment { get; init; }
    }

    private sealed record PeriodFile(int Period, int Month, int Day);

    private sealed record PointsFile(int PerCorrectAnswer, int SeriesCompletedBonus, int PerfectSeriesBonus)
    {
        [JsonPropertyName("$schema")]
        public string? Schema { get; init; }
    }
}
