using Educator.Core.Exercises;

namespace Educator.Core.Tests.Exercises;

[Trait("Category", "Unit")]
public sealed class PendingExercisesTests
{
    [Fact]
    public void TryGet_ReturnsAnAddedExercise()
    {
        var store = new PendingExercises();
        var exercise = new AdditionExercise(Guid.NewGuid(), 1, 2);

        store.Add(exercise);

        Assert.True(store.TryGet(exercise.Id, out var found));
        Assert.Same(exercise, found);
        Assert.False(store.TryGet(Guid.NewGuid(), out _));
    }

    [Fact]
    public void Add_BeyondCapacity_DropsTheOldest()
    {
        var store = new PendingExercises();
        var first = new AdditionExercise(Guid.NewGuid(), 1, 1);
        store.Add(first);

        for (var i = 0; i < PendingExercises.Capacity; i++)
        {
            store.Add(new AdditionExercise(Guid.NewGuid(), 2, 2));
        }

        Assert.Equal(PendingExercises.Capacity, store.Count);
        Assert.False(store.TryGet(first.Id, out _));
    }
}
