using Educator.Content.Skills;
using Educator.Core;
using Educator.Core.Skills;

namespace Educator.Content.Tests.Skills;

[Trait("Category", "Unit")]
public sealed class SkillLoaderTests
{
    private const string Valid = """
        {
          "id": "test-add",
          "subject": "math",
          "grade": "CE1",
          "period": 2,
          "title": "Additions",
          "instruction": "Calcule.",
          "operation": "addition",
          "operands": [ { "min": 0, "max": 10 }, { "min": 0, "max": 10, "multipleOf": 2 } ],
          "constraints": { "maxResult": 20, "carry": "forbidden", "excludeTrivial": ["plusZero"] },
          "formats": [ "result", "missingOperand" ],
          "answerModes": [ "keyboard" ],
          "source": "Tests"
        }
        """;

    [Fact]
    public void ValidFile_GivesTheSkill()
    {
        var (skill, errors) = SkillLoader.Parse(Valid, "test-add");

        Assert.Empty(errors);
        Assert.NotNull(skill);
        Assert.Equal(Grade.CE1, skill.Grade);
        Assert.Equal(2, skill.Period);
        Assert.Equal(new OperandRule(0, 10, 2), skill.Right);
        Assert.Equal(CarryRule.Forbidden, skill.Constraints.Carry);
        Assert.Equal([TrivialCase.PlusZero], skill.Constraints.Excluded);
        Assert.Equal([ExerciseFormat.Result, ExerciseFormat.MissingOperand], skill.Formats);
        Assert.Equal([AnswerMode.Keyboard], skill.AnswerModes);
    }

    [Theory]
    [InlineData("\"operation\": \"addition\"", "\"operation\": \"multiplication\"", "operation 'multiplication' is unknown")]
    [InlineData("\"grade\": \"CE1\"", "\"grade\": \"ce1\"", "grade 'ce1' is unknown")]
    [InlineData("\"period\": 2", "\"period\": 6", "period 6")]
    [InlineData("\"source\": \"Tests\"", "\"source\": \"Tests\", \"level\": 3", "Invalid JSON")]
    [InlineData("\"source\": \"Tests\"", "\"source\": \"\"", "source is required")]
    [InlineData("{ \"min\": 0, \"max\": 10 },", "{ \"min\": 12, \"max\": 10 },", "need 0 <= min <= max")]
    [InlineData("\"maxResult\": 20", "\"maxResult\": 3", "Rule too narrow")]
    [InlineData("\"answerModes\": [ \"keyboard\" ]", "\"answerModes\": []", "answerModes must not be empty")]
    [InlineData("[ { \"min\": 0, \"max\": 10 }, { \"min\": 0, \"max\": 10, \"multipleOf\": 2 } ]", "[ { \"min\": 0, \"max\": 99999 }, { \"min\": 0, \"max\": 99999 } ]", "Rule too wide")]
    public void InvalidFile_IsReported(string from, string to, string expectedError)
    {
        var (skill, errors) = SkillLoader.Parse(Valid.Replace(from, to, StringComparison.Ordinal), "test-add");

        Assert.Null(skill);
        Assert.Contains(errors, e => e.Contains(expectedError, StringComparison.Ordinal));
    }

    [Fact]
    public void IdDifferentFromTheFileName_IsReported()
    {
        var (skill, errors) = SkillLoader.Parse(Valid, "other-name");

        Assert.Null(skill);
        Assert.Contains(errors, e => e.Contains("must match the file name", StringComparison.Ordinal));
    }

    [Fact]
    public void LoadDirectory_SkipsInvalidFilesAndReportsThem()
    {
        var directory = Directory.CreateTempSubdirectory("educator-skills-");
        try
        {
            File.WriteAllText(Path.Combine(directory.FullName, "test-add.json"), Valid);
            File.WriteAllText(Path.Combine(directory.FullName, "broken.json"), "{ not json");

            var result = SkillLoader.LoadDirectory(directory.FullName);

            Assert.Equal("test-add", Assert.Single(result.Value).Id);
            Assert.Equal("broken.json", Assert.Single(result.Problems).File);
        }
        finally
        {
            directory.Delete(recursive: true);
        }
    }
}
