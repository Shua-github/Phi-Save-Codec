pub(crate) use crate::phi_field::base::*;
use bitvec::prelude::*;
use shua_struct::field::{BinaryField, Options};
use shua_struct_macro::binary_struct;
use std::cell::Cell;

#[derive(Debug, Default)]
#[binary_struct]
pub struct Base {
    pub is_first_run: bool,
    pub legacy_chapter_finished: bool,
    pub already_show_collection_tip: bool,
    pub already_show_auto_unlock_in_tip: bool,
}

#[derive(Debug, Default)]
#[binary_struct]
pub struct Money {
    pub kib: VarInt,
    pub mib: VarInt,
    pub gib: VarInt,
    pub tib: VarInt,
    pub pib: VarInt
}

#[derive(Debug, Default)]
#[binary_struct]
pub struct Chapter8Base {
    pub unlock_begin: bool,
    pub unlock_second_phase: bool,
    pub passed: bool,
}

#[derive(Debug, Default)]
#[binary_struct]
pub struct GameProgress {
    #[binary_field(align = 8)]
    pub base: Base,
    pub completed: PhiString,
    pub song_update_info: VarInt,
    pub challenge_mode_rank: u16,
    pub money: Money,
    #[binary_field(align = 8)]
    pub unlock_flag_of_spasmodic: [bool; 4],
    #[binary_field(align = 8)]
    pub unlock_flag_of_igallta: [bool; 4],
    #[binary_field(align = 8)]
    pub unlock_flag_of_rrharil: [bool; 4],
    pub flag_of_song_record_key: [bool; 8],
    #[binary_field(align = 8)]
    pub random_version_unlocked: [bool; 6],
    #[binary_field(align = 8)]
    pub chapter8_base: Chapter8Base,
    #[binary_field(align = 8)]
    pub chapter8_song_unlocked: [bool; 6],
    #[binary_field(align = 8)]
    pub flag_of_song_record_key_takumi: [bool; 3],
}
