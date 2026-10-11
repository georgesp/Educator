namespace Educator.Content.Speech;

/// <summary>Files of a usable Piper installation (engine + one voice), as found by <see cref="PiperLocator"/>.</summary>
public sealed record PiperInstallation(
    string ExecutablePath,
    string EspeakDataDirectory,
    string ModelPath,
    string ModelConfigPath,
    string VoiceName)
{
    /// <summary>Folder of the executable; also holds its native libraries.</summary>
    public string EngineDirectory => Path.GetDirectoryName(ExecutablePath)!;
}

/// <summary>Result of <see cref="PiperLocator.Locate"/>: an installation, or why none was found.</summary>
public sealed record PiperLocatorResult(PiperInstallation? Installation, string? Problem)
{
    public static PiperLocatorResult Found(PiperInstallation installation) => new(installation, null);

    public static PiperLocatorResult NotFound(string problem) => new(null, problem);
}
