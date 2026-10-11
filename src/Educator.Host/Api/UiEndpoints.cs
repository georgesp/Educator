using System.Reflection;
using Educator.Core.Speech;
using Educator.Data;
using Microsoft.AspNetCore.Http.HttpResults;

namespace Educator.Host.Api;

/// <summary>
/// Routes used by the web front (<c>/api/ui</c>): token required, front origins only (CORS).
/// Thin adapters over the application services — no business rule lives here. The lot 1 routes are in
/// <see cref="LearningEndpoints"/>. Errors are RFC 7807 problems (401 bad token, 404, 503 voice unavailable).
/// </summary>
internal static class UiEndpoints
{
    public const string RoutePrefix = "/api/ui";
    public const string CorsPolicyName = "educator-ui";

    public static IEndpointRouteBuilder MapUiEndpoints(this IEndpointRouteBuilder routes)
    {
        var group = routes.MapGroup(RoutePrefix)
            .RequireCors(CorsPolicyName)
            .AddEndpointFilter<LocalTokenEndpointFilter>()
            .WithTags("UI");

        group.MapGet("/status", GetStatus).WithName("getStatus");
        group.MapLearningEndpoints();

        return routes;
    }

    private static Ok<StatusResponse> GetStatus(DatabaseInitializer database, ISpeechSynthesizer speech)
    {
        var speechStatus = speech.Status;
        return TypedResults.Ok(new StatusResponse
        {
            Version = typeof(UiEndpoints).Assembly
                .GetCustomAttribute<AssemblyInformationalVersionAttribute>()?.InformationalVersion.Split('+')[0] ?? "0.0.0",
            Database = new DatabaseStatusResponse { IsReady = database.IsInitialized },
            Speech = new SpeechStatusResponse
            {
                IsAvailable = speechStatus.IsAvailable,
                Voice = speechStatus.Voice,
                Problem = speechStatus.Problem,
            },
        });
    }
}
