using System.Net;
using Educator.Core.Characters;
using Educator.Host.Api;

namespace Educator.Host.Tests.Api;

[Trait("Category", "Integration")]
public sealed class CharacterAndSpeechEndpointsTests
{
    [Fact]
    public async Task Characters_ListAndSvg()
    {
        await using var host = await ApiTestHost.StartAsync();

        using var list = await host.SendAsync(HttpMethod.Get, "/characters");
        var characters = await list.ReadAsync<CharacterResponse[]>();
        using var svg = await host.SendAsync(HttpMethod.Get, "/characters/lion.svg");
        using var unknown = await host.SendAsync(HttpMethod.Get, "/characters/dragon.svg");

        Assert.True(characters.Length >= 11);
        Assert.Contains(characters, c => c.Id == "lion" && c.Kind == CharacterKind.Animal);
        Assert.Contains(characters, c => c.Kind == CharacterKind.Child);
        Assert.Equal("image/svg+xml", svg.Content.Headers.ContentType?.MediaType);
        Assert.StartsWith("<svg", (await svg.Content.ReadAsStringAsync()).TrimStart(), StringComparison.Ordinal);
        Assert.Equal(HttpStatusCode.NotFound, unknown.StatusCode);
    }

    [Fact]
    public async Task Speech_ReadsNumbersAndSignsAsWords_AndUsesTheCache()
    {
        await using var host = await ApiTestHost.StartAsync();

        using var first = await host.SendAsync(HttpMethod.Post, "/speech", new { text = "7 + 5 = 12" });
        using var second = await host.SendAsync(HttpMethod.Post, "/speech", new { text = "7 + 5 = 12" });
        using var faster = await host.SendAsync(HttpMethod.Post, "/speech", new { text = "7 + 5 = 12", speed = 1.2 });

        Assert.Equal(HttpStatusCode.OK, first.StatusCode);
        Assert.Equal("audio/wav", first.Content.Headers.ContentType?.MediaType);
        Assert.Equal(FakeSpeechSynthesizer.Wav, await second.Content.ReadAsByteArrayAsync());
        Assert.Equal(
            [("sept plus cinq égale douze", LearningEndpoints.ChildSpeechSpeed), ("sept plus cinq égale douze", 1.2)],
            host.Speech.Requests);
    }

    [Theory]
    [InlineData("", null)]
    [InlineData("Bonjour", 3.0)]
    public async Task Speech_InvalidRequest_Returns400(string text, double? speed)
    {
        await using var host = await ApiTestHost.StartAsync();

        using var response = await host.SendAsync(HttpMethod.Post, "/speech", new { text, speed });

        Assert.Equal(HttpStatusCode.BadRequest, response.StatusCode);
    }

    [Fact]
    public async Task Speech_VoiceUnavailable_Returns503()
    {
        await using var host = await ApiTestHost.StartAsync();
        host.Speech.IsAvailable = false;

        using var response = await host.SendAsync(HttpMethod.Post, "/speech", new { text = "Bonjour" });

        Assert.Equal(HttpStatusCode.ServiceUnavailable, response.StatusCode);
    }
}
