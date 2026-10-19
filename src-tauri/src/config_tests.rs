use super::Config;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

struct ConfigDirectory(PathBuf);

impl ConfigDirectory {
    fn new() -> Self {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let sequence = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "controller-input-config-{}-{unique}-{sequence}",
            std::process::id()
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn write(&self, directory: &str, contents: &str) {
        let path = self.0.join(directory);
        fs::create_dir_all(&path).unwrap();
        fs::write(path.join("config.toml"), contents).unwrap();
    }
}

impl Drop for ConfigDirectory {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn settings(sensitivity: f32) -> String {
    format!(
        "[settings]\ndeadzone = 0.2\nmouse_sensitivity = {sensitivity}\nstick_to_key_threshold = 0.6\n[buttons]\nsouth = 'Space'\n[axes]\n"
    )
}

#[test]
fn config_path_uses_the_new_application_name() {
    assert!(Config::config_path().ends_with("controller-input/config.toml"));
}

#[test]
fn loads_previous_settings_without_changing_the_old_file() {
    let directory = ConfigDirectory::new();
    let contents = settings(4.0);
    directory.write("minecraft-input", &contents);

    let config = Config::load_from_directory(&directory.0);

    assert_eq!(config.settings.mouse_sensitivity, 4.0);
    assert_eq!(config.buttons.get("south").unwrap(), "Space");
    assert_eq!(
        fs::read_to_string(directory.0.join("minecraft-input/config.toml")).unwrap(),
        contents
    );
    assert!(!directory.0.join("controller-input/config.toml").exists());
}

#[test]
fn new_settings_take_precedence_over_previous_settings() {
    let directory = ConfigDirectory::new();
    directory.write("minecraft-input", &settings(4.0));
    directory.write("controller-input", &settings(7.0));

    let config = Config::load_from_directory(&directory.0);

    assert_eq!(config.settings.mouse_sensitivity, 7.0);
}

#[test]
fn loads_new_settings_without_a_previous_installation() {
    let directory = ConfigDirectory::new();
    directory.write("controller-input", &settings(7.0));

    assert_eq!(
        Config::load_from_directory(&directory.0)
            .settings
            .mouse_sensitivity,
        7.0
    );
}

#[test]
fn missing_settings_use_defaults() {
    let directory = ConfigDirectory::new();

    assert_eq!(
        Config::load_from_directory(&directory.0)
            .settings
            .mouse_sensitivity,
        2.5
    );
}

#[test]
fn invalid_new_settings_do_not_restore_outdated_settings() {
    let directory = ConfigDirectory::new();
    directory.write("minecraft-input", &settings(4.0));
    directory.write("controller-input", "invalid configuration");

    assert_eq!(
        Config::load_from_directory(&directory.0)
            .settings
            .mouse_sensitivity,
        2.5
    );
}
