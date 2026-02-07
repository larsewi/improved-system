use crate::storage;

const HEAD_FILE: &str = "HEAD";
const DEFAULT_HEAD: &str = "0000000000000000000000000000000000000000";

pub fn load() -> Result<String, Box<dyn std::error::Error>> {
    let hash = match storage::load(HEAD_FILE)? {
        Some(data) => String::from_utf8(data)?.trim().to_string(),
        None => DEFAULT_HEAD.to_string(),
    };
    log::info!("Current head is '{:.7}...'", hash);
    Ok(hash)
}

pub fn save(hash: &str) -> Result<(), Box<dyn std::error::Error>> {
    storage::save(HEAD_FILE, hash.as_bytes())?;
    log::info!("Updated head to '{:.7}...'", hash);
    Ok(())
}
