use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
    time::{SystemTime, UNIX_EPOCH},
};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json::Value;

pub fn platform_user_data_root_dir() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        return env::var_os("HOME")
            .map(PathBuf::from)
            .map(|path| path.join("Library").join("Application Support"));
    }

    env::var_os("LOCALAPPDATA").map(PathBuf::from).or_else(|| {
        env::var_os("USERPROFILE")
            .map(PathBuf::from)
            .map(|path| path.join("AppData").join("Local"))
    })
}

pub fn load_image_as_data_url(path: &Path) -> Option<String> {
    let bytes = fs::read(path).ok()?;
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())?;
    let mime_type = match extension.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        _ => return None,
    };

    Some(format!(
        "data:{mime_type};base64,{}",
        STANDARD.encode(bytes)
    ))
}

pub fn read_json_file(path: &Path) -> Option<Value> {
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

pub fn decode_base64_literal(encoded: &str) -> Option<String> {
    let bytes = STANDARD.decode(encoded).ok()?;
    String::from_utf8(bytes).ok()
}

pub fn first_non_empty<'a>(values: impl IntoIterator<Item = Option<&'a str>>) -> Option<&'a str> {
    values
        .into_iter()
        .flatten()
        .find(|value| !value.trim().is_empty())
}

pub struct TempSqliteCopy {
    path: PathBuf,
    directory: PathBuf,
}

impl TempSqliteCopy {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempSqliteCopy {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.directory);
    }
}

pub fn copy_sqlite_database_to_temp(path: &Path) -> Option<TempSqliteCopy> {
    let unique_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_nanos();
    let directory = env::temp_dir().join(format!("ct-cache-{}-{unique_id:x}", process::id()));
    let temp_base_name = format!("cache_{unique_id:x}.tmp");

    fs::create_dir_all(&directory).ok()?;

    let main_target = directory.join(&temp_base_name);
    fs::copy(path, &main_target).ok()?;

    for suffix in ["-wal", "-shm"] {
        let source = PathBuf::from(format!("{}{}", path.display(), suffix));
        if source.is_file() {
            let target = directory.join(format!("{temp_base_name}{suffix}"));
            let _ = fs::copy(source, target);
        }
    }

    Some(TempSqliteCopy {
        path: main_target,
        directory,
    })
}
