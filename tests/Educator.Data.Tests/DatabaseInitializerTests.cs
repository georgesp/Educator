using Microsoft.EntityFrameworkCore;

namespace Educator.Data.Tests;

[Trait("Category", "Integration")]
public sealed class DatabaseInitializerTests
{
    [Fact]
    public async Task InitializeAsync_NewDatabase_AppliesInitialMigrationWithoutBackup()
    {
        using var database = new TemporaryDatabase();

        await database.InitializeAsync();

        var initializer = database.Get<DatabaseInitializer>();
        Assert.True(File.Exists(database.DatabasePath));
        Assert.Contains(initializer.LastAppliedMigrations, m => m.EndsWith("_InitialCreate", StringComparison.Ordinal));
        Assert.Null(initializer.LastBackupPath);

        await using var db = await database.ContextFactory.CreateDbContextAsync();
        Assert.Empty(await db.Database.GetPendingMigrationsAsync());
        Assert.Equal(0, await db.AppSettings.CountAsync());
    }

    [Fact]
    public async Task InitializeAsync_UpToDateDatabase_DoesNothing()
    {
        using var database = new TemporaryDatabase();
        await database.InitializeAsync();

        await database.InitializeAsync();

        var initializer = database.Get<DatabaseInitializer>();
        Assert.Empty(initializer.LastAppliedMigrations);
        Assert.Null(initializer.LastBackupPath);
    }

    [Fact]
    public async Task InitializeAsync_ExistingDatabaseWithPendingMigrations_BacksUpFirst()
    {
        var clock = new ManualTimeProvider(new DateTimeOffset(2026, 10, 10, 20, 0, 0, TimeSpan.Zero));
        using var database = new TemporaryDatabase(clock);

        // An existing database file that has never been migrated (as an older version would leave it).
        Directory.CreateDirectory(database.Directory);
        await using (var db = await database.ContextFactory.CreateDbContextAsync())
        {
            await db.Database.ExecuteSqlRawAsync("CREATE TABLE Legacy (Id INTEGER PRIMARY KEY); INSERT INTO Legacy VALUES (42);");
        }

        await database.InitializeAsync();

        var backup = database.Get<DatabaseInitializer>().LastBackupPath;
        Assert.Equal($"{database.DatabasePath}.backup-20261010-200000", backup);
        Assert.True(File.Exists(backup));
    }

    [Fact]
    public async Task InitializeAsync_SetsWalJournalMode()
    {
        using var database = new TemporaryDatabase();

        await database.InitializeAsync();

        await using var db = await database.ContextFactory.CreateDbContextAsync();
        var mode = await db.Database.SqlQueryRaw<string>("SELECT journal_mode AS Value FROM pragma_journal_mode").SingleAsync();
        Assert.Equal("wal", mode, ignoreCase: true);
    }

    [Fact]
    public async Task Migrations_AreInSyncWithTheModel()
    {
        using var database = new TemporaryDatabase();

        await using var db = await database.ContextFactory.CreateDbContextAsync();

        // Fails when an entity or a configuration changed without `dotnet ef migrations add`.
        Assert.False(db.Database.HasPendingModelChanges());
    }
}
