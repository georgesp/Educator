using System.Text.Json;
using Educator.Data;
using Educator.Host.Api;
using Microsoft.AspNetCore.Hosting.Server;
using Microsoft.AspNetCore.Hosting.Server.Features;
using Serilog;

namespace Educator.Host;

/// <summary>
/// Standalone backend (<c>educator-host</c>): SQLite, Piper, Serilog logs and the local HTTP API,
/// without any UI. Launched by the Tauri shell, or by hand in dev:
/// <code>EDUCATOR_UI_TOKEN=dev dotnet run --project src/Educator.Host -- --port 5250</code>
///
/// <para>
/// Protocol with the parent process (stdout, one JSON object per line):
/// <c>{"event":"ready","baseUrl":"http://127.0.0.1:54321"}</c> once the API listens, or
/// <c>{"event":"error","message":"…"}</c> before exiting with code 1. The UI token is never written:
/// the parent passes it through <c>EDUCATOR_UI_TOKEN</c>.
/// </para>
/// </summary>
public static class Program
{
    public static async Task<int> Main(string[] args)
    {
        // File logging first, so that even a crash during start-up is recorded.
        Log.Logger = AppLogging.CreateLogger(AppDataPaths.LogsDirectory);
        AppDomain.CurrentDomain.UnhandledException += (_, e) =>
            Log.Fatal(e.ExceptionObject as Exception, "Unhandled exception (terminating: {IsTerminating}).", e.IsTerminating);
        TaskScheduler.UnobservedTaskException += (_, e) => Log.Error(e.Exception, "Unobserved task exception.");
        var log = Log.ForContext("SourceContext", "Educator.Host.Program");

        try
        {
            var commandLine = HostCommandLine.Parse(args);
            if (commandLine.ExportOpenApiPath is not null)
            {
                await OpenApiExporter.ExportAsync(commandLine.ExportOpenApiPath).ConfigureAwait(false);
                return 0;
            }

            log.Information(
                "Starting educator-host {Version} on {OS} (port {Port}, parent {ParentProcessId}, data {DataDirectory}).",
                typeof(Program).Assembly.GetName().Version,
                Environment.OSVersion,
                commandLine.Port,
                commandLine.ParentProcessId,
                AppDataPaths.AppDataDirectory);
            if (!LocalApiOptions.IsTokenConfigured)
            {
                log.Warning(
                    "{Variable} is not set: the UI routes (/api/ui) use a random token and cannot be called.",
                    LocalApiOptions.UiTokenEnvironmentVariable);
            }

            await using var app = HostApplication.Build(new HostSettings
            {
                Port = commandLine.Port,
                ParentProcessId = commandLine.ParentProcessId,
                DatabasePath = AppDataPaths.DatabasePath,
                SpeechCacheDirectory = Path.Combine(AppDataPaths.AppDataDirectory, "speech-cache"),
            });

            var database = app.Services.GetRequiredService<DatabaseInitializer>();
            await database.InitializeAsync().ConfigureAwait(false);
            if (database.LastBackupPath is not null)
            {
                log.Information("Database backed up to {BackupPath} before migration.", database.LastBackupPath);
            }

            await app.StartAsync().ConfigureAwait(false);
            var baseUrl = GetBaseUrl(app);
            log.Information("Local API listening on {BaseUrl}.", baseUrl);
            WriteToParent(new { @event = "ready", baseUrl });

            await app.WaitForShutdownAsync().ConfigureAwait(false);
            log.Information("educator-host stopped.");
            return 0;
        }
        catch (Exception ex)
        {
            log.Fatal(ex, "educator-host terminated unexpectedly.");
            WriteToParent(new { @event = "error", message = ex.Message });
            return 1;
        }
        finally
        {
            await Log.CloseAndFlushAsync().ConfigureAwait(false);
        }
    }

    /// <summary>Actual address (with <c>--port 0</c>, the port chosen by the OS).</summary>
    internal static string GetBaseUrl(WebApplication app)
        => app.Services.GetRequiredService<IServer>().Features.Get<IServerAddressesFeature>()?.Addresses.FirstOrDefault()
           ?? app.Urls.First();

    private static void WriteToParent(object message)
    {
        Console.Out.WriteLine(JsonSerializer.Serialize(message));
        Console.Out.Flush();
    }
}
