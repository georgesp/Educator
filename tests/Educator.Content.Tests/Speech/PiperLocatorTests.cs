using Educator.Content.Speech;

namespace Educator.Content.Tests.Speech;

[Trait("Category", "Unit")]
public sealed class PiperLocatorTests : IDisposable
{
    private readonly string temp = Path.Combine(Path.GetTempPath(), "educator-tests", Guid.NewGuid().ToString("N"));

    public void Dispose()
    {
        if (Directory.Exists(temp))
        {
            Directory.Delete(temp, recursive: true);
        }
    }

    [Fact]
    public void Locate_CompleteLayout_ReturnsTheInstallation()
    {
        var root = CreateLayout(Path.Combine(temp, "piper"));

        var result = PiperLocator.Locate(Options(explicitRoot: root));

        Assert.Null(result.Problem);
        var installation = Assert.IsType<PiperInstallation>(result.Installation);
        Assert.Equal(Path.Combine(root, "macos", "piper"), installation.ExecutablePath);
        Assert.Equal(Path.Combine(root, "voices", "fr_FR-siwis-medium.onnx"), installation.ModelPath);
        Assert.Equal(installation.ModelPath + ".json", installation.ModelConfigPath);
        Assert.Equal("fr_FR-siwis-medium", installation.VoiceName);
    }

    [Fact]
    public void Locate_WithoutExplicitRoot_FindsVendorPiperAboveTheApplication()
    {
        CreateLayout(Path.Combine(temp, "vendor", "piper"));
        var appDirectory = Directory.CreateDirectory(Path.Combine(temp, "src", "Educator.Host", "bin", "Debug")).FullName;

        var result = PiperLocator.Locate(Options(baseDirectory: appDirectory));

        Assert.NotNull(result.Installation);
    }

    [Fact]
    public void Locate_PackagedApplication_PrefersThePiperFolderNextToIt()
    {
        var appDirectory = Directory.CreateDirectory(Path.Combine(temp, "app")).FullName;
        var packaged = CreateLayout(Path.Combine(appDirectory, "piper"));
        CreateLayout(Path.Combine(temp, "vendor", "piper"));

        var result = PiperLocator.Locate(Options(baseDirectory: appDirectory));

        Assert.StartsWith(packaged, result.Installation!.ExecutablePath, StringComparison.Ordinal);
    }

    [Fact]
    public void Locate_NothingInstalled_ExplainsHowToFetchPiper()
    {
        var appDirectory = Directory.CreateDirectory(Path.Combine(temp, "empty")).FullName;

        var result = PiperLocator.Locate(Options(baseDirectory: appDirectory));

        Assert.Null(result.Installation);
        Assert.Contains("fetch-piper", result.Problem, StringComparison.Ordinal);
    }

    [Fact]
    public void Locate_MissingVoice_ReportsTheVoice()
    {
        var root = CreateLayout(Path.Combine(temp, "piper"));

        var result = PiperLocator.Locate(Options(explicitRoot: root) with { VoiceName = "fr_FR-tom-medium" });

        Assert.Null(result.Installation);
        Assert.Contains("fr_FR-tom-medium", result.Problem, StringComparison.Ordinal);
    }

    [Fact]
    public void Locate_MissingExecutable_ReportsIt()
    {
        var root = CreateLayout(Path.Combine(temp, "piper"));
        File.Delete(Path.Combine(root, "macos", "piper"));

        var result = PiperLocator.Locate(Options(explicitRoot: root));

        Assert.Contains("executable", result.Problem, StringComparison.Ordinal);
    }

    private PiperLocatorOptions Options(string? explicitRoot = null, string? baseDirectory = null) => new()
    {
        ExplicitRoot = explicitRoot,
        BaseDirectory = baseDirectory ?? temp,
        Platform = PiperPlatform.MacOS,
    };

    private static string CreateLayout(string root)
    {
        Directory.CreateDirectory(Path.Combine(root, "macos", "espeak-ng-data"));
        File.WriteAllText(Path.Combine(root, "macos", "piper"), string.Empty);
        Directory.CreateDirectory(Path.Combine(root, "voices"));
        File.WriteAllText(Path.Combine(root, "voices", "fr_FR-siwis-medium.onnx"), string.Empty);
        File.WriteAllText(Path.Combine(root, "voices", "fr_FR-siwis-medium.onnx.json"), "{}");
        return root;
    }
}
