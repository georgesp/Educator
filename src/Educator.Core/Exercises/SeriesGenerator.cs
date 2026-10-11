using Educator.Core.Skills;

namespace Educator.Core.Exercises;

/// <summary>
/// Generates a series of exercises from a skill (SUB000107). Pure and deterministic: the same skill,
/// seed, length and answer mode always give the same series (debugging, tests).
/// </summary>
public static class SeriesGenerator
{
    public const int DefaultLength = 10;

    /// <summary>Share of <see cref="ExerciseFormat.Result"/> when the skill allows both formats.</summary>
    public const double ResultFormatShare = 0.6;

    public const int ChoiceCount = 4;

    /// <exception cref="InvalidOperationException">The skill allows fewer calculations than <paramref name="length"/>.</exception>
    public static IReadOnlyList<Exercise> Generate(Skill skill, int seed, AnswerMode answerMode, int length = DefaultLength)
    {
        ArgumentNullException.ThrowIfNull(skill);
        ArgumentOutOfRangeException.ThrowIfLessThan(length, 1);

        var space = CalculationSpace.For(skill);
        if (space.DistinctCount < length)
        {
            throw new InvalidOperationException(
                $"Skill {skill.Id} allows {space.DistinctCount} different calculations, fewer than {length}.");
        }

        var random = new Random(seed);
        var used = new HashSet<(ArithmeticOperation, int, int)>();
        var exercises = new List<Exercise>(length);
        for (var index = 0; index < length; index++)
        {
            var calculation = PickUnused(space.Calculations, used, random);
            used.Add(calculation.DuplicateKey);
            var hidden = PickHidden(skill.Formats, random);
            var exercise = new Exercise(index, calculation, hidden, null);
            exercises.Add(answerMode == AnswerMode.Choice
                ? exercise with { Choices = BuildChoices(exercise, random) }
                : exercise);
        }

        return exercises;
    }

    /// <summary>
    /// Four proposals: the right answer and three credible mistakes (±1, carry error ±10, inverted operation or copied number),
    /// never negative, never twice the same, in a random order.
    /// </summary>
    public static IReadOnlyList<int> BuildChoices(Exercise exercise, Random random)
    {
        ArgumentNullException.ThrowIfNull(exercise);
        ArgumentNullException.ThrowIfNull(random);

        var expected = exercise.Expected;
        var groups = new List<int[]>
        {
            Shuffle([expected + 1, expected - 1], random),
            Shuffle([expected + 10, expected - 10], random),
            new[] { InvertedOperation(exercise) },
        };

        var distractors = new List<int>();
        void TryAdd(int candidate)
        {
            if ((distractors.Count < ChoiceCount - 1) && (candidate >= 0) && candidate != expected && !distractors.Contains(candidate))
            {
                distractors.Add(candidate);
            }
        }

        // One from each kind of mistake first, then the remaining candidates, then numbers close to the answer.
        foreach (var group in groups)
        {
            foreach (var candidate in group)
            {
                var before = distractors.Count;
                TryAdd(candidate);
                if (distractors.Count > before)
                {
                    break;
                }
            }
        }

        foreach (var candidate in groups.SelectMany(g => g))
        {
            TryAdd(candidate);
        }

        for (var delta = 2; (distractors.Count < ChoiceCount - 1); delta++)
        {
            TryAdd(expected + delta);
            TryAdd(expected - delta);
        }

        return Shuffle([expected, .. distractors], random);
    }

    private static Calculation PickUnused(IReadOnlyList<Calculation> calculations, HashSet<(ArithmeticOperation, int, int)> used, Random random)
    {
        // Random draws first; the scan afterwards guarantees termination when few calculations are left.
        for (var attempt = 0; attempt < 64; attempt++)
        {
            var candidate = calculations[random.Next(calculations.Count)];
            if (!used.Contains(candidate.DuplicateKey))
            {
                return candidate;
            }
        }

        var start = random.Next(calculations.Count);
        for (var offset = 0; offset < calculations.Count; offset++)
        {
            var candidate = calculations[(start + offset) % calculations.Count];
            if (!used.Contains(candidate.DuplicateKey))
            {
                return candidate;
            }
        }

        throw new InvalidOperationException("No unused calculation left.");
    }

    private static HiddenPart PickHidden(IReadOnlyList<ExerciseFormat> formats, Random random)
    {
        var result = formats.Contains(ExerciseFormat.Result);
        var missing = formats.Contains(ExerciseFormat.MissingOperand);
        var asksResult = result && (!missing || random.NextDouble() < ResultFormatShare);
        if (asksResult)
        {
            return HiddenPart.Result;
        }

        return random.Next(2) == 0 ? HiddenPart.Left : HiddenPart.Right;
    }

    /// <summary>
    /// A classic mistake: the other operation when the result is asked (27 + 15 → 12); when a number is
    /// missing, copying the result (68 + ? = 70 → 70) or, for <c>? − b = r</c>, subtracting instead of adding.
    /// </summary>
    private static int InvertedOperation(Exercise exercise)
    {
        var (left, right, result) = (exercise.Calculation.Left, exercise.Calculation.Right, exercise.Calculation.Result);
        var addition = exercise.Calculation.Operation == ArithmeticOperation.Addition;
        return exercise.Hidden switch
        {
            HiddenPart.Result => addition ? Math.Abs(left - right) : left + right,
            HiddenPart.Left when !addition => Math.Abs(result - right),
            _ => result,
        };
    }

    private static int[] Shuffle(int[] values, Random random)
    {
        random.Shuffle(values);
        return values;
    }
}
