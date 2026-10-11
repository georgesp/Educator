using System.Net;

namespace Educator.Host.Tests.Api;

[Trait("Category", "Integration")]
public sealed class LocalApiSecurityTests
{
    [Theory]
    [InlineData(null)]
    [InlineData("")]
    [InlineData("wrong-token")]
    public async Task UiRoute_WithoutTheRightToken_Returns401(string? token)
    {
        await using var host = await ApiTestHost.StartAsync();

        using var response = await host.Client.SendAsync(host.Request(HttpMethod.Get, "/status", token));

        Assert.Equal(HttpStatusCode.Unauthorized, response.StatusCode);
        Assert.Equal("application/problem+json", response.Content.Headers.ContentType?.MediaType);
    }

    [Fact]
    public async Task UiRoute_WithTheToken_Returns200()
    {
        await using var host = await ApiTestHost.StartAsync();

        using var response = await host.Client.SendAsync(host.Request(HttpMethod.Get, "/status"));

        Assert.Equal(HttpStatusCode.OK, response.StatusCode);
    }

    [Theory]
    [InlineData("http://localhost:1421")]
    [InlineData("tauri://localhost")]
    [InlineData("http://tauri.localhost")]
    public async Task Preflight_FromAFrontOrigin_IsAllowed(string origin)
    {
        await using var host = await ApiTestHost.StartAsync();

        using var response = await host.Client.SendAsync(Preflight(origin));

        Assert.Equal(origin, response.Headers.GetValues("Access-Control-Allow-Origin").Single());
    }

    [Fact]
    public async Task Preflight_FromAnotherOrigin_IsRefused()
    {
        await using var host = await ApiTestHost.StartAsync();

        using var response = await host.Client.SendAsync(Preflight("https://example.com"));

        Assert.False(response.Headers.Contains("Access-Control-Allow-Origin"));
    }

    private static HttpRequestMessage Preflight(string origin)
    {
        var request = new HttpRequestMessage(HttpMethod.Options, "/api/ui/status");
        request.Headers.Add("Origin", origin);
        request.Headers.Add("Access-Control-Request-Method", "GET");
        request.Headers.Add("Access-Control-Request-Headers", "x-local-token");
        return request;
    }
}
