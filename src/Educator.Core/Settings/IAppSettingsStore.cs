namespace Educator.Core.Settings;

/// <summary>
/// Key / value application settings stored locally (table <c>AppSetting</c>, SUB006004).
/// Keys are case-sensitive; values are plain strings (JSON when structured).
/// </summary>
public interface IAppSettingsStore
{
    Task<string?> GetAsync(string key, CancellationToken cancellationToken = default);

    Task SetAsync(string key, string value, CancellationToken cancellationToken = default);

    /// <returns><c>true</c> when the key existed.</returns>
    Task<bool> RemoveAsync(string key, CancellationToken cancellationToken = default);
}
