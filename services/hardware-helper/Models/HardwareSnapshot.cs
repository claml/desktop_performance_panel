using System.Text.Json.Serialization;

namespace HardwareHelper.Models;

public sealed class HardwareSnapshot
{
    [JsonPropertyName("timestamp")]
    public long Timestamp { get; set; }

    [JsonPropertyName("cpu")]
    public CpuSnapshot Cpu { get; set; } = new();

    [JsonPropertyName("gpu")]
    public GpuSnapshot Gpu { get; set; } = new();

    [JsonPropertyName("memory")]
    public MemorySnapshot Memory { get; set; } = new();

    [JsonPropertyName("network")]
    public NetworkSnapshot Network { get; set; } = new();

    [JsonPropertyName("disk")]
    public DiskSnapshot Disk { get; set; } = new();

    [JsonPropertyName("battery")]
    public BatterySnapshot Battery { get; set; } = new();

    [JsonPropertyName("error")]
    public string? Error { get; set; }
}

public sealed class CpuSnapshot
{
    [JsonPropertyName("usagePercent")]
    public float? UsagePercent { get; set; }

    [JsonPropertyName("temperatureC")]
    public float? TemperatureC { get; set; }

    [JsonPropertyName("frequencyMhz")]
    public float? FrequencyMhz { get; set; }

    [JsonPropertyName("powerW")]
    public float? PowerW { get; set; }
}

public sealed class GpuSnapshot
{
    [JsonPropertyName("name")]
    public string? Name { get; set; }

    [JsonPropertyName("usagePercent")]
    public float? UsagePercent { get; set; }

    [JsonPropertyName("temperatureC")]
    public float? TemperatureC { get; set; }

    [JsonPropertyName("memoryUsedMb")]
    public float? MemoryUsedMb { get; set; }

    [JsonPropertyName("memoryTotalMb")]
    public float? MemoryTotalMb { get; set; }

    [JsonPropertyName("powerW")]
    public float? PowerW { get; set; }

    [JsonPropertyName("fanRpm")]
    public float? FanRpm { get; set; }
}

public sealed class MemorySnapshot
{
    [JsonPropertyName("usedGb")]
    public float? UsedGb { get; set; }

    [JsonPropertyName("totalGb")]
    public float? TotalGb { get; set; }

    [JsonPropertyName("usagePercent")]
    public float? UsagePercent { get; set; }
}

public sealed class NetworkSnapshot
{
    [JsonPropertyName("downloadBps")]
    public float? DownloadBps { get; set; }

    [JsonPropertyName("uploadBps")]
    public float? UploadBps { get; set; }
}

public sealed class DiskSnapshot
{
    [JsonPropertyName("readBps")]
    public float? ReadBps { get; set; }

    [JsonPropertyName("writeBps")]
    public float? WriteBps { get; set; }
}

public sealed class BatterySnapshot
{
    [JsonPropertyName("percent")]
    public float? Percent { get; set; }

    [JsonPropertyName("charging")]
    public bool? Charging { get; set; }
}
