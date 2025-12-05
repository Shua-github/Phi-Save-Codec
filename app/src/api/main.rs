use std::alloc::{Layout, alloc, dealloc};
use std::os::raw::{c_void};
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
    if ptr.is_null() { return; }
    
    unsafe {
        let layout = match Layout::array::<u8>(len) {
            Ok(l) => l,
            Err(_) => return,
        };
        dealloc(ptr as *mut u8, layout);
    }
}