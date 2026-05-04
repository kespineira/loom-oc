use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use serde::ser::SerializeStruct;
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("path is not allowed: {0}")]
    PathNotAllowed(PathBuf),
    #[error("could not resolve {target} config path")]
    PathResolutionFailed { target: &'static str },
    #[error("config was modified externally: expected {expected_revision:?}, actual {actual_revision:?}")]
    RevisionConflict {
        path: PathBuf,
        expected_revision: Option<String>,
        actual_revision: Option<String>,
        actual_mtime_ms: Option<u128>,
    },
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
        let mut state = serializer.serialize_struct("ConfigError", 6)?;
        state.serialize_field("message", &self.to_string())?;
        match self {
            ConfigError::PathNotAllowed(path) => {
                state.serialize_field("kind", "path_not_allowed")?;
                state.serialize_field("path", path)?;
            }
            ConfigError::PathResolutionFailed { target } => {
                state.serialize_field("kind", "path_resolution_failed")?;
                state.serialize_field("target", target)?;
            }
            ConfigError::RevisionConflict {
                path,
                expected_revision,
                actual_revision,
                actual_mtime_ms,
            } => {
                state.serialize_field("kind", "revision_conflict")?;
                state.serialize_field("path", path)?;
                state.serialize_field("expected_revision", expected_revision)?;
                state.serialize_field("actual_revision", actual_revision)?;
                state.serialize_field("actual_mtime_ms", actual_mtime_ms)?;
            }
            ConfigError::Io { path, source } => {
                state.serialize_field("kind", "io")?;
                state.serialize_field("path", path)?;
                state.serialize_field("source", &source.to_string())?;
            }
            ConfigError::Json { path, source } => {
                state.serialize_field("kind", "json")?;
                state.serialize_field("path", path)?;
                state.serialize_field("source", &source.to_string())?;
            }
        }
        state.end()
    }
}

#[derive(Debug, Serialize)]
pub struct ReadResult {
    pub path: PathBuf,
    pub exists: bool,
    pub raw: Option<String>,
    pub value: Option<Value>,
    pub revision: Option<String>,
    pub mtime_ms: Option<u128>,
}

#[derive(Debug, Serialize)]
pub struct WriteResult {
    pub path: PathBuf,
    pub revision: String,
    pub mtime_ms: Option<u128>,
    pub backup_path: Option<PathBuf>,
}

#[derive(Debug)]
struct RevisionState {
    revision: Option<String>,
    mtime_ms: Option<u128>,
}

fn ensure_allowed(path: &Path) -> Result<(), ConfigError> {
    if path
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name == "opencode.json")
        .unwrap_or(false)
    {
        Ok(())
    } else {
        Err(ConfigError::PathNotAllowed(path.to_path_buf()))
    }
}

fn map_io(path: PathBuf) -> impl FnOnce(std::io::Error) -> ConfigError {
    move |source| ConfigError::Io { path, source }
}

fn metadata_mtime_ms(metadata: &fs::Metadata) -> Option<u128> {
    metadata
        .modified()
        .ok()
        .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_millis())
}

fn revision(raw: &[u8], metadata: &fs::Metadata) -> (String, Option<u128>) {
    let mtime_ms = metadata_mtime_ms(metadata);
    let checksum = raw.iter().fold(0xcbf29ce484222325u64, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    });
    (
        format!(
            "mtime:{}:len:{}:fnv64:{:016x}",
            mtime_ms.unwrap_or_default(),
            metadata.len(),
            checksum
        ),
        mtime_ms,
    )
}

fn current_revision(path: &Path) -> Result<RevisionState, ConfigError> {
    ensure_allowed(path)?;

    if !path.exists() {
        return Ok(RevisionState {
            revision: None,
            mtime_ms: None,
        });
    }

    let raw = fs::read(path).map_err(map_io(path.to_path_buf()))?;
    let metadata = fs::metadata(path).map_err(map_io(path.to_path_buf()))?;
    let (revision, mtime_ms) = revision(&raw, &metadata);

    Ok(RevisionState {
        revision: Some(revision),
        mtime_ms,
    })
}

pub fn read(path: PathBuf) -> Result<ReadResult, ConfigError> {
    ensure_allowed(&path)?;

    if !path.exists() {
        return Ok(ReadResult {
            path,
            exists: false,
            raw: None,
            value: None,
            revision: None,
            mtime_ms: None,
        });
    }

    let raw_bytes = fs::read(&path).map_err(map_io(path.clone()))?;
    let metadata = fs::metadata(&path).map_err(map_io(path.clone()))?;
    let (revision, mtime_ms) = revision(&raw_bytes, &metadata);
    let raw = String::from_utf8(raw_bytes).map_err(|source| ConfigError::Io {
        path: path.clone(),
        source: std::io::Error::new(std::io::ErrorKind::InvalidData, source),
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
        revision: Some(revision),
        mtime_ms,
    })
}

pub fn write(
    path: PathBuf,
    value: &Value,
    expected_revision: Option<String>,
) -> Result<WriteResult, ConfigError> {
    ensure_allowed(&path)?;

    let current = current_revision(&path)?;
    if current.revision != expected_revision {
        return Err(ConfigError::RevisionConflict {
            path,
            expected_revision,
            actual_revision: current.revision,
            actual_mtime_ms: current.mtime_ms,
        });
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(map_io(parent.to_path_buf()))?;
    }

    let mut serialized = serde_json::to_string_pretty(value).map_err(|source| ConfigError::Json {
        path: path.clone(),
        source,
    })?;
    serialized.push('\n');

    let backup_path = if path.exists() {
        let backup = path.with_file_name("opencode.json.bak");
        fs::copy(&path, &backup).map_err(map_io(backup.clone()))?;
        Some(backup)
    } else {
        None
    };

    let tmp = path.with_extension("json.tmp");
    {
        let mut file = fs::File::create(&tmp).map_err(map_io(tmp.clone()))?;
        file.write_all(serialized.as_bytes())
            .map_err(map_io(tmp.clone()))?;
        file.sync_all().map_err(map_io(tmp.clone()))?;
    }

    fs::rename(&tmp, &path).map_err(map_io(path.clone()))?;

    let written = current_revision(&path)?;
    Ok(WriteResult {
        path,
        revision: written.revision.unwrap_or_default(),
        mtime_ms: written.mtime_ms,
        backup_path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::TempDir;

    fn opencode_path(dir: &TempDir) -> PathBuf {
        dir.path().join("opencode.json")
    }

    fn write_with_revision(path: PathBuf, value: &Value) -> WriteResult {
        let expected = current_revision(&path).unwrap().revision;
        write(path, value, expected).unwrap()
    }

    #[test]
    fn read_returns_exists_false_when_missing() {
        let dir = TempDir::new().unwrap();
        let result = read(opencode_path(&dir)).unwrap();
        assert!(!result.exists);
        assert!(result.raw.is_none());
        assert!(result.value.is_none());
        assert!(result.revision.is_none());
        assert!(result.mtime_ms.is_none());
    }

    #[test]
    fn write_then_read_roundtrips() {
        let dir = TempDir::new().unwrap();
        let path = opencode_path(&dir);
        let value = json!({ "model": "anthropic/claude-opus-4-7" });

        write(path.clone(), &value, None).unwrap();
        let result = read(path).unwrap();

        assert!(result.exists);
        assert!(result.revision.is_some());
        assert!(result.mtime_ms.is_some());
        assert_eq!(result.value.unwrap(), value);
    }

    #[test]
    fn write_creates_parent_directories() {
        let dir = TempDir::new().unwrap();
        let nested = dir.path().join("deep").join("nested").join("opencode.json");
        write(nested.clone(), &json!({}), None).unwrap();
        assert!(nested.exists());
    }

    #[test]
    fn write_is_atomic_via_tmp_file() {
        let dir = TempDir::new().unwrap();
        let path = opencode_path(&dir);
        write(path.clone(), &json!({ "ok": true }), None).unwrap();

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
        let err = write(path, &json!({}), None).unwrap_err();
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
    fn write_overwrites_existing_file_with_matching_revision() {
        let dir = TempDir::new().unwrap();
        let path = opencode_path(&dir);
        write(path.clone(), &json!({ "v": 1 }), None).unwrap();
        write_with_revision(path.clone(), &json!({ "v": 2 }));
        let result = read(path).unwrap();
        assert_eq!(result.value.unwrap(), json!({ "v": 2 }));
    }

    #[test]
    fn write_with_stale_revision_returns_conflict() {
        let dir = TempDir::new().unwrap();
        let path = opencode_path(&dir);
        write(path.clone(), &json!({ "v": 1 }), None).unwrap();
        let revision = read(path.clone()).unwrap().revision;
        fs::write(&path, "{\"v\":3}\n").unwrap();

        let err = write(path.clone(), &json!({ "v": 2 }), revision).unwrap_err();
        assert!(matches!(err, ConfigError::RevisionConflict { .. }));
        assert_eq!(read(path).unwrap().value.unwrap(), json!({ "v": 3 }));
    }

    #[test]
    fn write_expected_missing_conflicts_when_file_appeared() {
        let dir = TempDir::new().unwrap();
        let path = opencode_path(&dir);
        fs::write(&path, "{\"v\":1}\n").unwrap();

        let err = write(path, &json!({ "v": 2 }), None).unwrap_err();
        assert!(matches!(err, ConfigError::RevisionConflict { .. }));
    }

    #[test]
    fn write_existing_file_creates_backup() {
        let dir = TempDir::new().unwrap();
        let path = opencode_path(&dir);
        write(path.clone(), &json!({ "v": 1 }), None).unwrap();
        let result = write_with_revision(path.clone(), &json!({ "v": 2 }));

        let backup = result.backup_path.unwrap();
        assert!(backup.exists());
        let backup_value: Value = serde_json::from_str(&fs::read_to_string(backup).unwrap()).unwrap();
        assert_eq!(backup_value, json!({ "v": 1 }));
        assert_eq!(read(path).unwrap().value.unwrap(), json!({ "v": 2 }));
    }

    #[test]
    fn write_missing_file_does_not_create_backup() {
        let dir = TempDir::new().unwrap();
        let path = opencode_path(&dir);
        let result = write(path.clone(), &json!({ "v": 1 }), None).unwrap();

        assert!(result.backup_path.is_none());
        assert!(!path.with_file_name("opencode.json.bak").exists());
    }
}
