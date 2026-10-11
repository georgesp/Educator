namespace Educator.Core.Speech;

/// <summary>
/// Local, offline text-to-speech (Piper, SUB006006). Implemented in <c>Educator.Content</c>.
/// </summary>
public interface ISpeechSynthesizer
{
    /// <summary>Whether the engine and the voice were found, without starting the engine.</summary>
    SpeechEngineStatus Status { get; }

    /// <summary>
    /// Synthesizes <paramref name="text"/> to a WAV file (16-bit PCM mono) returned as bytes.
    /// </summary>
    /// <param name="text">French text to read.</param>
    /// <param name="speed">Speaking rate, 1 = normal, from <see cref="SpeechSpeed.Min"/> to <see cref="SpeechSpeed.Max"/>.</param>
    /// <exception cref="SpeechUnavailableException">The engine or the voice is missing, or the engine failed.</exception>
    Task<byte[]> SynthesizeAsync(string text, double speed = SpeechSpeed.Normal, CancellationToken cancellationToken = default);

    /// <summary>Starts the engine ahead of the first request (loads the voice model). Never throws.</summary>
    Task WarmUpAsync(CancellationToken cancellationToken = default);
}

/// <summary>State of the speech engine, exposed by <c>GET /api/ui/status</c>.</summary>
/// <param name="IsAvailable">Engine and voice found.</param>
/// <param name="Voice">Voice name (e.g. <c>fr_FR-siwis-medium</c>), when found.</param>
/// <param name="Problem">Why the engine is unavailable, in English (logs and diagnostics).</param>
public sealed record SpeechEngineStatus(bool IsAvailable, string? Voice, string? Problem)
{
    public static SpeechEngineStatus Available(string voice) => new(true, voice, null);

    public static SpeechEngineStatus Unavailable(string problem) => new(false, null, problem);
}

/// <summary>Speaking rate bounds.</summary>
public static class SpeechSpeed
{
    public const double Min = 0.5;
    public const double Normal = 1.0;
    public const double Max = 1.5;
}

/// <summary>The speech engine cannot synthesize (missing files, crash, timeout).</summary>
public sealed class SpeechUnavailableException : Exception
{
    public SpeechUnavailableException()
    {
    }

    public SpeechUnavailableException(string message)
        : base(message)
    {
    }

    public SpeechUnavailableException(string message, Exception innerException)
        : base(message, innerException)
    {
    }
}
