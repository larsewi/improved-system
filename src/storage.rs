use std::fs;
use std::io::Write;

use crate::config;

pub fn read_head() -> String {
    let head_path = format!("{}/HEAD", config::get_work_dir());
    fs::read_to_string(&head_path)
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "0".repeat(40))
}

pub fn ensure_work_dir() -> Result<(), std::io::Error> {
    fs::create_dir_all(config::get_work_dir())
}

pub fn write_block(hash: &str, data: &[u8]) -> Result<(), std::io::Error> {
    let path = format!("{}/{}", config::get_work_dir(), hash);
    let mut file = fs::File::create(&path)?;
    file.write_all(data)
}

pub fn write_head(hash: &str) -> Result<(), std::io::Error> {
    let head_path = format!("{}/HEAD", config::get_work_dir());
    let mut file = fs::File::create(&head_path)?;
    file.write_all(hash.as_bytes())
}
