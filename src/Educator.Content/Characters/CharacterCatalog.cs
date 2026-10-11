using System.Text.Json;
using System.Text.Json.Serialization;
using System.Xml;
using System.Xml.Linq;
using Educator.Content.Skills;
using Educator.Core.Characters;

namespace Educator.Content.Characters;

/// <summary>
/// Profile characters (SUB000101): <c>content/characters/characters.json</c> plus one SVG per character,
/// loaded once. A character whose entry or SVG is invalid is skipped and reported.
/// </summary>
public sealed class CharacterCatalog : ICharacterCatalog
{
    /// <summary>Named groups every character SVG must contain (accessories of lot 3 attach to them).</summary>
    public static readonly IReadOnlyList<string> RequiredGroups = ["body", "head", "face", "clothes"];

    /// <summary>At least one of these groups: hair for the children, ears or mane for the animals.</summary>
    public static readonly IReadOnlyList<string> TopGroups = ["hair", "ears", "mane"];

    private readonly Dictionary<string, string> svgs;

    private CharacterCatalog(IReadOnlyList<CharacterDefinition> definitions, Dictionary<string, string> svgs)
    {
        Definitions = definitions;
        All = [.. definitions.Select(d => d.Info)];
        this.svgs = svgs;
    }

    public IReadOnlyList<CharacterInfo> All { get; }

    public IReadOnlyList<CharacterDefinition> Definitions { get; }

    public string? GetSvg(string id) => id is not null && svgs.TryGetValue(id, out var svg) ? svg : null;

    public static ContentLoadResult<CharacterCatalog> Load(string charactersDirectory)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(charactersDirectory);

        var problems = new List<ContentProblem>();
        var definitions = new List<CharacterDefinition>();
        var svgs = new Dictionary<string, string>(StringComparer.Ordinal);
        const string indexName = "characters.json";

        CharactersFile? index = null;
        try
        {
            index = JsonSerializer.Deserialize<CharactersFile>(File.ReadAllText(Path.Combine(charactersDirectory, indexName)), SkillLoader.JsonOptions);
        }
        catch (Exception ex) when (ex is IOException or JsonException or UnauthorizedAccessException)
        {
            problems.Add(new ContentProblem(indexName, ex.Message));
        }

        foreach (var entry in index?.Characters ?? [])
        {
            if (definitions.Any(d => d.Info.Id == entry.Id))
            {
                problems.Add(new ContentProblem(indexName, $"Duplicate character id {entry.Id}."));
                continue;
            }

            if (entry.Kind is not ("animal" or "child") || string.IsNullOrWhiteSpace(entry.Name))
            {
                problems.Add(new ContentProblem(indexName, $"{entry.Id}: kind must be animal or child, name is required."));
                continue;
            }

            var path = Path.Combine(charactersDirectory, entry.File);
            var svgProblem = ValidateSvg(path, out var svg);
            if (svgProblem is not null)
            {
                problems.Add(new ContentProblem(entry.File, svgProblem));
                continue;
            }

            var kind = entry.Kind == "animal" ? CharacterKind.Animal : CharacterKind.Child;
            definitions.Add(new CharacterDefinition(new CharacterInfo(entry.Id, entry.Name, kind), entry.File, entry.Anchors));
            svgs[entry.Id] = svg!;
        }

        return new ContentLoadResult<CharacterCatalog>(new CharacterCatalog(definitions, svgs), problems);
    }

    /// <summary>Checks one SVG file: well-formed, 256 × 256 viewBox, named groups, no script or external reference.</summary>
    public static string? ValidateSvg(string path, out string? svg)
    {
        svg = null;
        XDocument document;
        try
        {
            svg = File.ReadAllText(path);
            document = XDocument.Parse(svg);
        }
        catch (Exception ex) when (ex is IOException or XmlException or UnauthorizedAccessException)
        {
            return ex.Message;
        }

        XNamespace ns = "http://www.w3.org/2000/svg";
        var root = document.Root;
        if (root is null || root.Name != ns + "svg")
        {
            return "Root element must be <svg> in the SVG namespace.";
        }

        if ((string?)root.Attribute("viewBox") != "0 0 256 256")
        {
            return "viewBox must be \"0 0 256 256\".";
        }

        var ids = root.Descendants(ns + "g").Select(g => (string?)g.Attribute("id")).ToHashSet();
        var missing = RequiredGroups.Where(g => !ids.Contains(g)).ToArray();
        if (missing.Length > 0)
        {
            return $"Missing groups: {string.Join(", ", missing)}.";
        }

        if (!TopGroups.Any(ids.Contains))
        {
            return $"Needs one of the groups {string.Join(", ", TopGroups)}.";
        }

        var forbidden = root.DescendantsAndSelf().FirstOrDefault(e =>
            e.Name.LocalName is "script" or "foreignObject" or "image" or "text" or "metadata"
            || e.Attributes().Any(a => a.Name.LocalName.StartsWith("on", StringComparison.OrdinalIgnoreCase)
                || (a.Name.LocalName == "href" && !a.Value.StartsWith('#'))));
        return forbidden is null ? null : $"Forbidden element or attribute in <{forbidden.Name.LocalName}>.";
    }

    private sealed record CharactersFile(IReadOnlyList<CharacterEntry> Characters)
    {
        [JsonPropertyName("$schema")]
        public string? Schema { get; init; }
    }

    private sealed record CharacterEntry(string Id, string Name, string File, string Kind, CharacterAnchors Anchors);
}

/// <summary>A character and where its accessories attach (lot 3).</summary>
public sealed record CharacterDefinition(CharacterInfo Info, string File, CharacterAnchors Anchors);

public sealed record CharacterAnchors(AnchorPoint HeadTop, AnchorPoint Eyes, AnchorPoint Neck);

/// <summary>Point in the 256 × 256 viewBox.</summary>
public sealed record AnchorPoint(double X, double Y);
