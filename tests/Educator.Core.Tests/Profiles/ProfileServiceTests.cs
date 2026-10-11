using Educator.Core.Profiles;

namespace Educator.Core.Tests.Profiles;

[Trait("Category", "Unit")]
public sealed class ProfileServiceTests
{
    private static readonly DateTimeOffset Start = new(2026, 10, 11, 8, 0, 0, TimeSpan.Zero);

    private readonly InMemoryProfileStore store = new();
    private readonly ManualTimeProvider clock = new(Start);

    [Fact]
    public async Task Create_DrawsACharacterAtRandom()
    {
        var service = Service(new ScriptedRandom(1));

        var profile = await service.CreateAsync("  Léa  ", Grade.CP);

        Assert.Equal("Léa", profile.Name);
        Assert.Equal(Grade.CP, profile.Grade);
        Assert.Equal("ours", profile.CharacterId);
        Assert.Equal(0, profile.Points);
        Assert.Equal(Start, profile.LastUsedAt);
        Assert.Equal(7, profile.Id.Version);
    }

    [Theory]
    [InlineData("", "Écris ton prénom.")]
    [InlineData("   ", "Écris ton prénom.")]
    [InlineData("Maximilien-Alexandre-Jean", "Ton prénom est trop long : 20 lettres au maximum.")]
    public async Task Create_InvalidName_SaysWhyInFrench(string name, string message)
    {
        var error = await Assert.ThrowsAsync<ProfileValidationException>(() => Service().CreateAsync(name, Grade.CP));

        Assert.Equal("name", error.Field);
        Assert.Equal(message, error.Message);
    }

    [Fact]
    public async Task Create_NameAlreadyTaken_IgnoringCase()
    {
        var service = Service();
        await service.CreateAsync("Léa", Grade.CP);

        var error = await Assert.ThrowsAsync<ProfileValidationException>(() => service.CreateAsync("LÉA", Grade.CE1));

        Assert.Equal("Ce prénom est déjà pris. Choisis-en un autre.", error.Message);
    }

    [Fact]
    public async Task Create_KeepsTheCharacterShownOnScreen_IfItExists()
    {
        var service = Service();

        var profile = await service.CreateAsync("Léa", Grade.CP, characterId: "lapin");

        Assert.Equal("lapin", profile.CharacterId);
        await Assert.ThrowsAsync<ProfileValidationException>(() => service.CreateAsync("Tom", Grade.CP, characterId: "dragon"));
    }

    [Fact]
    public void DrawCharacter_NeverGivesTheExcludedOne()
    {
        var service = Service(new ScriptedRandom(0, 1, 2, 3, 4));

        for (var i = 0; i < 20; i++)
        {
            Assert.NotEqual("ours", service.DrawCharacter(except: "ours").Id);
        }
    }

    [Fact]
    public async Task Reroll_GivesAnotherCharacter()
    {
        var service = Service(new ScriptedRandom(0, 0, 1, 0));
        var profile = await service.CreateAsync("Tom", Grade.CE1);

        for (var i = 0; i < 5; i++)
        {
            var before = profile.CharacterId;
            profile = await service.RerollCharacterAsync(profile.Id);
            Assert.NotEqual(before, profile.CharacterId);
        }
    }

    [Fact]
    public async Task Update_ChangesOnlyTheGivenFields()
    {
        var service = Service();
        var profile = await service.CreateAsync("Tom", Grade.CP);

        var updated = await service.UpdateAsync(profile.Id, name: null, grade: Grade.CE1, characterId: "lapin");

        Assert.Equal("Tom", updated.Name);
        Assert.Equal(Grade.CE1, updated.Grade);
        Assert.Equal("lapin", updated.CharacterId);
        await Assert.ThrowsAsync<ProfileValidationException>(() => service.UpdateAsync(profile.Id, null, null, "dragon"));
    }

    [Fact]
    public async Task Update_KeepingItsOwnName_IsAllowed()
    {
        var service = Service();
        var profile = await service.CreateAsync("Tom", Grade.CP);

        var updated = await service.UpdateAsync(profile.Id, "TOM", null, null);

        Assert.Equal("TOM", updated.Name);
    }

    [Fact]
    public async Task Select_UpdatesLastUse()
    {
        var service = Service();
        var profile = await service.CreateAsync("Tom", Grade.CP);
        clock.Now = Start.AddHours(2);

        var selected = await service.SelectAsync(profile.Id);

        Assert.Equal(Start.AddHours(2), selected.LastUsedAt);
    }

    [Fact]
    public async Task UnknownProfile_IsReported()
    {
        var service = Service();

        await Assert.ThrowsAsync<ProfileNotFoundException>(() => service.SelectAsync(Guid.NewGuid()));
        await Assert.ThrowsAsync<ProfileNotFoundException>(() => service.DeleteAsync(Guid.NewGuid()));
    }

    private ProfileService Service(IRandomSource? random = null)
        => new(store, new FakeCharacters(), random ?? new ScriptedRandom(0), clock);
}
