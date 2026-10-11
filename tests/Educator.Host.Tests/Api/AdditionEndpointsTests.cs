using System.Net;
using System.Net.Http.Json;
using Educator.Host.Api;

namespace Educator.Host.Tests.Api;

[Trait("Category", "Integration")]
public sealed class AdditionEndpointsTests
{
    [Fact]
    public async Task CreateAddition_ReturnsTwoDigitsWithoutTheAnswer()
    {
        await using var host = await ApiTestHost.StartAsync();

        using var response = await host.Client.SendAsync(host.Request(HttpMethod.Post, "/exercises/additions"));
        var json = await response.Content.ReadAsStringAsync();
        var exercise = await response.Content.ReadFromJsonAsync<AdditionExerciseResponse>();

        Assert.Equal(HttpStatusCode.OK, response.StatusCode);
        Assert.NotNull(exercise);
        Assert.InRange(exercise.Left, 0, 9);
        Assert.InRange(exercise.Right, 0, 9);
        Assert.Equal($"{exercise.Left} + {exercise.Right}", exercise.Statement);
        Assert.DoesNotContain("expected", json, StringComparison.OrdinalIgnoreCase);
    }

    [Fact]
    public async Task AnswerAddition_GoodAndBadAnswers()
    {
        await using var host = await ApiTestHost.StartAsync();
        var exercise = await CreateAsync(host);
        var sum = exercise.Left + exercise.Right;

        var good = await AnswerAsync(host, exercise.Id, sum);
        var bad = await AnswerAsync(host, exercise.Id, sum + 1);

        Assert.True(good.IsCorrect);
        Assert.False(bad.IsCorrect);
        Assert.Equal(sum, bad.ExpectedAnswer);
    }

    [Fact]
    public async Task AnswerAddition_UnknownExercise_Returns404()
    {
        await using var host = await ApiTestHost.StartAsync();
        using var request = host.Request(HttpMethod.Post, $"/exercises/additions/{Guid.NewGuid()}/answer");
        request.Content = JsonContent.Create(new { answer = 3 });

        using var response = await host.Client.SendAsync(request);

        Assert.Equal(HttpStatusCode.NotFound, response.StatusCode);
    }

    [Fact]
    public async Task ReadAddition_ReturnsWavOfTheSpokenStatement()
    {
        await using var host = await ApiTestHost.StartAsync();
        var exercise = await CreateAsync(host);

        using var response = await host.Client.SendAsync(host.Request(HttpMethod.Get, $"/exercises/additions/{exercise.Id}/speech?speed=0.8"));

        Assert.Equal(HttpStatusCode.OK, response.StatusCode);
        Assert.Equal("audio/wav", response.Content.Headers.ContentType?.MediaType);
        Assert.Equal(FakeSpeechSynthesizer.Wav, await response.Content.ReadAsByteArrayAsync());
        var (text, speed) = Assert.Single(host.Speech.Requests);
        Assert.Equal($"Combien font {exercise.Left} plus {exercise.Right} ?", text);
        Assert.Equal(0.8, speed);
    }

    [Fact]
    public async Task ReadAddition_VoiceUnavailable_Returns503()
    {
        await using var host = await ApiTestHost.StartAsync();
        var exercise = await CreateAsync(host);
        host.Speech.IsAvailable = false;

        using var response = await host.Client.SendAsync(host.Request(HttpMethod.Get, $"/exercises/additions/{exercise.Id}/speech"));

        Assert.Equal(HttpStatusCode.ServiceUnavailable, response.StatusCode);
    }

    private static async Task<AdditionExerciseResponse> CreateAsync(ApiTestHost host)
    {
        using var response = await host.Client.SendAsync(host.Request(HttpMethod.Post, "/exercises/additions"));
        response.EnsureSuccessStatusCode();
        return (await response.Content.ReadFromJsonAsync<AdditionExerciseResponse>())!;
    }

    private static async Task<AnswerResponse> AnswerAsync(ApiTestHost host, Guid id, int answer)
    {
        using var request = host.Request(HttpMethod.Post, $"/exercises/additions/{id}/answer");
        request.Content = JsonContent.Create(new { answer });
        using var response = await host.Client.SendAsync(request);
        response.EnsureSuccessStatusCode();
        return (await response.Content.ReadFromJsonAsync<AnswerResponse>())!;
    }
}
