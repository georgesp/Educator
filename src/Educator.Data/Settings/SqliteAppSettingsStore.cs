using Educator.Core.Settings;
using Educator.Data.Entities;
using Microsoft.EntityFrameworkCore;

namespace Educator.Data.Settings;

/// <summary><see cref="IAppSettingsStore"/> backed by the <c>AppSetting</c> table.</summary>
public sealed class SqliteAppSettingsStore : IAppSettingsStore
{
    private readonly IDbContextFactory<AppDbContext> contextFactory;

    public SqliteAppSettingsStore(IDbContextFactory<AppDbContext> contextFactory)
    {
        this.contextFactory = contextFactory ?? throw new ArgumentNullException(nameof(contextFactory));
    }

    public async Task<string?> GetAsync(string key, CancellationToken cancellationToken = default)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(key);

        await using var db = await contextFactory.CreateDbContextAsync(cancellationToken).ConfigureAwait(false);
        return await db.AppSettings
            .AsNoTracking()
            .Where(s => s.Key == key)
            .Select(s => s.Value)
            .SingleOrDefaultAsync(cancellationToken)
            .ConfigureAwait(false);
    }

    public async Task SetAsync(string key, string value, CancellationToken cancellationToken = default)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(key);
        ArgumentNullException.ThrowIfNull(value);

        await using var db = await contextFactory.CreateDbContextAsync(cancellationToken).ConfigureAwait(false);
        var existing = await db.AppSettings.FindAsync([key], cancellationToken).ConfigureAwait(false);
        if (existing is null)
        {
            db.AppSettings.Add(new AppSettingEntity { Key = key, Value = value });
        }
        else
        {
            existing.Value = value;
        }

        await db.SaveChangesAsync(cancellationToken).ConfigureAwait(false);
    }

    public async Task<bool> RemoveAsync(string key, CancellationToken cancellationToken = default)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(key);

        await using var db = await contextFactory.CreateDbContextAsync(cancellationToken).ConfigureAwait(false);
        var removed = await db.AppSettings
            .Where(s => s.Key == key)
            .ExecuteDeleteAsync(cancellationToken)
            .ConfigureAwait(false);
        return removed > 0;
    }
}
