#![allow(dead_code)]
use serde::Serialize;
use serde_dwarf;
use serde_json;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::env;
use std::io;

#[derive(Serialize, Copy, Clone)]
struct Foo<'a> {
    bar: u32,
    baz: &'a str,
}

#[derive(Serialize, Copy, Clone)]
struct MyUnitStruct;

#[derive(Serialize, Copy, Clone)]
struct MyTupleStruct(u8);

#[derive(Serialize, Copy, Clone)]
#[repr(C)]
enum MyCLikeEnum {
    A = 1,
    B = 2,
    C = 3,
}

#[derive(Serialize, Copy, Clone)]
enum MyRustEnum {
    UnitVariant,
    TupleVariant(u8),
    StructVariant { a: u8 },
}

macro_rules! serialize {
        ($serializer:ident, $($var:ident => $val:expr),*) => {
            $(
                let $var = $val;
                $var.serialize(&mut $serializer).unwrap();
            )*
        }
    }

fn main() {
    let mut serializer = serde_json::Serializer::new(io::sink());

    serialize!(serializer,
               my_u8 => 1u8,
           my_foo => Foo{bar: 1, baz: "test"},
           my_array => [1,2,3],
           my_slice => &my_array[0..1],
           my_foo_array => [Foo{bar: 1, baz: "test"} ; 5],
           my_foo_slice => &my_foo_array[0..1],
           my_unit_struct => MyUnitStruct,
           my_tuple_struct => MyTupleStruct(42),
           my_clike_enum => MyCLikeEnum::A,
           my_rustlike_enum => MyRustEnum::UnitVariant,
           my_option => Some(42u8),
           my_vec => Vec::<u8>::new(),
           my_hashmap => HashMap::<u8,u8>::new(),
           my_btreemap => BTreeMap::<u8,u8>::new(),
           my_btreeset => BTreeSet::<u8>::new(),
           my_hashset => HashSet::<u8>::new(),
           my_boxed_myunitstruct => Box::new(MyUnitStruct),
           my_string => String::from("test_string"),
           my_tuple => (1,2,3)
    );

    let my_exe = env::current_exe().unwrap();
    let debuginfo = serde_dwarf::DebugInfoBuilder::new()
        .filter_sym_list(&["my_u8"])
        .parse_path(my_exe)
        .unwrap();
    println!("{:?}", debuginfo.types());
    assert_eq!(
        type_name_of_val(&my_u8),
        debuginfo.symtype("my_u8").unwrap()
    )
}

pub fn type_name_of_val<T: ?Sized>(_val: &T) -> &'static str {
    std::any::type_name::<T>()
}
