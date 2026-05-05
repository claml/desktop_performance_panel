use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ─── Data structures ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub opacity: f64,
    pub always_on_top: bool,
    pub click_through: bool,
    pub position: Position,
    pub polling_interval_ms: u64,
    pub visible_modules: Vec<String>,
    pub show_temperatures: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_on_top: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_through: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polling_interval_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_modules: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_temperatures: Option<bool>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            opacity: 0.85,
            always_on_top: true,
            click_through: false,
            position: Position { x: 0, y: 0 },
            polling_interval_ms: 1000,
            visible_modules: vec![
                "cpu".into(),
                "memory".into(),
                "network".into(),
                "gpu".into(),
            ],
            show_temperatures: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsUpdateArgs {
    pub patch: SettingsPatch,
}

// ─── File path ───────────────────────────────────────────

fn settings_path() -> Result<PathBuf, String> {
    let dir = dirs::config_dir()
        .ok_or_else(|| "Cannot determine config directory".to_string())?
        .join("desktop-performance-panel");
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Cannot create config directory: {}", e))?;
    Ok(dir.join("settings.json"))
}

// ─── Public API ──────────────────────────────────────────

pub fn load_settings() -> Result<Settings, String> {
    let path = settings_path()?;
    if !path.exists() {
        let defaults = Settings::default();
        save_settings(&defaults)?;
        return Ok(defaults);
    }

    let raw = std::fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read settings.json: {}", e))?;
    serde_json::from_str(&raw)
        .map_err(|e| format!("Cannot parse settings.json: {}", e))
}

pub fn save_settings(settings: &Settings) -> Result<(), String> {
    let path = settings_path()?;
    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Cannot serialize settings: {}", e))?;
    std::fs::write(&path, json)
        .map_err(|e| format!("Cannot write settings.json: {}", e))
}

pub fn update_settings(
    current: &Settings,
    patch: &SettingsPatch,
) -> Result<Settings, String> {
    validate_patch(patch)?;

    let mut updated = current.clone();

    if let Some(v) = patch.opacity {
        updated.opacity = v;
    }
    if let Some(v) = patch.always_on_top {
        updated.always_on_top = v;
    }
    if let Some(v) = patch.click_through {
        updated.click_through = v;
    }
    if let Some(ref v) = patch.position {
        updated.position = v.clone();
    }
    if let Some(v) = patch.polling_interval_ms {
        updated.polling_interval_ms = v;
    }
    if let Some(ref v) = patch.visible_modules {
        updated.visible_modules = v.clone();
    }
    if let Some(v) = patch.show_temperatures {
        updated.show_temperatures = v;
    }

    Ok(updated)
}

// ─── Validation ──────────────────────────────────────────

const VALID_MODULES: &[&str] = &["cpu", "memory", "network", "gpu", "disk", "battery"];

fn validate_patch(patch: &SettingsPatch) -> Result<(), String> {
    if let Some(o) = patch.opacity {
        if o < 0.1 || o > 1.0 {
            return Err("opacity must be between 0.1 and 1.0".into());
        }
    }
    if let Some(ref p) = patch.position {
        if p.x < 0 || p.y < 0 {
            return Err("position.x and position.y must be >= 0".into());
        }
    }
    if let Some(ms) = patch.polling_interval_ms {
        if ms < 500 || ms > 10000 {
            return Err("pollingIntervalMs must be between 500 and 10000".into());
        }
    }
    if let Some(ref modules) = patch.visible_modules {
        for m in modules {
            if !VALID_MODULES.contains(&m.as_str()) {
                return Err(format!(
                    "Invalid visible module '{}'. Allowed: {}",
                    m,
                    VALID_MODULES.join(", ")
                ));
            }
        }
    }
    Ok(())
}
