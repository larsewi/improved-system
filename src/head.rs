use crate::storage;
use crate::utils::GENESIS_HASH;

const HEAD_FILE: &str = "HEAD";

pub fn load() -> Result<String, Box<dyn std::error::Error>> {
    let hash = match storage::load(HEAD_FILE)? {
        Some(data) => String::from_utf8(data)?.trim().to_string(),
        None => GENESIS_HASH.to_string(),
    };
    log::info!("Current head is '{:.7}...'", hash);
    Ok(hash)
}

pub fn save(hash: &str) -> Result<(), Box<dyn std::error::Error>> {
    storage::save(HEAD_FILE, hash.as_bytes())?;
    log::info!("Updated head to '{:.7}...'", hash);
    Ok(())
}
