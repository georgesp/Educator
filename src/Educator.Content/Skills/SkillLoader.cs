using System.Text.Json;
using System.Text.Json.Serialization;
using System.Text.RegularExpressions;
using Educator.Core;
using Educator.Core.Exercises;
using Educator.Core.Skills;

namespace Educator.Content.Skills;

/// <summary>
/// Loads and validates the skills (<c>content/skills/&lt;subject&gt;/*.json</c>, SUB000106). The rules mirror
/// <c>content/schemas/skill.schema.json</c> and add what a schema cannot say (bounds in order, enough
/// different exercises). An invalid file is skipped and reported: the tests fail on it, the child never sees it.
/// </summary>
public static partial class SkillLoader
{
    /// <summary>A skill must allow at least this many different calculations (rule too narrow otherwise).</summary>
    public const int MinDistinctExercises = 20;

    public const int MaxOperand = 100_000;

    /// <summary>
    /// Pairs of operands a rule may combine. The engine lists every valid calculation once: a wider rule
    /// would make the start-up slow and the memory heavy (three-digit additions are about 640 000 pairs).
    /// </summary>
    public const long MaxCombinations = 2_000_000;

    internal static readonly JsonSerializerOptions JsonOptions = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
        UnmappedMemberHandling = JsonUnmappedMemberHandling.Disallow,
        ReadCommentHandling = JsonCommentHandling.Disallow,
        AllowTrailingCommas = false,
        RespectRequiredConstructorParameters = true,
        RespectNullableAnnotations = true,
    };

    public static ContentLoadResult<IReadOnlyList<Skill>> LoadDirectory(string skillsDirectory)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(skillsDirectory);

        var skills = new List<Skill>();
        var problems = new List<ContentProblem>();
        if (!Directory.Exists(skillsDirectory))
        {
            problems.Add(new ContentProblem(skillsDirectory, "Skills folder not found."));
            return new ContentLoadResult<IReadOnlyList<Skill>>(skills, problems);
        }

        var files = Directory.EnumerateFiles(skillsDirectory, "*.json", SearchOption.AllDirectories)
            .Order(StringComparer.Ordinal);
        foreach (var file in files)
        {
            var name = Path.GetRelativePath(skillsDirectory, file).Replace('\\', '/');
            var (skill, errors) = Parse(File.ReadAllText(file), Path.GetFileNameWithoutExtension(file));
            if (skill is null)
            {
                problems.AddRange(errors.Select(e => new ContentProblem(name, e)));
            }
            else if (skills.Any(s => s.Id == skill.Id))
            {
                problems.Add(new ContentProblem(name, $"Duplicate skill id {skill.Id}."));
            }
            else
            {
                skills.Add(skill);
            }
        }

        return new ContentLoadResult<IReadOnlyList<Skill>>(skills, problems);
    }

    /// <summary>Parses and validates one skill; <paramref name="expectedId"/> is the file name.</summary>
    public static (Skill? Skill, IReadOnlyList<string> Errors) Parse(string json, string? expectedId = null)
    {
        SkillFile? file;
        try
        {
            file = JsonSerializer.Deserialize<SkillFile>(json, JsonOptions);
        }
        catch (JsonException ex)
        {
            return (null, [$"Invalid JSON: {ex.Message}"]);
        }

        if (file is null)
        {
            return (null, ["Empty file."]);
        }

        var errors = new List<string>();
        void Check(bool ok, string message)
        {
            if (!ok)
            {
                errors.Add(message);
            }
        }

        Check(IdPattern().IsMatch(file.Id), $"id '{file.Id}' must be lowercase words separated by hyphens.");
        Check(expectedId is null || expectedId == file.Id, $"id '{file.Id}' must match the file name '{expectedId}'.");
        var subject = ParseEnum<Subject>(file.Subject, "subject", errors);
        var grade = ParseEnum<Grade>(file.Grade, "grade", errors, camelCase: false);
        Check(file.Period is >= Skill.FirstPeriod and <= Skill.LastPeriod, $"period {file.Period} must be between 1 and 5.");
        Check(!string.IsNullOrWhiteSpace(file.Title) && file.Title.Length <= 60, "title must be 1 to 60 characters.");
        Check(!string.IsNullOrWhiteSpace(file.Instruction) && file.Instruction.Length <= 120, "instruction must be 1 to 120 characters.");
        Check(!string.IsNullOrWhiteSpace(file.Source), "source is required (official programme reference).");
        var operation = ParseEnum<SkillOperation>(file.Operation, "operation", errors);
        var formats = ParseList<ExerciseFormat>(file.Formats, "formats", errors);
        var answerModes = ParseList<AnswerMode>(file.AnswerModes, "answerModes", errors);

        var operands = new List<OperandRule>();
        if (file.Operands.Count != 2)
        {
            errors.Add("operands must hold exactly two rules (left, right).");
        }

        foreach (var (operand, position) in file.Operands.Select((o, i) => (o, i == 0 ? "left" : "right")))
        {
            Check(operand.Min >= 0 && operand.Max <= MaxOperand && operand.Min <= operand.Max, $"{position} operand: need 0 <= min <= max <= {MaxOperand}.");
            Check(operand.MultipleOf is null or > 0, $"{position} operand: multipleOf must be positive.");
            Check(operand.Values is null || (operand.Values.Count > 0 && operand.Values.Distinct().Count() == operand.Values.Count), $"{position} operand: values must be a non-empty list without duplicates.");
            Check(operand.Values is null || operand.Values.All(v => v >= operand.Min && v <= operand.Max), $"{position} operand: values must lie between min and max.");
            operands.Add(new OperandRule(operand.Min, operand.Max, operand.MultipleOf, operand.Values));
        }

        var constraintsFile = file.Constraints;
        var carry = constraintsFile.Carry is null ? CarryRule.Any : ParseEnum<CarryRule>(constraintsFile.Carry, "constraints.carry", errors);
        var excluded = ParseList<TrivialCase>(constraintsFile.ExcludeTrivial ?? [], "constraints.excludeTrivial", errors, allowEmpty: true);
        Check(constraintsFile.MinResult is null || constraintsFile.MaxResult is null || constraintsFile.MinResult <= constraintsFile.MaxResult, "constraints: minResult must not exceed maxResult.");
        Check(constraintsFile.ResultMultipleOf is null or > 0, "constraints: resultMultipleOf must be positive.");
        Check(constraintsFile.MinResult is null or >= 0 && constraintsFile.MaxResult is null or >= 0 && constraintsFile.ExactResult is null or >= 0, "constraints: results are natural numbers.");

        if (errors.Count > 0)
        {
            return (null, errors);
        }

        var skill = new Skill(
            file.Id,
            subject,
            grade,
            file.Period,
            file.Title.Trim(),
            file.Instruction.Trim(),
            operation,
            operands,
            new SkillConstraints(constraintsFile.MinResult, constraintsFile.MaxResult, constraintsFile.ExactResult, constraintsFile.ResultMultipleOf, carry, excluded),
            formats,
            answerModes,
            file.Source.Trim());

        var combinations = (long)skill.Left.AllowedValues().Count * skill.Right.AllowedValues().Count
            * (skill.Operation == SkillOperation.Mixed ? 2 : 1);
        if (combinations > MaxCombinations)
        {
            return (null, [$"Rule too wide: {combinations} pairs of operands, at most {MaxCombinations}."]);
        }

        var distinct = CalculationSpace.For(skill).DistinctCount;
        if (distinct < MinDistinctExercises)
        {
            return (null, [$"Rule too narrow: {distinct} different exercises, at least {MinDistinctExercises} needed."]);
        }

        return (skill, []);
    }

    private static T ParseEnum<T>(string? value, string field, List<string> errors, bool camelCase = true)
        where T : struct, Enum
    {
        foreach (var candidate in Enum.GetValues<T>())
        {
            var name = candidate.ToString();
            var expected = camelCase ? JsonNamingPolicy.CamelCase.ConvertName(name) : name;
            if (string.Equals(value, expected, StringComparison.Ordinal))
            {
                return candidate;
            }
        }

        var allowed = string.Join(", ", Enum.GetNames<T>().Select(n => camelCase ? JsonNamingPolicy.CamelCase.ConvertName(n) : n));
        errors.Add($"{field} '{value}' is unknown (allowed: {allowed}).");
        return default;
    }

    private static IReadOnlyList<T> ParseList<T>(IReadOnlyList<string> values, string field, List<string> errors, bool allowEmpty = false)
        where T : struct, Enum
    {
        if (!allowEmpty && values.Count == 0)
        {
            errors.Add($"{field} must not be empty.");
        }

        if (values.Distinct(StringComparer.Ordinal).Count() != values.Count)
        {
            errors.Add($"{field} must not repeat a value.");
        }

        return [.. values.Select(v => ParseEnum<T>(v, field, errors))];
    }

    [GeneratedRegex("^[a-z0-9]+(-[a-z0-9]+)*$")]
    private static partial Regex IdPattern();

    // File shapes (camelCase JSON). Unknown properties are rejected.
    private sealed record SkillFile(
        string Id,
        string Subject,
        string Grade,
        int Period,
        string Title,
        string Instruction,
        string Operation,
        IReadOnlyList<OperandFile> Operands,
        ConstraintsFile Constraints,
        IReadOnlyList<string> Formats,
        IReadOnlyList<string> AnswerModes,
        string Source)
    {
        [JsonPropertyName("$schema")]
        public string? Schema { get; init; }
    }

    private sealed record OperandFile(int Min, int Max, int? MultipleOf = null, IReadOnlyList<int>? Values = null);

    private sealed record ConstraintsFile(
        int? MinResult = null,
        int? MaxResult = null,
        int? ExactResult = null,
        int? ResultMultipleOf = null,
        string? Carry = null,
        IReadOnlyList<string>? ExcludeTrivial = null);
}
