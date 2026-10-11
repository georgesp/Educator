using Educator.Content.Speech;
using Educator.Core.Speech;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

namespace Educator.Content;

/// <summary>DI registration of the content layer.</summary>
public static class ContentServiceCollectionExtensions
{
    /// <summary>
    /// Registers the speech synthesizer: Piper when it is found (see <see cref="PiperLocator"/>),
    /// otherwise a synthesizer reporting why the voice is unavailable.
    /// </summary>
    public static IServiceCollection AddContent(this IServiceCollection services, PiperLocatorOptions? piperOptions = null)
    {
        ArgumentNullException.ThrowIfNull(services);

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
