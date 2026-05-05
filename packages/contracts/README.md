# Contracts

> **contractVersion**: 1.0.0
> **Last Updated**: 2026-05-05

This directory contains the **single source of truth** for all cross-layer, cross-process, and cross-language data structures and communication protocols in the desktop-performance-panel project.

## Purpose

The contracts package defines stable, versioned JSON Schemas that every component MUST conform to:

| Component | Role | Consumes |
|-----------|------|----------|
| React Frontend (TypeScript) | UI rendering | `hardware.ts`, `settings.ts`, `panel_status.ts`, `helper_message.ts` |
| Rust Backend (Tauri) | Shell, window management, IPC | Serde structs derived from schemas |
| C# Hardware Helper (.NET 8) | Sensor data collection | `HardwareSnapshot.cs`, `HelperMessage.cs` |
| Flutter (future) | Alternative UI or data consumer | Dart types generated from JSON Schemas |

## Files

| File | Description |
|------|-------------|
| `header.json` | Global contract version identifier |
| `hardware_snapshot.schema.json` | Single hardware reading (CPU, GPU, memory, network, disk, battery) |
| `helper_message.schema.json` | Envelope protocol for hardware-helper.exe stdout (init/snapshot/error/status) |
| `settings.schema.json` | Persistent user settings structure |
| `panel_status.schema.json` | Full panel runtime status |
| `commands.schema.json` | All invocable commands (logicalName + tauriCommandName mapping) |
| `events.schema.json` | All emitted events (name, direction, payload) |

## Version Management Rules

### contractVersion

- All schemas and protocols in this directory share the same `contractVersion` (currently `1.0.0`).
- The version is recorded in `header.json` and referenced by other schemas.
- Version bump rules follow [SemVer](https://semver.org/):
  - **PATCH** (e.g. 1.0.x): Clarifications, description fixes, non-breaking schema additions (new optional fields at end)
  - **MINOR** (e.g. 1.x.0): New commands, new events, new HelperMessage types, new optional fields
  - **MAJOR** (e.g. x.0.0): Breaking changes (field removal, type change, command/event rename)

### HardwareSnapshot Compatibility

| Operation | Allowed? | Rule |
|-----------|----------|------|
| Add new field | ✅ | Append at bottom of object; default value must be `null` |
| Remove field | ❌ | Must first mark `deprecated` in description; keep for 3 minor versions before removal |
| Change field type | ❌ | Never (e.g. cannot change `null` to `string`) |
| Change field name | ❌ | Create new field, deprecate old one |
| Add new top-level module | ✅ | Append new key after existing ones; existing keys remain |

**JSON object field order is NOT significant.** The schemas do not impose ordering requirements on JSON serialization. Consumers MUST NOT rely on field positions.

### HelperMessage Compatibility

| Operation | Allowed? | Rule |
|-----------|----------|------|
| Add new `type` value | ✅ | Consumers ignore unknown types for forward compatibility |
| Remove existing `type` value | ❌ | Never |
| Add fields to existing type | ✅ | New fields must be optional |
| Change required fields | ❌ | Never |

### Command Compatibility

| Operation | Allowed? | Rule |
|-----------|----------|------|
| Add new command | ✅ | No impact on existing code |
| Rename command | ❌ | Can add new name as alias; keep old name 3 minor versions |
| Remove command | ❌ | Mark `deprecated: true`; keep 3 minor versions then remove |
| Change parameter schema | ❌ | Breaking change; requires major version bump |
| Change return schema | ❌ | Breaking change except adding optional fields |

### Event Compatibility

| Operation | Allowed? | Rule |
|-----------|----------|------|
| Add new event | ✅ | No impact on existing code |
| Rename event | ❌ | Same as command rule |
| Remove event | ❌ | Same as command rule |
| Change payload schema | ❌ | Breaking change |

## Cross-Language Type Generation

| Language | Location | Generation Method |
|----------|----------|-------------------|
| TypeScript | `packages/shared-types/` | Hand-written, synchronized with schemas |
| Rust | `apps/desktop/src-tauri/src/` | Hand-written `serde` structs, synchronized with schemas |
| C# | `services/hardware-helper/Models/` | Hand-written, synchronized with schemas |
| Dart (future) | `packages/dart-types/` (planned) | Auto-generated from JSON Schemas via `json_serializable` |

## Validation

All schemas in this directory must pass standard JSON Schema validators (Draft 2020-12). To validate:

```bash
# Using ajv-cli (Node.js)
npx ajv validate -s packages/contracts/hardware_snapshot.schema.json -d sample.json

# Or any JSON Schema validator of your choice
```

## Notes

1. **Field order** in JSON objects is intentionally NOT part of the contract. Do not write code that depends on key ordering.
2. **All numeric fields use `null` (not 0, not -1)** for unavailable sensor readings. The number 0 means zero — it does not mean "unavailable".
3. **Strings use `null`** for unavailable strings. Do not use empty string `""` as sentinel.
4. When adding new contract files, update this README's file table below.
