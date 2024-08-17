#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fs;

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
        if let Some(bin) = HEX_FIELD_BINARY_DICT.get(&hex_byte) {
            binary += bin;
        }
    }
    binary
}

#[allow(dead_code)]
#[derive(Debug)]
enum PacketContents {
    Number(u128),
    SubPackets(Vec<Packet>),
}

#[allow(dead_code)]
#[derive(Debug)]
struct Packet {
    // hex_string: String,
    binary_string: String,
    version: u128,
    type_id: u128,
    contents: PacketContents,
    version_sum: u128,
}

impl Packet {

    #[allow(dead_code)]
    fn new() -> Packet {
        Packet {
            // hex_string: "".to_string(),
            binary_string: "".to_string(),
            version: 0,
            type_id: 0,
            contents: PacketContents::Number(0),
            version_sum: 0,
        }
    }

    fn from_hex_string(hex_string: &str) -> Packet {
        let binary_string = binary_string_from_hex_string(hex_string);
        Self::from_binary_string(&binary_string)
    }

    fn from_binary_string(binary_string: &str) -> Packet {

        let version = u128::from_str_radix(&binary_string[..=2], 2).unwrap();
        let type_id = u128::from_str_radix(&binary_string[3..=5], 2).unwrap();
        let mut version_sum = version;
        match type_id {

            // Literal packet
            4 => {
                let mut literal = String::new();
                let mut last = false;
                let mut last_idx = 0;
                for bit_group_start_index in (6..binary_string.len()).step_by(5) {
                    for (i, bit) in binary_string[bit_group_start_index..(bit_group_start_index + 5)].chars().enumerate() {
                        if i == 0 && bit == '0' {
                            last = true;
                            last_idx = bit_group_start_index + 5;
                        } else if i > 0 {
                            literal.push(bit);
                        }
                    }
                    if last { break }
                }
                let number = u128::from_str_radix(&literal, 2).unwrap();
                Packet {
                    // hex_string: String::from(hex_string),
                    binary_string: binary_string[..last_idx].to_string(),
                    version,
                    type_id,
                    contents: PacketContents::Number(number),
                    version_sum,
                }
            },

            // Operator packet
            type_id => {
                let length_type_id = u128::from_str_radix(&binary_string[6..7], 2).unwrap();
                let mut last_bit = 0;
                let mut subpackets = Vec::new();
                match length_type_id {
                    0 => {
                        let total_length_in_bits = u128::from_str_radix(&binary_string[7..22], 2).unwrap();
                        let mut length_in_bits = 0;
                        while length_in_bits < total_length_in_bits {
                            let subpacket = Packet::from_binary_string(&binary_string[((22 + length_in_bits) as usize)..]);
                            length_in_bits += subpacket.binary_string.len() as u128;
                            version_sum += subpacket.version_sum;
                            subpackets.push(subpacket);
                        }
                        last_bit = 22 + total_length_in_bits
                    },
                    1 => {
                        let num_subpackets = u128::from_str_radix(&binary_string[7..18], 2).unwrap();
                        let mut length_in_bits = 0;
                        while (subpackets.len() as u128) < num_subpackets {
                            let subpacket = Packet::from_binary_string(&binary_string[((18 + length_in_bits) as usize)..]);
                            length_in_bits += subpacket.binary_string.len() as u128;
                            version_sum += subpacket.version_sum;
                            subpackets.push(subpacket);
                        }
                        last_bit = 18 + length_in_bits
                    },
                    _ => {},
                }
                Packet {
                    binary_string: binary_string[..(last_bit as usize)].to_string(),
                    version,
                    type_id,
                    contents: PacketContents::SubPackets(subpackets),
                    version_sum,
                }
            },
        }
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").expect("Cannot read input.txt");
    let packet = Packet::from_hex_string(&input);
    println!("{:?}", packet.version_sum);
}

#[cfg(test)]
mod tests {
    use crate::Packet;

    #[test]
    fn version_sums() {
        let packet = Packet::from_hex_string("8A004A801A8002F478");
        println!("\n\n\n{:?}", packet);
        assert_eq!(packet.version_sum, 16);

        let packet = Packet::from_hex_string("620080001611562C8802118E34");
        assert_eq!(packet.version_sum, 12);

        let packet = Packet::from_hex_string("C0015000016115A2E0802F182340");
        assert_eq!(packet.version_sum, 23);

        let packet = Packet::from_hex_string("A0016C880162017C3686B18A3D4780");
        assert_eq!(packet.version_sum, 31);
    }
}