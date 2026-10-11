using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.DependencyInjection;

namespace Educator.Data.Tests;

/// <summary>Real SQLite database in a temporary folder, deleted on dispose.</summary>
internal sealed class TemporaryDatabase : IDisposable
{
    private readonly ServiceProvider provider;

    public TemporaryDatabase(TimeProvider? timeProvider = null)
    {
        Directory = Path.Combine(Path.GetTempPath(), "educator-tests", Guid.NewGuid().ToString("N"));
        DatabasePath = Path.Combine(Directory, AppDataPaths.DatabaseFileName);

        var services = new ServiceCollection();
        if (timeProvider is not null)
        {
            services.AddSingleton(timeProvider);
        }

        services.AddData(DatabasePath);
        provider = services.BuildServiceProvider();
    }

    public string Directory { get; }

    public string DatabasePath { get; }

    public T Get<T>()
        where T : notnull
        => provider.GetRequiredService<T>();

    public IDbContextFactory<AppDbContext> ContextFactory => Get<IDbContextFactory<AppDbContext>>();

    public Task InitializeAsync() => Get<DatabaseInitializer>().InitializeAsync();

    public void Dispose()
    {
        provider.Dispose();
        try
        {
            System.IO.Directory.Delete(Directory, recursive: true);
        }
        catch (IOException)
        {
            // Best effort: the OS may still hold the file for a moment on Windows.
        }
    }
}

/// <summary>Controllable clock for the tests.</summary>
internal sealed class ManualTimeProvider : TimeProvider
{
    public ManualTimeProvider(DateTimeOffset now) => Now = now;

    public DateTimeOffset Now { get; set; }

    public override DateTimeOffset GetUtcNow() => Now;
}
