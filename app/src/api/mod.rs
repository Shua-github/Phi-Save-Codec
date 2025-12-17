pub mod game_key;
pub mod game_record;
pub mod user;
pub mod summary;
pub mod game_progress;

use std::alloc::{Layout, alloc, dealloc};

#[repr(C)]
pub struct Data {
    pub len: usize,
    pub ptr: *mut u8,
}

#[inline(always)]
fn empty_data() -> Data {
    Data {
        len: 0,
        ptr: std::ptr::null_mut(),
    }
}

#[inline(always)]
pub unsafe fn malloc_data(mut bytes: Vec<u8>) -> Data {
    bytes.shrink_to_fit();
    let len = bytes.len();
    let ptr = bytes.as_mut_ptr();
    std::mem::forget(bytes);
    Data { ptr, len }
}

#[unsafe(no_mangle)]
pub extern "C" fn malloc(len: usize) -> *mut u8 {
    if len == 0 {
        return std::ptr::null_mut();
    }

    unsafe {
        let layout = match Layout::array::<u8>(len) {
            Ok(l) => l,
            Err(_) => return std::ptr::null_mut(),
        };

        return alloc(layout);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free(ptr: *mut u8, len: usize) {
    unsafe {
        let layout = Layout::array::<u8>(len).unwrap();
        dealloc(ptr, layout);
    }
}
