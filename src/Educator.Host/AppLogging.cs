using Serilog;
using Serilog.Events;

namespace Educator.Host;

/// <summary>
/// Builds the application-wide Serilog logger. Log files live in <c>AppDataPaths.LogsDirectory</c>,
/// one file per day (<c>educator-yyyyMMdd.log</c>), rolled over at 10 MB and kept 14 days.
/// </summary>
internal static class AppLogging
{
    public const string FileNamePattern = "educator-.log";

    private const long FileSizeLimitBytes = 10 * 1024 * 1024;

    private const string OutputTemplate =
        "{Timestamp:yyyy-MM-dd HH:mm:ss.fff zzz} [{Level:u3}] {SourceContext}: {Message:lj}{NewLine}{Exception}";

    private static readonly TimeSpan RetentionPeriod = TimeSpan.FromDays(14);

    /// <summary>
    /// Creates the logger writing to <paramref name="logsDirectory"/>. Never throws: if the folder
    /// cannot be created, a silent logger is returned so that logging never prevents start-up.
    /// </summary>
    public static Serilog.ILogger CreateLogger(string logsDirectory)
    {
        try
        {
            Directory.CreateDirectory(logsDirectory);

            return new LoggerConfiguration()
                .MinimumLevel.Information()
                .MinimumLevel.Override("Microsoft", LogEventLevel.Warning)
                .MinimumLevel.Override("Microsoft.Hosting.Lifetime", LogEventLevel.Information)
                .Enrich.FromLogContext()
                .WriteTo.File(
                    Path.Combine(logsDirectory, FileNamePattern),
                    outputTemplate: OutputTemplate,
                    rollingInterval: RollingInterval.Day,
                    rollOnFileSizeLimit: true,
                    fileSizeLimitBytes: FileSizeLimitBytes,
                    retainedFileTimeLimit: RetentionPeriod)
                .CreateLogger();
        }
        catch (Exception ex) when (ex is IOException or UnauthorizedAccessException)
        {
            System.Diagnostics.Debug.WriteLine($"Could not initialise file logging in {logsDirectory}: {ex}");
            return new LoggerConfiguration().CreateLogger();
        }
    }
}
