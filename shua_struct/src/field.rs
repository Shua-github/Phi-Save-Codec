use std::cell::Cell;

use bitvec::prelude::*;

#[derive(Debug, Default)]
pub struct Options {
    pub name: String,
    pub size: usize,
    pub align: usize,
    pub sub_align: Cell<u8>,
}

impl Options {
    pub fn get_align(&self) -> Option<usize> {
        let n = self.sub_align.get();
        if n == 0 {
            return None;
        }
        let new = n - 1;
        self.sub_align.set(new);
        if new == 0 { Some(self.align) } else { None }
    }
}
pub trait BinaryField: Sized {
    fn parse(bits: &BitSlice<u8, Lsb0>, opts: &Option<Options>) -> Result<(Self, usize), String>;

    fn build(&self, opts: &Option<Options>) -> Result<BitVec<u8, Lsb0>, String>;
}

macro_rules! impl_bit_primitive {
    ($t:ty, $size_bits:expr) => {
        impl BinaryField for $t {
            fn parse(
                bits: &BitSlice<u8, Lsb0>,
                _opts: &Option<Options>,
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

            fn build(&self, _opts: &Option<Options>) -> Result<BitVec<u8, Lsb0>, String> {
                let mut bv = BitVec::<u8, Lsb0>::new();
                let bytes = self.to_le_bytes();
                bv.extend_from_raw_slice(&bytes);
                bv.truncate($size_bits);
                Ok(bv)
            }
        }
    };
}

impl_bit_primitive!(u8, 8);
impl_bit_primitive!(u16, 16);
impl_bit_primitive!(u32, 32);

impl BinaryField for f32 {
    fn parse(bits: &BitSlice<u8, Lsb0>, _opts: &Option<Options>) -> Result<(Self, usize), String> {
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

    fn build(&self, _opts: &Option<Options>) -> Result<BitVec<u8, Lsb0>, String> {
        let mut bv = BitVec::<u8, Lsb0>::new();
        let bytes = self.to_bits().to_le_bytes();
        bv.extend_from_raw_slice(&bytes);
        bv.truncate(32);
        Ok(bv)
    }
}

impl BinaryField for bool {
    fn parse(bits: &BitSlice<u8, Lsb0>, _opts: &Option<Options>) -> Result<(Self, usize), String> {
        if bits.len() < 1 {
            return Err("bool parse error: not enough bits".to_string());
        }
        Ok((bits[0], 1))
    }

    fn build(&self, _opts: &Option<Options>) -> Result<BitVec<u8>, String> {
        let mut bv = BitVec::<u8, Lsb0>::new();
        bv.push(*self);
        Ok(bv)
    }
}

impl<T, const N: usize> BinaryField for [T; N]
where
    T: BinaryField + Default + Copy,
{
    fn parse(
        bits: &BitSlice<u8, Lsb0>,
        raw_opts: &Option<Options>,
    ) -> Result<(Self, usize), String> {
        let align = raw_opts
            .as_ref()
            .ok_or("[T; N] parse error: missing opts")?
            .get_align();
        let mut offset = 0;
        let mut arr: [T; N] = [T::default(); N];

        for i in 0..N {
            let (v, l) = T::parse(&bits[offset..], raw_opts)?;
            offset += l;
            match align {
                Some(align) => {
                    let remainder = offset % align;
                    if remainder != 0 {
                        offset += align - remainder;
                    }
                }
                None => {}
            }
            arr[i] = v;
        }

        Ok((arr, offset))
    }

    fn build(&self, raw_opts: &Option<Options>) -> Result<BitVec<u8, Lsb0>, String> {
        let align = raw_opts
            .as_ref()
            .ok_or("Vec parse error: missing opts")?
            .get_align();
        let mut bv = BitVec::new();
        for item in self.iter() {
            match align {
                Some(align) => {
                    let remainder = bv.len() % align;
                    if remainder != 0 {
                        bv.resize(bv.len() + (align - remainder), false);
                    }
                }
                None => {}
            }
            let item_bv = item.build(raw_opts)?;
            bv.extend(item_bv);
        }
        Ok(bv)
    }
}

impl<T> BinaryField for Vec<T>
where
    T: BinaryField + Default,
{
    fn parse(
        bits: &BitSlice<u8, Lsb0>,
        raw_opts: &Option<Options>,
    ) -> Result<(Self, usize), String> {
        let opts = raw_opts.as_ref().ok_or("Vec parse error: missing opts")?;
        if opts.size == 0 {
            return Err("Vec parse error: missing size".to_string());
        }
        let align = opts.get_align();

        let mut vec = Vec::with_capacity(opts.size);
        let mut offset = 0;

        for _ in 0..opts.size {
            let (item, l) = T::parse(&bits[offset..], raw_opts)?;
            offset += l;
            match align {
                Some(align) => {
                    let remainder = offset % align;
                    if remainder != 0 {
                        offset += align - remainder;
                    }
                }
                None => {}
            }
            vec.push(item);
        }
        Ok((vec, offset))
    }

    fn build(&self, raw_opts: &Option<Options>) -> Result<BitVec<u8, Lsb0>, String> {
        let opts = raw_opts.as_ref().ok_or("Vec parse error: missing opts")?;
        let align = opts.get_align();
        let mut bv = BitVec::new();
        for item in self.iter() {
            let item_bv = item.build(raw_opts)?;
            bv.extend(item_bv);
            match align {
                Some(align) => {
                    let remainder = bv.len() % align;
                    if remainder != 0 {
                        bv.resize(bv.len() + (align - remainder), false);
                    }
                }
                None => {}
            }
        }
        Ok(bv)
    }
}
