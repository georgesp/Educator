using System.Net.Http.Json;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace Educator.Host.Tests.Api;

internal static class LearningApiJson
{
    /// <summary>Same enum handling as the host (names as strings).</summary>
    public static readonly JsonSerializerOptions Options = new(JsonSerializerDefaults.Web)
    {
        Converters = { new JsonStringEnumConverter() },
    };

    public static async Task<T> ReadAsync<T>(this HttpResponseMessage response)
        => (await response.Content.ReadFromJsonAsync<T>(Options))!;
}
