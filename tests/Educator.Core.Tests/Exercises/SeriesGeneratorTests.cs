using Educator.Core.Exercises;
using Educator.Core.Skills;

namespace Educator.Core.Tests.Exercises;

[Trait("Category", "Unit")]
public sealed class SeriesGeneratorTests
{
    [Fact]
    public void SameSeedSameSkill_GiveTheSameSeries()
    {
        var skill = TestSkills.Make();

        var first = SeriesGenerator.Generate(skill, 42, AnswerMode.Choice);
        var second = SeriesGenerator.Generate(skill, 42, AnswerMode.Choice);
        var other = SeriesGenerator.Generate(skill, 43, AnswerMode.Choice);

        Assert.Equal(Describe(first), Describe(second));
        Assert.NotEqual(Describe(first), Describe(other));
    }

    [Fact]
    public void Series_HasTenExercisesWithoutDuplicates()
    {
        var skill = TestSkills.Make(constraints: new SkillConstraints(MaxResult: 10, ExcludeTrivial: [TrivialCase.PlusZero]));

        for (var seed = 0; seed < 500; seed++)
        {
            var series = SeriesGenerator.Generate(skill, seed, AnswerMode.Keyboard);

            Assert.Equal(SeriesGenerator.DefaultLength, series.Count);
            Assert.Equal(Enumerable.Range(0, 10), series.Select(e => e.Index));
            Assert.Equal(series.Count, series.Select(e => e.Calculation.DuplicateKey).Distinct().Count());
            Assert.All(series, e => Assert.Null(e.Choices));
        }
    }

    [Fact]
    public void Constraints_AreAlwaysRespected()
    {
        var skill = TestSkills.Make(
            SkillOperation.Subtraction,
            new OperandRule(20, 90, MultipleOf: 10),
            new OperandRule(1, 9),
            new SkillConstraints(Carry: CarryRule.Required));

        var exercises = Enumerable.Range(0, 1000).SelectMany(seed => SeriesGenerator.Generate(skill, seed, AnswerMode.Choice));

        Assert.All(exercises, e =>
        {
            Assert.Equal(ArithmeticOperation.Subtraction, e.Calculation.Operation);
            Assert.Equal(0, e.Calculation.Left % 10);
            Assert.InRange(e.Calculation.Right, 1, 9);
            Assert.True(e.Calculation.HasCarry);
            Assert.True(CalculationSpace.IsValid(skill, e.Calculation));
        });
    }

    [Fact]
    public void Formats_FollowTheSkill()
    {
        var resultOnly = TestSkills.Make(formats: [ExerciseFormat.Result]);
        var missingOnly = TestSkills.Make(formats: [ExerciseFormat.MissingOperand], id: "missing");
        var both = TestSkills.Make(id: "both");

        var all = Enumerable.Range(0, 200).SelectMany(seed => SeriesGenerator.Generate(both, seed, AnswerMode.Keyboard)).ToList();

        Assert.All(SeriesGenerator.Generate(resultOnly, 1, AnswerMode.Keyboard), e => Assert.Equal(HiddenPart.Result, e.Hidden));
        Assert.All(SeriesGenerator.Generate(missingOnly, 1, AnswerMode.Keyboard), e => Assert.NotEqual(HiddenPart.Result, e.Hidden));
        var resultShare = all.Count(e => e.Hidden == HiddenPart.Result) / (double)all.Count;
        Assert.InRange(resultShare, 0.5, 0.7);
        Assert.Contains(all, e => e.Hidden == HiddenPart.Left);
        Assert.Contains(all, e => e.Hidden == HiddenPart.Right);
    }

    [Fact]
    public void Choices_AreFourCredibleDifferentNaturalNumbers()
    {
        var skills = new[]
        {
            TestSkills.Make(constraints: new SkillConstraints(MaxResult: 10)),
            TestSkills.Make(SkillOperation.Subtraction, new OperandRule(0, 20), new OperandRule(0, 10), id: "sub"),
            TestSkills.Make(SkillOperation.Mixed, new OperandRule(100, 999), new OperandRule(10, 90, MultipleOf: 10), id: "mixed"),
        };

        foreach (var skill in skills)
        {
            for (var seed = 0; seed < 300; seed++)
            {
                foreach (var exercise in SeriesGenerator.Generate(skill, seed, AnswerMode.Choice))
                {
                    var choices = exercise.Choices!;
                    Assert.Equal(SeriesGenerator.ChoiceCount, choices.Count);
                    Assert.Equal(choices.Count, choices.Distinct().Count());
                    Assert.All(choices, c => Assert.True(c >= 0));
                    Assert.Single(choices, c => c == exercise.Expected);
                }
            }
        }
    }

    [Fact]
    public void Choices_IncludeTheClassicMistakes()
    {
        var exercise = new Exercise(0, new Calculation(ArithmeticOperation.Addition, 27, 15), HiddenPart.Result, null);

        var choices = SeriesGenerator.BuildChoices(exercise, new Random(3));

        Assert.Contains(42, choices);
        Assert.Contains(choices, c => c is 41 or 43);
        Assert.Contains(choices, c => c is 32 or 52);
        Assert.Contains(12, choices); // 27 − 15: inverted operation
    }

    [Fact]
    public void Choices_ForAMissingNumber_IncludeTheCopiedResult()
    {
        var exercise = new Exercise(0, new Calculation(ArithmeticOperation.Addition, 68, 2), HiddenPart.Right, null);

        var choices = SeriesGenerator.BuildChoices(exercise, new Random(5));

        Assert.Contains(2, choices);
        Assert.Contains(70, choices);
        Assert.All(choices, c => Assert.InRange(c, 0, 80));
    }

    [Fact]
    public void NarrowSkill_IsRefused()
    {
        var narrow = TestSkills.Make(constraints: new SkillConstraints(ExactResult: 10), formats: [ExerciseFormat.MissingOperand]);

        Assert.Equal(6, CalculationSpace.For(narrow).DistinctCount); // 0+10, 1+9 … 5+5
        Assert.Throws<InvalidOperationException>(() => SeriesGenerator.Generate(narrow, 1, AnswerMode.Keyboard));
    }

    private static string Describe(IEnumerable<Exercise> series)
        => string.Join(" | ", series.Select(e => $"{e.Prompt} [{string.Join(',', e.Choices ?? [])}]"));
}
