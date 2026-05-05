use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{AppHandle, Emitter};

use crate::ipc_contract::event;

// ─── JSON types matching the contracts ───────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareSnapshot {
    pub timestamp: i64,
    pub cpu: CpuSnapshot,
    pub gpu: GpuSnapshot,
    pub memory: MemorySnapshot,
    pub network: NetworkSnapshot,
    pub disk: DiskSnapshot,
    pub battery: BatterySnapshot,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuSnapshot {
    pub usage_percent: Option<f64>,
    pub temperature_c: Option<f64>,
    pub frequency_mhz: Option<f64>,
    pub power_w: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuSnapshot {
    pub name: Option<String>,
    pub usage_percent: Option<f64>,
    pub temperature_c: Option<f64>,
    pub memory_used_mb: Option<f64>,
    pub memory_total_mb: Option<f64>,
    pub power_w: Option<f64>,
    pub fan_rpm: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemorySnapshot {
    pub used_gb: Option<f64>,
    pub total_gb: Option<f64>,
    pub usage_percent: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSnapshot {
    pub download_bps: Option<f64>,
    pub upload_bps: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskSnapshot {
    pub read_bps: Option<f64>,
    pub write_bps: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatterySnapshot {
    pub percent: Option<f64>,
    pub charging: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HelperMessage {
    #[serde(rename = "init")]
    Init {
        version: String,
        timestamp: i64,
    },
    #[serde(rename = "snapshot")]
    Snapshot {
        version: String,
        timestamp: i64,
        data: HardwareSnapshot,
    },
    #[serde(rename = "error")]
    Error {
        version: String,
        timestamp: i64,
        message: String,
        recoverable: bool,
    },
    #[serde(rename = "status")]
    Status {
        version: String,
        timestamp: i64,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HelperStatusPayload {
    pub status: String,
    pub pid: Option<u32>,
    pub message: Option<String>,
}

// ─── Hardware helper process manager ─────────────────────

pub struct HardwareService {
    latest_snapshot: Arc<Mutex<Option<HardwareSnapshot>>>,
    child: Arc<Mutex<Option<Child>>>,
    running: Arc<Mutex<bool>>,
    reader_thread: Option<thread::JoinHandle<()>>,
    app_handle: AppHandle,
}

impl HardwareService {
    fn resolve_helper_path() -> Result<std::path::PathBuf, String> {
        let mut checked = Vec::new();

        // 1. Release: resources/hardware-helper.exe alongside the running exe
        if let Ok(exe) = std::env::current_exe() {
            if let Some(exe_dir) = exe.parent() {
                let resources = exe_dir.join("resources").join("hardware-helper.exe");
                checked.push(resources.clone());
                if resources.exists() {
                    let canonical = std::fs::canonicalize(&resources).unwrap_or(resources);
                    eprintln!("[hardware-service] Using helper: {}", canonical.display());
                    return Ok(canonical);
                }

                let sibling = exe_dir.join("hardware-helper.exe");
                checked.push(sibling.clone());
                if sibling.exists() {
                    let canonical = std::fs::canonicalize(&sibling).unwrap_or(sibling);
                    eprintln!("[hardware-service] Using helper: {}", canonical.display());
                    return Ok(canonical);
                }
            }
        }

        // 2. Development: walk up from cwd
        let dev_rels = [
            "services/hardware-helper/bin/Release/net8.0/hardware-helper.exe",
            "services/hardware-helper/bin/Debug/net8.0/hardware-helper.exe",
            "services/hardware-helper/bin/Release/net8.0/win-x64/publish/hardware-helper.exe",
        ];

        let mut search_roots = Vec::new();
        if let Ok(cwd) = std::env::current_dir() {
            search_roots.push(cwd.clone());
            let mut parent = cwd;
            for _ in 0..3 {
                if let Some(p) = parent.parent() {
                    let p = p.to_path_buf();
                    search_roots.push(p.clone());
                    parent = p;
                }
            }
        }

        for root in &search_roots {
            for rel in &dev_rels {
                let candidate = root.join(rel);
                checked.push(candidate.clone());
                if candidate.exists() {
                    let canonical = std::fs::canonicalize(&candidate).unwrap_or(candidate);
                    eprintln!("[hardware-service] Using helper: {}", canonical.display());
                    return Ok(canonical);
                }
            }
        }

        let checked_strs: Vec<String> = checked
            .iter()
            .map(|p| p.display().to_string())
            .collect();

        Err(format!(
            "hardware-helper.exe not found. Checked:\n  {}\n\
             Build it first: cd services/hardware-helper && dotnet build -c Release",
            checked_strs.join("\n  ")
        ))
    }

    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            latest_snapshot: Arc::new(Mutex::new(None)),
            child: Arc::new(Mutex::new(None)),
            running: Arc::new(Mutex::new(false)),
            reader_thread: None,
            app_handle,
        }
    }

    pub fn start(&mut self, interval_ms: u64) -> Result<(), String> {
        let helper_path = Self::resolve_helper_path()?;
        let interval = interval_ms.to_string();

        let mut child = Command::new(helper_path)
            .arg("--interval-ms")
            .arg(&interval)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| format!("Failed to start hardware-helper: {}", e))?;

        let pid = child.id();
        let stdout = child.stdout.take().ok_or("Failed to capture helper stdout")?;
        *self.child.lock().unwrap() = Some(child);

        let app_handle = self.app_handle.clone();
        let latest_snapshot = self.latest_snapshot.clone();
        let running = self.running.clone();

        *running.lock().unwrap() = true;

        // Emit running status immediately after successful spawn
        let _ = app_handle.emit(
            event::HARDWARE_STATUS,
            HelperStatusPayload {
                status: "running".to_string(),
                pid: Some(pid),
                message: None,
            },
        );

        let reader = thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if !*running.lock().unwrap() {
                    break;
                }

                match line {
                    Ok(raw) => {
                        let trimmed = raw.trim();
                        if trimmed.is_empty() {
                            continue;
                        }

                        match serde_json::from_str::<HelperMessage>(trimmed) {
                            Ok(msg) => {
                                // Serialize once before matching so borrows
                                // don't conflict with destructured moves
                                let helper_value =
                                    serde_json::to_value(&msg).unwrap_or_default();

                                match msg {
                                    HelperMessage::Init { .. } => {
                                        let _ = app_handle.emit(
                                            event::HARDWARE_STATUS,
                                            HelperStatusPayload {
                                                status: "running".to_string(),
                                                pid: Some(pid),
                                                message: None,
                                            },
                                        );
                                    }
                                    HelperMessage::Snapshot { data, .. } => {
                                        *latest_snapshot.lock().unwrap() =
                                            Some(data.clone());
                                        let _ = app_handle.emit(
                                            event::HARDWARE_SNAPSHOT,
                                            data,
                                        );
                                    }
                                    HelperMessage::Error {
                                        message,
                                        recoverable,
                                        ..
                                    } => {
                                        let _ = app_handle.emit(
                                            event::HELPER_MESSAGE,
                                            helper_value,
                                        );
                                        if !recoverable {
                                            *running.lock().unwrap() = false;
                                            let _ = app_handle.emit(
                                                event::HARDWARE_STATUS,
                                                HelperStatusPayload {
                                                    status: "error".to_string(),
                                                    pid: Some(pid),
                                                    message: Some(message),
                                                },
                                            );
                                        }
                                    }
                                    HelperMessage::Status { .. } => {
                                        let _ = app_handle.emit(
                                            event::HELPER_MESSAGE,
                                            helper_value,
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!(
                                    "[hardware-service] Failed to parse message: {}",
                                    e
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[hardware-service] stdout read error: {}", e);
                        break;
                    }
                }
            }

            *running.lock().unwrap() = false;
            let _ = app_handle.emit(
                event::HARDWARE_STATUS,
                HelperStatusPayload {
                    status: "stopped".to_string(),
                    pid: None,
                    message: Some("helper process exited".to_string()),
                },
            );
        });

        self.reader_thread = Some(reader);
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        *self.running.lock().unwrap() = false;

        if let Ok(mut guard) = self.child.lock() {
            if let Some(ref mut child) = *guard {
                let _ = child.kill();
                let _ = child.wait();
            }
            *guard = None;
        }

        if let Some(handle) = self.reader_thread.take() {
            let _ = handle.join();
        }

        let _ = self.app_handle.emit(
            event::HARDWARE_STATUS,
            HelperStatusPayload {
                status: "stopped".to_string(),
                pid: None,
                message: None,
            },
        );

        Ok(())
    }

    pub fn restart(&mut self, interval_ms: u64) -> Result<(), String> {
        self.stop()?;
        self.start(interval_ms)
    }

    pub fn get_latest_snapshot(&self) -> Option<HardwareSnapshot> {
        self.latest_snapshot.lock().unwrap().clone()
    }
}

impl Drop for HardwareService {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
