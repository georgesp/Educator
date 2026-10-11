namespace Educator.Core.Skills;

/// <summary>
/// A skill: one notion worked on at a grade and a school period, described by generation rules
/// (<c>content/skills/math/*.json</c>, SUB000106). Exercises are never stored: the engine generates them
/// from these rules (<see cref="Exercises.SeriesGenerator"/>).
/// </summary>
/// <param name="Id">Stable identifier, e.g. <c>cp-add-sum-10</c> (also the file name).</param>
/// <param name="Period">School period, 1 (rentrée → Toussaint) to 5 (printemps → été).</param>
/// <param name="Title">Short title shown to the child, in French.</param>
/// <param name="Instruction">Instruction read and shown before the first exercise, in French.</param>
/// <param name="Operands">Exactly two operand rules: left then right.</param>
/// <param name="Source">Reference of the official programme justifying the skill.</param>
public sealed record Skill(
    string Id,
    Subject Subject,
    Grade Grade,
    int Period,
    string Title,
    string Instruction,
    SkillOperation Operation,
    IReadOnlyList<OperandRule> Operands,
    SkillConstraints Constraints,
    IReadOnlyList<ExerciseFormat> Formats,
    IReadOnlyList<AnswerMode> AnswerModes,
    string Source)
{
    public const int FirstPeriod = 1;
    public const int LastPeriod = 5;

    public OperandRule Left => Operands[0];

    public OperandRule Right => Operands[1];
}

public enum Subject
{
    Math,
}

/// <summary>Operation of a skill. <see cref="Mixed"/>: each exercise is an addition or a subtraction.</summary>
public enum SkillOperation
{
    Addition,
    Subtraction,
    Mixed,
}

/// <summary>
/// Allowed values of one operand: the integers of <c>[Min, Max]</c>, optionally only the multiples of
/// <see cref="MultipleOf"/>, or exactly the listed <see cref="Values"/>.
/// For a subtraction, the left operand is the number subtracted from (minuend).
/// </summary>
public sealed record OperandRule(int Min, int Max, int? MultipleOf = null, IReadOnlyList<int>? Values = null)
{
    /// <summary>Every allowed value, in increasing order.</summary>
    public IReadOnlyList<int> AllowedValues()
    {
        if (Values is { Count: > 0 })
        {
            return [.. Values.Distinct().Order()];
        }

        var step = MultipleOf is > 0 ? MultipleOf.Value : 1;
        var first = Min % step == 0 ? Min : Min + (step - (Min % step));
        var values = new List<int>();
        for (var value = first; value <= Max; value += step)
        {
            values.Add(value);
        }

        return values;
    }
}

/// <summary>Constraints on the result and the kind of calculation.</summary>
/// <param name="MinResult">Smallest result allowed.</param>
/// <param name="MaxResult">Largest result allowed.</param>
/// <param name="ExactResult">Result imposed (e.g. 10 for the complements to 10).</param>
/// <param name="ResultMultipleOf">The result must be a multiple of this number (e.g. 10: complement to the next ten).</param>
/// <param name="Carry">Carry (addition) or borrow (subtraction) in at least one column.</param>
/// <param name="ExcludeTrivial">Trivial calculations never generated.</param>
public sealed record SkillConstraints(
    int? MinResult = null,
    int? MaxResult = null,
    int? ExactResult = null,
    int? ResultMultipleOf = null,
    CarryRule Carry = CarryRule.Any,
    IReadOnlyList<TrivialCase>? ExcludeTrivial = null)
{
    public static SkillConstraints None { get; } = new();

    public IReadOnlyList<TrivialCase> Excluded => ExcludeTrivial ?? [];
}

/// <summary>Carry (addition) or borrow (subtraction): "passage de la dizaine", "retenue".</summary>
public enum CarryRule
{
    Any,
    Forbidden,
    Required,
}

public enum TrivialCase
{
    /// <summary>An addition with 0 (<c>7 + 0</c>).</summary>
    PlusZero,

    /// <summary>A subtraction of 0 (<c>7 − 0</c>).</summary>
    MinusZero,

    /// <summary>A number minus itself (<c>7 − 7</c>).</summary>
    MinusSelf,
}

/// <summary>How the calculation is asked.</summary>
public enum ExerciseFormat
{
    /// <summary>The result is asked: <c>7 + 5 = ?</c>.</summary>
    Result,

    /// <summary>An operand is asked: <c>7 + ? = 12</c>.</summary>
    MissingOperand,
}

/// <summary>How the child answers.</summary>
public enum AnswerMode
{
    /// <summary>Types the number (on-screen keypad or keyboard).</summary>
    Keyboard,

    /// <summary>Picks one of 4 proposals.</summary>
    Choice,
}
