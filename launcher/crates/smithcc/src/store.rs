use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::endpoint::Environment;
use crate::error::{Error, Result};

pub trait TokenStore: Send {
    fn load(&self) -> Result<Option<String>>;
    fn save(&self, token: &str) -> Result<()>;
    fn clear(&self) -> Result<()>;
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenFile {
    refresh_token: String,
}

pub struct FileStore {
    path: PathBuf,
}

impl FileStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl TokenStore for FileStore {
    fn load(&self) -> Result<Option<String>> {
        if !self.path.exists() {
            return Ok(None);
        }
        let mut file = File::open(&self.path)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;
        let parsed: TokenFile = serde_json::from_str(&text).map_err(|_| Error::msg("the saved sign-in file is unreadable"))?;
        if parsed.refresh_token.is_empty() { Ok(None) } else { Ok(Some(parsed.refresh_token)) }
    }

    fn save(&self, token: &str) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let tmp = temp_path(&self.path);
        {
            let mut file = File::create(&tmp)?;
            let body = serde_json::to_vec(&TokenFile { refresh_token: token.to_string() }).map_err(|e| Error::msg(e.to_string()))?;
            file.write_all(&body)?;
            file.sync_all()?;
        }
        replace_file(&tmp, &self.path)?;
        set_private(&self.path)?;
        Ok(())
    }

    fn clear(&self) -> Result<()> {
        if self.path.exists() {
            std::fs::remove_file(&self.path)?;
        }
        Ok(())
    }
}

fn temp_path(path: &Path) -> PathBuf {
    let mut name = path.as_os_str().to_os_string();
    name.push(".tmp");
    PathBuf::from(name)
}

fn replace_file(from: &Path, to: &Path) -> Result<()> {
    match std::fs::rename(from, to) {
        Ok(()) => Ok(()),
        Err(_) => {
            std::fs::copy(from, to)?;
            let _ = std::fs::remove_file(from);
            Ok(())
        }
    }
}

fn set_private(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(path)?.permissions();
        perms.set_mode(0o600);
        std::fs::set_permissions(path, perms)?;
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
    Ok(())
}

#[cfg(windows)]
pub struct KeyringStore {
    environment: Environment,
}

#[cfg(windows)]
impl KeyringStore {
    pub fn new(environment: Environment) -> Self {
        Self { environment }
    }
}

#[cfg(windows)]
impl KeyringStore {
    fn entry(&self) -> Result<keyring::Entry> {
        let account = format!("refresh-token-{}", self.environment.as_str());
        keyring::Entry::new("SmithCreativeCloud", &account).map_err(|e| Error::msg(format!("Windows Credential Manager is unavailable ({e})")))
    }
}

#[cfg(windows)]
impl TokenStore for KeyringStore {
    fn load(&self) -> Result<Option<String>> {
        match self.entry()?.get_password() {
            Ok(token) if !token.is_empty() => Ok(Some(token)),
            Ok(_) => Ok(None),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(Error::msg(format!("could not read the refresh token ({e})"))),
        }
    }

    fn save(&self, token: &str) -> Result<()> {
        self.entry()?.set_password(token).map_err(|e| Error::TokenSave(e.to_string()))
    }

    fn clear(&self) -> Result<()> {
        match self.entry()?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(Error::msg(format!("could not remove the refresh token ({e})"))),
        }
    }
}

pub fn open_store(paths: &crate::paths::Paths, environment: Environment) -> Box<dyn TokenStore> {
    #[cfg(windows)]
    {
        if environment == Environment::Mock { Box::new(FileStore::new(paths.token_file(environment))) } else { Box::new(KeyringStore::new(environment)) }
    }
    #[cfg(not(windows))]
    {
        let _ = environment;
        Box::new(FileStore::new(paths.token_file(environment)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_store_roundtrip_replaces_the_token() {
        let dir = tempfile::tempdir().unwrap();
        let store = FileStore::new(dir.path().join("token.json"));
        assert!(store.load().unwrap().is_none());
        store.save("one").unwrap();
        assert_eq!(store.load().unwrap().as_deref(), Some("one"));
        store.save("two").unwrap();
        assert_eq!(store.load().unwrap().as_deref(), Some("two"));
        store.clear().unwrap();
        assert!(store.load().unwrap().is_none());
    }
}
