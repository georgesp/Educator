using System.Collections.Concurrent;
using System.Diagnostics.CodeAnalysis;

namespace Educator.Core.Exercises;

/// <summary>
/// In-memory store of the exercises waiting for an answer, so that the host checks answers
/// against what it generated (the front is never trusted with the expected answer).
/// Bounded: the oldest exercises are dropped beyond <see cref="Capacity"/>. Thread-safe.
/// Nothing is persisted before lot 2 (SUB001401).
/// </summary>
public sealed class PendingExercises
{
    public const int Capacity = 200;

    private readonly ConcurrentDictionary<Guid, AdditionExercise> exercises = new();
    private readonly ConcurrentQueue<Guid> order = new();

    public int Count => exercises.Count;

    public void Add(AdditionExercise exercise)
    {
        ArgumentNullException.ThrowIfNull(exercise);

        if (exercises.TryAdd(exercise.Id, exercise))
        {
            order.Enqueue(exercise.Id);
        }

        while (exercises.Count > Capacity && order.TryDequeue(out var oldest))
        {
            exercises.TryRemove(oldest, out _);
        }
    }

    public bool TryGet(Guid id, [MaybeNullWhen(false)] out AdditionExercise exercise)
        => exercises.TryGetValue(id, out exercise);
}
