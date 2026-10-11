using Microsoft.Extensions.DependencyInjection;
using System.Net;
using Educator.Core.Exercises;
using Educator.Core.Skills;
using Educator.Host.Api;

namespace Educator.Host.Tests.Api;

[Trait("Category", "Integration")]
public sealed class SeriesEndpointsTests
{
    [Fact]
    public async Task Skills_OfAGrade_SortedByPeriod_WithTheCurrentOne()
    {
        await using var host = await ApiTestHost.StartAsync();

        using var response = await host.SendAsync(HttpMethod.Get, "/skills?grade=CP");
        var skills = await response.ReadAsync<SkillsResponse>();
        using var ce2 = await host.SendAsync(HttpMethod.Get, "/skills?grade=CE2");

        Assert.Equal(HttpStatusCode.OK, response.StatusCode);
        Assert.InRange(skills.CurrentPeriod, 1, 5);
        Assert.NotEmpty(skills.Skills);
        Assert.Equal(skills.Skills.OrderBy(s => s.Period).Select(s => s.Id), skills.Skills.Select(s => s.Id));
        Assert.All(skills.Skills, s => Assert.Equal(s.Period == skills.CurrentPeriod, s.IsCurrentPeriod));
        var fallback = await ce2.ReadAsync<SkillsResponse>();
        Assert.Equal(Core.Grade.CP, skills.SkillsGrade);
        Assert.Equal(Core.Grade.CE2, fallback.Grade);
        Assert.Equal(Core.Grade.CE1, fallback.SkillsGrade);
        Assert.NotEmpty(fallback.Skills);
    }

    [Fact]
    public async Task Series_HidesTheAnswers_ThenAwardsThePoints()
    {
        await using var host = await ApiTestHost.StartAsync();
        var profileId = await CreateProfileAsync(host);

        using var started = await host.SendAsync(HttpMethod.Post, "/series", new { profileId, skillId = "cp-add-sum-10", answerMode = "Keyboard", seed = 12 });
        var json = await started.Content.ReadAsStringAsync();
        var series = await started.ReadAsync<SeriesResponse>();

        Assert.Equal(HttpStatusCode.OK, started.StatusCode);
        Assert.Equal(12, series.Seed);
        Assert.Equal(10, series.Exercises.Count);
        Assert.DoesNotContain("expected", json, StringComparison.OrdinalIgnoreCase);
        Assert.All(series.Exercises, e =>
        {
            var hidden = e.Hidden switch { HiddenPart.Left => e.Left, HiddenPart.Right => e.Right, _ => e.Result };
            Assert.Null(hidden);
            Assert.Contains("?", e.Prompt, StringComparison.Ordinal);
            Assert.Null(e.Choices);
        });

        // Replay the same series in the Core to know the answers; answer all but the first correctly.
        var skill = host.App.Services.GetRequiredService<ISkillCatalog>().Find("cp-add-sum-10")!;
        var expected = SeriesGenerator.Generate(skill, 12, AnswerMode.Keyboard).Select(e => e.Expected).ToArray();
        SeriesAnswerResponse? last = null;
        for (var i = 0; i < 10; i++)
        {
            using var answered = await host.SendAsync(HttpMethod.Post, $"/series/{series.SeriesId}/answers", new { index = i, answer = i == 0 ? expected[i] + 1 : expected[i] });
            last = await answered.ReadAsync<SeriesAnswerResponse>();
            Assert.Equal(i != 0, last.Correct);
            Assert.Equal(expected[i], last.Expected);
        }

        Assert.Equal(9, last!.Summary!.CorrectCount);
        Assert.Equal(110, last.Summary.PointsEarned);
        Assert.Equal(110, last.Summary.ProfilePoints);
        using var profiles = await host.SendAsync(HttpMethod.Get, "/profiles");
        Assert.Equal(110, Assert.Single(await profiles.ReadAsync<ProfileResponse[]>()).Points);
    }

    [Fact]
    public async Task ChoiceMode_SendsFourProposals()
    {
        await using var host = await ApiTestHost.StartAsync();
        var profileId = await CreateProfileAsync(host);

        using var started = await host.SendAsync(HttpMethod.Post, "/series", new { profileId, skillId = "ce1-add-3-digits", answerMode = "Choice" });
        var series = await started.ReadAsync<SeriesResponse>();

        Assert.All(series.Exercises, e => Assert.Equal(4, e.Choices!.Distinct().Count()));
    }

    [Fact]
    public async Task Abandon_KeepsCorrectAnswers_ThenTheSeriesIsGone()
    {
        await using var host = await ApiTestHost.StartAsync();
        var profileId = await CreateProfileAsync(host);
        using var started = await host.SendAsync(HttpMethod.Post, "/series", new { profileId, skillId = "cp-add-tables", answerMode = "Keyboard", seed = 1 });
        var series = await started.ReadAsync<SeriesResponse>();
        var skill = host.App.Services.GetRequiredService<ISkillCatalog>().Find("cp-add-tables")!;
        var first = SeriesGenerator.Generate(skill, 1, AnswerMode.Keyboard)[0].Expected;
        (await host.SendAsync(HttpMethod.Post, $"/series/{series.SeriesId}/answers", new { index = 0, answer = first })).Dispose();

        using var again = await host.SendAsync(HttpMethod.Post, $"/series/{series.SeriesId}/answers", new { index = 0, answer = first });
        using var abandoned = await host.SendAsync(HttpMethod.Post, $"/series/{series.SeriesId}/abandon");
        var summary = await abandoned.ReadAsync<SeriesSummaryResponse>();
        using var afterwards = await host.SendAsync(HttpMethod.Post, $"/series/{series.SeriesId}/abandon");

        Assert.Equal(HttpStatusCode.Conflict, again.StatusCode);
        Assert.Equal(10, summary.PointsEarned);
        Assert.Equal(HttpStatusCode.NotFound, afterwards.StatusCode);
    }

    [Theory]
    [InlineData("unknown-skill", "Keyboard", HttpStatusCode.BadRequest)]
    [InlineData("cp-add-sum-10", "Typing", HttpStatusCode.BadRequest)]
    public async Task InvalidStart_Returns400(string skillId, string answerMode, HttpStatusCode status)
    {
        await using var host = await ApiTestHost.StartAsync();
        var profileId = await CreateProfileAsync(host);

        using var response = await host.SendAsync(HttpMethod.Post, "/series", new { profileId, skillId, answerMode });

        Assert.Equal(status, response.StatusCode);
    }

    [Fact]
    public async Task UnknownProfile_Returns404()
    {
        await using var host = await ApiTestHost.StartAsync();

        using var response = await host.SendAsync(HttpMethod.Post, "/series", new { profileId = Guid.NewGuid(), skillId = "cp-add-sum-10", answerMode = "Keyboard" });

        Assert.Equal(HttpStatusCode.NotFound, response.StatusCode);
    }

    private static async Task<Guid> CreateProfileAsync(ApiTestHost host)
    {
        using var response = await host.SendAsync(HttpMethod.Post, "/profiles", new { name = "Léa", grade = "CP" });
        response.EnsureSuccessStatusCode();
        return (await response.ReadAsync<ProfileResponse>()).Id;
    }
}
