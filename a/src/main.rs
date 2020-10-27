extern crate base64;

// use std::str;
use std::u8;

// static ENGLISH_LETTER_FREQUENCIES: [[f32; 2]; 26] = [
//     [8.2, 7.8],  // "a"
//     [1.5, 2.0],  // "b"
//     [2.8, 4.0],  // "c"
//     [4.3, 3.8],  // "d"
//     [13.0, 11.0],  // "e"
//     [2.2, 1.4],  // "f"
//     [2.0, 3.0],  // "g"
//     [6.1, 2.3],  // "h"
//     [7.0, 8.6],  // "i"
//     [0.15, 0.21],  // "j"
//     [0.77, 0.97],  // "k"
//     [4.0, 5.3],  // "l"
//     [2.4, 2.7],  // "m"
//     [6.7, 7.2],  // "n"
//     [7.5, 6.1],  // "o"
//     [1.9, 2.8],  // "p"
//     [0.095, 0.19],  // "q"
//     [6.0, 7.3],  // "r"
//     [6.3, 8.7],  // "s"
//     [9.1, 6.7],  // "t"
//     [2.8, 3.3],  // "u"
//     [0.98, 1.0],  // "v"
//     [2.4, 0.91],  // "w"
//     [0.15, 0.27],  // "x"
//     [2.0, 1.6],  // "y"
//     [0.074, 0.44]  // "z"
// ];


fn main() {
    // one();
    // two();
    three();
}


pub fn one() {
    let source_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let destination_b64 = hex_to_base64(source_hex.to_string());
    println!("{}", source_hex);
    println!("{}", destination_b64);
}

pub fn two() {
    let first = "1c0111001f010100061a024b53535009181c";
    let second = "686974207468652062756c6c277320657965";

    let first_bytes = hex_to_bytes(first.to_string());
    let second_bytes = hex_to_bytes(second.to_string());
    let xor_result = exclusive_or(&first_bytes, &second_bytes);
    println!("{:x?}", xor_result);
}


pub fn three() {
    let c_text = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let c_bytes = hex_to_bytes(c_text.to_string());
    let c_len = c_bytes.len();
    println!("{:?}", c_bytes);
    let start = 'a' as u8;
    for offset in 0..26 {
        let k: u8 = start + offset;
        print!("{}: ", k);
        let k_vec = vec![k; c_len];
        let xor_result = exclusive_or(&c_bytes, &k_vec);
        println!("{:?}", score_p_text(&xor_result));
        // let res = str::from_utf8(&xor_result);
        // match res {
        //     Ok(v) => println!("{:?}", score_p_text(&v)),
        //     Err(e) => println!("Unable to decode xor result: {}", e),
        // };
    }
}

pub fn score_p_text(p_text_bytes: &Vec<u8>) -> Vec<f32> {
    let len_p_text = p_text_bytes.len();
    let mut count_vec = vec![0; 26];
    let base = 'a' as u8;
    for b in p_text_bytes {
        count_vec[(b - base) as usize] += 1;
    };
    let mut frequency_vec: Vec<f32> = Vec::new();
    for count in count_vec {
        let res = count as f32 * 100.0 / len_p_text as f32;
        frequency_vec.push(res);
    };
    frequency_vec
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
        let res = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Unable to decode hex: {}", e),
        };
    };
    bytes
}

pub fn exclusive_or(first: &Vec<u8>, second: &Vec<u8>) -> Vec<u8> {
    let mut xor_result: Vec<u8> = Vec::new();
    if first.len() == second.len() {
        let iter = first.iter().zip(second);
        for pair in iter {
            xor_result.push(pair.0 ^ pair.1)
        }
    };
    xor_result
}