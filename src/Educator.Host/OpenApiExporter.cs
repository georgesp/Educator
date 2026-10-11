using System.Text.Json;
using System.Text.Json.Nodes;
using Educator.Host.Api;

namespace Educator.Host;

/// <summary>
/// <c>educator-host --export-openapi &lt;file&gt;</c>: starts the API on a free port (no database,
/// no Piper), downloads <c>/openapi/v1.json</c>, writes it indented and exits. The front generates
/// its TypeScript client from that file (<c>npm run api:generate</c>).
/// </summary>
internal static class OpenApiExporter
{
    private static readonly JsonSerializerOptions Indented = new() { WriteIndented = true };

    public static async Task ExportAsync(string path, CancellationToken cancellationToken = default)
    {
        await using var app = HostApplication.Build(new HostSettings
        {
            DatabasePath = Path.Combine(Path.GetTempPath(), $"educator-openapi-{Guid.NewGuid():N}.db"),
            ApiOptions = new LocalApiOptions { UiToken = "openapi-export" },
            RunBackgroundServices = false,
        });

        await app.StartAsync(cancellationToken).ConfigureAwait(false);
        try
        {
            using var client = new HttpClient { BaseAddress = new Uri(Program.GetBaseUrl(app)) };
            var json = await client.GetStringAsync(new Uri("/openapi/v1.json", UriKind.Relative), cancellationToken).ConfigureAwait(false);

            var fullPath = Path.GetFullPath(path);
            Directory.CreateDirectory(Path.GetDirectoryName(fullPath)!);
            await File.WriteAllTextAsync(fullPath, Normalize(json) + "\n", cancellationToken).ConfigureAwait(false);
            Console.Out.WriteLine($"OpenAPI description written to {fullPath}");
        }
        finally
        {
            await app.StopAsync(cancellationToken).ConfigureAwait(false);
        }
    }

    /// <summary>Indented, with LF line endings: identical bytes on macOS and Windows.</summary>
    internal static string Normalize(string json)
        => JsonNode.Parse(json)!.ToJsonString(Indented).ReplaceLineEndings("\n");
}
