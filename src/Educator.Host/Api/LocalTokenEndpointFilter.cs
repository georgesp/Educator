using System.Security.Cryptography;
using System.Text;

namespace Educator.Host.Api;

/// <summary>
/// Endpoint filter protecting the UI routes: every request must carry the <c>X-Local-Token</c> header
/// matching <see cref="LocalApiOptions.UiToken"/>, otherwise <c>401</c>. Prevents other local processes
/// and web pages from using the API. Constant-time comparison.
/// </summary>
internal sealed class LocalTokenEndpointFilter : IEndpointFilter
{
    public const string HeaderName = "X-Local-Token";

    private readonly byte[] expected;

    public LocalTokenEndpointFilter(LocalApiOptions options)
    {
        ArgumentNullException.ThrowIfNull(options);
        expected = Encoding.UTF8.GetBytes(options.UiToken);
    }

    public async ValueTask<object?> InvokeAsync(EndpointFilterInvocationContext context, EndpointFilterDelegate next)
    {
        ArgumentNullException.ThrowIfNull(context);
        ArgumentNullException.ThrowIfNull(next);

        var provided = context.HttpContext.Request.Headers[HeaderName].ToString();
        if (!CryptographicOperations.FixedTimeEquals(Encoding.UTF8.GetBytes(provided), expected))
        {
            return TypedResults.Problem(
                title: "Missing or invalid local token",
                detail: $"The {HeaderName} header is required.",
                statusCode: StatusCodes.Status401Unauthorized);
        }

        return await next(context).ConfigureAwait(false);
    }
}
