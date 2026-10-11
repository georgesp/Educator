using System.Diagnostics;
using System.Globalization;
using System.Text;
using System.Text.Json;
using Educator.Core.Speech;
using Microsoft.Extensions.Logging;

namespace Educator.Content.Speech;

/// <summary>
/// <see cref="ISpeechSynthesizer"/> running the Piper command-line engine as a long-lived child process
/// (<c>--json-input</c>): the voice model is loaded once, then each request is one JSON line on stdin;
/// Piper writes a WAV file in a private folder and prints its path on stdout.
/// Requests are serialized (one at a time). The process is restarted after a crash, a timeout or a
/// change of speed (<c>--length_scale</c> is a process-wide option).
/// </summary>
public sealed class PiperSpeechSynthesizer : ISpeechSynthesizer, IAsyncDisposable, IDisposable
{
    public const int MaxTextLength = 1000;

    private static readonly TimeSpan RequestTimeout = TimeSpan.FromSeconds(20);
    private static readonly UTF8Encoding Utf8NoBom = new(encoderShouldEmitUTF8Identifier: false);

    private readonly PiperInstallation installation;
    private readonly ILogger<PiperSpeechSynthesizer> logger;
    private readonly SemaphoreSlim gate = new(1, 1);
    private readonly string outputDirectory;

    private Process? process;
    private double processLengthScale;
    private bool disposed;

    public PiperSpeechSynthesizer(PiperInstallation installation, ILogger<PiperSpeechSynthesizer> logger)
    {
        this.installation = installation ?? throw new ArgumentNullException(nameof(installation));
        this.logger = logger ?? throw new ArgumentNullException(nameof(logger));
        outputDirectory = Path.Combine(Path.GetTempPath(), "educator-piper", Guid.NewGuid().ToString("N"));
    }

    public SpeechEngineStatus Status => SpeechEngineStatus.Available(installation.VoiceName);

    public async Task<byte[]> SynthesizeAsync(string text, double speed = SpeechSpeed.Normal, CancellationToken cancellationToken = default)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(text);
        if (text.Length > MaxTextLength)
        {
            throw new ArgumentException($"Text longer than {MaxTextLength} characters.", nameof(text));
        }

        ObjectDisposedException.ThrowIf(disposed, this);
        var lengthScale = ToLengthScale(speed);

        await gate.WaitAsync(cancellationToken).ConfigureAwait(false);
        try
        {
            return await SynthesizeLockedAsync(text, lengthScale, cancellationToken).ConfigureAwait(false);
        }
        finally
        {
            gate.Release();
        }
    }

    public async Task WarmUpAsync(CancellationToken cancellationToken = default)
    {
        try
        {
            var stopwatch = Stopwatch.StartNew();
            await SynthesizeAsync("Bonjour.", SpeechSpeed.Normal, cancellationToken).ConfigureAwait(false);
            logger.LogInformation("Piper ready with voice {Voice} in {ElapsedMs} ms.", installation.VoiceName, stopwatch.ElapsedMilliseconds);
        }
        catch (Exception ex) when (ex is not OperationCanceledException)
        {
            logger.LogWarning(ex, "Piper warm-up failed.");
        }
    }

    /// <summary>
    /// One input line of <c>piper --json-input</c>. Non-ASCII characters are escaped
    /// (<c>é</c>), so the line is pure ASCII whatever the console encoding of the OS.
    /// </summary>
    internal static string BuildInputLine(string text)
        => JsonSerializer.Serialize(new Dictionary<string, string> { ["text"] = text.ReplaceLineEndings(" ") });

    /// <summary>Piper's <c>--length_scale</c> is a duration factor: speed 2 = length 0.5.</summary>
    internal static double ToLengthScale(double speed)
    {
        var clamped = double.IsFinite(speed) ? Math.Clamp(speed, SpeechSpeed.Min, SpeechSpeed.Max) : SpeechSpeed.Normal;
        return Math.Round(1 / clamped, 2);
    }

    private async Task<byte[]> SynthesizeLockedAsync(string text, double lengthScale, CancellationToken cancellationToken)
    {
        var piper = EnsureProcess(lengthScale);

        using var timeout = CancellationTokenSource.CreateLinkedTokenSource(cancellationToken);
        timeout.CancelAfter(RequestTimeout);

        try
        {
            ClearOutputDirectory();
            await piper.StandardInput.WriteLineAsync(BuildInputLine(text).AsMemory(), timeout.Token).ConfigureAwait(false);
            await piper.StandardInput.FlushAsync(timeout.Token).ConfigureAwait(false);

            // The printed path is only a signal: the file is looked up in our private folder, so that
            // a path with non-ASCII characters printed in the console encoding cannot break us.
            var line = await piper.StandardOutput.ReadLineAsync(timeout.Token).ConfigureAwait(false)
                ?? throw new SpeechUnavailableException($"Piper exited unexpectedly (exit code {SafeExitCode(piper)}).");
            logger.LogDebug("Piper wrote {Output}.", line);

            var wav = Directory.EnumerateFiles(outputDirectory, "*.wav").SingleOrDefault()
                ?? throw new SpeechUnavailableException("Piper did not write any audio file.");
            var bytes = await File.ReadAllBytesAsync(wav, cancellationToken).ConfigureAwait(false);
            File.Delete(wav);
            return bytes;
        }
        catch (OperationCanceledException)
        {
            // Piper is still working on the interrupted sentence and will print its line later:
            // restart it, otherwise the next request would read that stale answer.
            StopProcess();
            if (cancellationToken.IsCancellationRequested)
            {
                throw;
            }

            throw new SpeechUnavailableException($"Piper did not answer within {RequestTimeout.TotalSeconds:0} s.");
        }
        catch (Exception ex) when (ex is IOException or InvalidOperationException)
        {
            StopProcess();
            throw new SpeechUnavailableException("Piper failed.", ex);
        }
        catch (SpeechUnavailableException)
        {
            StopProcess();
            throw;
        }
    }

    private Process EnsureProcess(double lengthScale)
    {
        if (process is { HasExited: false } && processLengthScale.Equals(lengthScale))
        {
            return process;
        }

        StopProcess();
        Directory.CreateDirectory(outputDirectory);

        var startInfo = new ProcessStartInfo(installation.ExecutablePath)
        {
            WorkingDirectory = installation.EngineDirectory,
            RedirectStandardInput = true,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,
            CreateNoWindow = true,
            StandardInputEncoding = Utf8NoBom,
            StandardOutputEncoding = Utf8NoBom,
            StandardErrorEncoding = Utf8NoBom,
        };
        foreach (var argument in new[]
        {
            "--model", installation.ModelPath,
            "--config", installation.ModelConfigPath,
            "--espeak_data", installation.EspeakDataDirectory,
            "--output_dir", outputDirectory,
            "--length_scale", lengthScale.ToString(CultureInfo.InvariantCulture),
            "--json-input",
        })
        {
            startInfo.ArgumentList.Add(argument);
        }

        var libraryVariable = PiperLocator.CurrentPlatform().LibraryPathVariable;
        if (libraryVariable is not null)
        {
            startInfo.Environment[libraryVariable] = installation.EngineDirectory;
        }

        var started = new Process { StartInfo = startInfo };
        started.ErrorDataReceived += (_, e) =>
        {
            if (!string.IsNullOrWhiteSpace(e.Data))
            {
                logger.LogDebug("[piper] {Line}", e.Data);
            }
        };

        try
        {
            started.Start();
        }
        catch (Exception ex) when (ex is System.ComponentModel.Win32Exception or InvalidOperationException)
        {
            started.Dispose();
            throw new SpeechUnavailableException($"Could not start Piper ({installation.ExecutablePath}).", ex);
        }

        started.BeginErrorReadLine();
        logger.LogInformation(
            "Piper started (pid {ProcessId}, voice {Voice}, length scale {LengthScale}).",
            started.Id,
            installation.VoiceName,
            lengthScale);

        process = started;
        processLengthScale = lengthScale;
        return started;
    }

    private void ClearOutputDirectory()
    {
        foreach (var file in Directory.EnumerateFiles(outputDirectory))
        {
            File.Delete(file);
        }
    }

    private void StopProcess()
    {
        var current = process;
        process = null;
        if (current is null)
        {
            return;
        }

        try
        {
            if (!current.HasExited)
            {
                current.StandardInput.Close();
                if (!current.WaitForExit(TimeSpan.FromSeconds(2)))
                {
                    current.Kill(entireProcessTree: true);
                }
            }
        }
        catch (Exception ex) when (ex is InvalidOperationException or IOException or System.ComponentModel.Win32Exception)
        {
            logger.LogDebug(ex, "Piper already stopped.");
        }
        finally
        {
            current.Dispose();
        }
    }

    private static string SafeExitCode(Process piper)
    {
        try
        {
            return piper.WaitForExit(TimeSpan.FromSeconds(1)) ? piper.ExitCode.ToString(CultureInfo.InvariantCulture) : "running";
        }
        catch (InvalidOperationException)
        {
            return "unknown";
        }
    }

    public void Dispose()
    {
        if (disposed)
        {
            return;
        }

        disposed = true;
        StopProcess();
        gate.Dispose();
        try
        {
            if (Directory.Exists(outputDirectory))
            {
                Directory.Delete(outputDirectory, recursive: true);
            }
        }
        catch (IOException)
        {
            // Temporary folder: best effort.
        }
    }

    public ValueTask DisposeAsync()
    {
        Dispose();
        return ValueTask.CompletedTask;
    }
}
