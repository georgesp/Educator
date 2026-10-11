using Educator.Core.Speech;
using Educator.Data;
using Educator.Host.Api;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.TestHost;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.DependencyInjection.Extensions;

namespace Educator.Host.Tests;

/// <summary>
/// The real host pipeline on an in-memory server, with a temporary database and a fake voice.
/// </summary>
internal sealed class ApiTestHost : IAsyncDisposable
{
    public const string Token = "test-token";

    private readonly string directory;

    private ApiTestHost(WebApplication app, string directory)
    {
        App = app;
        this.directory = directory;
        Client = app.GetTestClient();
    }

    public WebApplication App { get; }

    public HttpClient Client { get; }

    public FakeSpeechSynthesizer Speech => (FakeSpeechSynthesizer)App.Services.GetRequiredService<ISpeechSynthesizer>();

    public static async Task<ApiTestHost> StartAsync(bool initializeDatabase = true)
    {
        var directory = Path.Combine(Path.GetTempPath(), "educator-tests", Guid.NewGuid().ToString("N"));
        var app = HostApplication.Build(
            new HostSettings
            {
                DatabasePath = Path.Combine(directory, AppDataPaths.DatabaseFileName),
                ApiOptions = new LocalApiOptions { UiToken = Token },
                RunBackgroundServices = false,
            },
            builder =>
            {
                builder.WebHost.UseTestServer();
                builder.Services.RemoveAll<ISpeechSynthesizer>();
                builder.Services.AddSingleton<ISpeechSynthesizer, FakeSpeechSynthesizer>();
            });

        if (initializeDatabase)
        {
            await app.Services.GetRequiredService<DatabaseInitializer>().InitializeAsync();
        }

        await app.StartAsync();
        return new ApiTestHost(app, directory);
    }

    /// <summary>Request to <c>/api/ui{path}</c> with the right token.</summary>
    public HttpRequestMessage Request(HttpMethod method, string path, string? token = Token)
    {
        var request = new HttpRequestMessage(method, UiEndpoints.RoutePrefix + path);
        if (token is not null)
        {
            request.Headers.Add(LocalTokenEndpointFilter.HeaderName, token);
        }

        return request;
    }

    public async ValueTask DisposeAsync()
    {
        Client.Dispose();
        await App.StopAsync();
        await App.DisposeAsync();
        try
        {
            if (Directory.Exists(directory))
            {
                Directory.Delete(directory, recursive: true);
            }
        }
        catch (IOException)
        {
            // Temporary folder: best effort.
        }
    }
}

internal sealed class FakeSpeechSynthesizer : ISpeechSynthesizer
{
    public static readonly byte[] Wav = "RIFF0000WAVEfake"u8.ToArray();

    public bool IsAvailable { get; set; } = true;

    public List<(string Text, double Speed)> Requests { get; } = [];

    public SpeechEngineStatus Status => IsAvailable
        ? SpeechEngineStatus.Available("fake-voice")
        : SpeechEngineStatus.Unavailable("No voice in tests");

    public Task<byte[]> SynthesizeAsync(string text, double speed = SpeechSpeed.Normal, CancellationToken cancellationToken = default)
    {
        if (!IsAvailable)
        {
            throw new SpeechUnavailableException("No voice in tests");
        }

        Requests.Add((text, speed));
        return Task.FromResult(Wav);
    }

    public Task WarmUpAsync(CancellationToken cancellationToken = default) => Task.CompletedTask;
}
