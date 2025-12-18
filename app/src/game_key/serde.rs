use super::field::*;
use crate::phi_base::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SerializableGameKey {
    #[serde(rename = "key_list")]
    pub keys: Vec<SerializableKey>,
    pub lanota_read_keys: [bool; 6],
    pub camellia_read_key: [bool; 8],
    pub side_story4_begin_read_key: bool,
    pub old_score_cleared_v390: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SerializableKey {
    pub name: String,
    #[serde(rename = "type")]
    pub ktype: [bool; 5],
    pub flag: Vec<bool>,
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
