use std::{
    path::PathBuf,
    sync::{Mutex, MutexGuard},
    time::{SystemTime, UNIX_EPOCH},
};

static DB_LOCK: Mutex<()> = Mutex::new(());

pub struct TestDb {
    path: PathBuf,
    _guard: MutexGuard<'static, ()>,
}

impl TestDb {
    pub fn new() -> Self {
        let guard = DB_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let dir = std::env::current_dir()
            .expect("current directory should be available")
            .join("target")
            .join("test-dbs");
        std::fs::create_dir_all(&dir).expect("test database directory should be created");
        let path = dir.join(format!(
            "devpaste-test-{}-{}.sql",
            std::process::id(),
            timestamp
        ));

        unsafe {
            std::env::set_var("DEVPASTE_DB_PATH", &path);
        }

        devpaste::utils::initialize_database().expect("test database should initialize");

        Self {
            path,
            _guard: guard,
        }
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}
