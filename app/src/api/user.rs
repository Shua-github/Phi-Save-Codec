use crate::api::{Data, empty_data, malloc_data};
use crate::phi_field::user::User;
use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use shua_struct::field::BinaryField;

#[derive(Serialize, Deserialize)]
struct SerializableUser {
    show_player_id: bool,
    self_intro: String,
    avatar: String,
    background: String,
}

impl From<User> for SerializableUser {
    fn from(user: User) -> Self {
        SerializableUser {
            show_player_id: user.show_player_id,
            self_intro: user.self_intro.into(),
            avatar: user.avatar.into(),
            background: user.background.into(),
        }
    }
}

impl From<SerializableUser> for User {
    fn from(su: SerializableUser) -> Self {
        User {
            show_player_id: su.show_player_id,
            self_intro: su.self_intro.into(),
            avatar: su.avatar.into(),
            background: su.background.into(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn parse_user(data_ptr: *const u8, data_len: usize) -> Data {
    if data_ptr.is_null() || data_len == 0 {
        return empty_data();
    }
    let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
    let bits = BitSlice::<u8, Lsb0>::from_slice(bytes);

    let (user, _) = match User::parse(bits, &None) {
        Ok(r) => r,
        Err(_) => return empty_data(),
    };

    let json = match rmp_serde::to_vec_named(&SerializableUser::from(user)) {
        Ok(v) => v,
        Err(_) => return empty_data(),
    };
    unsafe { malloc_data(json) }
}

#[unsafe(no_mangle)]
pub extern "C" fn build_user(data_ptr: *const u8, data_len: usize) -> Data {
    if data_ptr.is_null() || data_len == 0 {
        return empty_data();
    }
    let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
    let serializable: SerializableUser = match rmp_serde::from_slice(bytes) {
        Ok(v) => v,
        Err(_) => return empty_data(),
    };

    let bitvec = match User::from(serializable).build(&None) {
        Ok(v) => v,
        Err(_) => return empty_data(),
    };

    unsafe { malloc_data(bitvec.into_vec()) }
}
