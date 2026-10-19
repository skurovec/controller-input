use crate::config::Config;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::State;

pub struct AppState {
    pub config: Mutex<Config>,
    pub is_running: AtomicBool,
    pub controller_connected: AtomicBool,
    pub controller_name: Mutex<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            config: Mutex::new(Config::load()),
            is_running: AtomicBool::new(false),
            controller_connected: AtomicBool::new(false),
            controller_name: Mutex::new(None),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[tauri::command]
pub fn get_status(state: State<Arc<AppState>>) -> StatusResponse {
    StatusResponse {
        is_running: state.is_running.load(Ordering::SeqCst),
        controller_connected: state.controller_connected.load(Ordering::SeqCst),
        controller_name: state.controller_name.lock().clone(),
    }
}

#[tauri::command]
pub fn start_mapping(state: State<Arc<AppState>>) -> Result<(), String> {
    if !state.controller_connected.load(Ordering::SeqCst) {
        return Err("No controller connected".to_string());
    }
    state.is_running.store(true, Ordering::SeqCst);
    log::info!("Mapping started");
    Ok(())
}

#[tauri::command]
pub fn stop_mapping(state: State<Arc<AppState>>) -> Result<(), String> {
    state.is_running.store(false, Ordering::SeqCst);
    log::info!("Mapping stopped");
    Ok(())
}

#[tauri::command]
pub fn toggle_mapping(state: State<Arc<AppState>>) -> Result<bool, String> {
    let current = state.is_running.load(Ordering::SeqCst);
    if !current && !state.controller_connected.load(Ordering::SeqCst) {
        return Err("No controller connected".to_string());
    }
    state.is_running.store(!current, Ordering::SeqCst);
    log::info!("Mapping toggled to: {}", !current);
    Ok(!current)
}

#[tauri::command]
pub fn get_config(state: State<Arc<AppState>>) -> Config {
    state.config.lock().clone()
}

#[tauri::command]
pub fn update_config(state: State<Arc<AppState>>, config: Config) -> Result<(), String> {
    let mut current = state.config.lock();
    *current = config.clone();
    current.save()?;
    log::info!("Config updated and saved");
    Ok(())
}

#[tauri::command]
pub fn update_settings(
    state: State<Arc<AppState>>,
    deadzone: Option<f32>,
    mouse_sensitivity: Option<f32>,
    stick_to_key_threshold: Option<f32>,
) -> Result<(), String> {
    let mut config = state.config.lock();
    if let Some(dz) = deadzone {
        config.settings.deadzone = dz.clamp(0.0, 0.9);
    }
    if let Some(sens) = mouse_sensitivity {
        config.settings.mouse_sensitivity = sens.clamp(0.1, 10.0);
    }
    if let Some(thresh) = stick_to_key_threshold {
        config.settings.stick_to_key_threshold = thresh.clamp(0.1, 0.9);
    }
    config.save()?;
    log::info!("Settings updated");
    Ok(())
}

#[tauri::command]
pub fn update_button_mapping(
    state: State<Arc<AppState>>,
    button: String,
    action: Option<String>,
) -> Result<(), String> {
    let mut config = state.config.lock();
    match action.as_deref() {
        None | Some("") | Some("None") => {
            config.buttons.remove(&button);
        }
        Some(value) => {
            config.buttons.insert(button.clone(), value.to_string());
        }
    }
    config.save()?;
    log::info!("Button mapping updated: {} -> {:?}", button, action);
    Ok(())
}

#[tauri::command]
pub fn reset_config(state: State<Arc<AppState>>) -> Result<Config, String> {
    let mut config = state.config.lock();
    *config = Config::default();
    config.save()?;
    log::info!("Config reset to defaults");
    Ok(config.clone())
}

#[derive(serde::Serialize)]
pub struct StatusResponse {
    pub is_running: bool,
    pub controller_connected: bool,
    pub controller_name: Option<String>,
}
