using Educator.Core.Exercises;

namespace Educator.Core.Tests.Exercises;

[Trait("Category", "Unit")]
public sealed class AdditionGeneratorTests
{
    [Fact]
    public void Generate_OperandsStayBetweenZeroAndNine()
    {
        var generator = new AdditionGenerator(new Random(1234), TimeProvider.System);

        var exercises = Enumerable.Range(0, 1000).Select(_ => generator.Generate()).ToList();

        Assert.All(exercises, e =>
        {
            Assert.InRange(e.Left, 0, 9);
            Assert.InRange(e.Right, 0, 9);
        });
        Assert.Contains(exercises, e => e.Left == 0);
        Assert.Contains(exercises, e => e.Right == 9);
    }

    [Fact]
    public void Generate_GivesUniqueVersion7Ids()
    {
        var generator = new AdditionGenerator(new Random(1), TimeProvider.System);

        var ids = Enumerable.Range(0, 100).Select(_ => generator.Generate().Id).ToList();

        Assert.Equal(ids.Count, ids.Distinct().Count());
        Assert.All(ids, id => Assert.Equal(7, id.Version));
    }

    [Theory]
    [InlineData(3, 4, 7, true)]
    [InlineData(3, 4, 8, false)]
    [InlineData(0, 0, 0, true)]
    [InlineData(9, 9, 18, true)]
    public void Check_ComparesWithTheSum(int left, int right, int answer, bool expected)
    {
        var exercise = new AdditionExercise(Guid.NewGuid(), left, right);

        var result = exercise.Check(answer);

        Assert.Equal(expected, result.IsCorrect);
        Assert.Equal(left + right, result.ExpectedAnswer);
    }

    [Fact]
    public void Statements_AreReadyToDisplayAndToRead()
    {
        var exercise = new AdditionExercise(Guid.NewGuid(), 3, 4);

        Assert.Equal("3 + 4", exercise.Statement);
        Assert.Equal("Combien font 3 plus 4 ?", exercise.SpokenStatement);
    }
}
