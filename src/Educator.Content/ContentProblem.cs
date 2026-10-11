namespace Educator.Content;

/// <summary>An invalid content file: which file and why (English, for the logs and the tests).</summary>
public sealed record ContentProblem(string File, string Message)
{
    public override string ToString() => $"{File}: {Message}";
}

/// <summary>Result of a content load: what is valid, and the problems of what was skipped.</summary>
public sealed record ContentLoadResult<T>(T Value, IReadOnlyList<ContentProblem> Problems)
{
    public bool IsValid => Problems.Count == 0;
}
