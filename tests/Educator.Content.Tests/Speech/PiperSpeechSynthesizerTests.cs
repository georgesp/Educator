using System.Diagnostics;
using System.Text;
using System.Text.Json;
using Educator.Content.Speech;
using Educator.Core.Speech;
using Microsoft.Extensions.Logging.Abstractions;
using Xunit.Abstractions;

namespace Educator.Content.Tests.Speech;

[Trait("Category", "Unit")]
public sealed class PiperSpeechSynthesizerTests
{
    [Fact]
    public void BuildInputLine_IsAsciiJsonKeepingAccents()
    {
        var line = PiperSpeechSynthesizer.BuildInputLine("L'élève dit \"ça\"\nfin");

        Assert.All(line, c => Assert.True(c < 128, $"Non-ASCII character '{c}' in {line}"));
        using var json = JsonDocument.Parse(line);
        Assert.Equal("L'élève dit \"ça\" fin", json.RootElement.GetProperty("text").GetString());
    }

    [Theory]
    [InlineData(1.0, 1.0)]
    [InlineData(1.25, 0.8)]
    [InlineData(0.5, 2.0)]
    [InlineData(10, 0.67)]
    [InlineData(0.1, 2.0)]
    [InlineData(double.NaN, 1.0)]
    public void ToLengthScale_InvertsAndClampsTheSpeed(double speed, double expected)
        => Assert.Equal(expected, PiperSpeechSynthesizer.ToLengthScale(speed));

    [Fact]
    public async Task UnavailableSynthesizer_ThrowsWithTheProblem()
    {
        var synthesizer = new UnavailableSpeechSynthesizer("Piper not found");

        Assert.False(synthesizer.Status.IsAvailable);
        var error = await Assert.ThrowsAsync<SpeechUnavailableException>(() => synthesizer.SynthesizeAsync("Bonjour"));
        Assert.Equal("Piper not found", error.Message);
    }
}

/// <summary>
/// Runs the real Piper engine when it is installed (scripts/fetch-piper). Without Piper the tests
/// return early and say so in the output: CI machines do not download the engine.
/// </summary>
[Trait("Category", "Integration")]
public sealed class PiperSpeechSynthesizerIntegrationTests
{
    private readonly ITestOutputHelper output;

    public PiperSpeechSynthesizerIntegrationTests(ITestOutputHelper output) => this.output = output;

    [Fact]
    public async Task SynthesizeAsync_TenWords_ReturnsWavInLessThanOneAndAHalfSeconds()
    {
        var installation = PiperLocator.Locate(PiperLocator.OptionsFromEnvironment()).Installation;
        if (installation is null)
        {
            output.WriteLine("Piper is not installed: test skipped (run scripts/fetch-piper).");
            return;
        }

        await using var synthesizer = new PiperSpeechSynthesizer(installation, NullLogger<PiperSpeechSynthesizer>.Instance);
        await synthesizer.WarmUpAsync();

        var stopwatch = Stopwatch.StartNew();
        var wav = await synthesizer.SynthesizeAsync("Le petit élève écrit sa dictée avec beaucoup de soin.");
        stopwatch.Stop();
        output.WriteLine($"Synthesis: {stopwatch.ElapsedMilliseconds} ms, {wav.Length} bytes.");

        Assert.Equal("RIFF", Encoding.ASCII.GetString(wav, 0, 4));
        Assert.Equal("WAVE", Encoding.ASCII.GetString(wav, 8, 4));
        Assert.True(stopwatch.Elapsed < TimeSpan.FromSeconds(1.5), $"Took {stopwatch.ElapsedMilliseconds} ms.");
    }

    [Fact]
    public async Task SynthesizeAsync_ChangingSpeed_StillWorks()
    {
        var installation = PiperLocator.Locate(PiperLocator.OptionsFromEnvironment()).Installation;
        if (installation is null)
        {
            output.WriteLine("Piper is not installed: test skipped (run scripts/fetch-piper).");
            return;
        }

        await using var synthesizer = new PiperSpeechSynthesizer(installation, NullLogger<PiperSpeechSynthesizer>.Instance);

        var normal = await synthesizer.SynthesizeAsync("Un, deux, trois, quatre, cinq.");
        var slow = await synthesizer.SynthesizeAsync("Un, deux, trois, quatre, cinq.", speed: 0.5);

        Assert.True(slow.Length > normal.Length, "Slower speech must give a longer audio file.");
    }

    [Fact]
    public async Task SynthesizeAsync_AfterACancelledRequest_ReadsTheNewSentence()
    {
        var installation = PiperLocator.Locate(PiperLocator.OptionsFromEnvironment()).Installation;
        if (installation is null)
        {
            output.WriteLine("Piper is not installed: test skipped (run scripts/fetch-piper).");
            return;
        }

        await using var synthesizer = new PiperSpeechSynthesizer(installation, NullLogger<PiperSpeechSynthesizer>.Instance);
        await synthesizer.WarmUpAsync();
        var expectedShort = await synthesizer.SynthesizeAsync("Oui.");

        // A long sentence cancelled while Piper is still working on it (the child left the screen).
        using var cancelled = new CancellationTokenSource(TimeSpan.FromMilliseconds(50));
        await Assert.ThrowsAnyAsync<OperationCanceledException>(() => synthesizer.SynthesizeAsync(
            "Il était une fois un petit lapin qui aimait beaucoup compter les carottes de son jardin, une par une, chaque matin.",
            cancellationToken: cancelled.Token));

        var actual = await synthesizer.SynthesizeAsync("Oui.");

        // The audio of "Oui." (lengths vary slightly: Piper adds noise), not the long cancelled sentence.
        Assert.InRange(actual.Length, expectedShort.Length / 2, expectedShort.Length * 2);
    }
}
