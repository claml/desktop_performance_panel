using System.Text.Json;
using HardwareHelper.Models;
using HardwareHelper.Services;

namespace HardwareHelper;

public static class Program
{
    private static readonly JsonSerializerOptions JsonOptions = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
        WriteIndented = false,
        DefaultIgnoreCondition = System.Text.Json.Serialization.JsonIgnoreCondition.Never,
    };

    private static volatile bool _running = true;

    public static int Main(string[] args)
    {
        var intervalMs = ParseIntervalMs(args);

        Console.CancelKeyPress += (_, e) =>
        {
            e.Cancel = true;
            _running = false;
        };

        WriteLine(HelperMessage.CreateInit());

        using var lhmReader = new LibreHardwareReader();
        lhmReader.Open();

        CheckPrivileges();

        var sysReader = new SystemMetricsReader();

        while (_running)
        {
            try
            {
                var snapshot = CollectSnapshot(lhmReader, sysReader);
                WriteLine(HelperMessage.CreateSnapshot(snapshot));
            }
            catch (Exception ex)
            {
                WriteLine(HelperMessage.CreateError(
                    $"Collection error: {ex.Message}", recoverable: true));
                Console.Error.WriteLine($"[hardware-helper] {ex}");
            }

            if (_running)
                Thread.Sleep(intervalMs);
        }

        WriteLine(HelperMessage.CreateStatus("helper shutting down"));
        return 0;
    }

    private static int ParseIntervalMs(string[] args)
    {
        for (var i = 0; i < args.Length - 1; i++)
        {
            if (args[i] == "--interval-ms" && int.TryParse(args[i + 1], out var val))
            {
                val = Math.Clamp(val, 500, 10000);
                Console.Error.WriteLine(
                    $"[hardware-helper] Polling interval set to {val} ms");
                return val;
            }
        }
        return 1000;
    }

    private static void CheckPrivileges()
    {
        try
        {
            using var identity = System.Security.Principal.WindowsIdentity.GetCurrent();
            var principal = new System.Security.Principal.WindowsPrincipal(identity);
            if (!principal.IsInRole(System.Security.Principal.WindowsBuiltInRole.Administrator))
            {
                WriteLine(HelperMessage.CreateError(
                    "Running without administrator privileges. " +
                    "Some sensors (CPU temperature, GPU sensors) may be unavailable.",
                    recoverable: true));
            }
        }
        catch
        {
        }
    }

    private static HardwareSnapshot CollectSnapshot(
        LibreHardwareReader lhmReader,
        SystemMetricsReader sysReader)
    {
        var snapshot = new HardwareSnapshot
        {
            Timestamp = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds(),
        };

        var errors = new List<string>();

        try { lhmReader.PopulateCpu(snapshot.Cpu); }
        catch (Exception ex) { errors.Add($"CPU(LHM): {ex.Message}"); }

        try { sysReader.PopulateCpuFallback(snapshot.Cpu); }
        catch (Exception ex) { errors.Add($"CPU(perfc): {ex.Message}"); }

        try { lhmReader.PopulateGpu(snapshot.Gpu); }
        catch (Exception ex) { errors.Add($"GPU: {ex.Message}"); }

        try { sysReader.PopulateMemory(snapshot.Memory); }
        catch (Exception ex) { errors.Add($"Memory: {ex.Message}"); }

        try { sysReader.PopulateNetwork(snapshot.Network); }
        catch (Exception ex) { errors.Add($"Network: {ex.Message}"); }

        try { sysReader.PopulateDisk(snapshot.Disk); }
        catch (Exception ex) { errors.Add($"Disk: {ex.Message}"); }

        try { sysReader.PopulateBattery(snapshot.Battery); }
        catch (Exception ex) { errors.Add($"Battery: {ex.Message}"); }

        if (errors.Count > 0)
            snapshot.Error = string.Join("; ", errors);

        return snapshot;
    }

    private static void WriteLine(HelperMessage msg)
    {
        var json = JsonSerializer.Serialize(msg, JsonOptions);
        Console.WriteLine(json);
        Console.Out.Flush();
    }
}
