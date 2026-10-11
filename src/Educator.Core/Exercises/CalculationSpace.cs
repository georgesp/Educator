using System.Collections.Concurrent;
using Educator.Core.Skills;

namespace Educator.Core.Exercises;

/// <summary>
/// Every calculation a skill allows (operands in their rules, constraints respected), enumerated once
/// and cached. Sampling from this list makes every generated exercise valid by construction, and makes
/// the "rule too narrow" check exact (<see cref="DistinctCount"/>).
/// </summary>
public sealed class CalculationSpace
{
    private static readonly ConcurrentDictionary<Skill, CalculationSpace> Cache = new();

    private CalculationSpace(IReadOnlyList<Calculation> calculations)
    {
        Calculations = calculations;
        DistinctCount = calculations.Select(c => c.DuplicateKey).Distinct().Count();
    }

    /// <summary>Valid calculations, in a stable order.</summary>
    public IReadOnlyList<Calculation> Calculations { get; }

    /// <summary>Number of different exercises (commuted additions counted once).</summary>
    public int DistinctCount { get; }

    public static CalculationSpace For(Skill skill)
    {
        ArgumentNullException.ThrowIfNull(skill);
        return Cache.GetOrAdd(skill, static s => new CalculationSpace(Enumerate(s)));
    }

    /// <summary>Whether <paramref name="calculation"/> respects every rule of <paramref name="skill"/>.</summary>
    public static bool IsValid(Skill skill, Calculation calculation)
    {
        ArgumentNullException.ThrowIfNull(skill);
        return IsAllowedOperation(skill.Operation, calculation.Operation)
            && IsAllowed(skill.Left, calculation.Left)
            && IsAllowed(skill.Right, calculation.Right)
            && RespectsConstraints(skill.Constraints, calculation);
    }

    private static List<Calculation> Enumerate(Skill skill)
    {
        var lefts = skill.Left.AllowedValues();
        var rights = skill.Right.AllowedValues();
        var operations = skill.Operation switch
        {
            SkillOperation.Addition => [ArithmeticOperation.Addition],
            SkillOperation.Subtraction => [ArithmeticOperation.Subtraction],
            _ => new[] { ArithmeticOperation.Addition, ArithmeticOperation.Subtraction },
        };

        var calculations = new List<Calculation>();
        foreach (var operation in operations)
        {
            foreach (var left in lefts)
            {
                foreach (var right in rights)
                {
                    var calculation = new Calculation(operation, left, right);
                    if (RespectsConstraints(skill.Constraints, calculation))
                    {
                        calculations.Add(calculation);
                    }
                }
            }
        }

        return calculations;
    }

    private static bool IsAllowedOperation(SkillOperation allowed, ArithmeticOperation operation) => allowed switch
    {
        SkillOperation.Addition => operation == ArithmeticOperation.Addition,
        SkillOperation.Subtraction => operation == ArithmeticOperation.Subtraction,
        _ => true,
    };

    private static bool IsAllowed(OperandRule rule, int value)
    {
        if (rule.Values is { Count: > 0 })
        {
            return rule.Values.Contains(value);
        }

        return value >= rule.Min && value <= rule.Max && (rule.MultipleOf is not > 0 || value % rule.MultipleOf.Value == 0);
    }

    private static bool RespectsConstraints(SkillConstraints constraints, Calculation calculation)
    {
        if (calculation.Left < 0 || calculation.Right < 0)
        {
            return false;
        }

        var result = calculation.Result;
        if (result < 0
            || result < constraints.MinResult
            || result > constraints.MaxResult
            || (constraints.ExactResult is int exact && result != exact)
            || (constraints.ResultMultipleOf is > 0 && result % constraints.ResultMultipleOf.Value != 0))
        {
            return false;
        }

        var carry = calculation.HasCarry;
        if ((constraints.Carry == CarryRule.Forbidden && carry) || (constraints.Carry == CarryRule.Required && !carry))
        {
            return false;
        }

        foreach (var trivial in constraints.Excluded)
        {
            var isTrivial = trivial switch
            {
                TrivialCase.PlusZero => calculation.Operation == ArithmeticOperation.Addition
                    && (calculation.Left == 0 || calculation.Right == 0),
                TrivialCase.MinusZero => calculation.Operation == ArithmeticOperation.Subtraction && calculation.Right == 0,
                TrivialCase.MinusSelf => calculation.Operation == ArithmeticOperation.Subtraction && calculation.Left == calculation.Right,
                _ => false,
            };
            if (isTrivial)
            {
                return false;
            }
        }

        return true;
    }
}
