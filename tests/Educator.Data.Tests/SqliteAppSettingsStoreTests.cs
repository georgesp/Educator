using Educator.Core.Settings;
using Microsoft.EntityFrameworkCore;

namespace Educator.Data.Tests;

[Trait("Category", "Integration")]
public sealed class SqliteAppSettingsStoreTests
{
    [Fact]
    public async Task GetAsync_UnknownKey_ReturnsNull()
    {
        using var database = new TemporaryDatabase();
        await database.InitializeAsync();

        Assert.Null(await database.Get<IAppSettingsStore>().GetAsync("missing"));
    }

    [Fact]
    public async Task SetAsync_ThenGetAsync_ReturnsTheValue()
    {
        using var database = new TemporaryDatabase();
        await database.InitializeAsync();
        var store = database.Get<IAppSettingsStore>();

        await store.SetAsync("speech.speed", "1.0");
        await store.SetAsync("speech.speed", "0.8");

        Assert.Equal("0.8", await store.GetAsync("speech.speed"));
    }

    [Fact]
    public async Task SetAsync_StampsCreatedAtAndUpdatedAt()
    {
        var created = new DateTimeOffset(2026, 10, 10, 20, 0, 0, 123, TimeSpan.Zero);
        var clock = new ManualTimeProvider(created);
        using var database = new TemporaryDatabase(clock);
        await database.InitializeAsync();
        var store = database.Get<IAppSettingsStore>();

        await store.SetAsync("key", "a");
        clock.Now = created.AddMinutes(5);
        await store.SetAsync("key", "a"); // unchanged value: UpdatedAt must not move
        await store.SetAsync("other", "x");
        clock.Now = created.AddMinutes(10);
        await store.SetAsync("key", "b");

        await using var db = await database.ContextFactory.CreateDbContextAsync();
        var row = await db.AppSettings.SingleAsync(s => s.Key == "key");
        Assert.Equal(created, row.CreatedAt);
        Assert.Equal(created.AddMinutes(10), row.UpdatedAt);
    }

    [Fact]
    public async Task RemoveAsync_DeletesTheKey()
    {
        using var database = new TemporaryDatabase();
        await database.InitializeAsync();
        var store = database.Get<IAppSettingsStore>();
        await store.SetAsync("key", "value");

        Assert.True(await store.RemoveAsync("key"));
        Assert.False(await store.RemoveAsync("key"));
        Assert.Null(await store.GetAsync("key"));
    }
}
