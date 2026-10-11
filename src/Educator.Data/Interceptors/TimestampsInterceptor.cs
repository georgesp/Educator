using Educator.Data.Entities;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.ChangeTracking;
using Microsoft.EntityFrameworkCore.Diagnostics;

namespace Educator.Data.Interceptors;

/// <summary>
/// Fills <see cref="ITimestamped.CreatedAt"/> and <see cref="ITimestamped.UpdatedAt"/> just before
/// <c>SaveChanges</c>, from the injected <see cref="TimeProvider"/> (testable).
/// <c>UpdatedAt</c> only moves when another column actually changed.
/// </summary>
public sealed class TimestampsInterceptor : SaveChangesInterceptor
{
    private readonly TimeProvider timeProvider;

    public TimestampsInterceptor(TimeProvider timeProvider)
    {
        this.timeProvider = timeProvider ?? throw new ArgumentNullException(nameof(timeProvider));
    }

    public override InterceptionResult<int> SavingChanges(DbContextEventData eventData, InterceptionResult<int> result)
    {
        ArgumentNullException.ThrowIfNull(eventData);
        StampEntries(eventData.Context);
        return base.SavingChanges(eventData, result);
    }

    public override ValueTask<InterceptionResult<int>> SavingChangesAsync(
        DbContextEventData eventData,
        InterceptionResult<int> result,
        CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(eventData);
        StampEntries(eventData.Context);
        return base.SavingChangesAsync(eventData, result, cancellationToken);
    }

    private void StampEntries(DbContext? context)
    {
        if (context is null)
        {
            return;
        }

        var now = timeProvider.GetUtcNowMilliseconds();

        // Entries() runs DetectChanges, so property-level IsModified is accurate here.
        foreach (var entry in context.ChangeTracker.Entries<ITimestamped>())
        {
            switch (entry.State)
            {
                case EntityState.Added:
                    entry.Property(nameof(ITimestamped.CreatedAt)).CurrentValue = now;
                    entry.Property(nameof(ITimestamped.UpdatedAt)).CurrentValue = now;
                    break;

                case EntityState.Modified when HasBusinessChanges(entry):
                    entry.Property(nameof(ITimestamped.UpdatedAt)).CurrentValue = now;
                    break;
            }
        }
    }

    private static bool HasBusinessChanges(EntityEntry entry)
        => entry.Properties.Any(p => p.IsModified
            && p.Metadata.Name is not (nameof(ITimestamped.CreatedAt) or nameof(ITimestamped.UpdatedAt)));
}
