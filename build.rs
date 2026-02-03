fn main() {
    prost_build::compile_protos(&["proto/block.proto"], &["proto/"]).unwrap();
}
