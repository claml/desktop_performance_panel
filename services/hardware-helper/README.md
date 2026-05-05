# hardware-helper

Standalone .NET 8 console process that collects hardware sensor data via
LibreHardwareMonitorLib and outputs HelperMessage JSON Lines to stdout.

## Build

```powershell
dotnet restore
dotnet build -c Release
```

## Run

```powershell
dotnet run -- --interval-ms 1000
```

## Output format

Every line written to stdout is a complete JSON object (JSON Lines).
All diagnostic messages go to stderr.

### init

```json
{"type":"init","version":"1.0.0","timestamp":1700000000000}
```

### snapshot

```json
{"type":"snapshot","version":"1.0.0","timestamp":1700000001000,"data":{}}
```

### error

```json
{"type":"error","version":"1.0.0","timestamp":1700000002000,"message":"...","recoverable":true}
```

### status

```json
{"type":"status","version":"1.0.0","timestamp":1700000003000,"message":"helper shutting down"}
```

## Protocol

See `packages/contracts/helper_message.schema.json` for the full specification.
See `packages/contracts/hardware_snapshot.schema.json` for the `data` object schema.

## Dependencies

- .NET 8 SDK
- LibreHardwareMonitorLib (NuGet)
- System.Diagnostics.PerformanceCounter (NuGet)
- Windows only

## Exit

Press `Ctrl+C` to gracefully shut down.
