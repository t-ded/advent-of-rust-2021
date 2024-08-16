#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref HEX_FIELD_BINARY_DICT: HashMap<char, &'static str> = HashMap::from([
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]);
}

fn binary_string_from_hex_string(hex_string: &str) -> String {
    let mut binary = String::new();
    for hex_byte in hex_string.chars() {
        binary += HEX_FIELD_BINARY_DICT.get(&hex_byte).unwrap();
    }
    binary
}

#[derive(Debug)]
enum PacketContents {
    Number(u32),
    SubPackets(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    hex_string: String,
    binary_string: String,
    version: u8,
    type_id: u8,
    contents: PacketContents,
}

impl Packet {

    fn new() -> Packet {
        Packet {
            hex_string: "".to_string(),
            binary_string: "".to_string(),
            version: 0,
            type_id: 0,
            contents: PacketContents::Number(0),
        }
    }

    fn from_hex_string(hex_string: &str) -> Packet {

        let binary_string = binary_string_from_hex_string(hex_string);
        let version = u8::from_str_radix(&binary_string[..=2], 2).unwrap();
        let type_id = u8::from_str_radix(&binary_string[3..=5], 2).unwrap();
        match type_id {

            // Literal packet
            4 => {
                let mut literal = String::new();
                let mut last = false;
                for bit_group_start_index in (6..binary_string.len()).step_by(5) {
                    for (i, bit) in binary_string[bit_group_start_index..(bit_group_start_index + 5)].chars().enumerate() {
                        if i == 0 && bit == '0' {
                            last = true;
                        } else if i > 0 {
                            literal.push(bit);
                        }
                    }
                    if last { break }
                }
                let number = u32::from_str_radix(&literal, 2).unwrap();
                Packet {
                    hex_string: String::from(hex_string),
                    binary_string,
                    version,
                    type_id,
                    contents: PacketContents::Number(number),
                }
            },

            // Operator packet
            type_id => {
                let length_type_id = u8::from_str_radix(&binary_string[6..7], 2).unwrap();
                match length_type_id {
                    0 => {
                        let total_length_in_bits = u32::from_str_radix(&binary_string[7..22], 2).unwrap();
                    },
                    1 => {
                        let num_subpackets = u32::from_str_radix(&binary_string[7..18], 2).unwrap();
                    },
                    _ => {},
                }
                Packet::new()
            },
        }

    }
}


fn main() {
    println!("{}", binary_string_from_hex_string("D2FE28"));
    let packet = Packet::from_hex_string("D2FE28");
    println!("{:?}", packet);println!("{}", binary_string_from_hex_string("38006F45291200"));
    let packet = Packet::from_hex_string("38006F45291200");
    println!("{:?}", packet);
}
