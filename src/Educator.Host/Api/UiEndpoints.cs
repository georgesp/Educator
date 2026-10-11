using System.Reflection;
using Educator.Core.Exercises;
using Educator.Core.Speech;
using Educator.Data;
using Microsoft.AspNetCore.Http.HttpResults;

namespace Educator.Host.Api;

/// <summary>
/// Routes used by the web front (<c>/api/ui</c>): token required, front origins only (CORS).
/// Thin adapters over the application services — no business rule lives here.
/// Errors are RFC 7807 problems: 401 bad token, 404 unknown exercise, 503 voice unavailable.
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

        var additions = group.MapGroup("/exercises/additions");
        additions.MapPost("/", CreateAddition).WithName("createAddition");
        additions.MapPost("/{id:guid}/answer", AnswerAddition)
            .WithName("answerAddition")
            .ProducesProblem(StatusCodes.Status404NotFound);
        additions.MapGet("/{id:guid}/speech", ReadAdditionAsync)
            .WithName("readAddition")
            .Produces<byte[]>(StatusCodes.Status200OK, "audio/wav")
            .ProducesProblem(StatusCodes.Status404NotFound)
            .ProducesProblem(StatusCodes.Status503ServiceUnavailable);

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

    private static Ok<AdditionExerciseResponse> CreateAddition(AdditionGenerator generator, PendingExercises pending)
    {
        var exercise = generator.Generate();
        pending.Add(exercise);
        return TypedResults.Ok(new AdditionExerciseResponse
        {
            Id = exercise.Id,
            Left = exercise.Left,
            Right = exercise.Right,
            Statement = exercise.Statement,
        });
    }

    private static Results<Ok<AnswerResponse>, ProblemHttpResult> AnswerAddition(Guid id, AnswerRequest request, PendingExercises pending)
    {
        if (!pending.TryGet(id, out var exercise))
        {
            return UnknownExercise(id);
        }

        var result = exercise.Check(request.Answer);
        return TypedResults.Ok(new AnswerResponse { IsCorrect = result.IsCorrect, ExpectedAnswer = result.ExpectedAnswer });
    }

    // IResult (not Results<…>) so that the OpenAPI metadata comes from Produces* only (audio/wav).
    private static async Task<IResult> ReadAdditionAsync(
        Guid id,
        PendingExercises pending,
        ISpeechSynthesizer speech,
        CancellationToken cancellationToken,
        double speed = SpeechSpeed.Normal)
    {
        if (!pending.TryGet(id, out var exercise))
        {
            return UnknownExercise(id);
        }

        try
        {
            var wav = await speech.SynthesizeAsync(exercise.SpokenStatement, speed, cancellationToken).ConfigureAwait(false);
            return TypedResults.File(wav, "audio/wav");
        }
        catch (SpeechUnavailableException ex)
        {
            return TypedResults.Problem(
                title: "Speech unavailable",
                detail: ex.Message,
                statusCode: StatusCodes.Status503ServiceUnavailable);
        }
    }

    private static ProblemHttpResult UnknownExercise(Guid id) => TypedResults.Problem(
        title: "Unknown exercise",
        detail: $"No pending exercise with id {id}.",
        statusCode: StatusCodes.Status404NotFound);
}
