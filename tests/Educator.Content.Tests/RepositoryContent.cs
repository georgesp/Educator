namespace Educator.Content.Tests;

/// <summary>The real <c>content/</c> folder of the repository (not the copy in bin/): tests check what is committed.</summary>
internal static class RepositoryContent
{
    private static readonly Lazy<string> Root = new(() =>
    {
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
        {
            if (File.Exists(Path.Combine(dir.FullName, "Educator.sln")))
            {
                return dir.FullName;
            }
        }

        throw new InvalidOperationException("Educator.sln not found above the test output directory.");
    });

    private static readonly Lazy<EducatorContent> Loaded = new(() => EducatorContent.Load(Directory));

    public static string Directory => Path.Combine(Root.Value, "content");

    public static EducatorContent Content => Loaded.Value;

    public static string PathOf(params string[] parts) => Path.Combine([Directory, .. parts]);
}
