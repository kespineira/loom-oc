use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("path is not allowed: {0}")]
    PathNotAllowed(PathBuf),
    #[error("io error on {path:?}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("invalid JSON in {path:?}: {source}")]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
}

impl Serialize for ConfigError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Serialize)]
pub struct ReadResult {
    pub path: PathBuf,
    pub exists: bool,
    pub raw: Option<String>,
    pub value: Option<Value>,
}

fn ensure_allowed(path: &Path) -> Result<(), ConfigError> {
    if path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| n == "opencode.json")
        .unwrap_or(false)
    {
        Ok(())
    } else {
        Err(ConfigError::PathNotAllowed(path.to_path_buf()))
    }
}

pub fn read(path: PathBuf) -> Result<ReadResult, ConfigError> {
    ensure_allowed(&path)?;

    if !path.exists() {
        return Ok(ReadResult {
            path,
            exists: false,
            raw: None,
            value: None,
        });
    }

    let raw = fs::read_to_string(&path).map_err(|source| ConfigError::Io {
        path: path.clone(),
        source,
    })?;

    let value = serde_json::from_str(&raw).map_err(|source| ConfigError::Json {
        path: path.clone(),
        source,
    })?;

    Ok(ReadResult {
        path,
        exists: true,
        raw: Some(raw),
        value: Some(value),
    })
}

pub fn write(path: PathBuf, value: &Value) -> Result<(), ConfigError> {
    ensure_allowed(&path)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| ConfigError::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    let mut serialized =
        serde_json::to_string_pretty(value).map_err(|source| ConfigError::Json {
            path: path.clone(),
            source,
        })?;
    serialized.push('\n');

    let tmp = path.with_extension("json.tmp");
    {
        let mut f = fs::File::create(&tmp).map_err(|source| ConfigError::Io {
            path: tmp.clone(),
            source,
        })?;
        f.write_all(serialized.as_bytes())
            .map_err(|source| ConfigError::Io {
                path: tmp.clone(),
                source,
            })?;
        f.sync_all().map_err(|source| ConfigError::Io {
            path: tmp.clone(),
            source,
        })?;
    }

    fs::rename(&tmp, &path).map_err(|source| ConfigError::Io {
        path: path.clone(),
        source,
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::TempDir;

    fn opencode_path(dir: &TempDir) -> PathBuf {
        dir.path().join("opencode.json")
    }

    #[test]
    fn read_returns_exists_false_when_missing() {
        let dir = TempDir::new().unwrap();
        let result = read(opencode_path(&dir)).unwrap();
        assert!(!result.exists);
        assert!(result.raw.is_none());
        assert!(result.value.is_none());
    }

    #[test]
    fn write_then_read_roundtrips() {
        let dir = TempDir::new().unwrap();
        let path = opencode_path(&dir);
        let value = json!({ "model": "anthropic/claude-opus-4-7" });

        write(path.clone(), &value).unwrap();
        let result = read(path).unwrap();

        assert!(result.exists);
        assert_eq!(result.value.unwrap(), value);
    }

    #[test]
    fn write_creates_parent_directories() {
        let dir = TempDir::new().unwrap();
        let nested = dir.path().join("deep").join("nested").join("opencode.json");
        write(nested.clone(), &json!({})).unwrap();
        assert!(nested.exists());
    }

    #[test]
    fn write_is_atomic_via_tmp_file() {
        let dir = TempDir::new().unwrap();
        let path = opencode_path(&dir);
        write(path.clone(), &json!({ "ok": true })).unwrap();

        // tmp companion file should not be left behind
        let tmp = path.with_extension("json.tmp");
        assert!(!tmp.exists());
    }

    #[test]
    fn read_rejects_disallowed_filenames() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("evil.json");
        let err = read(path).unwrap_err();
        assert!(matches!(err, ConfigError::PathNotAllowed(_)));
    }

    #[test]
    fn write_rejects_disallowed_filenames() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("evil.json");
        let err = write(path, &json!({})).unwrap_err();
        assert!(matches!(err, ConfigError::PathNotAllowed(_)));
    }

    #[test]
    fn read_returns_json_error_on_invalid_content() {
        let dir = TempDir::new().unwrap();
        let path = opencode_path(&dir);
        fs::write(&path, "{ not json").unwrap();
        let err = read(path).unwrap_err();
        assert!(matches!(err, ConfigError::Json { .. }));
    }

    #[test]
    fn write_overwrites_existing_file() {
        let dir = TempDir::new().unwrap();
        let path = opencode_path(&dir);
        write(path.clone(), &json!({ "v": 1 })).unwrap();
        write(path.clone(), &json!({ "v": 2 })).unwrap();
        let result = read(path).unwrap();
        assert_eq!(result.value.unwrap(), json!({ "v": 2 }));
    }
}
