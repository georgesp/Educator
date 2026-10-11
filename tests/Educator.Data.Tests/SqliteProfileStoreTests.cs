using Educator.Core;
using Educator.Core.Profiles;
using Educator.Data.Profiles;
using Microsoft.Data.Sqlite;

namespace Educator.Data.Tests;

[Trait("Category", "Integration")]
public sealed class SqliteProfileStoreTests : IAsyncLifetime, IDisposable
{
    private static readonly DateTimeOffset Start = new(2026, 10, 11, 8, 0, 0, TimeSpan.Zero);

    private readonly ManualTimeProvider clock = new(Start);
    private readonly TemporaryDatabase database;
    private readonly SqliteProfileStore store;

    public SqliteProfileStoreTests()
    {
        database = new TemporaryDatabase(clock);
        store = new SqliteProfileStore(database.ContextFactory);
    }

    public Task InitializeAsync() => database.InitializeAsync();

    public Task DisposeAsync() => Task.CompletedTask;

    public void Dispose() => database.Dispose();

    [Fact]
    public async Task Add_StoresEveryFieldWithTimestamps()
    {
        var added = await store.AddAsync(NewProfile("Léa", Grade.CE1, lastUsed: Start.AddMinutes(-5)));

        var read = await store.GetAsync(added.Id);

        Assert.NotNull(read);
        Assert.Equal("Léa", read.Name);
        Assert.Equal(Grade.CE1, read.Grade);
        Assert.Equal("lion", read.CharacterId);
        Assert.Equal(Start, read.CreatedAt);
        Assert.Equal(Start, read.UpdatedAt);
        Assert.Equal(Start.AddMinutes(-5), read.LastUsedAt);
        Assert.Equal("CE1", await ScalarAsync($"SELECT Grade FROM Profile WHERE Id = '{added.Id.ToString().ToUpperInvariant()}'"));
    }

    [Fact]
    public async Task List_MostRecentlyUsedFirst()
    {
        await store.AddAsync(NewProfile("Tom", lastUsed: Start.AddDays(-2)));
        await store.AddAsync(NewProfile("Léa", lastUsed: Start));
        await store.AddAsync(NewProfile("Inès", lastUsed: Start.AddDays(-1)));

        var names = (await store.ListAsync()).Select(p => p.Name);

        Assert.Equal(["Léa", "Inès", "Tom"], names);
    }

    [Fact]
    public async Task NameExists_IgnoresCaseAndTheProfileItself()
    {
        var lea = await store.AddAsync(NewProfile("Léa"));

        Assert.True(await store.NameExistsAsync(Profile.NormalizeName("léa"), null));
        Assert.False(await store.NameExistsAsync(Profile.NormalizeName("léa"), lea.Id));
        Assert.False(await store.NameExistsAsync(Profile.NormalizeName("Lea"), null));
    }

    [Fact]
    public async Task DuplicateName_IsRefusedByTheDatabase()
    {
        await store.AddAsync(NewProfile("Léa"));

        var error = await Assert.ThrowsAsync<ProfileValidationException>(() => store.AddAsync(NewProfile("LÉA")));

        Assert.Equal("name", error.Field);
    }

    [Fact]
    public async Task Update_ChangesNameGradeCharacterAndLastUse_NotPoints()
    {
        var lea = await store.AddAsync(NewProfile("Léa"));
        await store.AddPointsAsync(lea.Id, 30);
        clock.Now = Start.AddHours(1);

        var updated = await store.UpdateAsync(lea with { Name = "Léana", Grade = Grade.CE1, CharacterId = "ours", Points = 999, LastUsedAt = clock.Now });

        Assert.NotNull(updated);
        Assert.Equal("Léana", updated.Name);
        Assert.Equal(Grade.CE1, updated.Grade);
        Assert.Equal("ours", updated.CharacterId);
        Assert.Equal(30, updated.Points);
        Assert.Equal(Start.AddHours(1), updated.UpdatedAt);
        Assert.Null(await store.UpdateAsync(lea with { Id = Guid.NewGuid() }));
    }

    [Fact]
    public async Task AddPoints_AddsAtomicallyAndReturnsTheTotal()
    {
        var lea = await store.AddAsync(NewProfile("Léa"));

        await Task.WhenAll(Enumerable.Range(0, 10).Select(_ => store.AddPointsAsync(lea.Id, 10)));
        var total = await store.AddPointsAsync(lea.Id, 5);

        Assert.Equal(105, total);
        Assert.Null(await store.AddPointsAsync(Guid.NewGuid(), 5));
    }

    [Fact]
    public async Task Delete_RemovesTheProfile()
    {
        var lea = await store.AddAsync(NewProfile("Léa"));

        Assert.True(await store.DeleteAsync(lea.Id));
        Assert.False(await store.DeleteAsync(lea.Id));
        Assert.Null(await store.GetAsync(lea.Id));
    }

    private static Profile NewProfile(string name, Grade grade = Grade.CP, DateTimeOffset? lastUsed = null) => new()
    {
        Id = Guid.CreateVersion7(),
        Name = name,
        Grade = grade,
        CharacterId = "lion",
        Points = 0,
        LastUsedAt = lastUsed ?? Start,
    };

    private async Task<object?> ScalarAsync(string sql)
    {
        await using var connection = new SqliteConnection(DataServiceCollectionExtensions.BuildConnectionString(database.DatabasePath));
        await connection.OpenAsync();
        await using var command = connection.CreateCommand();
#pragma warning disable CA2100 // test-only SQL built from a Guid
        command.CommandText = sql;
#pragma warning restore CA2100
        return await command.ExecuteScalarAsync();
    }
}
