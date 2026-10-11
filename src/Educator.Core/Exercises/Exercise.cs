using System.Globalization;

namespace Educator.Core.Exercises;

/// <summary>Which number of the equality the child must find.</summary>
public enum HiddenPart
{
    Left,
    Right,
    Result,
}

/// <summary>
/// One exercise of a series. The expected answer (<see cref="Expected"/>) stays in the host: the front
/// receives the equality with the hidden number removed, and the choices in QCM mode.
/// </summary>
/// <param name="Index">Position in the series, from 0.</param>
/// <param name="Choices">Four proposals including the right one (QCM mode), otherwise null.</param>
public sealed record Exercise(int Index, Calculation Calculation, HiddenPart Hidden, IReadOnlyList<int>? Choices)
{
    public const string Placeholder = "?";

    public Skills.ExerciseFormat Format => Hidden == HiddenPart.Result
        ? Skills.ExerciseFormat.Result
        : Skills.ExerciseFormat.MissingOperand;

    public int Expected => Hidden switch
    {
        HiddenPart.Left => Calculation.Left,
        HiddenPart.Right => Calculation.Right,
        _ => Calculation.Result,
    };

    /// <summary>The equality as shown, e.g. <c>7 + ? = 12</c>.</summary>
    public string Prompt => string.Create(
        CultureInfo.InvariantCulture,
        $"{Show(HiddenPart.Left, Calculation.Left)} {Calculation.Symbol} {Show(HiddenPart.Right, Calculation.Right)} = {Show(HiddenPart.Result, Calculation.Result)}");

    /// <summary>French question read aloud (numbers are converted to words by the speech layer).</summary>
    public string SpokenPrompt
    {
        get
        {
            var (left, right, result) = (Calculation.Left, Calculation.Right, Calculation.Result);
            var addition = Calculation.Operation == ArithmeticOperation.Addition;
            return Hidden switch
            {
                HiddenPart.Result when addition => Say($"Combien font {left} plus {right} ?"),
                HiddenPart.Result => Say($"Combien font {left} moins {right} ?"),
                HiddenPart.Right when addition => Say($"Combien faut-il ajouter à {left} pour obtenir {result} ?"),
                HiddenPart.Right => Say($"Combien faut-il enlever à {left} pour obtenir {result} ?"),
                _ when addition => Say($"Quel nombre plus {right} égale {result} ?"),
                _ => Say($"Quel nombre moins {right} égale {result} ?"),
            };
        }
    }

    /// <summary>Correction shown and read after a mistake, e.g. <c>7 + 5 = 12</c>.</summary>
    public string Explanation => Calculation.ToString();

    public bool IsCorrect(int answer) => answer == Expected;

    private string Show(HiddenPart part, int value)
        => part == Hidden ? Placeholder : value.ToString(CultureInfo.InvariantCulture);

    private static string Say(FormattableString text) => text.ToString(CultureInfo.InvariantCulture);
}
