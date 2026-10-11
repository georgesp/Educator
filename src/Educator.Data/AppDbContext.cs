using Educator.Core;
using Educator.Data.Converters;
using Educator.Data.Entities;
using Microsoft.EntityFrameworkCore;

namespace Educator.Data;

/// <summary>
/// EF Core context of the local SQLite database. Short-lived: always obtain instances
/// from <see cref="IDbContextFactory{TContext}"/> (one context per operation).
/// Conventions (ADR 0001 of GitHubProjectManager): tables in singular PascalCase,
/// dates as UTC Unix milliseconds, enums as text, Guid v7 identifiers.
/// </summary>
public sealed class AppDbContext : DbContext
{
    public AppDbContext(DbContextOptions<AppDbContext> options)
        : base(options)
    {
    }

    public DbSet<AppSettingEntity> AppSettings => Set<AppSettingEntity>();

    public DbSet<ProfileEntity> Profiles => Set<ProfileEntity>();

    protected override void ConfigureConventions(ModelConfigurationBuilder configurationBuilder)
    {
        ArgumentNullException.ThrowIfNull(configurationBuilder);

        // Dates: UTC Unix milliseconds (INTEGER) — sortable and comparable in SQL.
        configurationBuilder.Properties<DateTimeOffset>()
            .HaveConversion<DateTimeOffsetToUnixMillisecondsConverter>();

        // Enums are stored as text: add `configurationBuilder.Properties<MyEnum>().HaveConversion<string>();`
        // for every enum mapped to a column.
        configurationBuilder.Properties<Grade>().HaveConversion<string>().HaveMaxLength(8);
    }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        ArgumentNullException.ThrowIfNull(modelBuilder);
        modelBuilder.ApplyConfigurationsFromAssembly(typeof(AppDbContext).Assembly);
    }
}
