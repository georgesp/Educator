using Educator.Core.Exercises;
using Educator.Core.Profiles;
using Educator.Core.Rewards;
using Educator.Core.Skills;
using Educator.Core.Tests.Profiles;

namespace Educator.Core.Tests.Exercises;

[Trait("Category", "Unit")]
public sealed class SeriesServiceTests
{
    private readonly InMemoryProfileStore profiles = new();
    private readonly ManualTimeProvider clock = new(new DateTimeOffset(2026, 10, 11, 8, 0, 0, TimeSpan.Zero));

    [Fact]
    public async Task PerfectSeries_EarnsAnswersAndBothBonuses()
    {
        var (service, profile) = await SetUpAsync();
        var series = await service.StartAsync(profile.Id, "test-skill", AnswerMode.Keyboard, seed: 7);

        AnswerOutcome? last = null;
        foreach (var exercise in series.Exercises)
        {
            var outcome = await service.AnswerAsync(series.Id, exercise.Index, exercise.Expected);
            Assert.True(outcome.Correct);
            Assert.Equal(exercise.Index == 9, outcome.Summary is not null);
            last = outcome;
        }

        Assert.Equal(new SeriesSummary(10, 10, 140, 140), last!.Summary);
        Assert.Equal(140, (await profiles.GetAsync(profile.Id))!.Points);
    }

    [Fact]
    public async Task Mistakes_AreCorrectedWithoutLosingPoints()
    {
        var (service, profile) = await SetUpAsync();
        var series = await service.StartAsync(profile.Id, "test-skill", AnswerMode.Choice, seed: 7);

        var wrong = await service.AnswerAsync(series.Id, 0, series.Exercises[0].Expected + 1);
        foreach (var exercise in series.Exercises.Skip(1))
        {
            await service.AnswerAsync(series.Id, exercise.Index, exercise.Expected);
        }

        Assert.False(wrong.Correct);
        Assert.Equal(series.Exercises[0].Expected, wrong.Expected);
        Assert.Equal(series.Exercises[0].Explanation, wrong.Explanation);
        Assert.Equal(90 + 20, (await profiles.GetAsync(profile.Id))!.Points);
    }

    [Fact]
    public async Task AbandonedSeries_KeepsCorrectAnswersWithoutBonus()
    {
        var (service, profile) = await SetUpAsync();
        var series = await service.StartAsync(profile.Id, "test-skill", AnswerMode.Keyboard, seed: 3);
        await service.AnswerAsync(series.Id, 0, series.Exercises[0].Expected);
        await service.AnswerAsync(series.Id, 1, series.Exercises[1].Expected);
        await service.AnswerAsync(series.Id, 2, -1);

        var summary = await service.AbandonAsync(series.Id);

        Assert.Equal(new SeriesSummary(2, 10, 20, 20), summary);
        await Assert.ThrowsAsync<SeriesNotFoundException>(() => service.AnswerAsync(series.Id, 3, 0));
    }

    [Fact]
    public async Task SameExercise_CannotBeAnsweredTwice()
    {
        var (service, profile) = await SetUpAsync();
        var series = await service.StartAsync(profile.Id, "test-skill", AnswerMode.Keyboard);
        await service.AnswerAsync(series.Id, 0, 1);

        await Assert.ThrowsAsync<SeriesRequestException>(() => service.AnswerAsync(series.Id, 0, 1));
        await Assert.ThrowsAsync<SeriesRequestException>(() => service.AnswerAsync(series.Id, 10, 1));
    }

    [Fact]
    public async Task Start_ChecksProfileSkillAndMode()
    {
        var keyboardOnly = TestSkills.Make(id: "keyboard-only") with { AnswerModes = [AnswerMode.Keyboard] };
        var (service, profile) = await SetUpAsync(keyboardOnly);

        await Assert.ThrowsAsync<ProfileNotFoundException>(() => service.StartAsync(Guid.NewGuid(), "test-skill", AnswerMode.Keyboard));
        await Assert.ThrowsAsync<SeriesRequestException>(() => service.StartAsync(profile.Id, "unknown", AnswerMode.Keyboard));
        await Assert.ThrowsAsync<SeriesRequestException>(() => service.StartAsync(profile.Id, "keyboard-only", AnswerMode.Choice));
    }

    [Fact]
    public async Task SameSeed_ReplaysTheSameSeries()
    {
        var (service, profile) = await SetUpAsync();

        var first = await service.StartAsync(profile.Id, "test-skill", AnswerMode.Choice, seed: 99);
        var second = await service.StartAsync(profile.Id, "test-skill", AnswerMode.Choice, seed: 99);

        Assert.NotEqual(first.Id, second.Id);
        Assert.Equal(first.Exercises.Select(e => e.Prompt), second.Exercises.Select(e => e.Prompt));
    }

    private async Task<(SeriesService Service, Profile Profile)> SetUpAsync(params Skill[] extraSkills)
    {
        var profile = await profiles.AddAsync(new Profile
        {
            Id = Guid.CreateVersion7(),
            Name = "Léa",
            Grade = Grade.CP,
            CharacterId = "lion",
            Points = 0,
            LastUsedAt = clock.GetUtcNow(),
        });
        var catalog = new TestCatalog([TestSkills.Make(constraints: new SkillConstraints(MaxResult: 10)), .. extraSkills]);
        return (new SeriesService(catalog, profiles, PointsRules.Default, new ScriptedRandom(5), clock), profile);
    }

    [Fact]
    public void Catalog_FallsBackToCe1ForGradesWithoutSkills()
    {
        var ce1 = TestSkills.Make(id: "ce1-skill") with { Grade = Grade.CE1 };
        var catalog = new TestCatalog([TestSkills.Make(), ce1]);

        Assert.Equal(Grade.CP, catalog.ForOrFallback(Subject.Math, Grade.CP).Grade);
        var (grade, skills) = catalog.ForOrFallback(Subject.Math, Grade.CM2);
        Assert.Equal(Grade.CE1, grade);
        Assert.Equal("ce1-skill", Assert.Single(skills).Id);
    }

    private sealed class TestCatalog(IReadOnlyList<Skill> skills) : ISkillCatalog
    {
        public IReadOnlyList<Skill> All => skills;

        public Skill? Find(string id) => skills.FirstOrDefault(s => s.Id == id);
    }
}
