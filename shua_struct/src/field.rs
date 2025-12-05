use std::any::Any;
use std::collections::HashMap;

use bitvec::prelude::*;

pub type Ctx = HashMap<String, Box<dyn Any>>;
pub type GetLen = fn(name: &str, ctx: &Ctx) -> u64;

pub trait BinaryField: Sized {
    fn parse(
        bits: &BitSlice<u8, Lsb0>,
        ctx: &mut Ctx,
        name: Option<&str>,
        get_len: Option<GetLen>,
    ) -> Result<(Self, usize), String>;

    fn build(&self) -> BitVec<u8, Lsb0>;
}

macro_rules! impl_bit_primitive {
    ($t:ty, $size_bits:expr) => {
        impl BinaryField for $t {
            fn parse(
                bits: &BitSlice<u8, Lsb0>,
                _ctx: &mut Ctx,
                _name: Option<&str>,
                _get_len: Option<GetLen>,
            ) -> Result<(Self, usize), String> {
                if bits.len() < $size_bits {
                    return Err(format!(
                        "{} parse error: not enough bits (needed {}, got {})",
                        stringify!($t),
                        $size_bits,
                        bits.len()
                    ));
                }
                let value = bits[0..$size_bits].load_le::<$t>();
                Ok((value, $size_bits))
            }

            fn build(&self) -> BitVec<u8, Lsb0> {
                let mut bv = BitVec::<u8, Lsb0>::new();
                let bytes = self.to_le_bytes();
                bv.extend_from_raw_slice(&bytes);
                bv.truncate($size_bits);
                bv
            }
        }
    };
}

impl_bit_primitive!(u8, 8);
impl_bit_primitive!(u16, 16);
impl_bit_primitive!(u32, 32);

impl BinaryField for f32 {
    fn parse(
        bits: &BitSlice<u8, Lsb0>,
        _ctx: &mut Ctx,
        _name: Option<&str>,
        _get_len: Option<GetLen>,
    ) -> Result<(Self, usize), String> {
        const SIZE_BITS: usize = 32;
        if bits.len() < SIZE_BITS {
            return Err(format!(
                "f32 parse error: not enough bits (needed 32, got {})",
                bits.len()
            ));
        }
        let raw_bits = bits[0..SIZE_BITS].load_le::<u32>();
        Ok((f32::from_bits(raw_bits), SIZE_BITS))
    }

    fn build(&self) -> BitVec<u8, Lsb0> {
        let mut bv = BitVec::<u8, Lsb0>::new();
        let bytes = self.to_bits().to_le_bytes();
        bv.extend_from_raw_slice(&bytes);
        bv.truncate(32);
        bv
    }
}

impl BinaryField for bool {
    fn parse(
        bits: &BitSlice<u8, Lsb0>,
        _ctx: &mut Ctx,
        _name: Option<&str>,
        _get_len: Option<GetLen>,
    ) -> Result<(Self, usize), String> {
        if bits.len() < 8 {
            return Err("bool parse error: not enough bits".to_string());
        }
        let value = bits[0];
        Ok((value, 8))
    }

    fn build(&self) -> BitVec<u8, Lsb0> {
        let mut bv = BitVec::<u8, Lsb0>::with_capacity(8);
        bv.push(*self);
        for _ in 1..8 {
            bv.push(false);
        }
        bv
    }
}

impl<T, const N: usize> BinaryField for [T; N]
where
    T: BinaryField + Default + Copy,
{
    fn parse(
        bits: &BitSlice<u8, Lsb0>,
        ctx: &mut Ctx,
        name: Option<&str>,
        get_len: Option<GetLen>,
    ) -> Result<(Self, usize), String> {
        let mut offset = 0;
        let mut arr: [T; N] = [T::default(); N];

        for i in 0..N {
            let (v, l) = T::parse(&bits[offset..], ctx, name, get_len)?;
            offset += l;
            arr[i] = v;
        }

        Ok((arr, offset))
    }

    fn build(&self) -> BitVec<u8, Lsb0> {
        let mut bv = BitVec::new();
        for item in self.iter() {
            bv.extend(item.build());
        }
        bv
    }
}

impl<T> BinaryField for Vec<T>
where
    T: BinaryField + Default,
{
    fn parse(
        bits: &BitSlice<u8, Lsb0>,
        ctx: &mut Ctx,
        name: Option<&str>,
        get_len: Option<GetLen>,
    ) -> Result<(Self, usize), String> {
        let get_len_fn = get_len.ok_or("Vec parse error: missing get_len function")?;
        let field_name = name.ok_or("Vec parse error: empty name")?;

        let len = get_len_fn(field_name, ctx) as usize;

        let mut vec = Vec::with_capacity(len);
        let mut offset = 0;

        for _ in 0..len {
            let (item, l) = T::parse(&bits[offset..], ctx, None, get_len)?;
            offset += l;
            vec.push(item);
        }

        Ok((vec, offset))
    }

    fn build(&self) -> BitVec<u8, Lsb0> {
        let mut bv = BitVec::new();
        for item in self.iter() {
            bv.extend(item.build());
        }
        bv
    }
}
