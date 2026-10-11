namespace Educator.Content;

/// <summary>
/// Location of the pedagogical content (<c>content/</c> at the repository root, copied next to the host
/// at build time). The environment variable <c>EDUCATOR_CONTENT_DIR</c> overrides it (tests, editing the
/// content without rebuilding).
/// </summary>
public static class ContentDirectory
{
    public const string EnvironmentVariable = "EDUCATOR_CONTENT_DIR";

    public const string FolderName = "content";

    public static string Resolve()
    {
        var overridden = Environment.GetEnvironmentVariable(EnvironmentVariable);
        return string.IsNullOrWhiteSpace(overridden)
            ? Path.Combine(AppContext.BaseDirectory, FolderName)
            : Path.GetFullPath(overridden);
    }
}
