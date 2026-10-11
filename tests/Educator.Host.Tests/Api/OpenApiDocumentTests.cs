using System.Text.Json.Nodes;

namespace Educator.Host.Tests.Api;

/// <summary>
/// The front's TypeScript client is generated from <c>web/openapi/openapi.json</c>: that file must
/// match what the API really serves. When this test fails, run <c>npm run api:generate</c> in
/// <c>web/</c> and commit both generated files.
/// </summary>
[Trait("Category", "Integration")]
public sealed class OpenApiDocumentTests
{
    [Fact]
    public async Task CommittedOpenApiFile_MatchesTheApi()
    {
        await using var host = await ApiTestHost.StartAsync();

        var served = OpenApiExporter.Normalize(await host.Client.GetStringAsync("/openapi/v1.json"));
        var committedPath = Path.Combine(RepositoryRoot(), "web", "openapi", "openapi.json");
        var committed = File.Exists(committedPath) ? await File.ReadAllTextAsync(committedPath) : "{}";

        Assert.True(
            JsonNode.DeepEquals(JsonNode.Parse(served), JsonNode.Parse(committed)),
            "web/openapi/openapi.json is out of date: run `npm run api:generate` in web/ and commit the result.");
    }

    private static string RepositoryRoot()
    {
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
        {
            if (File.Exists(Path.Combine(dir.FullName, "Educator.sln")))
            {
                return dir.FullName;
            }
        }

        throw new InvalidOperationException("Educator.sln not found above the test output directory.");
    }
}
