using System.Text.Json.Serialization;

namespace HardwareHelper.Models;

public sealed class HelperMessage
{
    [JsonPropertyName("type")]
    public string Type { get; set; } = string.Empty;

    [JsonPropertyName("version")]
    public string Version { get; set; } = "1.0.0";

    [JsonPropertyName("timestamp")]
    public long Timestamp { get; set; }

    [JsonPropertyName("data")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public HardwareSnapshot? Data { get; set; }

    [JsonPropertyName("message")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public string? Message { get; set; }

    [JsonPropertyName("recoverable")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public bool? Recoverable { get; set; }

    public static HelperMessage CreateInit()
    {
        return new HelperMessage
        {
            Type = "init",
            Timestamp = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds(),
        };
    }

    public static HelperMessage CreateSnapshot(HardwareSnapshot data)
    {
        return new HelperMessage
        {
            Type = "snapshot",
            Timestamp = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds(),
            Data = data,
        };
    }

    public static HelperMessage CreateError(string message, bool recoverable)
    {
        return new HelperMessage
        {
            Type = "error",
            Timestamp = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds(),
            Message = message,
            Recoverable = recoverable,
        };
    }

    public static HelperMessage CreateStatus(string message)
    {
        return new HelperMessage
        {
            Type = "status",
            Timestamp = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds(),
            Message = message,
        };
    }
}
