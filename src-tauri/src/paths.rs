use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub const CONFIG_DIR_NAME: &str = "opencode";
pub const CONFIG_FILE_NAME: &str = "opencode.json";

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigTarget {
    Global,
    Project,
}

#[derive(Debug, Serialize)]
pub struct ConfigPaths {
    pub global: PathBuf,
    pub global_exists: bool,
    pub project: Option<PathBuf>,
    pub project_exists: bool,
}

pub fn xdg_config_home() -> Option<PathBuf> {
    if let Some(value) = std::env::var_os("XDG_CONFIG_HOME") {
        let path = PathBuf::from(value);
        if path.is_absolute() {
            return Some(path);
        }
    }

    std::env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join(".config"))
}

pub fn global_config_path() -> Option<PathBuf> {
    Some(xdg_config_home()?.join(CONFIG_DIR_NAME).join(CONFIG_FILE_NAME))
}

pub fn project_config_path(cwd: Option<&PathBuf>) -> Option<PathBuf> {
    let dir = match cwd {
        Some(p) => p.clone(),
        None => std::env::current_dir().ok()?,
    };
    Some(dir.join(CONFIG_FILE_NAME))
}

pub fn target_config_path(target: ConfigTarget, cwd: Option<PathBuf>) -> Option<PathBuf> {
    match target {
        ConfigTarget::Global => global_config_path(),
        ConfigTarget::Project => project_config_path(cwd.as_ref()),
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;
    use std::sync::{Mutex, OnceLock};

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn with_env<T>(xdg: Option<&str>, home: Option<&str>, test: impl FnOnce() -> T) -> T {
        let _guard = ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let old_xdg = std::env::var_os("XDG_CONFIG_HOME");
        let old_home = std::env::var_os("HOME");

        set_env("XDG_CONFIG_HOME", xdg);
        set_env("HOME", home);

        let result = test();

        restore_env("XDG_CONFIG_HOME", old_xdg);
        restore_env("HOME", old_home);
        result
    }

    fn set_env(key: &str, value: Option<&str>) {
        match value {
            Some(value) => std::env::set_var(key, value),
            None => std::env::remove_var(key),
        }
    }

    fn restore_env(key: &str, value: Option<OsString>) {
        match value {
            Some(value) => std::env::set_var(key, value),
            None => std::env::remove_var(key),
        }
    }

    #[test]
    fn global_config_path_uses_absolute_xdg_config_home() {
        with_env(Some("/tmp/loom-xdg"), Some("/tmp/loom-home"), || {
            assert_eq!(
                global_config_path().unwrap(),
                PathBuf::from("/tmp/loom-xdg/opencode/opencode.json")
            );
        });
    }

    #[test]
    fn global_config_path_ignores_relative_xdg_config_home() {
        with_env(Some("relative"), Some("/tmp/loom-home"), || {
            assert_eq!(
                global_config_path().unwrap(),
                PathBuf::from("/tmp/loom-home/.config/opencode/opencode.json")
            );
        });
    }

    #[test]
    fn global_config_path_falls_back_to_home_dot_config() {
        with_env(None, Some("/tmp/loom-home"), || {
            assert_eq!(
                global_config_path().unwrap(),
                PathBuf::from("/tmp/loom-home/.config/opencode/opencode.json")
            );
        });
    }

    #[test]
    fn project_config_path_uses_cwd() {
        let cwd = PathBuf::from("/tmp/loom-project");
        assert_eq!(
            project_config_path(Some(&cwd)).unwrap(),
            PathBuf::from("/tmp/loom-project/opencode.json")
        );
    }
}
