using Educator.Core.Speech;

namespace Educator.Host;

/// <summary>Starts Piper in the background at start-up, so that the first sentence is read without delay.</summary>
internal sealed class SpeechWarmUpService : BackgroundService
{
    private readonly ISpeechSynthesizer synthesizer;

    public SpeechWarmUpService(ISpeechSynthesizer synthesizer) => this.synthesizer = synthesizer;

    protected override Task ExecuteAsync(CancellationToken stoppingToken) => synthesizer.WarmUpAsync(stoppingToken);
}
