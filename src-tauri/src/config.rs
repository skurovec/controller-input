use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(test)]
#[path = "config_tests.rs"]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub buttons: HashMap<String, String>,
    pub axes: HashMap<String, AxisMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub deadzone: f32,
    pub mouse_sensitivity: f32,
    pub stick_to_key_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AxisMapping {
    #[serde(rename = "keys")]
    Keys { negative: String, positive: String },
    #[serde(rename = "mouse_x")]
    MouseX { sensitivity: f32 },
    #[serde(rename = "mouse_y")]
    MouseY { sensitivity: f32 },
}

impl Default for Config {
    fn default() -> Self {
        let mut buttons = HashMap::new();
        buttons.insert("south".to_string(), "5".to_string()); // A/Cross - Hotbar 5
        buttons.insert("east".to_string(), "Space".to_string()); // B/Circle - Jump
        buttons.insert("west".to_string(), "E".to_string()); // X/Square - Inventory
        buttons.insert("north".to_string(), "F".to_string()); // Y/Triangle - Swap hands
        buttons.insert("left_trigger".to_string(), "LeftClick".to_string());
        buttons.insert("right_trigger".to_string(), "RightClick".to_string());
        buttons.insert("left_bumper".to_string(), "ScrollDown".to_string()); // LB - next hotbar slot
        buttons.insert("right_bumper".to_string(), "ScrollUp".to_string()); // RB - prev hotbar slot
        buttons.insert("start".to_string(), "Escape".to_string());
        buttons.insert("select".to_string(), "Tab".to_string());
        buttons.insert("left_thumb".to_string(), "Control".to_string()); // L3 - Sprint
        buttons.insert("right_thumb".to_string(), "Shift".to_string()); // R3 - Sneak
        buttons.insert("dpad_up".to_string(), "1".to_string());
        buttons.insert("dpad_down".to_string(), "Q".to_string()); // D-pad down - Drop
        buttons.insert("dpad_left".to_string(), "2".to_string());
        buttons.insert("dpad_right".to_string(), "3".to_string());

        let mut axes = HashMap::new();
        axes.insert(
            "left_stick_x".to_string(),
            AxisMapping::Keys {
                negative: "A".to_string(),
                positive: "D".to_string(),
            },
        );
        axes.insert(
            "left_stick_y".to_string(),
            AxisMapping::Keys {
                negative: "S".to_string(),
                positive: "W".to_string(),
            },
        );
        axes.insert(
            "right_stick_x".to_string(),
            AxisMapping::MouseX { sensitivity: 2.5 },
        );
        axes.insert(
            "right_stick_y".to_string(),
            AxisMapping::MouseY { sensitivity: 2.5 },
        );

        Config {
            settings: Settings {
                deadzone: 0.15,
                mouse_sensitivity: 2.5,
                stick_to_key_threshold: 0.5,
            },
            buttons,
            axes,
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        let config_dir = dirs_config_dir().unwrap_or_else(|| PathBuf::from("."));
        config_dir.join("controller-input").join("config.toml")
    }

    pub fn load() -> Self {
        let config_dir = dirs_config_dir().unwrap_or_else(|| PathBuf::from("."));
        Self::load_from_directory(&config_dir)
    }

    fn load_from_directory(config_dir: &Path) -> Self {
        // Read pre-rename settings only until a new settings file is saved.
        for directory in ["controller-input", "minecraft-input"] {
            let path = config_dir.join(directory).join("config.toml");
            match fs::read_to_string(&path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => return config,
                    Err(e) => log::warn!("Failed to parse config {}: {}", path.display(), e),
                },
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
                Err(e) => log::warn!("Failed to read config {}: {}", path.display(), e),
            }
            break;
        }
        Config::default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let content = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(&path, content).map_err(|e| e.to_string())?;
        Ok(())
    }
}

fn dirs_config_dir() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        std::env::var("HOME")
            .ok()
            .map(|h| PathBuf::from(h).join("Library").join("Application Support"))
    }
    #[cfg(target_os = "windows")]
    {
        std::env::var("APPDATA").ok().map(PathBuf::from)
    }
    #[cfg(target_os = "linux")]
    {
        std::env::var("XDG_CONFIG_HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var("HOME")
                    .ok()
                    .map(|h| PathBuf::from(h).join(".config"))
            })
    }
}
