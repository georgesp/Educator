using System.Net.Http.Json;
using Educator.Host.Api;

namespace Educator.Host.Tests.Api;

[Trait("Category", "Integration")]
public sealed class StatusEndpointTests
{
    [Fact]
    public async Task Status_ReportsVersionDatabaseAndVoice()
    {
        await using var host = await ApiTestHost.StartAsync();

        using var response = await host.Client.SendAsync(host.Request(HttpMethod.Get, "/status"));
        var status = await response.Content.ReadFromJsonAsync<StatusResponse>();

        Assert.NotNull(status);
        Assert.False(string.IsNullOrWhiteSpace(status.Version));
        Assert.True(status.Database.IsReady);
        Assert.True(status.Speech.IsAvailable);
        Assert.Equal("fake-voice", status.Speech.Voice);
    }

    [Fact]
    public async Task Status_BeforeMigration_DatabaseNotReady()
    {
        await using var host = await ApiTestHost.StartAsync(initializeDatabase: false);

        using var response = await host.Client.SendAsync(host.Request(HttpMethod.Get, "/status"));
        var status = await response.Content.ReadFromJsonAsync<StatusResponse>();

        Assert.False(status!.Database.IsReady);
    }

    [Fact]
    public async Task Status_VoiceUnavailable_ReportsTheProblem()
    {
        await using var host = await ApiTestHost.StartAsync();
        host.Speech.IsAvailable = false;

        using var response = await host.Client.SendAsync(host.Request(HttpMethod.Get, "/status"));
        var status = await response.Content.ReadFromJsonAsync<StatusResponse>();

        Assert.False(status!.Speech.IsAvailable);
        Assert.Equal("No voice in tests", status.Speech.Problem);
    }
}
