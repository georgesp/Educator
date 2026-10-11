namespace Educator.Core.Rewards;

/// <summary>
/// Points scale (SUB000110), read from <c>content/rewards/points.json</c>. Points are never taken away.
/// </summary>
/// <param name="PerCorrectAnswer">Points per correct answer.</param>
/// <param name="SeriesCompletedBonus">Bonus for a series answered to the end, whatever the score (rewards effort).</param>
/// <param name="PerfectSeriesBonus">Extra bonus for a series without any mistake.</param>
public sealed record PointsRules(int PerCorrectAnswer, int SeriesCompletedBonus, int PerfectSeriesBonus)
{
    public static PointsRules Default { get; } = new(10, 20, 20);

    /// <summary>Points of a series answered to the end.</summary>
    public int ForCompletedSeries(int correctAnswers, int exercises)
    {
        ArgumentOutOfRangeException.ThrowIfNegative(correctAnswers);
        ArgumentOutOfRangeException.ThrowIfGreaterThan(correctAnswers, exercises);
        return (correctAnswers * PerCorrectAnswer)
            + SeriesCompletedBonus
            + (correctAnswers == exercises ? PerfectSeriesBonus : 0);
    }

    /// <summary>Points of an abandoned series: the correct answers already given, without any bonus.</summary>
    public int ForAbandonedSeries(int correctAnswers)
    {
        ArgumentOutOfRangeException.ThrowIfNegative(correctAnswers);
        return correctAnswers * PerCorrectAnswer;
    }
}
