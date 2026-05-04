mod config;
mod paths;

use std::path::PathBuf;

use serde_json::Value;

use crate::config::{ConfigError, ReadResult, WriteResult};
use crate::paths::{ConfigPaths, ConfigTarget};

#[tauri::command]
fn config_paths(cwd: Option<PathBuf>) -> ConfigPaths {
    paths::resolve(cwd)
}

#[tauri::command]
fn read_config(target: ConfigTarget, cwd: Option<PathBuf>) -> Result<ReadResult, ConfigError> {
    let path = paths::target_config_path(target, cwd).ok_or(ConfigError::PathResolutionFailed {
        target: match target {
            ConfigTarget::Global => "global",
            ConfigTarget::Project => "project",
        },
    })?;
    config::read(path)
}

#[tauri::command]
fn write_config(
    target: ConfigTarget,
    cwd: Option<PathBuf>,
    value: Value,
    expected_revision: Option<String>,
) -> Result<WriteResult, ConfigError> {
    let path = paths::target_config_path(target, cwd).ok_or(ConfigError::PathResolutionFailed {
        target: match target {
            ConfigTarget::Global => "global",
            ConfigTarget::Project => "project",
        },
    })?;
    config::write(path, &value, expected_revision)
}

#[tauri::command]
fn app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            config_paths,
            read_config,
            write_config,
            app_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
