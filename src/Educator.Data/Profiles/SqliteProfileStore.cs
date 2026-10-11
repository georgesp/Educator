using Educator.Core.Profiles;
using Educator.Data.Entities;
using Microsoft.Data.Sqlite;
using Microsoft.EntityFrameworkCore;

namespace Educator.Data.Profiles;

/// <summary><see cref="IProfileStore"/> backed by the <c>Profile</c> table.</summary>
public sealed class SqliteProfileStore : IProfileStore
{
    private const int SqliteConstraintError = 19;

    private readonly IDbContextFactory<AppDbContext> contextFactory;

    public SqliteProfileStore(IDbContextFactory<AppDbContext> contextFactory)
    {
        this.contextFactory = contextFactory ?? throw new ArgumentNullException(nameof(contextFactory));
    }

    public async Task<IReadOnlyList<Profile>> ListAsync(CancellationToken cancellationToken = default)
    {
        await using var db = await contextFactory.CreateDbContextAsync(cancellationToken).ConfigureAwait(false);
        var entities = await db.Profiles
            .AsNoTracking()
            .OrderByDescending(p => p.LastUsedAt)
            .ThenBy(p => p.Name)
            .ToListAsync(cancellationToken)
            .ConfigureAwait(false);
        return [.. entities.Select(ToProfile)];
    }

    public async Task<Profile?> GetAsync(Guid id, CancellationToken cancellationToken = default)
    {
        await using var db = await contextFactory.CreateDbContextAsync(cancellationToken).ConfigureAwait(false);
        var entity = await db.Profiles.AsNoTracking().SingleOrDefaultAsync(p => p.Id == id, cancellationToken).ConfigureAwait(false);
        return entity is null ? null : ToProfile(entity);
    }

    public async Task<bool> NameExistsAsync(string normalizedName, Guid? exceptId, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(normalizedName);

        await using var db = await contextFactory.CreateDbContextAsync(cancellationToken).ConfigureAwait(false);
        return await db.Profiles
            .AnyAsync(p => p.NormalizedName == normalizedName && (exceptId == null || p.Id != exceptId), cancellationToken)
            .ConfigureAwait(false);
    }

    public async Task<Profile> AddAsync(Profile profile, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(profile);

        await using var db = await contextFactory.CreateDbContextAsync(cancellationToken).ConfigureAwait(false);
        var entity = new ProfileEntity
        {
            Id = profile.Id,
            Name = profile.Name,
            NormalizedName = Profile.NormalizeName(profile.Name),
            Grade = profile.Grade,
            CharacterId = profile.CharacterId,
            Points = profile.Points,
            LastUsedAt = profile.LastUsedAt,
        };
        db.Profiles.Add(entity);
        await SaveAsync(db, cancellationToken).ConfigureAwait(false);
        return ToProfile(entity);
    }

    public async Task<Profile?> UpdateAsync(Profile profile, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(profile);

        await using var db = await contextFactory.CreateDbContextAsync(cancellationToken).ConfigureAwait(false);
        var entity = await db.Profiles.FindAsync([profile.Id], cancellationToken).ConfigureAwait(false);
        if (entity is null)
        {
            return null;
        }

        entity.Name = profile.Name;
        entity.NormalizedName = Profile.NormalizeName(profile.Name);
        entity.Grade = profile.Grade;
        entity.CharacterId = profile.CharacterId;
        entity.LastUsedAt = profile.LastUsedAt;
        await SaveAsync(db, cancellationToken).ConfigureAwait(false);
        return ToProfile(entity);
    }

    public async Task<bool> DeleteAsync(Guid id, CancellationToken cancellationToken = default)
    {
        await using var db = await contextFactory.CreateDbContextAsync(cancellationToken).ConfigureAwait(false);
        return await db.Profiles.Where(p => p.Id == id).ExecuteDeleteAsync(cancellationToken).ConfigureAwait(false) > 0;
    }

    public async Task<int?> AddPointsAsync(Guid id, int points, CancellationToken cancellationToken = default)
    {
        ArgumentOutOfRangeException.ThrowIfNegative(points);

        await using var db = await contextFactory.CreateDbContextAsync(cancellationToken).ConfigureAwait(false);
        var updated = await db.Profiles
            .Where(p => p.Id == id)
            .ExecuteUpdateAsync(set => set.SetProperty(p => p.Points, p => p.Points + points), cancellationToken)
            .ConfigureAwait(false);
        if (updated == 0)
        {
            return null;
        }

        return await db.Profiles.Where(p => p.Id == id).Select(p => p.Points).SingleAsync(cancellationToken).ConfigureAwait(false);
    }

    /// <summary>A unique-name race lost against another request reads as a validation error.</summary>
    private static async Task SaveAsync(AppDbContext db, CancellationToken cancellationToken)
    {
        try
        {
            await db.SaveChangesAsync(cancellationToken).ConfigureAwait(false);
        }
        catch (DbUpdateException ex) when (ex.InnerException is SqliteException { SqliteErrorCode: SqliteConstraintError })
        {
            throw new ProfileValidationException("name", "Ce prénom est déjà pris. Choisis-en un autre.");
        }
    }

    private static Profile ToProfile(ProfileEntity entity) => new()
    {
        Id = entity.Id,
        Name = entity.Name,
        Grade = entity.Grade,
        CharacterId = entity.CharacterId,
        Points = entity.Points,
        CreatedAt = entity.CreatedAt,
        UpdatedAt = entity.UpdatedAt,
        LastUsedAt = entity.LastUsedAt,
    };
}
