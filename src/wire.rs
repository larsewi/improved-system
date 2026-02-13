use prost::Message;

use crate::proto::patch::Patch;

const ZSTD_COMPRESSION_LEVEL: i32 = 3;

/// Encode a Patch to protobuf and compress with zstd.
pub fn encode_patch(patch: &Patch) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    patch.encode(&mut buf)?;
    let compressed = zstd::encode_all(buf.as_slice(), ZSTD_COMPRESSION_LEVEL)?;
    log::info!(
        "Patch encoded: {} bytes protobuf, {} bytes compressed ({:.0}% reduction)",
        buf.len(),
        compressed.len(),
        if buf.is_empty() {
            0.0
        } else {
            (1.0 - compressed.len() as f64 / buf.len() as f64) * 100.0
        }
    );
    Ok(compressed)
}

/// Decompress zstd and decode a Patch from protobuf.
pub fn decode_patch(data: &[u8]) -> Result<Patch, Box<dyn std::error::Error>> {
    let decompressed = zstd::decode_all(data)?;
    let patch = Patch::decode(decompressed.as_slice())?;
    Ok(patch)
}
