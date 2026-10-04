use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
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
    let path = resolve_resource_path(path.parent()?, path.file_name()?.to_str()?)?;
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

pub fn write_atomic_file(path: &Path, content: &[u8]) -> std::io::Result<()> {
    let parent = path.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing parent directory")
    })?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    match fs::metadata(path) {
        Ok(metadata) => temporary
            .as_file()
            .set_permissions(metadata.permissions())?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    temporary.write_all(content)?;
    temporary.as_file().sync_all()?;
    temporary.persist(path).map_err(|error| error.error)?;
    Ok(())
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
    _directory: tempfile::TempDir,
}

impl TempSqliteCopy {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

pub fn copy_sqlite_database_to_temp(path: &Path) -> Option<TempSqliteCopy> {
    let source_root = path.parent()?;
    ensure_contained_tree(source_root, path).ok()?;
    let mut builder = tempfile::Builder::new();
    builder.prefix("ct-cache-");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        builder.permissions(fs::Permissions::from_mode(0o700));
    }
    let directory = builder.tempdir().ok()?;
    let temp_base_name = "cache.tmp";
    let main_target = directory.path().join(temp_base_name);
    fs::copy(path, &main_target).ok()?;

    for suffix in ["-wal", "-shm"] {
        let source = PathBuf::from(format!("{}{}", path.display(), suffix));
        if source.is_file() {
            ensure_contained_tree(source_root, &source).ok()?;
            let target = directory.path().join(format!("{temp_base_name}{suffix}"));
            fs::copy(source, target).ok()?;
        }
    }

    Some(TempSqliteCopy {
        path: main_target,
        _directory: directory,
    })
}

pub fn validate_path_component(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value == "."
        || value == ".."
        || value.ends_with(['.', ' '])
        || value
            .chars()
            .any(|c| c.is_control() || "<>:\"/\\|?*".contains(c))
    {
        return Err("Invalid path component.".to_string());
    }
    Ok(())
}

pub fn resolve_profile_path(root: &Path, profile_id: &str) -> Result<PathBuf, String> {
    validate_path_component(profile_id)?;
    let root = root.canonicalize().map_err(|e| e.to_string())?;
    let profile = root
        .join(profile_id)
        .canonicalize()
        .map_err(|e| e.to_string())?;
    if profile.parent() != Some(root.as_path()) || !profile.is_dir() {
        return Err(
            "Profile directory must be a direct child of the browser data directory.".into(),
        );
    }
    Ok(profile)
}

/// Resolve an existing relative resource without following links outside its root.
pub fn resolve_resource_path(root: &Path, relative: &str) -> Option<PathBuf> {
    let normalized = relative.replace('\\', "/");
    for component in normalized.split('/') {
        validate_path_component(component).ok()?;
    }
    let root = root.canonicalize().ok()?;
    let path = root.join(normalized).canonicalize().ok()?;
    path.starts_with(&root).then_some(path)
}

/// Check existing descendants before modifying browser data, including junctions.
pub fn ensure_contained_tree(root: &Path, path: &Path) -> Result<(), String> {
    let root = root.canonicalize().map_err(|e| e.to_string())?;
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.to_string()),
    };
    let resolved = path.canonicalize().map_err(|e| e.to_string())?;
    if !resolved.starts_with(&root) || metadata.file_type().is_symlink() {
        return Err("Refusing to modify a linked path or a path outside the profile.".into());
    }
    if metadata.is_dir() {
        for entry in path.read_dir().map_err(|e| e.to_string())? {
            ensure_contained_tree(&root, &entry.map_err(|e| e.to_string())?.path())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomic_write_replaces_document_without_leaving_temporary_files() {
        let fixture = tempfile::tempdir().unwrap();
        let path = fixture.path().join("Preferences");
        fs::write(&path, b"original").unwrap();
        write_atomic_file(&path, br#"{"extensions":{}}"#).unwrap();
        assert_eq!(fs::read(&path).unwrap(), br#"{"extensions":{}}"#);
        assert_eq!(fixture.path().read_dir().unwrap().count(), 1);
    }

    #[test]
    fn rejects_path_traversal_and_windows_aliases() {
        for value in [
            "",
            ".",
            "..",
            "../Default",
            "..\\Default",
            "/tmp",
            "C:\\data",
            "Default:stream",
            "Default.",
            "Default ",
            "bad\0name",
        ] {
            assert!(
                validate_path_component(value).is_err(),
                "accepted {value:?}"
            );
        }
        for value in ["Default", "Profile 1", "ç”¨æˆ·èµ„æ–™", "abcdefghijklmnop"] {
            assert!(validate_path_component(value).is_ok());
        }
    }

    #[test]
    fn resources_and_profiles_stay_inside_root() {
        let fixture = tempfile::tempdir().unwrap();
        let root = fixture.path().join("data");
        fs::create_dir_all(root.join("Default/icons")).unwrap();
        fs::write(root.join("Default/icons/icon.png"), b"image").unwrap();
        fs::write(fixture.path().join("outside.png"), b"sentinel").unwrap();
        let profile = resolve_profile_path(&root, "Default").unwrap();
        assert!(resolve_profile_path(&root, "../data").is_err());
        assert!(resolve_resource_path(&profile, "icons/icon.png").is_some());
        assert!(resolve_resource_path(&profile, "../../outside.png").is_none());
        assert!(resolve_resource_path(&profile, "/outside.png").is_none());
        assert!(ensure_contained_tree(&profile, &fixture.path().join("outside.png")).is_err());
        assert_eq!(
            fs::read(fixture.path().join("outside.png")).unwrap(),
            b"sentinel"
        );
    }

    #[test]
    fn rejects_linked_profile_and_mutation_targets() {
        let fixture = tempfile::tempdir().unwrap();
        let root = fixture.path().join("data");
        let outside = fixture.path().join("outside");
        fs::create_dir(&root).unwrap();
        fs::create_dir(&outside).unwrap();
        fs::write(outside.join("History"), b"sentinel").unwrap();
        let link = root.join("Default");
        #[cfg(unix)]
        std::os::unix::fs::symlink(&outside, &link).unwrap();
        #[cfg(windows)]
        {
            let output = std::process::Command::new("cmd")
                .args(["/C", "mklink", "/J"])
                .arg(&link)
                .arg(&outside)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "junction creation failed: {output:?}"
            );
        }
        assert!(resolve_profile_path(&root, "Default").is_err());
        assert!(ensure_contained_tree(&root, &link).is_err());
        assert!(resolve_resource_path(&root, "Default/History").is_none());
        assert_eq!(fs::read(outside.join("History")).unwrap(), b"sentinel");
    }

    #[test]
    fn sqlite_copy_is_readable_and_removed_on_drop() {
        let fixture = tempfile::tempdir().unwrap();
        let source = fixture.path().join("Login Data");
        let connection = rusqlite::Connection::open(&source).unwrap();
        connection.execute_batch("CREATE TABLE logins (origin_url TEXT, signon_realm TEXT, blacklisted_by_user INTEGER); INSERT INTO logins VALUES ('https://example.test', 'https://example.test', 0);").unwrap();
        drop(connection);
        let copy = copy_sqlite_database_to_temp(&source).unwrap();
        let copied_path = copy.path().to_owned();
        let connection = rusqlite::Connection::open_with_flags(
            copy.path(),
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
        )
        .unwrap();
        let site: String = connection
            .query_row(
                "SELECT origin_url FROM logins WHERE blacklisted_by_user = 0",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(site, "https://example.test");
        drop(connection);
        drop(copy);
        assert!(!copied_path.exists());
        assert!(source.exists());
    }
}
