using System.Text.Json.Nodes;
using Educator.Content.Skills;
using Educator.Core;
using Educator.Core.Exercises;
using Educator.Core.Skills;

namespace Educator.Content.Tests;

/// <summary>
/// The committed content must be valid: a broken rule fails these tests (and the CI), never the
/// application in front of a child (SUB000106, SUB000107, SUB000108).
/// </summary>
[Trait("Category", "Unit")]
public sealed class ContentValidationTests
{
    public static TheoryData<string> SkillIds => new(RepositoryContent.Content.Skills.All.Select(s => s.Id));

    [Fact]
    public void Content_HasNoProblem()
    {
        var problems = RepositoryContent.Content.Problems;

        Assert.True(problems.Count == 0, string.Join(Environment.NewLine, problems));
    }

    [Theory]
    [InlineData("skills/math", "*.json", "skill.schema.json")]
    [InlineData("calendar", "periods.json", "calendar.schema.json")]
    [InlineData("rewards", "points.json", "points.schema.json")]
    [InlineData("characters", "characters.json", "characters.schema.json")]
    [InlineData("characters", "palette.json", "palette.schema.json")]
    public void EveryFile_MatchesItsJsonSchema(string folder, string pattern, string schemaName)
    {
        var schema = JsonNode.Parse(File.ReadAllText(RepositoryContent.PathOf("schemas", schemaName)))!.AsObject();
        var files = Directory.GetFiles(RepositoryContent.PathOf(folder.Split('/')), pattern);

        Assert.NotEmpty(files);
        foreach (var file in files)
        {
            var errors = MiniJsonSchema.Validate(JsonNode.Parse(File.ReadAllText(file)), schema);
            Assert.True(errors.Count == 0, $"{Path.GetFileName(file)}:{Environment.NewLine}{string.Join(Environment.NewLine, errors)}");
        }
    }

    [Fact]
    public void CpAndCe1_HaveSkillsInEveryPeriod()
    {
        foreach (var grade in new[] { Grade.CP, Grade.CE1 })
        {
            var periods = RepositoryContent.Content.Skills.For(Subject.Math, grade).Select(s => s.Period).Distinct().Order();
            Assert.Equal([1, 2, 3, 4, 5], periods);
        }
    }

    [Theory]
    [MemberData(nameof(SkillIds))]
    public void Skill_AllowsAtLeastTwentyDifferentExercises(string id)
    {
        var skill = RepositoryContent.Content.Skills.Find(id)!;

        Assert.True(CalculationSpace.For(skill).DistinctCount >= SkillLoader.MinDistinctExercises);
    }

    /// <summary>Property test: 10 000 exercises per skill (1 000 series) respect every rule.</summary>
    [Theory]
    [MemberData(nameof(SkillIds))]
    public void Skill_TenThousandDrawsRespectEveryConstraint(string id)
    {
        var skill = RepositoryContent.Content.Skills.Find(id)!;

        for (var seed = 0; seed < 1000; seed++)
        {
            var mode = skill.AnswerModes[seed % skill.AnswerModes.Count];
            var series = SeriesGenerator.Generate(skill, seed, mode);
            Assert.Equal(series.Count, series.Select(e => e.Calculation.DuplicateKey).Distinct().Count());
            foreach (var exercise in series)
            {
                Assert.True(CalculationSpace.IsValid(skill, exercise.Calculation), $"{id} seed {seed}: {exercise.Calculation}");
                Assert.Contains(exercise.Format, skill.Formats);
                Assert.True(exercise.Expected >= 0);
                if (exercise.Choices is { } choices)
                {
                    Assert.Equal(4, choices.Distinct().Count(c => c >= 0));
                    Assert.Contains(exercise.Expected, choices);
                }
            }
        }
    }

    [Fact]
    public void Skill_CurrentPeriodComesFromTheCalendarFile()
    {
        var calendar = RepositoryContent.Content.Calendar;

        Assert.Equal(1, calendar.GetPeriod(new DateOnly(2026, 10, 11)));
        Assert.Equal(5, calendar.GetPeriod(new DateOnly(2027, 6, 30)));
    }

    [Fact]
    public void Points_ComeFromTheRewardsFile()
        => Assert.Equal(new Core.Rewards.PointsRules(10, 20, 20), RepositoryContent.Content.Points);
}
