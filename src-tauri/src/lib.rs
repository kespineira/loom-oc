mod config;
mod paths;

use std::path::PathBuf;

use serde_json::Value;
use tauri::Emitter;
use tauri_plugin_updater::UpdaterExt;
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
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            config_paths,
            read_config,
            write_config,
            app_version
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let updater = match handle.updater() {
                    Ok(u) => u,
                    Err(e) => {
                        log::debug!("Updater init failed: {e}");
                        return;
                    }
                };

                let update = match updater.check().await {
                    Ok(Some(u)) => u,
                    Ok(None) => return,
                    Err(e) => {
                        log::debug!("Updater check failed: {e}");
                        return;
                    }
                };

                if let Err(e) = update
                    .download_and_install(|_, _| {}, || {})
                    .await
                {
                    log::debug!("Updater install failed: {e}");
                } else {
                    handle.emit("loom:update-available", true).ok();
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
