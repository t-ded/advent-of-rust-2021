#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref HEX_FIELD_BINARY_DICT: HashMap<HexField, &'static str> = HashMap::from([
        (HexField::IntField(0), "0000"),
        (HexField::IntField(1), "0001"),
        (HexField::IntField(2), "0010"),
        (HexField::IntField(3), "0011"),
        (HexField::IntField(4), "0100"),
        (HexField::IntField(5), "0101"),
        (HexField::IntField(6), "0110"),
        (HexField::IntField(7), "0111"),
        (HexField::IntField(8), "1000"),
        (HexField::IntField(9), "1001"),
        (HexField::CharField('A'), "1010"),
        (HexField::CharField('B'), "1011"),
        (HexField::CharField('C'), "1100"),
        (HexField::CharField('D'), "1101"),
        (HexField::CharField('E'), "1110"),
        (HexField::CharField('F'), "1111"),
    ]);
}

#[derive(Hash, Eq, PartialEq)]
enum HexField {
    CharField(char),
    IntField(u8),
}

impl HexField {
    fn translate(&self) -> &str {
        HEX_FIELD_BINARY_DICT.get(&self).unwrap()
    }
}

struct Packet {
    hex_string: String,
    version: Option<u8>,
    type_id: Option<u8>,
    numbers: Option<Vec<u8>>,
}

impl Packet {
    
}


fn main() {
    println!("Hello, world!");
}
