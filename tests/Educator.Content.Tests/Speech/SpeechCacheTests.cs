using Educator.Content.Speech;
using Educator.Core.Speech;
using Microsoft.Extensions.Logging.Abstractions;

namespace Educator.Content.Tests.Speech;

[Trait("Category", "Unit")]
public sealed class SpeechCacheTests : IDisposable
{
    private readonly string directory = Path.Combine(Path.GetTempPath(), "educator-tests", Guid.NewGuid().ToString("N"));
    private readonly CountingEngine engine = new();

    public void Dispose()
    {
        if (Directory.Exists(directory))
        {
            Directory.Delete(directory, recursive: true);
        }
    }

    [Fact]
    public async Task SameSentence_IsSynthesizedOnce_PerVoiceAndSpeed()
    {
        var cache = new SpeechCache(engine, directory, NullLogger<SpeechCache>.Instance);

        var first = await cache.SynthesizeAsync("sept plus cinq", 0.9);
        var second = await cache.SynthesizeAsync("sept plus cinq", 0.9);
        await cache.SynthesizeAsync("sept plus cinq", 1.0);

        Assert.Equal(first, second);
        Assert.Equal(2, engine.Calls);
    }

    [Fact]
    public async Task Prune_KeepsTheCacheBounded()
    {
        var cache = new SpeechCache(engine, directory, NullLogger<SpeechCache>.Instance);
        Directory.CreateDirectory(directory);
        for (var i = 0; i < SpeechCache.MaxFiles + 10; i++)
        {
            await File.WriteAllBytesAsync(Path.Combine(directory, $"{i:D5}.wav"), [1]);
        }

        cache.Prune();

        Assert.Equal(SpeechCache.MaxFiles * 9 / 10, Directory.GetFiles(directory, "*.wav").Length);
    }

    private sealed class CountingEngine : ISpeechSynthesizer
    {
        public int Calls { get; private set; }

        public SpeechEngineStatus Status => SpeechEngineStatus.Available("test-voice");

        public Task<byte[]> SynthesizeAsync(string text, double speed = SpeechSpeed.Normal, CancellationToken cancellationToken = default)
        {
            Calls++;
            return Task.FromResult(new byte[] { 1, 2, 3 });
        }

        public Task WarmUpAsync(CancellationToken cancellationToken = default) => Task.CompletedTask;
    }
}
