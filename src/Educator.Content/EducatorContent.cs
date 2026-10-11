using Educator.Content.Characters;
using Educator.Content.Settings;
using Educator.Content.Skills;
using Educator.Core.Rewards;
using Educator.Core.Skills;

namespace Educator.Content;

/// <summary>
/// Everything read from the <c>content/</c> folder, loaded once at start-up. Invalid files are skipped
/// (or replaced by defaults) and listed in <see cref="Problems"/>: the host logs them, the tests fail on them.
/// </summary>
public sealed class EducatorContent
{
    private EducatorContent(
        string directory,
        SkillCatalog skills,
        SchoolCalendar calendar,
        PointsRules points,
        CharacterCatalog characters,
        IReadOnlyList<ContentProblem> problems)
    {
        Directory = directory;
        Skills = skills;
        Calendar = calendar;
        Points = points;
        Characters = characters;
        Problems = problems;
    }

    public string Directory { get; }

    public SkillCatalog Skills { get; }

    public SchoolCalendar Calendar { get; }

    public PointsRules Points { get; }

    public CharacterCatalog Characters { get; }

    public IReadOnlyList<ContentProblem> Problems { get; }

    public static EducatorContent Load(string directory)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(directory);

        var skills = SkillLoader.LoadDirectory(Path.Combine(directory, "skills"));
        var calendar = SmallContentLoader.LoadCalendar(Path.Combine(directory, "calendar", "periods.json"));
        var points = SmallContentLoader.LoadPoints(Path.Combine(directory, "rewards", "points.json"));
        var characters = CharacterCatalog.Load(Path.Combine(directory, "characters"));

        return new EducatorContent(
            directory,
            new SkillCatalog(skills.Value),
            calendar.Value,
            points.Value,
            characters.Value,
            [.. skills.Problems, .. calendar.Problems, .. points.Problems, .. characters.Problems]);
    }
}
