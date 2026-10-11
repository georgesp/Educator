using System.Security.Cryptography;

namespace Educator.Host.Api;

/// <summary>
/// Security settings of the local HTTP API. It listens on loopback only; every <c>/api/ui</c> route
/// requires the <c>X-Local-Token</c> header and is only callable from the front origins (CORS).
/// </summary>
public sealed record LocalApiOptions
{
    /// <summary>Environment variable carrying the token (set by the Tauri shell, or by hand in dev).</summary>
    public const string UiTokenEnvironmentVariable = "EDUCATOR_UI_TOKEN";

    /// <summary>
    /// Secret expected in the <c>X-Local-Token</c> header. Taken from
    /// <see cref="UiTokenEnvironmentVariable"/> when set, otherwise random per run. Never logged.
    /// </summary>
    public string UiToken { get; init; } = ReadOrGenerateUiToken();

    /// <summary>Browser origins allowed to call <c>/api/ui</c>: Vite dev server and Tauri WebViews (macOS, Windows).</summary>
    public IReadOnlyList<string> AllowedOrigins { get; init; } =
    [
        "http://localhost:1421",
        "tauri://localhost",
        "http://tauri.localhost",
    ];

    public static bool IsTokenConfigured
        => !string.IsNullOrWhiteSpace(Environment.GetEnvironmentVariable(UiTokenEnvironmentVariable));

    private static string ReadOrGenerateUiToken()
    {
        var fromEnvironment = Environment.GetEnvironmentVariable(UiTokenEnvironmentVariable);
        return string.IsNullOrWhiteSpace(fromEnvironment)
            ? Convert.ToHexString(RandomNumberGenerator.GetBytes(32))
            : fromEnvironment.Trim();
    }
}
