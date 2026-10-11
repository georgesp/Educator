using System.Collections.Concurrent;
using Educator.Core.Characters;
using Educator.Core.Profiles;

namespace Educator.Core.Tests.Profiles;

/// <summary>In-memory <see cref="IProfileStore"/> for the service tests (the SQLite one is tested in Educator.Data.Tests).</summary>
internal sealed class InMemoryProfileStore : IProfileStore
{
    private readonly ConcurrentDictionary<Guid, Profile> profiles = new();

    public Task<IReadOnlyList<Profile>> ListAsync(CancellationToken cancellationToken = default)
        => Task.FromResult<IReadOnlyList<Profile>>([.. profiles.Values.OrderByDescending(p => p.LastUsedAt)]);

    public Task<Profile?> GetAsync(Guid id, CancellationToken cancellationToken = default)
        => Task.FromResult(profiles.GetValueOrDefault(id));

    public Task<bool> NameExistsAsync(string normalizedName, Guid? exceptId, CancellationToken cancellationToken = default)
        => Task.FromResult(profiles.Values.Any(p => Profile.NormalizeName(p.Name) == normalizedName && p.Id != exceptId));

    public Task<Profile> AddAsync(Profile profile, CancellationToken cancellationToken = default)
    {
        profiles[profile.Id] = profile;
        return Task.FromResult(profile);
    }

    public Task<Profile?> UpdateAsync(Profile profile, CancellationToken cancellationToken = default)
    {
        if (!profiles.ContainsKey(profile.Id))
        {
            return Task.FromResult<Profile?>(null);
        }

        // Points are only changed through AddPointsAsync, as in the SQLite store.
        var stored = profile with { Points = profiles[profile.Id].Points };
        profiles[profile.Id] = stored;
        return Task.FromResult<Profile?>(stored);
    }

    public Task<bool> DeleteAsync(Guid id, CancellationToken cancellationToken = default)
        => Task.FromResult(profiles.TryRemove(id, out _));

    public Task<int?> AddPointsAsync(Guid id, int points, CancellationToken cancellationToken = default)
    {
        if (!profiles.TryGetValue(id, out var profile))
        {
            return Task.FromResult<int?>(null);
        }

        var updated = profile with { Points = profile.Points + points };
        profiles[id] = updated;
        return Task.FromResult<int?>(updated.Points);
    }
}

internal sealed class FakeCharacters : ICharacterCatalog
{
    public IReadOnlyList<CharacterInfo> All { get; } =
    [
        new("lion", "Lion", CharacterKind.Animal),
        new("ours", "Ours", CharacterKind.Animal),
        new("lapin", "Lapin", CharacterKind.Animal),
    ];

    public string? GetSvg(string id) => All.Any(c => c.Id == id) ? "<svg/>" : null;
}

/// <summary>Draws the given numbers in a loop.</summary>
internal sealed class ScriptedRandom(params int[] values) : IRandomSource
{
    private int next;

    public int Next(int maxExclusive) => values[next++ % values.Length] % maxExclusive;
}

internal sealed class ManualTimeProvider(DateTimeOffset now) : TimeProvider
{
    public DateTimeOffset Now { get; set; } = now;

    public override DateTimeOffset GetUtcNow() => Now;
}
