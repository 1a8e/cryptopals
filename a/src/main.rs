extern crate base64;
use std::u8;
use self::base64::{encode};

fn main() {
    let source_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let dest_b64 = hex_to_base64(source_hex.to_string());
    println!("{}", source_hex);
    println!("{}", dest_b64);
}

pub fn hex_to_base64(hex: String) -> String {
    let mut bytes = Vec::new();
    for i in 0..(hex.len()/2) {
        let res = u8::from_str_radix(&hex[2*i .. 2*i+2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Unable to decode hex: {}", e),
        };
    };
    encode(&bytes)
}