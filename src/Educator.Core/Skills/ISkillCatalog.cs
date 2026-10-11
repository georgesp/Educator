namespace Educator.Core.Skills;

/// <summary>The skills loaded from the content (read-only). Implemented in <c>Educator.Content</c>.</summary>
public interface ISkillCatalog
{
    IReadOnlyList<Skill> All { get; }

    Skill? Find(string id);
}

public static class SkillCatalogExtensions
{
    /// <summary>Grade offered to the grades that have no skill yet (CE2 to CM2 in lot 1, SUB000108).</summary>
    public const Grade FallbackGrade = Grade.CE1;

    /// <summary>
    /// Skills of a grade; when it has none yet, those of <see cref="FallbackGrade"/>.
    /// <c>Grade</c> is the grade whose skills are returned.
    /// </summary>
    public static (Grade Grade, IReadOnlyList<Skill> Skills) ForOrFallback(this ISkillCatalog catalog, Subject subject, Grade grade)
    {
        var skills = catalog.For(subject, grade);
        return skills.Count > 0 || grade == FallbackGrade ? (grade, skills) : (FallbackGrade, catalog.For(subject, FallbackGrade));
    }

    /// <summary>Skills of a subject and a grade, sorted by period then id.</summary>
    public static IReadOnlyList<Skill> For(this ISkillCatalog catalog, Subject subject, Grade grade)
    {
        ArgumentNullException.ThrowIfNull(catalog);
        return
        [
            .. catalog.All
                .Where(s => s.Subject == subject && s.Grade == grade)
                .OrderBy(s => s.Period)
                .ThenBy(s => s.Id, StringComparer.Ordinal),
        ];
    }
}
