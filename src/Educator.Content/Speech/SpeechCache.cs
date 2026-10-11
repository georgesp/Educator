using System.Globalization;
using System.Security.Cryptography;
using System.Text;
using Educator.Core.Speech;
using Microsoft.Extensions.Logging;

namespace Educator.Content.Speech;

/// <summary>
/// Disk cache of the synthesized sentences (SUB000111): a sentence already read once is served from a
/// WAV file instead of running Piper again. Key: SHA-256 of voice + speed + text. The cache is
/// disposable: any read or write problem falls back to the engine.
/// </summary>
public sealed partial class SpeechCache
{
    /// <summary>Files kept at most (about 50 KB each); beyond, the least recently written are removed.</summary>
    public const int MaxFiles = 2000;

    /// <summary>The size is checked every this many writes.</summary>
    internal const int PruneEvery = 50;

    private readonly ISpeechSynthesizer engine;
    private readonly string directory;
    private readonly ILogger<SpeechCache> logger;
    private int writes;

    public SpeechCache(ISpeechSynthesizer engine, string directory, ILogger<SpeechCache> logger)
    {
        this.engine = engine ?? throw new ArgumentNullException(nameof(engine));
        ArgumentException.ThrowIfNullOrWhiteSpace(directory);
        this.directory = directory;
        this.logger = logger ?? throw new ArgumentNullException(nameof(logger));
    }

    public string Directory => directory;

    /// <exception cref="SpeechUnavailableException">Not cached and the engine cannot synthesize.</exception>
    public async Task<byte[]> SynthesizeAsync(string text, double speed, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(text);

        var voice = engine.Status.Voice ?? "none";
        var path = Path.Combine(directory, Key(voice, speed, text) + ".wav");
        try
        {
            if (File.Exists(path))
            {
                return await File.ReadAllBytesAsync(path, cancellationToken).ConfigureAwait(false);
            }
        }
        catch (Exception ex) when (ex is IOException or UnauthorizedAccessException)
        {
            LogCacheProblem(logger, ex, path);
        }

        var wav = await engine.SynthesizeAsync(text, speed, cancellationToken).ConfigureAwait(false);
        try
        {
            System.IO.Directory.CreateDirectory(directory);
            var temporary = path + "." + Guid.NewGuid().ToString("N") + ".tmp";
            await File.WriteAllBytesAsync(temporary, wav, cancellationToken).ConfigureAwait(false);
            File.Move(temporary, path, overwrite: true);
            if (Interlocked.Increment(ref writes) % PruneEvery == 0)
            {
                Prune();
            }
        }
        catch (Exception ex) when (ex is IOException or UnauthorizedAccessException)
        {
            LogCacheProblem(logger, ex, path);
        }

        return wav;
    }

    /// <summary>Removes the oldest files down to 90 % of <see cref="MaxFiles"/>. Only touches this cache's own .wav files.</summary>
    internal void Prune()
    {
        try
        {
            var files = new DirectoryInfo(directory).GetFiles("*.wav");
            if (files.Length <= MaxFiles)
            {
                return;
            }

            foreach (var file in files.OrderBy(f => f.LastWriteTimeUtc).Take(files.Length - (MaxFiles * 9 / 10)))
            {
                file.Delete();
            }
        }
        catch (Exception ex) when (ex is IOException or UnauthorizedAccessException)
        {
            LogCacheProblem(logger, ex, directory);
        }
    }

    internal static string Key(string voice, double speed, string text)
    {
        var material = string.Create(CultureInfo.InvariantCulture, $"{voice}\n{speed:0.###}\n{text}");
        return Convert.ToHexStringLower(SHA256.HashData(Encoding.UTF8.GetBytes(material)));
    }

    [LoggerMessage(Level = LogLevel.Warning, Message = "Speech cache unavailable for {Path}; using the engine.")]
    private static partial void LogCacheProblem(ILogger logger, Exception exception, string path);
}
