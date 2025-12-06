pub(crate) use crate::phi_field::base::*;
use bitvec::prelude::*;
use shua_struct::field::{BinaryField, Ctx, GetLen};
use shua_struct_macro::binary_struct;

#[derive(Clone, Debug, Default)]
#[binary_struct]
pub struct LevelRecord {
    pub score: u32,
    pub acc: f32,
}

#[derive(Clone, Debug, Default)]
#[binary_struct]
pub struct SongEntry {
    pub name: PhiString,
    pub length: VarInt,
    pub unlock: [BitBool; 5],
    pub fc: [BitBool; 5],
    #[binary_field(func = get_levels_len)]
    pub levels: Vec<LevelRecord>,
}

fn get_levels_len(name: &str, ctx: &Ctx) -> u64 {
    if name == "levels" {
        if let Some(unlock_any) = ctx.get("unlock") {
            if let Some(unlock_array) = unlock_any.downcast_ref::<[BitBool; 5]>() {
                let count = unlock_array.iter().filter(|bit_bool| bit_bool.0).count();

                return count as u64;
            }
        }
    }
    0
}

#[derive(Clone, Debug, Default)]
#[binary_struct]
pub struct GameRecord {
    pub song_sum: VarInt,

    #[binary_field(func = get_song_list_len)]
    pub song_list: Vec<SongEntry>,
}

fn get_song_list_len(name: &str, ctx: &Ctx) -> u64 {
    if name == "song_list" {
        if let Some(len_any) = ctx.get("song_sum") {
            if let Some(len) = len_any.downcast_ref::<VarInt>() {
                return len.0 as u64;
            }
        }
    }
    0
}
