namespace Educator.Core;

/// <summary>Source of randomness, injectable so that the tests control the draws.</summary>
public interface IRandomSource
{
    /// <summary>A number in <c>[0, maxExclusive)</c>.</summary>
    int Next(int maxExclusive);
}

/// <summary><see cref="IRandomSource"/> backed by <see cref="Random.Shared"/>.</summary>
public sealed class SystemRandomSource : IRandomSource
{
    public static SystemRandomSource Instance { get; } = new();

    public int Next(int maxExclusive) => Random.Shared.Next(maxExclusive);
}
