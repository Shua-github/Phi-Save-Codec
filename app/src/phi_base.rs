use bitvec::prelude::*;
use shua_struct::field::{BinaryField, Options};

#[derive(Debug, Default, Clone, Copy)]
pub struct VarInt(pub u16);

impl BinaryField<Lsb0> for VarInt {
    fn parse(bits: &BitSlice<u8, Lsb0>, _opts: &Option<Options>) -> Result<(Self, usize), String> {
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

    fn build(&self, _opts: &Option<Options>) -> Result<BitVec<u8>, String> {
        let mut bv = BitVec::new();

        if self.0 > 127 {
            let first = ((self.0 & 0x7F) as u8) | 0x80;
            let second = (self.0 >> 7) as u8;

            bv.extend_from_raw_slice(&[first, second]);
        } else {
            bv.extend_from_raw_slice(&[self.0 as u8]);
        }

        Ok(bv)
    }
}

impl From<VarInt> for usize {
    fn from(var: VarInt) -> Self {
        var.0 as usize
    }
}

#[derive(Debug, Default)]
pub struct PhiString(pub String);
impl BinaryField<Lsb0> for PhiString {
    fn parse(bits: &BitSlice<u8, Lsb0>, opts: &Option<Options>) -> Result<(Self, usize), String> {
        let (varint, offset_bits) = VarInt::parse(bits, opts)?;
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

    fn build(&self, opts: &Option<Options>) -> Result<BitVec<u8>, String> {
        let bytes = self.0.as_bytes();
        let mut bv = VarInt(bytes.len() as u16).build(opts)?;
        bv.extend_from_raw_slice(bytes);
        Ok(bv)
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
