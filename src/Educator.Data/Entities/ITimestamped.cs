namespace Educator.Data.Entities;

/// <summary>
/// Entity whose <see cref="CreatedAt"/> / <see cref="UpdatedAt"/> are filled by
/// <see cref="Interceptors.TimestampsInterceptor"/>. Never set them by hand.
/// </summary>
public interface ITimestamped
{
    DateTimeOffset CreatedAt { get; set; }

    DateTimeOffset UpdatedAt { get; set; }
}
