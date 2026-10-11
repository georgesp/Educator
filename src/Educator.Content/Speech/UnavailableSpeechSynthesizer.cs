using Educator.Core.Speech;

namespace Educator.Content.Speech;

/// <summary>Used when Piper or the voice is missing: the application works, without sound.</summary>
public sealed class UnavailableSpeechSynthesizer : ISpeechSynthesizer
{
    private readonly string problem;

    public UnavailableSpeechSynthesizer(string problem)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(problem);
        this.problem = problem;
    }

    public SpeechEngineStatus Status => SpeechEngineStatus.Unavailable(problem);

    public Task<byte[]> SynthesizeAsync(string text, double speed = SpeechSpeed.Normal, CancellationToken cancellationToken = default)
        => Task.FromException<byte[]>(new SpeechUnavailableException(problem));

    public Task WarmUpAsync(CancellationToken cancellationToken = default) => Task.CompletedTask;
}
