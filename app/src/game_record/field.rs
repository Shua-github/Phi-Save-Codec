pub(crate) use crate::phi_base::*;
use bitvec::prelude::*;
use shua_struct::field::{BinaryField, Options};
use shua_struct_macro::binary_struct;
use std::cell::Cell;

#[derive(Debug, Default)]
#[binary_struct(bit_order = Lsb0)]
pub struct LevelRecord {
    pub score: u32,
    pub acc: f32,
}
#[derive(Debug, Default)]
#[binary_struct(bit_order = Lsb0)]
pub struct SongEntry {
    pub name: PhiString,
    pub length: VarInt,
    #[binary_field(align = 8)]
    pub unlock: [bool; 5],
    #[binary_field(align = 8)]
    pub fc: [bool; 5],
    #[binary_field(size_func = get_levels_len)]
    pub levels: Vec<LevelRecord>,
}
impl SongEntry {
    fn get_levels_len(&self) -> usize {
        self.unlock.iter().filter(|bit_bool| **bit_bool).count()
    }
}
#[derive(Debug, Default)]
#[binary_struct(bit_order = Lsb0)]
pub struct GameRecord {
    pub song_sum: VarInt,
    #[binary_field(size_field = song_sum)]
    pub song_list: Vec<SongEntry>,
}
