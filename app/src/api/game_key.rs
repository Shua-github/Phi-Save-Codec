use crate::phi_field::game_key::*;
use base64::{Engine, engine::general_purpose};
use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use shua_struct::field::BinaryField;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[derive(Serialize, Deserialize)]
struct SerializableGameKey {
    key_list: SerializableKeyList,
    lanota_read_keys: Vec<bool>,
    camellia_read_key: Vec<bool>,
    side_story4_begin_read_key: bool,
    old_score_cleared_v390: bool,
}

#[derive(Serialize, Deserialize)]
struct SerializableKeyList {
    key_sum: u16,
    key_list: Vec<SerializableKey>,
}

#[derive(Serialize, Deserialize)]
struct SerializableKey {
    name: String,
    length: u8,
    #[serde(rename = "type")]
    ktype: Vec<bool>,
    flag: Vec<bool>,
}

impl From<GameKey> for SerializableGameKey {
    fn from(gk: GameKey) -> Self {
        SerializableGameKey {
            key_list: SerializableKeyList {
                key_sum: gk.key_list.key_sum.0,
                key_list: gk
                    .key_list
                    .key_list
                    .into_iter()
                    .map(|k| SerializableKey {
                        name: k.name.0,
                        length: k.length,
                        ktype: k.ktype.into_iter().map(|b| b.0).collect(),
                        flag: k.flag,
                    })
                    .collect(),
            },
            lanota_read_keys: gk.lanota_read_keys.into_iter().map(|b| b.0).collect(),
            camellia_read_key: gk.camellia_read_key.into_iter().map(|b| b.0).collect(),
            side_story4_begin_read_key: gk.side_story4_begin_read_key,
            old_score_cleared_v390: gk.old_score_cleared_v390,
        }
    }
}

impl From<SerializableGameKey> for GameKey {
    fn from(sgk: SerializableGameKey) -> Self {
        GameKey {
            key_list: KeyList {
                key_sum: VarInt(sgk.key_list.key_sum),
                key_list: sgk
                    .key_list
                    .key_list
                    .into_iter()
                    .map(|sk| Key {
                        name: PhiString(sk.name),
                        length: sk.length,
                        ktype: sk
                            .ktype
                            .into_iter()
                            .map(BitBool::from)
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap(),
                        flag: sk.flag,
                    })
                    .collect(),
            },
            lanota_read_keys: sgk
                .lanota_read_keys
                .into_iter()
                .map(BitBool::from)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            camellia_read_key: sgk
                .camellia_read_key
                .into_iter()
                .map(BitBool::from)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            side_story4_begin_read_key: sgk.side_story4_begin_read_key,
            old_score_cleared_v390: sgk.old_score_cleared_v390,
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn parse_game_key(base64_str_ptr: *const c_char) -> *mut c_char {
    if base64_str_ptr.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(base64_str_ptr) };
    let base64_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let bytes = match general_purpose::STANDARD.decode(base64_str) {
        Ok(d) => d,
        Err(_) => return std::ptr::null_mut(),
    };

    let bits = BitSlice::<u8, Lsb0>::from_slice(&bytes);
    let mut ctx = HashMap::new();

    let (game_key, _) = match GameKey::parse(&bits, &mut ctx, None, None) {
        Ok(r) => r,
        Err(_) => return std::ptr::null_mut(),
    };

    let serializable = SerializableGameKey::from(game_key);

    let json = match serde_json::to_string(&serializable) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    match CString::new(json) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn build_game_key(json_ptr: *const c_char) -> *mut c_char {
    if json_ptr.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(json_ptr) };
    let json_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let serializable: SerializableGameKey = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };

    let game_key: GameKey = GameKey::from(serializable);
    let bitvec = game_key.build();
    let bytes_vec = bitvec.into_vec();

    let base64_str = general_purpose::STANDARD.encode(&bytes_vec);

    match CString::new(base64_str) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}
