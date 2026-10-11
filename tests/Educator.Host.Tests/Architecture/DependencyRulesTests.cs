using System.Reflection;
using System.Xml.Linq;

namespace Educator.Host.Tests.Architecture;

/// <summary>
/// Enforces the dependency rule of ADR 0001:
/// Core references nothing; Data and Content reference Core only; Host references everything.
/// Checked twice: on the project files (declared references) and on the compiled assemblies
/// (references actually used by the code).
/// </summary>
[Trait("Category", "Unit")]
public sealed class DependencyRulesTests
{
    // Package / assembly name prefixes that belong to infrastructure and must never reach Core.
    private static readonly string[] InfrastructurePrefixes =
    [
        "Microsoft.EntityFrameworkCore",
        "Microsoft.Data.Sqlite",
        "SQLitePCLRaw",
        "Microsoft.AspNetCore",
        "Microsoft.Extensions.Hosting",
        "Microsoft.Extensions.Http",
        "Serilog",
    ];

    public static TheoryData<string, string[]> AllowedProjectReferences => new()
    {
        { "Educator.Core", [] },
        { "Educator.Data", ["Educator.Core"] },
        { "Educator.Content", ["Educator.Core"] },
        { "Educator.Host", ["Educator.Core", "Educator.Data", "Educator.Content"] },
    };

    [Theory]
    [MemberData(nameof(AllowedProjectReferences))]
    public void ProjectFile_OnlyReferencesAllowedProjects(string project, string[] allowed)
    {
        var references = ReadProjectFile(project)
            .Descendants("ProjectReference")
            .Select(e => Path.GetFileNameWithoutExtension(e.Attribute("Include")!.Value.Replace('\\', '/')))
            .ToArray();

        var forbidden = references.Except(allowed).ToArray();

        Assert.True(forbidden.Length == 0, $"{project} must not reference: {string.Join(", ", forbidden)}");
    }

    [Theory]
    [MemberData(nameof(AllowedProjectReferences))]
    public void Assembly_OnlyReferencesAllowedProjects(string project, string[] allowed)
    {
        var forbidden = LoadAssembly(project)
            .GetReferencedAssemblies()
            .Select(a => a.Name!)
            .Where(name => name.StartsWith("Educator.", StringComparison.Ordinal))
            .Except(allowed)
            .ToArray();

        Assert.True(forbidden.Length == 0, $"{project} must not reference: {string.Join(", ", forbidden)}");
    }

    [Fact]
    public void Core_ProjectFile_HasNoPackageReference()
    {
        var packages = ReadProjectFile("Educator.Core")
            .Descendants()
            .Where(e => e.Name.LocalName is "PackageReference" or "FrameworkReference")
            .Select(e => e.Attribute("Include")!.Value)
            .ToArray();

        Assert.True(packages.Length == 0, $"Educator.Core must stay pure, found: {string.Join(", ", packages)}");
    }

    [Fact]
    public void Core_Assembly_DoesNotUseInfrastructure()
    {
        var forbidden = LoadAssembly("Educator.Core")
            .GetReferencedAssemblies()
            .Select(a => a.Name!)
            .Where(name => InfrastructurePrefixes.Any(p => name.StartsWith(p, StringComparison.Ordinal)))
            .ToArray();

        Assert.True(forbidden.Length == 0, $"Educator.Core must not use: {string.Join(", ", forbidden)}");
    }

    private static Assembly LoadAssembly(string project) =>
        project == "Educator.Host"
            ? typeof(Program).Assembly
            : Assembly.Load(new AssemblyName(project));

    private static XDocument ReadProjectFile(string project) =>
        XDocument.Load(Path.Combine(RepositoryRoot.Value, "src", project, $"{project}.csproj"));

    private static readonly Lazy<string> RepositoryRoot = new(() =>
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
}
