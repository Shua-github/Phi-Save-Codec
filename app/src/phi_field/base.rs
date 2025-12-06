use bitvec::prelude::*;
use shua_struct::field::{BinaryField, Ctx, GetLen};

#[derive(Clone, Debug, Default)]
pub struct VarInt(pub u16);

impl BinaryField for VarInt {
    fn parse(
        bits: &BitSlice<u8, Lsb0>,
        _ctx: &mut Ctx,
        _name: Option<&str>,
        _get_len: Option<GetLen>,
    ) -> Result<(Self, usize), String> {
        if bits.len() < 8 {
            return Err("VarInt parse error: not enough bits".to_string());
        }

        let first = bits[0..8].load_le::<u8>();

        if first > 127 {
            if bits.len() < 16 {
                return Err("VarInt parse error: not enough bits for two-byte VarInt".to_string());
            }

            let second = bits[8..16].load_le::<u8>();

            let value = ((first & 0x7F) as u16) | ((second as u16) << 7);

            Ok((VarInt(value), 16))
        } else {
            Ok((VarInt(first as u16), 8))
        }
    }

    fn build(&self) -> BitVec<u8, Lsb0> {
        let mut bv = BitVec::new();

        if self.0 > 127 {
            let first = ((self.0 & 0x7F) as u8) | 0x80;
            let second = (self.0 >> 7) as u8;

            bv.extend_from_raw_slice(&[first, second]);
        } else {
            bv.extend_from_raw_slice(&[self.0 as u8]);
        }

        bv
    }
}

#[derive(Clone, Debug, Default)]
pub struct PhiString(pub String);
impl BinaryField for PhiString {
    fn parse(
        bits: &BitSlice<u8, Lsb0>,
        ctx: &mut Ctx,
        name: Option<&str>,
        get_len: Option<GetLen>,
    ) -> Result<(Self, usize), String> {
        let (varint, offset_bits) = VarInt::parse(bits, ctx, name, get_len)?;
        let length_bytes = varint.0 as usize;
        let length_bits = length_bytes
            .checked_mul(8)
            .ok_or_else(|| "String parse error: length overflow".to_string())?;

        if bits.len() < offset_bits + length_bits {
            return Err("String parse error: not enough bits".to_string());
        }

        let mut bytes: Vec<u8> = Vec::with_capacity(length_bytes);
        for i in 0..length_bytes {
            let start = offset_bits + i * 8;
            let end = start + 8;
            let b = bits[start..end].load_le::<u8>();
            bytes.push(b);
        }

        let s = std::str::from_utf8(&bytes)
            .map_err(|e| format!("String parse error: {}, raw: {:02X?}", e, bytes))?;
        Ok((PhiString(s.to_string()), offset_bits + length_bits))
    }

    fn build(&self) -> BitVec<u8, Lsb0> {
        let bytes = self.0.as_bytes();
        let mut bv = VarInt(bytes.len() as u16).build();
        bv.extend_from_raw_slice(bytes);
        bv
    }
}

#[derive(Clone, Debug, Default, Copy)]
pub struct BitBool(pub bool);

impl BinaryField for BitBool {
    fn parse(
        bits: &BitSlice<u8, Lsb0>,
        _ctx: &mut Ctx,
        _name: Option<&str>,
        _get_len: Option<GetLen>,
    ) -> Result<(Self, usize), String> {
        if bits.len() < 1 {
            return Err("BitBool parse error: not enough bits".to_string());
        }
        Ok((BitBool(bits[0]), 1))
    }

    fn build(&self) -> BitVec<u8, Lsb0> {
        let mut bv = BitVec::<u8, Lsb0>::new();
        bv.push(self.0);
        bv
    }
}

// <->
impl From<String> for PhiString {
    fn from(s: String) -> Self {
        PhiString(s)
    }
}

impl From<&str> for PhiString {
    fn from(s: &str) -> Self {
        PhiString(s.to_string())
    }
}

impl From<PhiString> for String {
    fn from(phi: PhiString) -> Self {
        phi.0
    }
}

impl From<u16> for VarInt {
    fn from(value: u16) -> Self {
        VarInt(value)
    }
}

impl From<VarInt> for u16 {
    fn from(varint: VarInt) -> Self {
        varint.0
    }
}
impl From<bool> for BitBool {
    fn from(b: bool) -> Self {
        BitBool(b)
    }
}

impl From<BitBool> for bool {
    fn from(bb: BitBool) -> Self {
        bb.0
    }
}
