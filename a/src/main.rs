extern crate base64;

use std::u8;

fn main() {
    let source_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let destination_b64 = hex_to_base64(source_hex.to_string());
    println!("{}", source_hex);
    println!("{}", destination_b64);

    let first = "1c0111001f010100061a024b53535009181c";
    let second = "686974207468652062756c6c277320657965";

    let first_bytes = hex_to_bytes(first.to_string());
    let second_bytes = hex_to_bytes(second.to_string());
    let xor_result = exclusive_or(first_bytes, second_bytes);
    println!("{:x?}", xor_result);
}

// pub fn bytes_to_hex(bytes: Vec<u8>) -> String {
//
// }

pub fn hex_to_base64(hex: String) -> String {
    base64::encode(hex_to_bytes(hex))
}

pub fn hex_to_bytes(hex: String) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    for i in 0..(hex.len() / 2) {
        let res = u8::from_str_radix(&hex[2 * i .. 2 * i + 2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Unable to decode hex: {}", e),
        };
    };
    bytes
}

pub fn exclusive_or(first: Vec<u8>, second: Vec<u8>) -> Vec<u8> {
    let mut xor_result: Vec<u8> = Vec::new();
    if first.len() == second.len() {
        let iter = first.iter().zip(second);
        for pair in iter {
            xor_result.push(pair.0 ^ pair.1)
        }
    };
    xor_result
}