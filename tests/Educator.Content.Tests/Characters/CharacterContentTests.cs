using System.Text.Json.Nodes;
using System.Text.RegularExpressions;
using Educator.Content.Characters;
using Educator.Core.Characters;

namespace Educator.Content.Tests.Characters;

/// <summary>SUB000101: the character set is complete, valid and drawn with the shared palette.</summary>
[Trait("Category", "Unit")]
public sealed partial class CharacterContentTests
{
    private static CharacterCatalog Catalog => RepositoryContent.Content.Characters;

    [Fact]
    public void AtLeastElevenCharacters_FiveAnimalsAndThreeBoysThreeGirls()
    {
        Assert.True(Catalog.All.Count >= 11);
        Assert.Equal(["cochon", "lapin", "lion", "ours", "renard"], Catalog.All.Where(c => c.Kind == CharacterKind.Animal).Select(c => c.Id).Order());
        Assert.True(Catalog.All.Count(c => c.Id.StartsWith("garcon-", StringComparison.Ordinal)) >= 3);
        Assert.True(Catalog.All.Count(c => c.Id.StartsWith("fille-", StringComparison.Ordinal)) >= 3);
    }

    [Fact]
    public void EveryCharacter_HasItsValidSvgAndAnchorsInsideTheViewBox()
    {
        foreach (var definition in Catalog.Definitions)
        {
            var problem = CharacterCatalog.ValidateSvg(RepositoryContent.PathOf("characters", definition.File), out _);
            Assert.True(problem is null, $"{definition.File}: {problem}");
            foreach (var point in new[] { definition.Anchors.HeadTop, definition.Anchors.Eyes, definition.Anchors.Neck })
            {
                Assert.InRange(point.X, 0, 256);
                Assert.InRange(point.Y, 0, 256);
            }
        }
    }

    [Fact]
    public void EverySvgFile_IsListed()
    {
        var listed = Catalog.Definitions.Select(d => d.File).Order();
        var files = Directory.GetFiles(RepositoryContent.PathOf("characters"), "*.svg").Select(Path.GetFileName).Order();

        Assert.Equal(files, listed);
    }

    [Fact]
    public void Colors_ComeFromThePalette()
    {
        var palette = JsonNode.Parse(File.ReadAllText(RepositoryContent.PathOf("characters", "palette.json")))!["colors"]!.AsObject();

        foreach (var definition in Catalog.Definitions)
        {
            var svg = Catalog.GetSvg(definition.Info.Id)!;
            var uses = PaletteVariable().Matches(svg);
            Assert.NotEmpty(uses);
            foreach (Match use in uses)
            {
                var name = use.Groups["name"].Value;
                Assert.True(palette.ContainsKey(name), $"{definition.File}: unknown palette color {name}");
                Assert.Equal(palette[name]!.GetValue<string>(), use.Groups["value"].Value);
            }

            Assert.DoesNotMatch(LiteralColor(), PaletteVariable().Replace(svg, string.Empty));
        }
    }

    [Fact]
    public void InvalidSvg_IsRefused()
    {
        var file = Path.Combine(Path.GetTempPath(), $"educator-{Guid.NewGuid():N}.svg");
        try
        {
            File.WriteAllText(file, """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><g id="head"/><script>alert(1)</script></svg>""");
            Assert.NotNull(CharacterCatalog.ValidateSvg(file, out _));
            File.WriteAllText(file, """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><g id="body"/><g id="head"/><g id="face"/><g id="clothes"/><g id="ears"/><a href="https://example.com"/></svg>""");
            Assert.Contains("Forbidden", CharacterCatalog.ValidateSvg(file, out _), StringComparison.Ordinal);
        }
        finally
        {
            File.Delete(file);
        }
    }

    [GeneratedRegex(@"var\(--(?<name>[a-z-]+), (?<value>#[0-9A-F]{6})\)")]
    private static partial Regex PaletteVariable();

    [GeneratedRegex("#[0-9A-Fa-f]{3,6}\\b")]
    private static partial Regex LiteralColor();
}
