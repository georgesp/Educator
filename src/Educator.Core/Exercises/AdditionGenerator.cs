namespace Educator.Core.Exercises;

/// <summary>
/// Generates random additions of two single-digit numbers (0 to 9).
/// Placeholder for the rule-driven engine of lot 1 (SUB000107).
/// </summary>
public sealed class AdditionGenerator
{
    public const int MinOperand = 0;
    public const int MaxOperand = 9;

    private readonly Random random;
    private readonly TimeProvider timeProvider;

    public AdditionGenerator(Random random, TimeProvider timeProvider)
    {
        this.random = random ?? throw new ArgumentNullException(nameof(random));
        this.timeProvider = timeProvider ?? throw new ArgumentNullException(nameof(timeProvider));
    }

    public AdditionExercise Generate() => new(
        Guid.CreateVersion7(timeProvider.GetUtcNow()),
        random.Next(MinOperand, MaxOperand + 1),
        random.Next(MinOperand, MaxOperand + 1));
}
