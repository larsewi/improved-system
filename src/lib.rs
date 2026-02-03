use std::fs;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use prost::Message;
use sha1::{Digest, Sha1};

// Include generated protobuf code
pub mod block {
    include!(concat!(env!("OUT_DIR"), "/block.rs"));
}

use block::Block;

#[unsafe(no_mangle)]
pub extern "C" fn init() {
    env_logger::init();
}

#[unsafe(no_mangle)]
pub extern "C" fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[unsafe(no_mangle)]
pub extern "C" fn commit() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32;

    let block = Block {
        version: 1,
        timestamp,
        parent: "0000000000000000000000000000000000000000".to_string(),
    };

    // Serialize the block to protobuf bytes
    let mut buf = Vec::new();
    block.encode(&mut buf).expect("Failed to encode block");

    // Calculate SHA-1 hash of the serialized protobuf
    let mut hasher = Sha1::new();
    hasher.update(&buf);
    let hash = hasher.finalize();
    let hash_hex = format!("{:x}", hash);

    // Create .improved directory if it doesn't exist
    fs::create_dir_all(".improved").expect("Failed to create .improved directory");

    // Write the serialized block to .improved/<sha1>
    let path = format!(".improved/{}", hash_hex);
    let mut file = fs::File::create(&path).expect("Failed to create block file");
    file.write_all(&buf).expect("Failed to write block");

    log::info!(
        "commit: created block {} (version={}, timestamp={}, parent={})",
        hash_hex, block.version, block.timestamp, block.parent
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
