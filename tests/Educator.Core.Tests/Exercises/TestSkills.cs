using Educator.Core.Skills;

namespace Educator.Core.Tests.Exercises;

/// <summary>Skills written in the tests, independent of the content folder.</summary>
internal static class TestSkills
{
    public static Skill Make(
        SkillOperation operation = SkillOperation.Addition,
        OperandRule? left = null,
        OperandRule? right = null,
        SkillConstraints? constraints = null,
        ExerciseFormat[]? formats = null,
        string id = "test-skill") => new(
            id,
            Subject.Math,
            Grade.CP,
            1,
            "Test",
            "Calcule.",
            operation,
            [left ?? new OperandRule(0, 10), right ?? new OperandRule(0, 10)],
            constraints ?? SkillConstraints.None,
            formats ?? [ExerciseFormat.Result, ExerciseFormat.MissingOperand],
            [AnswerMode.Keyboard, AnswerMode.Choice],
            "Tests");
}
