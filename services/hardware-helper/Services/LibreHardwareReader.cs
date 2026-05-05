using LibreHardwareMonitor.Hardware;
using HardwareHelper.Models;

namespace HardwareHelper.Services;

public sealed class LibreHardwareReader : IDisposable
{
    private readonly Computer _computer;
    private bool _initialized;

    public LibreHardwareReader()
    {
        _computer = new Computer
        {
            IsCpuEnabled = true,
            IsGpuEnabled = true,
            IsMemoryEnabled = false,
            IsMotherboardEnabled = false,
            IsControllerEnabled = false,
            IsNetworkEnabled = false,
            IsStorageEnabled = false,
        };
    }

    public void Open()
    {
        try
        {
            _computer.Open();
            _initialized = true;
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[hardware-helper] Failed to open LibreHardwareMonitor: {ex.Message}");
            _initialized = false;
        }
    }

    public void PopulateCpu(CpuSnapshot cpu)
    {
        if (!_initialized) return;

        try
        {
            foreach (var hardware in _computer.Hardware)
            {
                if (hardware.HardwareType != HardwareType.Cpu) continue;
                hardware.Update();

                foreach (var sensor in hardware.Sensors)
                {
                    if (!sensor.Value.HasValue) continue;

                    switch (sensor.SensorType)
                    {
                        case SensorType.Load when sensor.Name.Contains("Total", StringComparison.OrdinalIgnoreCase):
                            cpu.UsagePercent ??= sensor.Value;
                            break;
                        case SensorType.Load:
                            cpu.UsagePercent ??= sensor.Value;
                            break;
                        case SensorType.Temperature when sensor.Name.Contains("Package", StringComparison.OrdinalIgnoreCase):
                            cpu.TemperatureC ??= sensor.Value;
                            break;
                        case SensorType.Temperature when cpu.TemperatureC == null:
                            cpu.TemperatureC = sensor.Value;
                            break;
                        case SensorType.Clock:
                            cpu.FrequencyMhz ??= sensor.Value;
                            break;
                        case SensorType.Power when sensor.Name.Contains("Package", StringComparison.OrdinalIgnoreCase):
                            cpu.PowerW ??= sensor.Value;
                            break;
                        case SensorType.Power when cpu.PowerW == null:
                            cpu.PowerW = sensor.Value;
                            break;
                    }
                }
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[hardware-helper] CPU sensor read error: {ex.Message}");
        }
    }

    public void PopulateGpu(GpuSnapshot gpu)
    {
        if (!_initialized) return;

        try
        {
            foreach (var hardware in _computer.Hardware)
            {
                if (hardware.HardwareType != HardwareType.GpuNvidia &&
                    hardware.HardwareType != HardwareType.GpuAmd &&
                    hardware.HardwareType != HardwareType.GpuIntel)
                    continue;

                hardware.Update();

                gpu.Name ??= hardware.Name;

                foreach (var sensor in hardware.Sensors)
                {
                    if (!sensor.Value.HasValue) continue;

                    switch (sensor.SensorType)
                    {
                        case SensorType.Load when sensor.Name.Contains("Core", StringComparison.OrdinalIgnoreCase):
                            gpu.UsagePercent ??= sensor.Value;
                            break;
                        case SensorType.Load:
                            gpu.UsagePercent ??= sensor.Value;
                            break;
                        case SensorType.Temperature when sensor.Name.Contains("Core", StringComparison.OrdinalIgnoreCase):
                            gpu.TemperatureC ??= sensor.Value;
                            break;
                        case SensorType.Temperature when gpu.TemperatureC == null:
                            gpu.TemperatureC = sensor.Value;
                            break;
                        case SensorType.SmallData when sensor.Name.Contains("GPU Memory Used", StringComparison.OrdinalIgnoreCase):
                            gpu.MemoryUsedMb ??= sensor.Value;
                            break;
                        case SensorType.SmallData when sensor.Name.Contains("Memory Used", StringComparison.OrdinalIgnoreCase):
                            gpu.MemoryUsedMb ??= sensor.Value;
                            break;
                        case SensorType.SmallData when sensor.Name.Contains("GPU Memory Total", StringComparison.OrdinalIgnoreCase):
                            gpu.MemoryTotalMb ??= sensor.Value;
                            break;
                        case SensorType.SmallData when sensor.Name.Contains("Memory Total", StringComparison.OrdinalIgnoreCase):
                            gpu.MemoryTotalMb ??= sensor.Value;
                            break;
                        case SensorType.Power:
                            gpu.PowerW ??= sensor.Value;
                            break;
                        case SensorType.Fan:
                            gpu.FanRpm ??= sensor.Value;
                            break;
                    }
                }
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[hardware-helper] GPU sensor read error: {ex.Message}");
        }
    }

    public void Dispose()
    {
        try
        {
            if (_initialized)
                _computer.Close();
        }
        catch
        {
        }
    }
}
