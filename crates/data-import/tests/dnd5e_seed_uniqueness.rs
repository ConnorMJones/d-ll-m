use dllm_bindings::{
    DbConnection, dnd_5_e_class_feature_table::Dnd5EClassFeatureTableAccess,
    seed_dnd_5_e_class_feature_reducer::seed_dnd_5_e_class_feature,
};
use spacetimedb_sdk::{DbContext, Table};
use std::{
    fs::{OpenOptions, create_dir_all},
    io::Write,
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

#[test]
fn enforces_import_unique_keys_at_database_layer() {
    let log_path = workspace_root().join("temp/dnd5e-seed-uniqueness.log");
    let server = TestServer::start(&log_path);
    server.publish();

    let (conn, handle) = connect(&server.uri, &server.database_name);

    seed_class_feature(
        &conn,
        "Spellcasting",
        "PHB",
        "Cleric",
        "PHB",
        1,
        "Cleric spellcasting",
    );
    wait_for_class_feature_count(&conn, 1);

    seed_class_feature(
        &conn,
        "Spellcasting",
        "PHB",
        "Cleric",
        "PHB",
        1,
        "Cleric spellcasting duplicate",
    );
    wait_for_class_feature_count(&conn, 1);

    seed_class_feature(
        &conn,
        "Spellcasting",
        "PHB",
        "Wizard",
        "PHB",
        1,
        "Wizard spellcasting",
    );
    wait_for_class_feature_count(&conn, 2);

    let rows = conn.db.dnd_5_e_class_feature().iter().collect::<Vec<_>>();
    assert_eq!(rows.len(), 2, "expected exact duplicate to be ignored");
    assert!(rows.iter().any(|row| row.class_name == "Cleric"));
    assert!(rows.iter().any(|row| row.class_name == "Wizard"));

    conn.disconnect().ok();
    handle.join().ok();
}

fn seed_class_feature(
    conn: &DbConnection,
    name: &str,
    source: &str,
    class_name: &str,
    class_source: &str,
    level: u8,
    description: &str,
) {
    let (tx, rx) = mpsc::channel();
    conn.reducers
        .seed_dnd_5_e_class_feature_then(
            name.to_string(),
            source.to_string(),
            class_name.to_string(),
            class_source.to_string(),
            level,
            description.to_string(),
            move |_ctx, result| {
                tx.send(result.map_err(|err| err.to_string()))
                    .expect("failed to report reducer result");
            },
        )
        .expect("failed to send class feature reducer request");

    let result = rx
        .recv_timeout(Duration::from_secs(10))
        .expect("timed out waiting for class feature reducer result");
    match result {
        Ok(Ok(())) => {}
        Ok(Err(message)) => panic!("class feature reducer returned error: {message}"),
        Err(message) => panic!("class feature reducer failed internally: {message}"),
    }
}

fn wait_for_class_feature_count(conn: &DbConnection, expected: usize) {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let count = conn.db.dnd_5_e_class_feature().iter().count();
        if count == expected {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "timed out waiting for class feature count {expected}, current {count}"
        );
        thread::sleep(Duration::from_millis(100));
    }
}

fn connect(uri: &str, database_name: &str) -> (DbConnection, thread::JoinHandle<()>) {
    let (connected_tx, connected_rx) = mpsc::channel();
    let (subscribed_tx, subscribed_rx) = mpsc::channel();

    let conn = DbConnection::builder()
        .with_uri(uri)
        .with_database_name(database_name)
        .on_connect(move |_ctx, _identity, _token| {
            connected_tx
                .send(())
                .expect("failed to report connection success");
        })
        .on_connect_error(move |_ctx, err| {
            panic!("connection failed: {err}");
        })
        .on_disconnect(move |_ctx, err| {
            if let Some(err) = err {
                panic!("unexpected disconnect: {err}");
            }
        })
        .build()
        .expect("failed to build connection");

    conn.subscription_builder()
        .on_applied(move |_ctx| {
            subscribed_tx
                .send(())
                .expect("failed to report subscription success");
        })
        .subscribe(["SELECT * FROM dnd_5_e_class_feature"]);

    let handle = conn.run_threaded();

    connected_rx
        .recv_timeout(Duration::from_secs(10))
        .expect("timed out waiting for connection");
    subscribed_rx
        .recv_timeout(Duration::from_secs(10))
        .expect("timed out waiting for subscription");

    (conn, handle)
}

struct TestServer {
    child: Child,
    uri: String,
    database_name: String,
    _data_dir: PathBuf,
}

impl TestServer {
    fn start(log_path: &Path) -> Self {
        create_dir_all(log_path.parent().expect("log path should have parent"))
            .expect("failed to create temp log dir");

        let port = reserve_port();
        let uri = format!("http://127.0.0.1:{port}");
        let database_name = format!("dllm-seed-test-{}", unique_suffix());
        let data_dir = workspace_root().join(format!("temp/spacetime-test-{}", unique_suffix()));
        create_dir_all(&data_dir).expect("failed to create test data dir");

        append_log(
            log_path,
            &format!("starting spacetime at {uri} for {database_name}\n"),
        );

        let stdout = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .expect("failed to open test log");
        let stderr = stdout.try_clone().expect("failed to clone test log handle");

        let child = Command::new("spacetime")
            .args([
                "start",
                "--listen-addr",
                &format!("127.0.0.1:{port}"),
                "--data-dir",
                data_dir.to_str().expect("non-utf8 data dir"),
                "--in-memory",
                "--non-interactive",
            ])
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .spawn()
            .expect("failed to start spacetime server");

        let deadline = Instant::now() + Duration::from_secs(15);
        loop {
            if TcpStream::connect(("127.0.0.1", port)).is_ok() {
                break;
            }
            assert!(
                Instant::now() < deadline,
                "timed out waiting for spacetime server to bind {uri}"
            );
            thread::sleep(Duration::from_millis(100));
        }

        Self {
            child,
            uri,
            database_name,
            _data_dir: data_dir,
        }
    }

    fn publish(&self) {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = manifest_dir
            .parent()
            .and_then(|path| path.parent())
            .expect("failed to resolve workspace root");
        let log_path = workspace_root.join("temp/dnd5e-seed-uniqueness.log");

        let mut last_error = None;
        for _attempt in 0..20 {
            let output = Command::new("spacetime")
                .current_dir(workspace_root)
                .args([
                    "publish",
                    "-p",
                    "crates/server",
                    "-s",
                    &self.uri,
                    &self.database_name,
                ])
                .output()
                .expect("failed to run spacetime publish");

            append_output(&log_path, &output.stdout, &output.stderr);

            if output.status.success() {
                return;
            }

            last_error = Some(String::from_utf8_lossy(&output.stderr).into_owned());
            thread::sleep(Duration::from_millis(500));
        }

        panic!(
            "failed to publish test database {}: {}",
            self.database_name,
            last_error.unwrap_or_else(|| "unknown error".to_string())
        );
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn reserve_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .expect("failed to reserve test port")
        .local_addr()
        .expect("failed to read reserved port")
        .port()
}

fn unique_suffix() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_millis()
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("failed to resolve workspace root")
        .to_path_buf()
}

fn append_output(log_path: &Path, stdout: &[u8], stderr: &[u8]) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .expect("failed to open log file");
    let _ = file.write_all(stdout);
    let _ = file.write_all(stderr);
}

fn append_log(log_path: &Path, message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .expect("failed to open log file");
    let _ = file.write_all(message.as_bytes());
}
