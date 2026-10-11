namespace Educator.Data;

/// <summary>
/// Per-user locations of the application's data files.
/// <list type="bullet">
///   <item><description>macOS: <c>~/Library/Application Support/Educator/</c></description></item>
///   <item><description>Windows: <c>%APPDATA%\Educator\</c></description></item>
///   <item><description>Linux: <c>$XDG_CONFIG_HOME/Educator/</c> (or <c>~/.config</c>), CI only</description></item>
/// </list>
/// The environment variable <c>EDUCATOR_DATA_DIR</c> overrides the folder (tests, debugging).
/// </summary>
public static class AppDataPaths
{
    public const string AppFolderName = "Educator";

    public const string DatabaseFileName = "educator.db";

    public const string LogsFolderName = "logs";

    public const string DataDirectoryEnvironmentVariable = "EDUCATOR_DATA_DIR";

    /// <summary>Folder holding every data file of the application.</summary>
    public static string AppDataDirectory { get; } = ResolveAppDataDirectory();

    public static string DatabasePath => Path.Combine(AppDataDirectory, DatabaseFileName);

    public static string LogsDirectory => Path.Combine(AppDataDirectory, LogsFolderName);

    private static string ResolveAppDataDirectory()
    {
        var overridden = Environment.GetEnvironmentVariable(DataDirectoryEnvironmentVariable);
        if (!string.IsNullOrWhiteSpace(overridden))
        {
            return Path.GetFullPath(overridden);
        }

        // SpecialFolder.ApplicationData maps to ~/.config on macOS; the platform
        // convention there is ~/Library/Application Support.
        var baseDirectory = OperatingSystem.IsMacOS()
            ? Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), "Library", "Application Support")
            : Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData);

        return Path.Combine(baseDirectory, AppFolderName);
    }
}
