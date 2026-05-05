using System.Diagnostics;
using System.Net.NetworkInformation;
using System.Runtime.InteropServices;
using HardwareHelper.Models;

namespace HardwareHelper.Services;

public sealed class SystemMetricsReader
{
    private readonly Dictionary<string, (long BytesReceived, long BytesSent, long Timestamp)> _netPrev
        = new();

    private PerformanceCounter? _cpuCounter;
    private PerformanceCounter? _diskReadCounter;
    private PerformanceCounter? _diskWriteCounter;

    private bool _cpuCounterInitialized;
    private bool _diskCountersInitialized;

    public void PopulateCpuFallback(CpuSnapshot cpu)
    {
        if (cpu.UsagePercent.HasValue) return;

        try
        {
            if (!_cpuCounterInitialized)
            {
                try
                {
                    _cpuCounter = new PerformanceCounter(
                        "Processor", "% Processor Time", "_Total", readOnly: true);
                    _cpuCounterInitialized = true;
                }
                catch
                {
                    _cpuCounterInitialized = true;
                    return;
                }
            }

            if (_cpuCounter != null)
            {
                cpu.UsagePercent = _cpuCounter.NextValue();
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[hardware-helper] CPU fallback read error: {ex.Message}");
        }
    }

    public void PopulateMemory(MemorySnapshot memory)
    {
        try
        {
            var status = new MEMORYSTATUSEX();
            status.dwLength = (uint)Marshal.SizeOf<MEMORYSTATUSEX>();

            if (GlobalMemoryStatusEx(ref status))
            {
                memory.TotalGb = status.ullTotalPhys / (1024f * 1024f * 1024f);
                memory.UsedGb = (status.ullTotalPhys - status.ullAvailPhys)
                                / (1024f * 1024f * 1024f);
                if (status.ullTotalPhys > 0)
                {
                    memory.UsagePercent =
                        (float)(status.ullTotalPhys - status.ullAvailPhys) /
                        status.ullTotalPhys * 100f;
                }
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[hardware-helper] Memory read error: {ex.Message}");
        }
    }

    public void PopulateNetwork(NetworkSnapshot network)
    {
        try
        {
            var interfaces = NetworkInterface.GetAllNetworkInterfaces();
            long totalDown = 0;
            long totalUp = 0;
            var now = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds();

            foreach (var nic in interfaces)
            {
                if (nic.OperationalStatus != OperationalStatus.Up) continue;
                if (nic.NetworkInterfaceType == NetworkInterfaceType.Loopback) continue;
                if (nic.NetworkInterfaceType == NetworkInterfaceType.Tunnel) continue;

                try
                {
                    var stats = nic.GetIPStatistics();
                    var bytesRecv = stats.BytesReceived;
                    var bytesSent = stats.BytesSent;

                    if (_netPrev.TryGetValue(nic.Id, out var prev))
                    {
                        var elapsed = (now - prev.Timestamp) / 1000.0;
                        if (elapsed > 0.1)
                        {
                            totalDown += (long)((bytesRecv - prev.BytesReceived) / elapsed);
                            totalUp += (long)((bytesSent - prev.BytesSent) / elapsed);
                        }
                    }

                    _netPrev[nic.Id] = (bytesRecv, bytesSent, now);
                }
                catch
                {
                }
            }

            if (totalDown > 0 || totalUp > 0)
            {
                network.DownloadBps = totalDown;
                network.UploadBps = totalUp;
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[hardware-helper] Network read error: {ex.Message}");
        }
    }

    public void PopulateDisk(DiskSnapshot disk)
    {
        try
        {
            if (!_diskCountersInitialized)
            {
                try
                {
                    _diskReadCounter = new PerformanceCounter(
                        "PhysicalDisk", "Disk Read Bytes/sec", "_Total", readOnly: true);
                    _diskWriteCounter = new PerformanceCounter(
                        "PhysicalDisk", "Disk Write Bytes/sec", "_Total", readOnly: true);
                    _diskCountersInitialized = true;
                }
                catch
                {
                    _diskCountersInitialized = true;
                    return;
                }
            }

            if (_diskReadCounter != null && _diskWriteCounter != null)
            {
                disk.ReadBps = _diskReadCounter.NextValue();
                disk.WriteBps = _diskWriteCounter.NextValue();
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[hardware-helper] Disk read error: {ex.Message}");
        }
    }

    public void PopulateBattery(BatterySnapshot battery)
    {
        try
        {
            var status = new SYSTEM_POWER_STATUS();
            if (GetSystemPowerStatus(out status))
            {
                if (status.BatteryFlag == 128)
                    return;

                if (status.BatteryLifePercent < 255)
                    battery.Percent = status.BatteryLifePercent;

                battery.Charging = status.ACLineStatus == 1;
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[hardware-helper] Battery read error: {ex.Message}");
        }
    }

    [StructLayout(LayoutKind.Sequential)]
    private struct MEMORYSTATUSEX
    {
        public uint dwLength;
        public uint dwMemoryLoad;
        public ulong ullTotalPhys;
        public ulong ullAvailPhys;
        public ulong ullTotalPageFile;
        public ulong ullAvailPageFile;
        public ulong ullTotalVirtual;
        public ulong ullAvailVirtual;
        public ulong ullAvailExtendedVirtual;
    }

    [DllImport("kernel32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool GlobalMemoryStatusEx(ref MEMORYSTATUSEX lpBuffer);

    [StructLayout(LayoutKind.Sequential)]
    private struct SYSTEM_POWER_STATUS
    {
        public byte ACLineStatus;
        public byte BatteryFlag;
        public byte BatteryLifePercent;
        public byte Reserved1;
        public int BatteryLifeTime;
        public int BatteryFullLifeTime;
    }

    [DllImport("kernel32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool GetSystemPowerStatus(out SYSTEM_POWER_STATUS lpSystemPowerStatus);
}
