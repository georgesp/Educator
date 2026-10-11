using System.Collections.Concurrent;
using Educator.Core.Profiles;
using Educator.Core.Rewards;
using Educator.Core.Skills;

namespace Educator.Core.Exercises;

/// <summary>
/// Series of exercises (SUB000107, SUB000110): generation, answer checking, points at the end.
/// The series in progress lives in memory (short-lived); the detailed record of sessions comes with lot 2.
/// Points are added to the profile once, when the series ends (last answer) or is abandoned.
/// </summary>
public sealed class SeriesService
{
    /// <summary>Series kept in memory at most (the oldest are dropped).</summary>
    public const int Capacity = 50;

    private readonly ISkillCatalog skills;
    private readonly IProfileStore profiles;
    private readonly PointsRules points;
    private readonly IRandomSource random;
    private readonly TimeProvider timeProvider;
    private readonly ConcurrentDictionary<Guid, Series> series = new();
    private readonly ConcurrentQueue<Guid> order = new();

    public SeriesService(ISkillCatalog skills, IProfileStore profiles, PointsRules points, IRandomSource random, TimeProvider timeProvider)
    {
        this.skills = skills ?? throw new ArgumentNullException(nameof(skills));
        this.profiles = profiles ?? throw new ArgumentNullException(nameof(profiles));
        this.points = points ?? throw new ArgumentNullException(nameof(points));
        this.random = random ?? throw new ArgumentNullException(nameof(random));
        this.timeProvider = timeProvider ?? throw new ArgumentNullException(nameof(timeProvider));
    }

    public PointsRules Points => points;

    /// <param name="seed">Fixed seed (tests, reproducing a series); random when null.</param>
    /// <exception cref="ProfileNotFoundException">Unknown profile.</exception>
    /// <exception cref="SeriesRequestException">Unknown skill, or answer mode not allowed by the skill.</exception>
    public async Task<Series> StartAsync(Guid profileId, string skillId, AnswerMode answerMode, int? seed = null, CancellationToken cancellationToken = default)
    {
        _ = await profiles.GetAsync(profileId, cancellationToken).ConfigureAwait(false) ?? throw new ProfileNotFoundException(profileId);
        var skill = skills.Find(skillId) ?? throw new SeriesRequestException($"Unknown skill {skillId}.");
        if (!skill.AnswerModes.Contains(answerMode))
        {
            throw new SeriesRequestException($"Skill {skill.Id} does not allow the answer mode {answerMode}.");
        }

        var actualSeed = seed ?? random.Next(int.MaxValue);
        var created = new Series(
            Guid.CreateVersion7(timeProvider.GetUtcNow()),
            profileId,
            skill,
            answerMode,
            actualSeed,
            SeriesGenerator.Generate(skill, actualSeed, answerMode));
        Remember(created);
        return created;
    }

    /// <exception cref="SeriesNotFoundException">Unknown or expired series.</exception>
    /// <exception cref="SeriesRequestException">Bad index, exercise already answered or series closed.</exception>
    public async Task<AnswerOutcome> AnswerAsync(Guid seriesId, int index, int answer, CancellationToken cancellationToken = default)
    {
        var current = Get(seriesId);
        var (exercise, correct, completed) = current.Record(index, answer);
        SeriesSummary? summary = null;
        if (completed)
        {
            var earned = points.ForCompletedSeries(current.CorrectCount, current.Exercises.Count);
            summary = await AwardAsync(current, earned, cancellationToken).ConfigureAwait(false);
        }

        return new AnswerOutcome(correct, exercise.Expected, exercise.Explanation, summary);
    }

    /// <summary>The child leaves: the correct answers already given are kept, without bonus.</summary>
    public async Task<SeriesSummary> AbandonAsync(Guid seriesId, CancellationToken cancellationToken = default)
    {
        var current = Get(seriesId);
        current.Close();
        return await AwardAsync(current, points.ForAbandonedSeries(current.CorrectCount), cancellationToken).ConfigureAwait(false);
    }

    private async Task<SeriesSummary> AwardAsync(Series current, int earned, CancellationToken cancellationToken)
    {
        var total = await profiles.AddPointsAsync(current.ProfileId, earned, cancellationToken).ConfigureAwait(false)
            ?? throw new ProfileNotFoundException(current.ProfileId);
        series.TryRemove(current.Id, out _);
        return new SeriesSummary(current.CorrectCount, current.Exercises.Count, earned, total);
    }

    private Series Get(Guid seriesId)
        => series.TryGetValue(seriesId, out var found) ? found : throw new SeriesNotFoundException(seriesId);

    private void Remember(Series created)
    {
        series[created.Id] = created;
        order.Enqueue(created.Id);
        while (series.Count > Capacity && order.TryDequeue(out var oldest))
        {
            series.TryRemove(oldest, out _);
        }
    }
}

/// <summary>A series in progress. Thread-safe.</summary>
public sealed class Series
{
    private readonly Lock gate = new();
    private readonly bool?[] answers;
    private bool closed;

    internal Series(Guid id, Guid profileId, Skill skill, AnswerMode answerMode, int seed, IReadOnlyList<Exercise> exercises)
    {
        Id = id;
        ProfileId = profileId;
        Skill = skill;
        AnswerMode = answerMode;
        Seed = seed;
        Exercises = exercises;
        answers = new bool?[exercises.Count];
    }

    public Guid Id { get; }

    public Guid ProfileId { get; }

    public Skill Skill { get; }

    public AnswerMode AnswerMode { get; }

    public int Seed { get; }

    public IReadOnlyList<Exercise> Exercises { get; }

    public int CorrectCount
    {
        get
        {
            lock (gate)
            {
                return answers.Count(a => a == true);
            }
        }
    }

    internal (Exercise Exercise, bool Correct, bool Completed) Record(int index, int answer)
    {
        lock (gate)
        {
            if (closed)
            {
                throw new SeriesRequestException("The series is over.");
            }

            if (index < 0 || index >= Exercises.Count)
            {
                throw new SeriesRequestException($"No exercise {index} in this series.");
            }

            if (answers[index] is not null)
            {
                throw new SeriesRequestException($"Exercise {index} is already answered.");
            }

            var exercise = Exercises[index];
            var correct = exercise.IsCorrect(answer);
            answers[index] = correct;
            closed = answers.All(a => a is not null);
            return (exercise, correct, closed);
        }
    }

    internal void Close()
    {
        lock (gate)
        {
            if (closed)
            {
                throw new SeriesRequestException("The series is over.");
            }

            closed = true;
        }
    }
}

/// <param name="Summary">End of series (points awarded), when this answer was the last one.</param>
public sealed record AnswerOutcome(bool Correct, int Expected, string Explanation, SeriesSummary? Summary);

/// <param name="PointsEarned">Points earned by this series.</param>
/// <param name="ProfilePoints">New total of the profile.</param>
public sealed record SeriesSummary(int CorrectCount, int ExerciseCount, int PointsEarned, int ProfilePoints);

public sealed class SeriesNotFoundException : Exception
{
    public SeriesNotFoundException()
    {
    }

    public SeriesNotFoundException(string message)
        : base(message)
    {
    }

    public SeriesNotFoundException(string message, Exception innerException)
        : base(message, innerException)
    {
    }

    public SeriesNotFoundException(Guid id)
        : base($"No series in progress with id {id}.")
    {
    }
}

/// <summary>A request on a series that cannot be honoured (English, diagnostics).</summary>
public sealed class SeriesRequestException : Exception
{
    public SeriesRequestException()
    {
    }

    public SeriesRequestException(string message)
        : base(message)
    {
    }

    public SeriesRequestException(string message, Exception innerException)
        : base(message, innerException)
    {
    }
}
