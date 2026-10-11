namespace Educator.Core.Characters;

/// <summary>Profile characters (SUB000101), loaded from <c>content/characters</c>. Implemented in <c>Educator.Content</c>.</summary>
public interface ICharacterCatalog
{
    IReadOnlyList<CharacterInfo> All { get; }

    /// <summary>SVG source of a character, or null when unknown.</summary>
    string? GetSvg(string id);
}

/// <param name="Id">Identifier, e.g. <c>lion</c>.</param>
/// <param name="Name">French name, e.g. « Lion ».</param>
public sealed record CharacterInfo(string Id, string Name, CharacterKind Kind);

public enum CharacterKind
{
    Animal,
    Child,
}
