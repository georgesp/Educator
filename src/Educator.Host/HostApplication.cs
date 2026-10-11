using System.Text.Json.Serialization;
using Educator.Content;
using Educator.Core.Exercises;
using Educator.Data;
using Educator.Host.Api;
using Serilog;

namespace Educator.Host;

/// <summary>Composition root of the host: one container for the services and the local API.</summary>
internal static class HostApplication
{
    public static WebApplication Build(HostSettings settings, Action<WebApplicationBuilder>? configure = null)
    {
        ArgumentNullException.ThrowIfNull(settings);

        // Args are parsed by HostCommandLine; none is forwarded to the ASP.NET Core configuration.
        var builder = WebApplication.CreateBuilder(new WebApplicationOptions { Args = [] });
        builder.WebHost.UseUrls($"http://127.0.0.1:{settings.Port}");
        builder.Logging.ClearProviders();
        builder.Logging.AddSerilog(dispose: false);

        var services = builder.Services;
        services.AddSingleton(TimeProvider.System);
        services.AddData(settings.DatabasePath);
        services.AddContent();
        services.AddSingleton(sp => new AdditionGenerator(Random.Shared, sp.GetRequiredService<TimeProvider>()));
        services.AddSingleton<PendingExercises>();

        services.AddSingleton(settings.ApiOptions);
        services.AddProblemDetails();
        services.AddCors(cors => cors.AddPolicy(UiEndpoints.CorsPolicyName, policy => policy
            .WithOrigins([.. settings.ApiOptions.AllowedOrigins])
            .WithHeaders(LocalTokenEndpointFilter.HeaderName, "Content-Type", "Accept")
            .WithMethods("GET", "POST", "PUT", "PATCH", "DELETE")));
        services.AddOpenApi(options => options.AddDocumentTransformer((document, _, _) =>
        {
            document.Info.Title = "Educator local API";
            document.Info.Version = "v1";
            // The address changes at every run (free port): keep the document stable.
            document.Servers?.Clear();
            return Task.CompletedTask;
        }));
        services.ConfigureHttpJsonOptions(json =>
        {
            json.SerializerOptions.Converters.Add(new JsonStringEnumConverter());
            // Numbers as JSON numbers only: the OpenAPI schema (and the generated TS types)
            // then say `number` instead of `number | string`.
            json.SerializerOptions.NumberHandling = JsonNumberHandling.Strict;
        });

        if (settings.RunBackgroundServices)
        {
            services.AddHostedService<SpeechWarmUpService>();
            if (settings.ParentProcessId is int parentProcessId)
            {
                services.AddHostedService(sp => new ParentProcessWatcher(
                    parentProcessId,
                    sp.GetRequiredService<IHostApplicationLifetime>(),
                    sp.GetRequiredService<ILogger<ParentProcessWatcher>>()));
            }
        }

        configure?.Invoke(builder);

        var app = builder.Build();
        app.UseExceptionHandler();
        app.UseCors();
        app.MapOpenApi();
        app.MapUiEndpoints();
        return app;
    }
}

/// <summary>What <see cref="HostApplication.Build"/> needs; the tests override the paths and the token.</summary>
internal sealed record HostSettings
{
    public int Port { get; init; } = HostCommandLine.DefaultPort;

    public int? ParentProcessId { get; init; }

    public required string DatabasePath { get; init; }

    public LocalApiOptions ApiOptions { get; init; } = new();

    /// <summary>Speech warm-up and parent watcher; off for the OpenAPI export and the tests.</summary>
    public bool RunBackgroundServices { get; init; } = true;
}
