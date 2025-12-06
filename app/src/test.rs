use crate::phi_field::base::*;
use bitvec::prelude::*;
use shua_struct::field::{BinaryField, Ctx, GetLen};
use shua_struct_macro::binary_struct;
use std::collections::HashMap;

#[derive(Clone, Debug)]
#[binary_struct]
struct TestHeader {
    v: u8,
}

#[derive(Clone, Debug)]
#[binary_struct]
struct TestStruct {
    h: TestHeader,
    a: u8,
    b: u16,
    c: u32,
    d: f32,
    e: bool,
    f: VarInt,
    g: PhiString,
    z: BitBool,
}

fn get_len(name: &str, ctx: &Ctx) -> u64 {
    if name == "data" {
        if let Some(len_any) = ctx.get("len") {
            if let Some(len) = len_any.downcast_ref::<u8>() {
                return *len as u64;
            }
        }
    }
    0
}

#[binary_struct]
#[derive(Clone, Debug)]
struct VecHeader {
    len: u8,
    #[binary_field(func=get_len)]
    data: Vec<u8>,
}

#[test]
fn test_vec_header_with_data() {
    let original = VecHeader {
        len: 3,
        data: vec![10, 20, 30],
    };

    let bits = original.build();

    let mut ctx = HashMap::new();

    let parsed = VecHeader::parse(&bits, &mut ctx, None, None).unwrap().0;

    assert_eq!(parsed.len, original.len);
    assert_eq!(parsed.data, original.data);
}

#[test]
fn test_vec_header_empty() {
    let original = VecHeader {
        len: 0,
        data: vec![],
    };

    let bits = original.build();
    let mut ctx = HashMap::new();

    let parsed = VecHeader::parse(&bits, &mut ctx, None, None).unwrap().0;

    assert_eq!(parsed.len, 0);
    assert!(parsed.data.is_empty());
}

#[test]
fn test_array() {
    let arr: [u8; 3] = [1, 2, 3];
    let bits = arr.build();
    let mut ctx = HashMap::new();

    let parsed = <[u8; 3]>::parse(&bits, &mut ctx, None, None).unwrap().0;

    assert_eq!(arr, parsed);
}

#[test]
fn test_struct_roundtrip() {
    let original = TestHeader { v: 99 };
    let bits = original.build();
    let mut ctx = HashMap::new();

    let parsed = TestHeader::parse(&bits, &mut ctx, None, None).unwrap().0;

    assert_eq!(original.v, parsed.v);
}

#[test]
fn test_nested_struct() {
    let original = TestStruct {
        h: TestHeader { v: 114 },
        a: 42,
        b: 0x1234,
        c: 0x87654321,
        d: 3.14,
        e: true,
        f: VarInt(200),
        g: "hi".into(),
        z: false.into(),
    };

    let bits = original.build();
    let mut ctx = HashMap::new();
    let parsed = TestStruct::parse(&bits, &mut ctx, None, None).unwrap().0;

    assert_eq!(parsed.h.v, original.h.v);
    assert_eq!(parsed.a, original.a);
    assert_eq!(parsed.b, original.b);
    assert_eq!(parsed.c, original.c);
    assert!((parsed.d - original.d).abs() < f32::EPSILON);
    assert_eq!(parsed.e, original.e);
    assert_eq!(parsed.f.0, original.f.0);
    assert_eq!(parsed.g.0, original.g.0);
    assert_eq!(parsed.z.0, original.z.0);
}
