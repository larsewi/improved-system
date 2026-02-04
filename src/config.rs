use std::sync::OnceLock;

static WORK_DIR: OnceLock<String> = OnceLock::new();

pub fn get_work_dir() -> &'static str {
    WORK_DIR.get().map(|s| s.as_str()).unwrap_or(".improved")
}

pub fn set_work_dir(path: String) -> Result<(), String> {
    WORK_DIR.set(path).map_err(|e| e)
}
