pub(crate) use crate::phi_field::base::*;
use bitvec::prelude::*;
use shua_struct::field::BinaryField;
use shua_struct::field::Ctx;
use shua_struct::field::GetLen;
use shua_struct_macro::binary_struct;

#[derive(Clone, Debug, Default)]
#[binary_struct]
pub struct Key {
    pub name: PhiString,
    pub length: u8,
    pub ktype: [BitBool; 8],
    #[binary_field(func = get_flag_len)]
    pub flag: Vec<bool>,
}

fn get_flag_len(name: &str, ctx: &Ctx) -> u64 {
    if name == "flag" {
        if let Some(len_any) = ctx.get("length") {
            if let Some(len) = len_any.downcast_ref::<u8>() {
                return (*len as u64).saturating_sub(1);
            }
        }
    }
    0
}

#[derive(Clone, Debug)]
#[binary_struct]
pub struct KeyList {
    pub key_sum: VarInt,
    #[binary_field(func = get_key_len)]
    pub key_list: Vec<Key>,
}
fn get_key_len(name: &str, ctx: &Ctx) -> u64 {
    if name == "key_list" {
        if let Some(len_any) = ctx.get("key_sum") {
            if let Some(len) = len_any.downcast_ref::<VarInt>() {
                return len.0 as u64;
            }
        }
    }
    0
}

#[derive(Clone, Debug)]
#[binary_struct]
pub struct GameKey {
    pub key_list: KeyList,
    pub lanota_read_keys: [BitBool; 8],
    pub camellia_read_key: [BitBool; 8],
    pub side_story4_begin_read_key: bool,
    pub old_score_cleared_v390: bool,
}
