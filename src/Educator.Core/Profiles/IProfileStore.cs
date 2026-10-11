namespace Educator.Core.Profiles;

/// <summary>Storage of the profiles (SQLite, in <c>Educator.Data</c>).</summary>
public interface IProfileStore
{
    /// <summary>Every profile, most recently used first.</summary>
    Task<IReadOnlyList<Profile>> ListAsync(CancellationToken cancellationToken = default);

    Task<Profile?> GetAsync(Guid id, CancellationToken cancellationToken = default);

    /// <summary>Whether another profile already has this normalized name (<see cref="Profile.NormalizeName"/>).</summary>
    Task<bool> NameExistsAsync(string normalizedName, Guid? exceptId, CancellationToken cancellationToken = default);

    /// <summary>Adds the profile and returns it as stored (timestamps filled).</summary>
    Task<Profile> AddAsync(Profile profile, CancellationToken cancellationToken = default);

    /// <summary>Saves name, grade, character and last use; returns the stored profile, or null when unknown.</summary>
    Task<Profile?> UpdateAsync(Profile profile, CancellationToken cancellationToken = default);

    Task<bool> DeleteAsync(Guid id, CancellationToken cancellationToken = default);

    /// <summary>Adds points atomically; returns the new total, or null when the profile is unknown.</summary>
    Task<int?> AddPointsAsync(Guid id, int points, CancellationToken cancellationToken = default);
}
