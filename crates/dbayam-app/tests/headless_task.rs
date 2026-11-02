//! `dbayam --run-task`: a saved SQL script task on a saved SQLite connection
//! runs headless and reports its status through the exit code. Uses a
//! throw-away configuration directory (no Keychain access: SQLite needs no
//! secrets and only the secrets a connection uses are read).

use dbayam_app::backend::Backend;
use dbayam_config::{AppPaths, ConnectionStore};
use dbayam_tasks::{TaskDef, TaskKind, TaskStore};

#[test]
fn run_task_headless() {
    let root = std::env::temp_dir().join(format!("dbayam-headless-{}", uuid::Uuid::new_v4().simple()));
    let paths = AppPaths::with_root(&root);
    paths.ensure_dirs().unwrap();
    let db = root.join("tasks.db");

    let mut store = ConnectionStore::new();
    let mut cfg = dbayam_core::config::ConnectionConfig::new(dbayam_core::DbKind::Sqlite, "task db");
    cfg.file_path = Some(db.display().to_string());
    cfg.auth = dbayam_core::config::AuthMethod::None;
    let conn_id = store.add(cfg).unwrap();
    store.save(&paths).unwrap();

    let tasks = TaskStore::open(paths.tasks_dir()).unwrap();
    let ok = TaskDef::new(
        "nightly",
        TaskKind::SqlScript {
            connection_id: conn_id.clone(),
            database: None,
            schema: None,
            sql: Some("CREATE TABLE t (id INTEGER PRIMARY KEY, v TEXT);\nINSERT INTO t VALUES (1, 'a'), (2, 'b');".into()),
            script_path: None,
            stop_on_error: true,
            commit_at_end: true,
        },
    );
    tasks.create(ok).unwrap();
    let bad = TaskDef::new(
        "broken",
        TaskKind::SqlScript {
            connection_id: conn_id,
            database: None,
            schema: None,
            sql: Some("SELEC nonsense;".into()),
            script_path: None,
            stop_on_error: true,
            commit_at_end: true,
        },
    );
    tasks.create(bad).unwrap();
    drop(tasks);

    let backend = Backend::new().unwrap();
    let settings = dbayam_config::AppSettings::default();
    let code = dbayam_app::headless::run_task(&backend, &paths, &settings, "nightly");
    assert_eq!(format!("{code:?}"), format!("{:?}", std::process::ExitCode::from(0)), "successful task exits with 0");
    let code = dbayam_app::headless::run_task(&backend, &paths, &settings, "broken");
    assert_eq!(format!("{code:?}"), format!("{:?}", std::process::ExitCode::from(1)), "failed task exits with 1");
    let code = dbayam_app::headless::run_task(&backend, &paths, &settings, "missing");
    assert_eq!(format!("{code:?}"), format!("{:?}", std::process::ExitCode::from(4)), "unknown task exits with 4");

    // The task really ran: two rows in the table, and the run log has both runs.
    let conn = backend.block_on(async {
        let d = dbayam_driver_sqlite::SqliteDriver::new();
        let mut c = dbayam_core::config::ConnectionConfig::new(dbayam_core::DbKind::Sqlite, "check");
        c.file_path = Some(db.display().to_string());
        dbayam_core::Driver::connect(&d, &dbayam_core::ConnectParams::from_config(&c, None)).await
    });
    let mut s = conn.unwrap();
    let n = backend.block_on(dbayam_core::driver::query_scalar(s.as_mut(), "SELECT COUNT(*) FROM t", &[])).unwrap();
    assert_eq!(n.as_i64(), Some(2));
    let log = dbayam_tasks::RunLog::open(paths.tasks_dir(), Default::default()).unwrap();
    assert_eq!(log.runs().unwrap().len(), 2);
    let _ = std::fs::remove_dir_all(&root);
}
