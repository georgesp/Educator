namespace Educator.Content.Speech;

/// <summary>
/// Finds the Piper engine and the voice. Expected layout (produced by <c>scripts/fetch-piper.sh</c> /
/// <c>scripts/fetch-piper.ps1</c>, see <c>docs/adr/0002-voix-piper.md</c>):
/// <code>
/// piper/                       ← root
///   macos/ | windows/ | linux/ ← engine of each platform: piper[.exe], native libraries, espeak-ng-data/
///   voices/                    ← &lt;voice&gt;.onnx and &lt;voice&gt;.onnx.json
/// </code>
/// Root search order: <see cref="PiperLocatorOptions.ExplicitRoot"/> (environment variable
/// <c>EDUCATOR_PIPER_DIR</c>), then <c>&lt;app&gt;/piper</c> (packaged application), then the first
/// <c>vendor/piper</c> found above the application folder (development, repository root).
/// </summary>
public static class PiperLocator
{
    public const string RootEnvironmentVariable = "EDUCATOR_PIPER_DIR";
    public const string VoiceEnvironmentVariable = "EDUCATOR_PIPER_VOICE";
    public const string DefaultVoice = "fr_FR-siwis-medium";

    public static PiperLocatorOptions OptionsFromEnvironment() => new()
    {
        ExplicitRoot = NullIfBlank(Environment.GetEnvironmentVariable(RootEnvironmentVariable)),
        VoiceName = NullIfBlank(Environment.GetEnvironmentVariable(VoiceEnvironmentVariable)) ?? DefaultVoice,
        BaseDirectory = AppContext.BaseDirectory,
        Platform = CurrentPlatform(),
    };

    public static PiperLocatorResult Locate(PiperLocatorOptions options)
    {
        ArgumentNullException.ThrowIfNull(options);

        var root = FindRoot(options);
        if (root is null)
        {
            return PiperLocatorResult.NotFound(
                $"Piper not found: set {RootEnvironmentVariable} or run scripts/fetch-piper to fill vendor/piper.");
        }

        var engineDirectory = Path.Combine(root, options.Platform.FolderName);
        var executable = Path.Combine(engineDirectory, options.Platform.ExecutableName);
        if (!File.Exists(executable))
        {
            return PiperLocatorResult.NotFound($"Piper executable missing: {executable}.");
        }

        var espeakData = Path.Combine(engineDirectory, "espeak-ng-data");
        if (!Directory.Exists(espeakData))
        {
            return PiperLocatorResult.NotFound($"espeak-ng data missing: {espeakData}.");
        }

        var model = Path.Combine(root, "voices", options.VoiceName + ".onnx");
        var config = model + ".json";
        if (!File.Exists(model) || !File.Exists(config))
        {
            return PiperLocatorResult.NotFound($"Voice {options.VoiceName} missing: {model} (+ .json).");
        }

        return PiperLocatorResult.Found(new PiperInstallation(executable, espeakData, model, config, options.VoiceName));
    }

    public static PiperPlatform CurrentPlatform() =>
        OperatingSystem.IsWindows() ? PiperPlatform.Windows
        : OperatingSystem.IsMacOS() ? PiperPlatform.MacOS
        : PiperPlatform.Linux;

    private static string? FindRoot(PiperLocatorOptions options)
    {
        if (options.ExplicitRoot is not null)
        {
            return Directory.Exists(options.ExplicitRoot) ? Path.GetFullPath(options.ExplicitRoot) : null;
        }

        var packaged = Path.Combine(options.BaseDirectory, "piper");
        if (Directory.Exists(packaged))
        {
            return packaged;
        }

        for (var dir = new DirectoryInfo(options.BaseDirectory); dir is not null; dir = dir.Parent)
        {
            var candidate = Path.Combine(dir.FullName, "vendor", "piper");
            if (Directory.Exists(candidate))
            {
                return candidate;
            }
        }

        return null;
    }

    private static string? NullIfBlank(string? value) => string.IsNullOrWhiteSpace(value) ? null : value.Trim();
}

/// <summary>Where to look for Piper.</summary>
public sealed record PiperLocatorOptions
{
    public string? ExplicitRoot { get; init; }

    public string VoiceName { get; init; } = PiperLocator.DefaultVoice;

    public required string BaseDirectory { get; init; }

    public required PiperPlatform Platform { get; init; }
}

/// <summary>Engine folder and executable name of one operating system.</summary>
public sealed record PiperPlatform(string FolderName, string ExecutableName, string? LibraryPathVariable)
{
    public static readonly PiperPlatform MacOS = new("macos", "piper", "DYLD_LIBRARY_PATH");
    public static readonly PiperPlatform Windows = new("windows", "piper.exe", null);
    public static readonly PiperPlatform Linux = new("linux", "piper", "LD_LIBRARY_PATH");
}
