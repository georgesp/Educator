using System.Globalization;
using Microsoft.EntityFrameworkCore;

namespace Educator.Data;

/// <summary>
/// Prepares the SQLite database at start-up: creates the folder, backs up an existing database
/// before applying pending migrations (<c>VACUUM INTO</c>), applies them, and switches the journal
/// to WAL.
/// </summary>
public sealed class DatabaseInitializer
{
    private readonly IDbContextFactory<AppDbContext> contextFactory;
    private readonly DatabaseOptions options;
    private readonly TimeProvider timeProvider;

    public DatabaseInitializer(IDbContextFactory<AppDbContext> contextFactory, DatabaseOptions options, TimeProvider timeProvider)
    {
        this.contextFactory = contextFactory ?? throw new ArgumentNullException(nameof(contextFactory));
        this.options = options ?? throw new ArgumentNullException(nameof(options));
        this.timeProvider = timeProvider ?? throw new ArgumentNullException(nameof(timeProvider));
    }

    /// <summary>Path of the backup taken by the last <see cref="InitializeAsync"/>, if any.</summary>
    public string? LastBackupPath { get; private set; }

    /// <summary>True once <see cref="InitializeAsync"/> has completed (reported by <c>GET /api/ui/status</c>).</summary>
    public bool IsInitialized { get; private set; }

    /// <summary>Migrations applied by the last <see cref="InitializeAsync"/>.</summary>
    public IReadOnlyList<string> LastAppliedMigrations { get; private set; } = [];

    public async Task InitializeAsync(CancellationToken cancellationToken = default)
    {
        var directory = Path.GetDirectoryName(options.DatabasePath);
        if (!string.IsNullOrEmpty(directory))
        {
            Directory.CreateDirectory(directory);
        }

        var databaseExisted = File.Exists(options.DatabasePath);

        await using var db = await contextFactory.CreateDbContextAsync(cancellationToken).ConfigureAwait(false);

        var pending = (await db.Database.GetPendingMigrationsAsync(cancellationToken).ConfigureAwait(false)).ToList();
        LastBackupPath = null;
        if (pending.Count > 0)
        {
            if (databaseExisted)
            {
                LastBackupPath = await BackupAsync(db, cancellationToken).ConfigureAwait(false);
            }

            await db.Database.MigrateAsync(cancellationToken).ConfigureAwait(false);
        }

        LastAppliedMigrations = pending;

        // Persistent setting stored in the database file: readers never block the writer.
        await db.Database.ExecuteSqlRawAsync("PRAGMA journal_mode=WAL;", cancellationToken).ConfigureAwait(false);
        IsInitialized = true;
    }

    private async Task<string> BackupAsync(AppDbContext db, CancellationToken cancellationToken)
    {
        var stamp = timeProvider.GetUtcNow().ToString("yyyyMMdd-HHmmss", CultureInfo.InvariantCulture);
        var backupPath = $"{options.DatabasePath}.backup-{stamp}";

        // VACUUM INTO produces a consistent copy, WAL content included.
        IEnumerable<object> parameters = [backupPath];
        await db.Database.ExecuteSqlRawAsync("VACUUM INTO {0};", parameters, cancellationToken).ConfigureAwait(false);
        return backupPath;
    }
}
