using Educator.Content.Speech;
using Educator.Core.Characters;
using Educator.Core.Rewards;
using Educator.Core.Skills;
using Educator.Core.Speech;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

namespace Educator.Content;

/// <summary>DI registration of the content layer.</summary>
public static class ContentServiceCollectionExtensions
{
    /// <summary>
    /// Registers the pedagogical content (skills, school periods, points scale, characters) loaded once from
    /// <paramref name="contentDirectory"/> (default: <see cref="ContentDirectory.Resolve"/>), the speech
    /// synthesizer — Piper when it is found (see <see cref="PiperLocator"/>), otherwise a synthesizer reporting
    /// why the voice is unavailable — and, when <paramref name="speechCacheDirectory"/> is given, its disk cache.
    /// </summary>
    public static IServiceCollection AddContent(
        this IServiceCollection services,
        PiperLocatorOptions? piperOptions = null,
        string? contentDirectory = null,
        string? speechCacheDirectory = null)
    {
        ArgumentNullException.ThrowIfNull(services);

        services.AddSingleton(serviceProvider =>
        {
            var content = EducatorContent.Load(contentDirectory ?? ContentDirectory.Resolve());
            var logger = serviceProvider.GetRequiredService<ILoggerFactory>().CreateLogger(typeof(EducatorContent));
            foreach (var problem in content.Problems)
            {
                logger.LogError("Invalid content skipped: {Problem}", problem);
            }

            logger.LogInformation(
                "Content loaded from {Directory}: {Skills} skills, {Characters} characters.",
                content.Directory,
                content.Skills.All.Count,
                content.Characters.All.Count);
            return content;
        });
        services.AddSingleton<ISkillCatalog>(sp => sp.GetRequiredService<EducatorContent>().Skills);
        services.AddSingleton<ICharacterCatalog>(sp => sp.GetRequiredService<EducatorContent>().Characters);
        services.AddSingleton(sp => sp.GetRequiredService<EducatorContent>().Calendar);
        services.AddSingleton(sp => sp.GetRequiredService<EducatorContent>().Points);
        services.AddSingleton(sp => new SpeechCache(
            sp.GetRequiredService<ISpeechSynthesizer>(),
            speechCacheDirectory ?? Path.Combine(Path.GetTempPath(), "educator-speech-cache"),
            sp.GetRequiredService<ILogger<SpeechCache>>()));

        services.AddSingleton<ISpeechSynthesizer>(serviceProvider =>
        {
            var result = PiperLocator.Locate(piperOptions ?? PiperLocator.OptionsFromEnvironment());
            var logger = serviceProvider.GetRequiredService<ILoggerFactory>().CreateLogger(typeof(ContentServiceCollectionExtensions));
            if (result.Installation is null)
            {
                logger.LogWarning("Speech synthesis unavailable: {Problem}", result.Problem);
                return new UnavailableSpeechSynthesizer(result.Problem!);
            }

            logger.LogInformation("Piper found: {Executable}, voice {Voice}.", result.Installation.ExecutablePath, result.Installation.VoiceName);
            return new PiperSpeechSynthesizer(result.Installation, serviceProvider.GetRequiredService<ILogger<PiperSpeechSynthesizer>>());
        });

        return services;
    }
}
