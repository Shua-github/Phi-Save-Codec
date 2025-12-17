use crate::api::{Data, empty_data, malloc_data};
use crate::phi_field::game_key::*;
use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use shua_struct::field::BinaryField;

#[derive(Serialize, Deserialize)]
struct SerializableGameKey {
    #[serde(rename = "key_list")]
    keys: Vec<SerializableKey>,
    lanota_read_keys: [bool; 6],
    camellia_read_key: [bool; 8],
    side_story4_begin_read_key: bool,
    old_score_cleared_v390: bool,
}

#[derive(Serialize, Deserialize)]
struct SerializableKey {
    name: String,
    #[serde(rename = "type")]
    ktype: [bool; 5],
    flag: Vec<bool>,
}

impl From<GameKey> for SerializableGameKey {
    fn from(gk: GameKey) -> Self {
        SerializableGameKey {
            keys: gk
                .key_list
                .key_list
                .into_iter()
                .map(|k| SerializableKey {
                    name: k.name.into(),
                    ktype: k.ktype,
                    flag: k.flag,
                })
                .collect(),
            lanota_read_keys: gk.lanota_read_keys,
            camellia_read_key: gk.camellia_read_key,
            side_story4_begin_read_key: gk.side_story4_begin_read_key,
            old_score_cleared_v390: gk.old_score_cleared_v390,
        }
    }
}

impl From<SerializableGameKey> for GameKey {
    fn from(sgk: SerializableGameKey) -> Self {
        let key_sum = sgk.keys.len();
        let key_list = sgk
            .keys
            .into_iter()
            .map(|sk| {
                let length = sk.flag.len() as u8 + 1; // flag 长度 + 1 = length
                Key {
                    name: sk.name.into(),
                    length,
                    ktype: sk.ktype,
                    flag: sk.flag,
                }
            })
            .collect();

        GameKey {
            key_list: KeyList {
                key_sum: VarInt(key_sum as u16),
                key_list,
            },
            lanota_read_keys: sgk.lanota_read_keys,
            camellia_read_key: sgk.camellia_read_key,
            side_story4_begin_read_key: sgk.side_story4_begin_read_key,
            old_score_cleared_v390: sgk.old_score_cleared_v390,
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn parse_game_key(data_ptr: *const u8, data_len: usize) -> Data {
    if data_ptr.is_null() || data_len == 0 {
        return empty_data();
    }
    let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
    let bits = BitSlice::<u8, Lsb0>::from_slice(bytes);

    let (game_key, _) = match GameKey::parse(&bits, &None) {
        Ok(r) => r,
        Err(_) => return empty_data(),
    };

    let json = match rmp_serde::to_vec_named(&SerializableGameKey::from(game_key)) {
        Ok(v) => v,
        Err(_) => return empty_data(),
    };
    unsafe { malloc_data(json) }
}

#[unsafe(no_mangle)]
pub extern "C" fn build_game_key(data_ptr: *const u8, data_len: usize) -> Data {
    if data_ptr.is_null() || data_len == 0 {
        return empty_data();
    }
    let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
    let serializable: SerializableGameKey = match rmp_serde::from_slice(bytes) {
        Ok(v) => v,
        Err(_) => return empty_data(),
    };

    let bitvec = match GameKey::from(serializable).build(&None) {
        Ok(v) => v,
        Err(_) => return empty_data(),
    };

    unsafe { malloc_data(bitvec.into_vec()) }
}
