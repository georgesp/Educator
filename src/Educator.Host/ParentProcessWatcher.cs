using System.Diagnostics;

namespace Educator.Host;

/// <summary>
/// Stops the host when the parent process (the Tauri shell) exits, so that a crash of the shell
/// never leaves an orphan backend holding the database.
/// </summary>
internal sealed class ParentProcessWatcher : BackgroundService
{
    private static readonly TimeSpan PollInterval = TimeSpan.FromSeconds(2);

    private readonly int parentProcessId;
    private readonly IHostApplicationLifetime lifetime;
    private readonly ILogger<ParentProcessWatcher> logger;

    public ParentProcessWatcher(int parentProcessId, IHostApplicationLifetime lifetime, ILogger<ParentProcessWatcher> logger)
    {
        this.parentProcessId = parentProcessId;
        this.lifetime = lifetime;
        this.logger = logger;
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        using var timer = new PeriodicTimer(PollInterval);
        while (await timer.WaitForNextTickAsync(stoppingToken).ConfigureAwait(false))
        {
            if (!IsAlive(parentProcessId))
            {
                logger.LogInformation("Parent process {ParentProcessId} has exited: stopping.", parentProcessId);
                lifetime.StopApplication();
                return;
            }
        }
    }

    internal static bool IsAlive(int processId)
    {
        try
        {
            using var process = Process.GetProcessById(processId);
            return !process.HasExited;
        }
        catch (ArgumentException)
        {
            // No process with this id.
            return false;
        }
        catch (InvalidOperationException)
        {
            return false;
        }
    }
}
