using Educator.Content.Speech;
using Educator.Core;
using Educator.Core.Characters;
using Educator.Core.Exercises;
using Educator.Core.Profiles;
using Educator.Core.Skills;
using Educator.Core.Speech;
using Microsoft.AspNetCore.Http.HttpResults;
using Microsoft.AspNetCore.Mvc;

namespace Educator.Host.Api;

/// <summary>
/// Lot 1 routes (under <c>/api/ui</c>, token and CORS from the group): skills, characters, profiles,
/// series of exercises and speech. Thin adapters over the Core services.
/// Errors are RFC 7807 problems; for the profiles, <c>detail</c> is a simple French sentence for the child.
/// </summary>
internal static class LearningEndpoints
{
    /// <summary>Default speaking rate: slightly slow for children who are learning to read.</summary>
    public const double ChildSpeechSpeed = 0.9;

    public const int MaxSpeechLength = 300;

    public static RouteGroupBuilder MapLearningEndpoints(this RouteGroupBuilder group)
    {
        group.MapGet("/skills", GetSkills).WithName("getSkills");

        group.MapGet("/characters", GetCharacters).WithName("getCharacters");
        group.MapGet("/characters/draw", DrawCharacter).WithName("drawCharacter");
        group.MapGet("/characters/{id}.svg", GetCharacterSvg)
            .WithName("getCharacterSvg")
            .Produces<string>(StatusCodes.Status200OK, "image/svg+xml")
            .ProducesProblem(StatusCodes.Status404NotFound);

        var profiles = group.MapGroup("/profiles");
        profiles.MapGet("/", ListProfilesAsync).WithName("listProfiles");
        profiles.MapPost("/", CreateProfileAsync).WithName("createProfile");
        profiles.MapPatch("/{id:guid}", UpdateProfileAsync).WithName("updateProfile");
        profiles.MapPost("/{id:guid}/reroll-character", RerollCharacterAsync).WithName("rerollCharacter");
        profiles.MapPost("/{id:guid}/select", SelectProfileAsync).WithName("selectProfile");
        profiles.MapDelete("/{id:guid}", DeleteProfileAsync).WithName("deleteProfile");

        var series = group.MapGroup("/series");
        series.MapPost("/", StartSeriesAsync).WithName("startSeries");
        series.MapPost("/{id:guid}/answers", AnswerAsync).WithName("answerSeries");
        series.MapPost("/{id:guid}/abandon", AbandonAsync).WithName("abandonSeries");

        group.MapPost("/speech", SpeakAsync)
            .WithName("speak")
            .Produces<byte[]>(StatusCodes.Status200OK, "audio/wav")
            .ProducesProblem(StatusCodes.Status400BadRequest)
            .ProducesProblem(StatusCodes.Status503ServiceUnavailable);

        return group;
    }

    // ── Skills ───────────────────────────────────────────────────────────────────────────────────

    private static Ok<SkillsResponse> GetSkills(Grade grade, ISkillCatalog skills, SchoolCalendar calendar, TimeProvider timeProvider)
    {
        var currentPeriod = calendar.GetPeriod(DateOnly.FromDateTime(timeProvider.GetLocalNow().DateTime));
        var (skillsGrade, list) = skills.ForOrFallback(Subject.Math, grade);
        return TypedResults.Ok(new SkillsResponse
        {
            Grade = grade,
            SkillsGrade = skillsGrade,
            CurrentPeriod = currentPeriod,
            Skills =
            [
                .. list.Select(s => new SkillResponse
                {
                    Id = s.Id,
                    Title = s.Title,
                    Instruction = s.Instruction,
                    Period = s.Period,
                    IsCurrentPeriod = s.Period == currentPeriod,
                    AnswerModes = s.AnswerModes,
                }),
            ],
        });
    }

    // ── Characters ───────────────────────────────────────────────────────────────────────────────

    private static Ok<CharacterResponse[]> GetCharacters(ICharacterCatalog characters)
        => TypedResults.Ok(characters.All.Select(c => new CharacterResponse { Id = c.Id, Name = c.Name, Kind = c.Kind }).ToArray());

    /// <summary>A character at random for the creation screen, different from <paramref name="except"/>.</summary>
    private static Ok<CharacterResponse> DrawCharacter(ProfileService profiles, string? except = null)
    {
        var character = profiles.DrawCharacter(except);
        return TypedResults.Ok(new CharacterResponse { Id = character.Id, Name = character.Name, Kind = character.Kind });
    }

    private static IResult GetCharacterSvg(string id, ICharacterCatalog characters)
    {
        var svg = characters.GetSvg(id);
        return svg is null
            ? NotFound("Unknown character", $"No character with id {id}.")
            : TypedResults.Text(svg, "image/svg+xml");
    }

    // ── Profiles ─────────────────────────────────────────────────────────────────────────────────

    private static async Task<Ok<ProfileResponse[]>> ListProfilesAsync(ProfileService service, CancellationToken cancellationToken)
        => TypedResults.Ok((await service.ListAsync(cancellationToken).ConfigureAwait(false)).Select(ToResponse).ToArray());

    private static async Task<Results<Created<ProfileResponse>, ProblemHttpResult>> CreateProfileAsync(
        CreateProfileRequest request,
        ProfileService service,
        CancellationToken cancellationToken)
    {
        try
        {
            var profile = await service.CreateAsync(request.Name, request.Grade, request.CharacterId, cancellationToken).ConfigureAwait(false);
            return TypedResults.Created($"{UiEndpoints.RoutePrefix}/profiles/{profile.Id}", ToResponse(profile));
        }
        catch (ProfileValidationException ex)
        {
            return Invalid(ex);
        }
    }

    private static Task<Results<Ok<ProfileResponse>, ProblemHttpResult>> UpdateProfileAsync(
        Guid id,
        UpdateProfileRequest request,
        ProfileService service,
        CancellationToken cancellationToken)
        => ProfileActionAsync(() => service.UpdateAsync(id, request.Name, request.Grade, request.CharacterId, cancellationToken));

    private static Task<Results<Ok<ProfileResponse>, ProblemHttpResult>> RerollCharacterAsync(Guid id, ProfileService service, CancellationToken cancellationToken)
        => ProfileActionAsync(() => service.RerollCharacterAsync(id, cancellationToken));

    private static Task<Results<Ok<ProfileResponse>, ProblemHttpResult>> SelectProfileAsync(Guid id, ProfileService service, CancellationToken cancellationToken)
        => ProfileActionAsync(() => service.SelectAsync(id, cancellationToken));

    private static async Task<Results<NoContent, ProblemHttpResult>> DeleteProfileAsync(Guid id, ProfileService service, CancellationToken cancellationToken)
    {
        try
        {
            await service.DeleteAsync(id, cancellationToken).ConfigureAwait(false);
            return TypedResults.NoContent();
        }
        catch (ProfileNotFoundException ex)
        {
            return NotFound("Unknown profile", ex.Message);
        }
    }

    private static async Task<Results<Ok<ProfileResponse>, ProblemHttpResult>> ProfileActionAsync(Func<Task<Profile>> action)
    {
        try
        {
            return TypedResults.Ok(ToResponse(await action().ConfigureAwait(false)));
        }
        catch (ProfileValidationException ex)
        {
            return Invalid(ex);
        }
        catch (ProfileNotFoundException ex)
        {
            return NotFound("Unknown profile", ex.Message);
        }
    }

    private static ProfileResponse ToResponse(Profile profile) => new()
    {
        Id = profile.Id,
        Name = profile.Name,
        Grade = profile.Grade,
        CharacterId = profile.CharacterId,
        Points = profile.Points,
        LastUsedAt = profile.LastUsedAt,
    };

    // ── Series ───────────────────────────────────────────────────────────────────────────────────

    private static async Task<Results<Ok<SeriesResponse>, ProblemHttpResult>> StartSeriesAsync(
        StartSeriesRequest request,
        SeriesService service,
        CancellationToken cancellationToken)
    {
        try
        {
            var series = await service.StartAsync(request.ProfileId, request.SkillId, request.AnswerMode, request.Seed, cancellationToken).ConfigureAwait(false);
            return TypedResults.Ok(new SeriesResponse
            {
                SeriesId = series.Id,
                Seed = series.Seed,
                SkillId = series.Skill.Id,
                Instruction = series.Skill.Instruction,
                AnswerMode = series.AnswerMode,
                Exercises = [.. series.Exercises.Select(ToResponse)],
            });
        }
        catch (ProfileNotFoundException ex)
        {
            return NotFound("Unknown profile", ex.Message);
        }
        catch (SeriesRequestException ex)
        {
            return TypedResults.Problem(title: "Invalid series request", detail: ex.Message, statusCode: StatusCodes.Status400BadRequest);
        }
    }

    private static async Task<Results<Ok<SeriesAnswerResponse>, ProblemHttpResult>> AnswerAsync(
        Guid id,
        SeriesAnswerRequest request,
        SeriesService service,
        CancellationToken cancellationToken)
    {
        try
        {
            var outcome = await service.AnswerAsync(id, request.Index, request.Answer, cancellationToken).ConfigureAwait(false);
            return TypedResults.Ok(new SeriesAnswerResponse
            {
                Correct = outcome.Correct,
                Expected = outcome.Expected,
                Explanation = outcome.Explanation,
                Summary = outcome.Summary is null ? null : ToResponse(outcome.Summary),
            });
        }
        catch (Exception ex) when (ex is SeriesNotFoundException or ProfileNotFoundException)
        {
            return NotFound("Unknown series", ex.Message);
        }
        catch (SeriesRequestException ex)
        {
            return TypedResults.Problem(title: "Answer refused", detail: ex.Message, statusCode: StatusCodes.Status409Conflict);
        }
    }

    private static async Task<Results<Ok<SeriesSummaryResponse>, ProblemHttpResult>> AbandonAsync(Guid id, SeriesService service, CancellationToken cancellationToken)
    {
        try
        {
            return TypedResults.Ok(ToResponse(await service.AbandonAsync(id, cancellationToken).ConfigureAwait(false)));
        }
        catch (Exception ex) when (ex is SeriesNotFoundException or ProfileNotFoundException)
        {
            return NotFound("Unknown series", ex.Message);
        }
        catch (SeriesRequestException ex)
        {
            return TypedResults.Problem(title: "Series already over", detail: ex.Message, statusCode: StatusCodes.Status409Conflict);
        }
    }

    private static ExerciseResponse ToResponse(Exercise exercise) => new()
    {
        Index = exercise.Index,
        Left = exercise.Hidden == HiddenPart.Left ? null : exercise.Calculation.Left,
        Operator = exercise.Calculation.Symbol,
        Right = exercise.Hidden == HiddenPart.Right ? null : exercise.Calculation.Right,
        Result = exercise.Hidden == HiddenPart.Result ? null : exercise.Calculation.Result,
        Hidden = exercise.Hidden,
        Prompt = exercise.Prompt,
        SpokenPrompt = exercise.SpokenPrompt,
        Choices = exercise.Choices,
    };

    private static SeriesSummaryResponse ToResponse(SeriesSummary summary) => new()
    {
        CorrectCount = summary.CorrectCount,
        ExerciseCount = summary.ExerciseCount,
        PointsEarned = summary.PointsEarned,
        ProfilePoints = summary.ProfilePoints,
    };

    // ── Speech ───────────────────────────────────────────────────────────────────────────────────

    // IResult (not Results<…>) so that the OpenAPI metadata comes from Produces* only (audio/wav).
    private static async Task<IResult> SpeakAsync([FromBody] SpeechRequest request, SpeechCache speech, CancellationToken cancellationToken)
    {
        var text = FrenchSpeechText.Normalize(request.Text ?? string.Empty);
        var speed = request.Speed ?? ChildSpeechSpeed;
        if (text.Length == 0 || text.Length > MaxSpeechLength || speed is < SpeechSpeed.Min or > SpeechSpeed.Max)
        {
            return TypedResults.Problem(
                title: "Invalid speech request",
                detail: $"Text of 1 to {MaxSpeechLength} characters, speed from {SpeechSpeed.Min} to {SpeechSpeed.Max}.",
                statusCode: StatusCodes.Status400BadRequest);
        }

        try
        {
            var wav = await speech.SynthesizeAsync(text, speed, cancellationToken).ConfigureAwait(false);
            return TypedResults.File(wav, "audio/wav");
        }
        catch (SpeechUnavailableException ex)
        {
            return TypedResults.Problem(title: "Speech unavailable", detail: ex.Message, statusCode: StatusCodes.Status503ServiceUnavailable);
        }
    }

    // ── Problems ─────────────────────────────────────────────────────────────────────────────────

    private static ProblemHttpResult Invalid(ProfileValidationException ex) => TypedResults.Problem(
        title: "Invalid profile",
        detail: ex.Message,
        statusCode: StatusCodes.Status400BadRequest,
        extensions: new Dictionary<string, object?> { ["field"] = ex.Field });

    private static ProblemHttpResult NotFound(string title, string detail)
        => TypedResults.Problem(title: title, detail: detail, statusCode: StatusCodes.Status404NotFound);
}
