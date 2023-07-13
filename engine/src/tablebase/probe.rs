use std::ffi::CString;

use super::bindings::tb_init;

pub fn init_tablebase(path: &str) {
    unsafe {
        let syzygy_path = CString::new(path).unwrap();
        assert!(
            tb_init(syzygy_path.as_ptr()),
            "TB failed to initalize"
        );
    }
}
