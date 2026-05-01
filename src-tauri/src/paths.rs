use std::path::PathBuf;

use serde::Serialize;

const CONFIG_DIR_NAME: &str = "opencode";
const CONFIG_FILE_NAME: &str = "opencode.json";
const PROJECT_CONFIG_FILE_NAME: &str = "opencode.json";

#[derive(Debug, Serialize)]
pub struct ConfigPaths {
    pub global: PathBuf,
    pub global_exists: bool,
    pub project: Option<PathBuf>,
    pub project_exists: bool,
}

pub fn global_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join(CONFIG_DIR_NAME).join(CONFIG_FILE_NAME))
}

pub fn project_config_path(cwd: Option<&PathBuf>) -> Option<PathBuf> {
    let dir = match cwd {
        Some(p) => p.clone(),
        None => std::env::current_dir().ok()?,
    };
    Some(dir.join(PROJECT_CONFIG_FILE_NAME))
}

pub fn resolve(cwd: Option<PathBuf>) -> ConfigPaths {
    let global = global_config_path().unwrap_or_else(|| PathBuf::from(CONFIG_FILE_NAME));
    let global_exists = global.is_file();

    let project = project_config_path(cwd.as_ref());
    let project_exists = project.as_ref().map(|p| p.is_file()).unwrap_or(false);

    ConfigPaths {
        global,
        global_exists,
        project,
        project_exists,
    }
}
