use std::alloc::{Layout, alloc, dealloc};
use std::os::raw::{c_char, c_void};
use std::ptr;

#[unsafe(no_mangle)]
pub extern "C" fn malloc(len: usize) -> *mut c_void {
    if len == 0 {
        return ptr::null_mut();
    }

    unsafe {
        let layout = match Layout::array::<u8>(len) {
            Ok(l) => l,
            Err(_) => return ptr::null_mut(),
        };

        let ptr = alloc(layout);
        if ptr.is_null() {
            ptr::null_mut()
        } else {
            ptr as *mut c_void
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free(ptr: *mut c_void, len: usize) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let layout = Layout::array::<u8>(len).unwrap();
        dealloc(ptr as *mut u8, layout);
    }
}

pub unsafe fn malloc_str(s: String) -> *mut c_char {
    let bytes = s.as_bytes();
    let size = bytes.len() + 1;

    let ptr = malloc(size) as *mut u8;
    if ptr.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        *ptr.add(bytes.len()) = 0;
    }

    ptr as *mut c_char
}

#[unsafe(no_mangle)]
pub extern "C" fn free_str(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let mut len = 0usize;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        let total = len + 1;
        free(ptr as *mut _, total);
    }
}
