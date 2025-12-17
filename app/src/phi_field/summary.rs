pub(crate) use crate::phi_field::base::*;
use bitvec::prelude::*;
use shua_struct::field::{BinaryField, Options};
use shua_struct_macro::binary_struct;
use std::cell::Cell;

#[derive(Debug, Default)]
#[binary_struct]
pub struct Level {
    pub clear: u16,
    pub fc: u16,
    pub phi: u16
}

#[derive(Debug, Default)]
#[binary_struct]
pub struct MultiLevel {
    pub ez:Level,
    pub hd:Level,
    pub r#in:Level,
    pub at:Level,
}

#[derive(Debug, Default)]
#[binary_struct]
pub struct Summary {
    pub save_version: u8,
    pub challenge_mode_rank: u16,
    pub rks: f32,
    pub game_version: VarInt,
    pub avatar: PhiString,
    pub level: MultiLevel
}