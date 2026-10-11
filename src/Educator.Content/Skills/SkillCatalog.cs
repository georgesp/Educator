using Educator.Core.Skills;

namespace Educator.Content.Skills;

/// <summary><see cref="ISkillCatalog"/> over the skills loaded at start-up.</summary>
public sealed class SkillCatalog : ISkillCatalog
{
    private readonly Dictionary<string, Skill> byId;

    public SkillCatalog(IReadOnlyList<Skill> skills)
    {
        ArgumentNullException.ThrowIfNull(skills);
        All = skills;
        byId = skills.ToDictionary(s => s.Id, StringComparer.Ordinal);
    }

    public IReadOnlyList<Skill> All { get; }

    public Skill? Find(string id) => id is not null && byId.TryGetValue(id, out var skill) ? skill : null;
}
