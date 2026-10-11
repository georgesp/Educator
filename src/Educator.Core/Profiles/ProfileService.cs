using Educator.Core.Characters;

namespace Educator.Core.Profiles;

/// <summary>
/// Profiles use cases (SUB000102): validation of names (French messages shown to the child),
/// random character at creation and on demand, last use.
/// </summary>
public sealed class ProfileService
{
    private readonly IProfileStore store;
    private readonly ICharacterCatalog characters;
    private readonly IRandomSource random;
    private readonly TimeProvider timeProvider;

    public ProfileService(IProfileStore store, ICharacterCatalog characters, IRandomSource random, TimeProvider timeProvider)
    {
        this.store = store ?? throw new ArgumentNullException(nameof(store));
        this.characters = characters ?? throw new ArgumentNullException(nameof(characters));
        this.random = random ?? throw new ArgumentNullException(nameof(random));
        this.timeProvider = timeProvider ?? throw new ArgumentNullException(nameof(timeProvider));
    }

    public Task<IReadOnlyList<Profile>> ListAsync(CancellationToken cancellationToken = default)
        => store.ListAsync(cancellationToken);

    public async Task<Profile> GetAsync(Guid id, CancellationToken cancellationToken = default)
        => await store.GetAsync(id, cancellationToken).ConfigureAwait(false) ?? throw new ProfileNotFoundException(id);

    /// <param name="characterId">Character shown on the creation screen (drawn by <see cref="DrawCharacter"/>); drawn now when null.</param>
    /// <exception cref="ProfileValidationException">Empty, too long or already used name, unknown character.</exception>
    public async Task<Profile> CreateAsync(string name, Grade grade, string? characterId = null, CancellationToken cancellationToken = default)
    {
        var cleanName = await ValidateNameAsync(name, null, cancellationToken).ConfigureAwait(false);
        var character = characterId is null ? DrawCharacter(except: null).Id : ValidateCharacter(characterId);
        var now = timeProvider.GetUtcNow();
        return await store.AddAsync(
            new Profile
            {
                Id = Guid.CreateVersion7(now),
                Name = cleanName,
                Grade = ValidateGrade(grade),
                CharacterId = character,
                Points = 0,
                LastUsedAt = now,
            },
            cancellationToken).ConfigureAwait(false);
    }

    public async Task<Profile> UpdateAsync(Guid id, string? name, Grade? grade, string? characterId, CancellationToken cancellationToken = default)
    {
        var profile = await GetAsync(id, cancellationToken).ConfigureAwait(false);
        if (name is not null)
        {
            profile = profile with { Name = await ValidateNameAsync(name, id, cancellationToken).ConfigureAwait(false) };
        }

        if (grade is Grade newGrade)
        {
            profile = profile with { Grade = ValidateGrade(newGrade) };
        }

        if (characterId is not null)
        {
            profile = profile with { CharacterId = ValidateCharacter(characterId) };
        }

        return await SaveAsync(profile, cancellationToken).ConfigureAwait(false);
    }

    /// <summary>New random character, different from the current one.</summary>
    public async Task<Profile> RerollCharacterAsync(Guid id, CancellationToken cancellationToken = default)
    {
        var profile = await GetAsync(id, cancellationToken).ConfigureAwait(false);
        return await SaveAsync(profile with { CharacterId = DrawCharacter(except: profile.CharacterId).Id }, cancellationToken).ConfigureAwait(false);
    }

    /// <summary>The child opens this profile: updates <see cref="Profile.LastUsedAt"/>.</summary>
    public async Task<Profile> SelectAsync(Guid id, CancellationToken cancellationToken = default)
    {
        var profile = await GetAsync(id, cancellationToken).ConfigureAwait(false);
        return await SaveAsync(profile with { LastUsedAt = timeProvider.GetUtcNow() }, cancellationToken).ConfigureAwait(false);
    }

    public async Task DeleteAsync(Guid id, CancellationToken cancellationToken = default)
    {
        if (!await store.DeleteAsync(id, cancellationToken).ConfigureAwait(false))
        {
            throw new ProfileNotFoundException(id);
        }
    }

    private async Task<Profile> SaveAsync(Profile profile, CancellationToken cancellationToken)
        => await store.UpdateAsync(profile, cancellationToken).ConfigureAwait(false) ?? throw new ProfileNotFoundException(profile.Id);

    private async Task<string> ValidateNameAsync(string? name, Guid? exceptId, CancellationToken cancellationToken)
    {
        var clean = Profile.CleanName(name ?? string.Empty);
        if (clean.Length == 0)
        {
            throw new ProfileValidationException("name", "Écris ton prénom.");
        }

        if (clean.Length > Profile.MaxNameLength)
        {
            throw new ProfileValidationException("name", $"Ton prénom est trop long : {Profile.MaxNameLength} lettres au maximum.");
        }

        if (await store.NameExistsAsync(Profile.NormalizeName(clean), exceptId, cancellationToken).ConfigureAwait(false))
        {
            throw new ProfileValidationException("name", "Ce prénom est déjà pris. Choisis-en un autre.");
        }

        return clean;
    }

    private static Grade ValidateGrade(Grade grade)
        => Enum.IsDefined(grade) ? grade : throw new ProfileValidationException("grade", "Choisis ta classe.");

    /// <summary>A character at random, different from <paramref name="except"/> when there is a choice.</summary>
    public CharacterInfo DrawCharacter(string? except)
    {
        var candidates = characters.All.Where(c => c.Id != except).ToArray();
        if (candidates.Length == 0)
        {
            return characters.All.FirstOrDefault() ?? throw new InvalidOperationException("No character in the content.");
        }

        return candidates[random.Next(candidates.Length)];
    }

    private string ValidateCharacter(string characterId)
        => characters.All.Any(c => c.Id == characterId)
            ? characterId
            : throw new ProfileValidationException("characterId", "Ce personnage n'existe pas.");
}

/// <summary>Invalid input; <see cref="Exception.Message"/> is a simple French sentence for the child.</summary>
public sealed class ProfileValidationException : Exception
{
    public ProfileValidationException()
    {
        Field = string.Empty;
    }

    public ProfileValidationException(string message)
        : base(message)
    {
        Field = string.Empty;
    }

    public ProfileValidationException(string message, Exception innerException)
        : base(message, innerException)
    {
        Field = string.Empty;
    }

    public ProfileValidationException(string field, string message)
        : base(message)
    {
        Field = field;
    }

    /// <summary>Request field in error (<c>name</c>, <c>grade</c>, <c>characterId</c>).</summary>
    public string Field { get; }
}

public sealed class ProfileNotFoundException : Exception
{
    public ProfileNotFoundException()
    {
    }

    public ProfileNotFoundException(string message)
        : base(message)
    {
    }

    public ProfileNotFoundException(string message, Exception innerException)
        : base(message, innerException)
    {
    }

    public ProfileNotFoundException(Guid id)
        : base($"No profile with id {id}.")
    {
        Id = id;
    }

    public Guid Id { get; }
}
