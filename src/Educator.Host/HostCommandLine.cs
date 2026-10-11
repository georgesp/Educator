using System.Globalization;

namespace Educator.Host;

/// <summary>
/// Command line of the host:
/// <c>educator-host [--port &lt;n&gt;] [--parent-pid &lt;pid&gt;] [--export-openapi &lt;file&gt;]</c>.
/// <list type="bullet">
///   <item><description><c>--port</c>: default 0, a free port chosen by the OS and reported on stdout.</description></item>
///   <item><description><c>--parent-pid</c>: the host stops when that process exits (Tauri shell).</description></item>
///   <item><description><c>--export-openapi</c>: writes the OpenAPI description to the file and exits
///   (used by <c>npm run api:generate</c>).</description></item>
/// </list>
/// </summary>
internal sealed record HostCommandLine(int Port, int? ParentProcessId, string? ExportOpenApiPath)
{
    public const int DefaultPort = 0;

    private const string Usage = "Usage: educator-host [--port <n>] [--parent-pid <pid>] [--export-openapi <file>]";

    public static HostCommandLine Parse(IReadOnlyList<string> args)
    {
        ArgumentNullException.ThrowIfNull(args);

        var port = DefaultPort;
        int? parentProcessId = null;
        string? exportOpenApiPath = null;

        for (var i = 0; i < args.Count; i++)
        {
            switch (args[i])
            {
                case "--port":
                    port = ReadInt(args, ++i, "--port", min: 0, max: 65535);
                    break;
                case "--parent-pid":
                    parentProcessId = ReadInt(args, ++i, "--parent-pid", min: 1, max: int.MaxValue);
                    break;
                case "--export-openapi":
                    exportOpenApiPath = ReadString(args, ++i, "--export-openapi");
                    break;
                default:
                    throw new ArgumentException($"Unknown argument '{args[i]}'. {Usage}");
            }
        }

        return new HostCommandLine(port, parentProcessId, exportOpenApiPath);
    }

    private static int ReadInt(IReadOnlyList<string> args, int index, string name, int min, int max)
    {
        if (index >= args.Count
            || !int.TryParse(args[index], NumberStyles.None, CultureInfo.InvariantCulture, out var value)
            || value < min
            || value > max)
        {
            throw new ArgumentException($"{name} expects an integer between {min} and {max}.");
        }

        return value;
    }

    private static string ReadString(IReadOnlyList<string> args, int index, string name)
        => index < args.Count && !string.IsNullOrWhiteSpace(args[index])
            ? args[index]
            : throw new ArgumentException($"{name} expects a value.");
}
