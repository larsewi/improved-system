use std::ffi::{c_char, CStr};

mod block_ops;
mod config;
mod storage;

pub mod block {
    include!(concat!(env!("OUT_DIR"), "/block.rs"));
}

#[unsafe(no_mangle)]
pub extern "C" fn init(work_dir: *const c_char) -> i32 {
    env_logger::init();

    if work_dir.is_null() {
        log::error!("init: bad argument: work directory cannot be NULL");
        return -1;
    }

    match unsafe { CStr::from_ptr(work_dir) }.to_str() {
        Ok(path) => match config::set_work_dir(path.to_string()) {
            Ok(_) => {
                log::debug!("init: work directory: {}", path);
                0
            }
            Err(e) => {
                log::debug!("init: failed to set work directory {}: {}", path, e);
                -1
            }
        },
        Err(e) => {
            log::error!("init: bad argument: {e}");
            -1
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn commit() -> i32 {
    match block_ops::commit_impl() {
        Ok(_) => 0,
        Err(e) => {
            log::error!("commit: {}", e);
            -1
        }
    }
}
