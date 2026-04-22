use dllm_client::IdentityStore;
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn saves_and_loads_named_identity_tokens() {
    let path = temp_store_path("identity-store");
    let store = IdentityStore::new(&path);

    store
        .save_token("wizard", "token-1")
        .expect("failed to save token");

    let token = store.load_token("wizard").expect("failed to load token");
    assert_eq!(token.as_deref(), Some("token-1"));

    fs::remove_file(&path).ok();
}

#[test]
fn tracks_last_used_identity_label() {
    let path = temp_store_path("identity-store-last-used");
    let store = IdentityStore::new(&path);

    store.save_token("alpha", "token-a").expect("save alpha");
    store.save_token("beta", "token-b").expect("save beta");

    let labels = store.list().expect("failed to list identities");
    assert!(labels.iter().any(|entry| entry.label == "alpha"));
    assert!(
        labels
            .iter()
            .any(|entry| entry.label == "beta" && entry.last_used)
    );
    assert_eq!(store.preferred_label(), "beta");

    fs::remove_file(&path).ok();
}

#[test]
fn rejects_empty_identity_labels() {
    let path = temp_store_path("identity-store-empty");
    let store = IdentityStore::new(&path);

    let err = store
        .save_token("   ", "token")
        .expect_err("empty label should fail");
    assert!(err.contains("identity label cannot be empty"));
}

fn temp_store_path(prefix: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}.json",
        prefix,
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos()
    );
    let dir = std::env::temp_dir().join("dllm-client-tests");
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir.join(unique)
}
