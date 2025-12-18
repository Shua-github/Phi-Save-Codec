use crate::game_key::{field::GameKey, serde::SerializableGameKey};
use crate::game_progress::{field::GameProgress, serde::SerializableGameProgress};
use crate::game_record::{field::GameRecord, serde::SerializableGameRecord};
use crate::summary::{field::Summary, serde::SerializableSummary};
use crate::user::{field::User, serde::SerializableUser};
use bitvec::prelude::*;
use shua_struct::field::BinaryField;
use std::alloc::{Layout, alloc, dealloc};

#[repr(C)]
pub struct Data {
    pub len: usize,
    pub ptr: *mut u8,
}

#[inline(always)]
pub fn empty_data() -> Data {
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

#[macro_export]
macro_rules! impl_c_api {
    ($struct_ty:ty, $serializable_ty:ty, $parse_fn:ident, $build_fn:ident) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $parse_fn(data_ptr: *const u8, data_len: usize) -> Data {
            if data_ptr.is_null() || data_len == 0 {
                return empty_data();
            }
            let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
            let bits = BitSlice::<u8, Lsb0>::from_slice(bytes);

            let (item, _) = match <$struct_ty>::parse(bits, &None) {
                Ok(r) => r,
                Err(_) => return empty_data(),
            };

            let json = match rmp_serde::to_vec_named(&<$serializable_ty>::from(item)) {
                Ok(v) => v,
                Err(_) => return empty_data(),
            };

            unsafe { malloc_data(json) }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $build_fn(data_ptr: *const u8, data_len: usize) -> Data {
            if data_ptr.is_null() || data_len == 0 {
                return empty_data();
            }

            let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
            let serializable: $serializable_ty = match rmp_serde::from_slice(bytes) {
                Ok(v) => v,
                Err(_) => return empty_data(),
            };

            let bitvec = match <$struct_ty>::from(serializable).build(&None) {
                Ok(v) => v,
                Err(_) => return empty_data(),
            };

            unsafe { malloc_data(bitvec.into_vec()) }
        }
    };
}

impl_c_api!(User, SerializableUser, parse_user, build_user);
impl_c_api!(Summary, SerializableSummary, parse_summary, build_summary);
impl_c_api!(
    GameRecord,
    SerializableGameRecord,
    parse_game_record,
    build_game_record
);
impl_c_api!(
    GameProgress,
    SerializableGameProgress,
    parse_game_progress,
    build_game_progress
);
impl_c_api!(GameKey, SerializableGameKey, parse_game_key, build_game_key);
