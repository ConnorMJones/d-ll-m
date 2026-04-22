use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

const APP_DIR: &str = "dllm-client";
const STORE_FILE: &str = "identity-store.json";
const DEFAULT_LABEL: &str = "default";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredIdentity {
    pub label: String,
    pub has_token: bool,
    pub last_used: bool,
}

#[derive(Clone, Debug)]
pub struct IdentityStore {
    path: PathBuf,
}

#[derive(Default, Serialize, Deserialize)]
struct IdentityStoreData {
    last_used_label: Option<String>,
    identities: Vec<StoredIdentityEntry>,
}

#[derive(Serialize, Deserialize)]
struct StoredIdentityEntry {
    label: String,
    token: String,
}

impl IdentityStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_path() -> Option<PathBuf> {
        let root = env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")))?;
        Some(root.join(APP_DIR).join(STORE_FILE))
    }

    pub fn default_store() -> Option<Self> {
        Self::default_path().map(Self::new)
    }

    pub fn preferred_label(&self) -> String {
        self.load_data()
            .ok()
            .and_then(|data| data.last_used_label)
            .filter(|label| !label.trim().is_empty())
            .unwrap_or_else(|| DEFAULT_LABEL.to_string())
    }

    pub fn list(&self) -> Result<Vec<StoredIdentity>, String> {
        let data = self.load_data()?;
        Ok(data
            .identities
            .into_iter()
            .map(|identity| StoredIdentity {
                last_used: data.last_used_label.as_deref() == Some(identity.label.as_str()),
                label: identity.label,
                has_token: !identity.token.is_empty(),
            })
            .collect())
    }

    pub fn load_token(&self, label: &str) -> Result<Option<String>, String> {
        let label = normalize_label(label)?;
        let data = self.load_data()?;
        Ok(data
            .identities
            .into_iter()
            .find(|identity| identity.label == label)
            .map(|identity| identity.token))
    }

    pub fn save_token(&self, label: &str, token: &str) -> Result<(), String> {
        let label = normalize_label(label)?;
        let mut data = self.load_data()?;
        if let Some(identity) = data
            .identities
            .iter_mut()
            .find(|identity| identity.label == label)
        {
            identity.token = token.to_string();
        } else {
            data.identities.push(StoredIdentityEntry {
                label: label.clone(),
                token: token.to_string(),
            });
        }
        data.identities.sort_by(|a, b| a.label.cmp(&b.label));
        data.last_used_label = Some(label);
        self.save_data(&data)
    }

    fn load_data(&self) -> Result<IdentityStoreData, String> {
        let bytes = match fs::read(&self.path) {
            Ok(bytes) => bytes,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                return Ok(IdentityStoreData::default());
            }
            Err(err) => return Err(format!("failed to read identity store: {err}")),
        };
        serde_json::from_slice(&bytes)
            .map_err(|err| format!("failed to parse identity store: {err}"))
    }

    fn save_data(&self, data: &IdentityStoreData) -> Result<(), String> {
        ensure_parent_dir(&self.path)?;
        let bytes = serde_json::to_vec_pretty(data)
            .map_err(|err| format!("failed to serialize identity store: {err}"))?;
        fs::write(&self.path, bytes).map_err(|err| format!("failed to write identity store: {err}"))
    }
}

fn normalize_label(label: &str) -> Result<String, String> {
    let label = label.trim();
    if label.is_empty() {
        return Err("identity label cannot be empty".to_string());
    }
    Ok(label.to_string())
}

fn ensure_parent_dir(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create identity store dir: {err}"))?;
    }
    Ok(())
}
