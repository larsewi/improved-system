use std::sync::OnceLock;

static WORK_DIR: OnceLock<String> = OnceLock::new();

pub fn get_work_dir() -> &'static str {
    WORK_DIR.get().map(|s| s.as_str()).unwrap_or(".improved")
}

fn set_work_dir(path: String) -> Result<(), String> {
    WORK_DIR.set(path).map_err(|e| e)
}

pub fn init_impl(path: &str) -> Result<(), String> {
    env_logger::init();
    set_work_dir(path.to_string())?;
    log::debug!("init: work directory: {}", path);
    Ok(())
}
