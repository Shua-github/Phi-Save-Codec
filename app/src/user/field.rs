use crate::phi_base::*;
use bitvec::prelude::*;
use shua_struct::field::{BinaryField, Options};
use shua_struct_macro::binary_struct;
use std::cell::Cell;

#[derive(Debug, Default)]
#[binary_struct(bit_order = Lsb0)]
pub struct User {
    #[binary_field(align = 8)]
    pub show_player_id: bool,
    pub self_intro: PhiString,
    pub avatar: PhiString,
    pub background: PhiString,
}
