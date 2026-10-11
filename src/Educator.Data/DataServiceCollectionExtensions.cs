using Educator.Core.Settings;
using Educator.Data.Interceptors;
using Educator.Data.Settings;
using Microsoft.Data.Sqlite;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.DependencyInjection.Extensions;

namespace Educator.Data;

/// <summary>DI registration of the local storage layer.</summary>
public static class DataServiceCollectionExtensions
{
    /// <summary>
    /// Registers the <see cref="AppDbContext"/> factory, the <see cref="DatabaseInitializer"/>
    /// and the stores. <see cref="TimeProvider.System"/> is used unless a TimeProvider is already registered.
    /// </summary>
    public static IServiceCollection AddData(this IServiceCollection services, string databasePath)
    {
        ArgumentNullException.ThrowIfNull(services);
        ArgumentException.ThrowIfNullOrWhiteSpace(databasePath);

        services.TryAddSingleton(TimeProvider.System);
        services.AddSingleton(new DatabaseOptions { DatabasePath = databasePath });
        services.AddSingleton<TimestampsInterceptor>();

        services.AddDbContextFactory<AppDbContext>((serviceProvider, options) =>
            options
                .UseSqlite(BuildConnectionString(databasePath))
                .AddInterceptors(serviceProvider.GetRequiredService<TimestampsInterceptor>()));

        services.AddSingleton<DatabaseInitializer>();
        services.AddSingleton<IAppSettingsStore, SqliteAppSettingsStore>();

        return services;
    }

    /// <summary>
    /// Connection string for the database file. <c>Default Timeout</c> is the time
    /// Microsoft.Data.Sqlite keeps retrying when the file is busy. Pooling is disabled so that
    /// the file is released as soon as a context is disposed (backups, tests, uninstall).
    /// </summary>
    public static string BuildConnectionString(string databasePath) => new SqliteConnectionStringBuilder
    {
        DataSource = databasePath,
        Mode = SqliteOpenMode.ReadWriteCreate,
        ForeignKeys = true,
        DefaultTimeout = 30,
        Pooling = false,
    }.ToString();
}
