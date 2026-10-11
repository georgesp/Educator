using System.Net;
using System.Text.Json;
using Educator.Core;
using Educator.Host.Api;

namespace Educator.Host.Tests.Api;

[Trait("Category", "Integration")]
public sealed class ProfileEndpointsTests
{
    [Fact]
    public async Task Create_List_Select_Delete()
    {
        await using var host = await ApiTestHost.StartAsync();

        using var created = await host.SendAsync(HttpMethod.Post, "/profiles", new { name = "Léa", grade = "CP" });
        var lea = await created.ReadAsync<ProfileResponse>();
        using var second = await host.SendAsync(HttpMethod.Post, "/profiles", new { name = "Tom", grade = "CE1" });
        var tom = await second.ReadAsync<ProfileResponse>();
        using var selected = await host.SendAsync(HttpMethod.Post, $"/profiles/{lea.Id}/select");
        using var list = await host.SendAsync(HttpMethod.Get, "/profiles");
        var profiles = await list.ReadAsync<ProfileResponse[]>();

        Assert.Equal(HttpStatusCode.Created, created.StatusCode);
        Assert.Equal($"/api/ui/profiles/{lea.Id}", created.Headers.Location?.ToString());
        Assert.Equal(Grade.CP, lea.Grade);
        Assert.Equal(0, lea.Points);
        Assert.False(string.IsNullOrEmpty(lea.CharacterId));
        Assert.Equal(HttpStatusCode.OK, selected.StatusCode);
        Assert.Equal(["Léa", "Tom"], profiles.Select(p => p.Name));

        using var deleted = await host.SendAsync(HttpMethod.Delete, $"/profiles/{tom.Id}");
        using var deletedAgain = await host.SendAsync(HttpMethod.Delete, $"/profiles/{tom.Id}");
        Assert.Equal(HttpStatusCode.NoContent, deleted.StatusCode);
        Assert.Equal(HttpStatusCode.NotFound, deletedAgain.StatusCode);
    }

    [Theory]
    [InlineData("", "Écris ton prénom.")]
    [InlineData("Un prénom vraiment beaucoup trop long", "Ton prénom est trop long : 20 lettres au maximum.")]
    [InlineData("LÉA", "Ce prénom est déjà pris. Choisis-en un autre.")]
    public async Task InvalidName_Returns400WithAFrenchSentence(string name, string detail)
    {
        await using var host = await ApiTestHost.StartAsync();
        (await host.SendAsync(HttpMethod.Post, "/profiles", new { name = "Léa", grade = "CP" })).Dispose();

        using var response = await host.SendAsync(HttpMethod.Post, "/profiles", new { name, grade = "CP" });
        using var problem = JsonDocument.Parse(await response.Content.ReadAsStringAsync());

        Assert.Equal(HttpStatusCode.BadRequest, response.StatusCode);
        Assert.Equal(detail, problem.RootElement.GetProperty("detail").GetString());
        Assert.Equal("name", problem.RootElement.GetProperty("field").GetString());
    }

    [Fact]
    public async Task CreationScreen_DrawsACharacter_ThenKeepsIt()
    {
        await using var host = await ApiTestHost.StartAsync();

        using var drawn = await host.SendAsync(HttpMethod.Get, "/characters/draw?except=lion");
        var character = await drawn.ReadAsync<CharacterResponse>();
        using var created = await host.SendAsync(HttpMethod.Post, "/profiles", new { name = "Léa", grade = "CP", characterId = character.Id });

        Assert.NotEqual("lion", character.Id);
        Assert.Equal(character.Id, (await created.ReadAsync<ProfileResponse>()).CharacterId);
    }

    [Fact]
    public async Task Patch_And_Reroll()
    {
        await using var host = await ApiTestHost.StartAsync();
        using var created = await host.SendAsync(HttpMethod.Post, "/profiles", new { name = "Léa", grade = "CP" });
        var lea = await created.ReadAsync<ProfileResponse>();

        using var patched = await host.SendAsync(HttpMethod.Patch, $"/profiles/{lea.Id}", new { grade = "CE1", characterId = "renard" });
        var afterPatch = await patched.ReadAsync<ProfileResponse>();
        using var rerolled = await host.SendAsync(HttpMethod.Post, $"/profiles/{lea.Id}/reroll-character");
        var afterReroll = await rerolled.ReadAsync<ProfileResponse>();
        using var unknownCharacter = await host.SendAsync(HttpMethod.Patch, $"/profiles/{lea.Id}", new { characterId = "dragon" });
        using var unknownProfile = await host.SendAsync(HttpMethod.Post, $"/profiles/{Guid.NewGuid()}/reroll-character");

        Assert.Equal("Léa", afterPatch.Name);
        Assert.Equal(Grade.CE1, afterPatch.Grade);
        Assert.Equal("renard", afterPatch.CharacterId);
        Assert.NotEqual("renard", afterReroll.CharacterId);
        Assert.Equal(HttpStatusCode.BadRequest, unknownCharacter.StatusCode);
        Assert.Equal(HttpStatusCode.NotFound, unknownProfile.StatusCode);
    }
}
