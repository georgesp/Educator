using Educator.Core.Rewards;

namespace Educator.Core.Tests.Rewards;

[Trait("Category", "Unit")]
public sealed class PointsRulesTests
{
    private static readonly PointsRules Rules = new(10, 20, 20);

    [Theory]
    [InlineData(10, 140)] // 100 + 20 + 20
    [InlineData(9, 110)]
    [InlineData(0, 20)] // effort is rewarded
    public void CompletedSeries(int correct, int points)
        => Assert.Equal(points, Rules.ForCompletedSeries(correct, 10));

    [Theory]
    [InlineData(0, 0)]
    [InlineData(4, 40)]
    public void AbandonedSeries_KeepsCorrectAnswersWithoutBonus(int correct, int points)
        => Assert.Equal(points, Rules.ForAbandonedSeries(correct));
}
