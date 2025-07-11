// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

///// For test during implementation //////
/*
use cryptocol::symmetric::DES;
use cryptocol::number::{ IntUnion };

pub fn main()
{
    des_permutate_initially_finally();
    des_permutate_expansion();
    des_split();
    des_make_round_keys();
    des_slice_indices_combine();
    des_f();
}

trait TestDes
{
    fn get_block(&self) -> u64;
    fn set_block(&mut self, block: u64);
    fn permutate_initially(&mut self);
    fn permutate_finally(&mut self);
    fn expand(&self, right: u32) -> u64;
    fn compress_into_56bits(&self) -> u64;
    fn split(&self) -> (IntUnion, IntUnion);
    fn make_round_keys(&mut self);
    fn get_round_key(&self, round: usize) -> u64;
    fn slice_indices(&self, indices: u64, array: &mut [usize; 8]);
    fn combine(&self, collector: &mut u32, piece: u32);
    fn f(&mut self, round: usize, right: u32) -> u32;
}

impl TestDes for DES
{
    fn get_block(&self) -> u64          { self.test_get_block() }
    fn set_block(&mut self, block: u64) { self.test_set_block(block); }
    fn permutate_initially(&mut self)   { self.test_permutate_initially(); }
    fn permutate_finally(&mut self)     { self.test_permutate_finally(); }
    fn expand(&self, right: u32) -> u64     { self.test_expand(right) }
    fn compress_into_56bits(&self) -> u64   { self.test_compress_into_56bits() }
    fn split(&self) -> (IntUnion, IntUnion)     { self.test_split() }
    fn make_round_keys(&mut self)    { self.test_make_round_keys(); }
    fn get_round_key(&self, round: usize) -> u64  { self.test_get_round_key(round) }
    fn slice_indices(&self, indices: u64, array: &mut [usize; 8])   { self.test_slice_indices(indices, array) }
    fn combine(&self, collector: &mut u32, piece: u32) { self.test_combine(collector, piece); }
    fn f(&mut self, round: usize, right: u32) -> u32   { self.test_f(round, right) }
}


fn des_permutate_initially_finally()
{
    println!("des_permutate_initially_finally");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::number::LongUnion;
    use cryptocol::symmetric::DES;

    let mut a_des = DES::new();
    let block = (1_u64 << (8-2)) | (1_u64 << ((50-1) / 8 * 8 + (7 - (50-1) % 8)));
    a_des.set_block(block);
    a_des.permutate_initially();
    let out = a_des.get_block();
    let bu = LongUnion::new_with(block);
    print!("block =\t");
    for i in 0..8
        { print!("{:08b} ", bu.get_ubyte_(i)); }
    println!();
    let mut txt = String::new();
    for i in 0..8
        { write!(txt, "{:08b} ", bu.get_ubyte_(i)); }
    assert_eq!(txt, "01000000 00000000 00000000 00000000 00000000 00000000 01000000 00000000 ");

    let ou = LongUnion::new_with(out);
    print!("out =\t");
    for i in 0..8
        { print!("{:08b} ", ou.get_ubyte_(i)); }
    println!();
    let mut txt = String::new();
    for i in 0..8
        { write!(txt, "{:08b} ", ou.get_ubyte_(i)); }
    assert_eq!(txt, "01000001 00000000 00000000 00000000 00000000 00000000 00000000 00000000 ");

    a_des.permutate_finally();
    let back = a_des.get_block();
    let cu = LongUnion::new_with(back);
    print!("back =\t");
    for i in 0..8
        { print!("{:08b} ", cu.get_ubyte_(i)); }
    println!();
    let mut txt = String::new();
    for i in 0..8
        { write!(txt, "{:08b} ", cu.get_ubyte_(i)); }
    assert_eq!(txt, "01000000 00000000 00000000 00000000 00000000 00000000 01000000 00000000 ");
    println!("-------------------------------");
}

fn des_permutate_expansion()
{
    println!("des_permutate_expansion");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::number::{ IntUnion, LongUnion };
    use cryptocol::symmetric::DES;
    
    let mut right = IntUnion::new();
    let mut i = 0;
    for val in [0b_1111_0000_u8, 0b_1010_1010, 0b_1111_0000, 0b_1010_1010]
    {
        right.set_ubyte_(i, val);
        i += 1;
    }
    print!("right =\t");
    for i in 0..4
        { print!("{:08b} ", right.get_ubyte_(i)); }
    println!();

    let a_des = DES::new();
    let out = a_des.expand(right.get());

    let ou = LongUnion::new_with(out);
    print!("out =\t");
    for i in 0..6
        { print!("{:08b} ", ou.get_ubyte_(i)); }
    println!();
    let mut txt = String::new();
    for i in 0..6
        { write!(txt, "{:08b} ", ou.get_ubyte_(i)); }
    assert_eq!(txt, "01111010 00010101 01010101 01111010 00010101 01010101 ");
    println!("-------------------------------");
}

fn des_split()
{
    println!("des_split");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::number::LongUnion;
    use cryptocol::symmetric::DES;
    
    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for i in 0..8
        { print!("{:08b} ", key[i]); }
    println!();
    let mut txt = String::new();
    for i in 0..8
        { write!(txt, "{:08b} ", key[i]); }
    assert_eq!(txt, "00010011 00110100 01010111 01111001 10011011 10111100 11011111 11110001 ");

    let a_des = DES::new_with_key(key.clone());
    let key_56bit = LongUnion::new_with(a_des.compress_into_56bits());
    print!("K+ =\t");
    for i in 0..7
        { print!("{:08b} ", key_56bit.get_ubyte_(i)); }
    println!();
    let mut txt = String::new();
    for i in 0..7
        { write!(txt, "{:08b} ", key_56bit.get_ubyte_(i)); }
    assert_eq!(txt, "11110000 11001100 10101010 11110101 01010110 01100111 10001111 ");

    let a_des = DES::new_with_key(key.clone());
    let (left, right) = a_des.split();
    print!("L =\t");
    for i in 0..4
        { print!("{:08b} ", left.get_ubyte_(i)); }
    println!();
    let mut txt = String::new();
    for i in 0..4
        { write!(txt, "{:08b} ", left.get_ubyte_(i)); }
    assert_eq!(txt, "11110000 11001100 10101010 11110000 ");

    print!("R =\t");
    for i in 0..4
        { print!("{:08b} ", right.get_ubyte_(i)); }
    println!();
    let mut txt = String::new();
    for i in 0..4
        { write!(txt, "{:08b} ", right.get_ubyte_(i)); }
    assert_eq!(txt, "01010101 01100110 01111000 11110000 ");
    println!("-------------------------------");
}

fn des_make_round_keys()
{
    println!("des_make_round_keys");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::number::LongUnion;
    use cryptocol::symmetric::DES;

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for i in 0..8
        { print!("{:08b} ", key[i]); }
    println!();

    let a_des = DES::new_with_key(key);
    for i in 0..16
    {
        let round_key = LongUnion::new_with(a_des.get_round_key(i));
        print!("K({}) =\t", i);
        for j in 0..6
            { print!("{:08b} ", round_key.get_ubyte_(j)); }
        println!();
    }

    let round_key = LongUnion::new_with(a_des.get_round_key(0));
    let mut txt = String::new();
    for j in 0..6
        { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
    assert_eq!(txt, "00011011 00000010 11101111 11111100 01110000 01110010 ");

    let round_key = LongUnion::new_with(a_des.get_round_key(1));
    let mut txt = String::new();
    for j in 0..6
        { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
    assert_eq!(txt, "01111001 10101110 11011001 11011011 11001001 11100101 ");

    let round_key = LongUnion::new_with(a_des.get_round_key(2));
    let mut txt = String::new();
    for j in 0..6
        { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
    assert_eq!(txt, "01010101 11111100 10001010 01000010 11001111 10011001 ");

    let round_key = LongUnion::new_with(a_des.get_round_key(3));
    let mut txt = String::new();
    for j in 0..6
        { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
    assert_eq!(txt, "01110010 10101101 11010110 11011011 00110101 00011101 ");

    let round_key = LongUnion::new_with(a_des.get_round_key(4));
    let mut txt = String::new();
    for j in 0..6
        { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
    assert_eq!(txt, "01111100 11101100 00000111 11101011 01010011 10101000 ");

    let round_key = LongUnion::new_with(a_des.get_round_key(5));
    let mut txt = String::new();
    for j in 0..6
        { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
    assert_eq!(txt, "01100011 10100101 00111110 01010000 01111011 00101111 ");

    let round_key = LongUnion::new_with(a_des.get_round_key(6));
    let mut txt = String::new();
    for j in 0..6
        { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
    assert_eq!(txt, "11101100 10000100 10110111 11110110 00011000 10111100 ");

    let round_key = LongUnion::new_with(a_des.get_round_key(7));
    let mut txt = String::new();
    for j in 0..6
        { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
    assert_eq!(txt, "11110111 10001010 00111010 11000001 00111011 11111011 ");

    let round_key = LongUnion::new_with(a_des.get_round_key(8));
    let mut txt = String::new();
    for j in 0..6
        { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
    assert_eq!(txt, "11100000 11011011 11101011 11101101 11100111 10000001 ");

    let round_key = LongUnion::new_with(a_des.get_round_key(9));
    let mut txt = String::new();
    for j in 0..6
        { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
    assert_eq!(txt, "10110001 11110011 01000111 10111010 01000110 01001111 ");

    let round_key = LongUnion::new_with(a_des.get_round_key(10));
    let mut txt = String::new();
    for j in 0..6
        { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
    assert_eq!(txt, "00100001 01011111 11010011 11011110 11010011 10000110 ");

    let round_key = LongUnion::new_with(a_des.get_round_key(11));
    let mut txt = String::new();
    for j in 0..6
        { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
    assert_eq!(txt, "01110101 01110001 11110101 10010100 01100111 11101001 ");

    let round_key = LongUnion::new_with(a_des.get_round_key(12));
    let mut txt = String::new();
    for j in 0..6
        { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
    assert_eq!(txt, "10010111 11000101 11010001 11111010 10111010 01000001 ");

    let round_key = LongUnion::new_with(a_des.get_round_key(13));
    let mut txt = String::new();
    for j in 0..6
        { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
    assert_eq!(txt, "01011111 01000011 10110111 11110010 11100111 00111010 ");

    let round_key = LongUnion::new_with(a_des.get_round_key(14));
    let mut txt = String::new();
    for j in 0..6
        { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
    assert_eq!(txt, "10111111 10010001 10001101 00111101 00111111 00001010 ");

    let round_key = LongUnion::new_with(a_des.get_round_key(15));
    let mut txt = String::new();
    for j in 0..6
        { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
    assert_eq!(txt, "11001011 00111101 10001011 00001110 00010111 11110101 ");
    println!("-------------------------------");
}

fn des_slice_indices_combine()
{
    println!("des_slice_indices_combine");
    use cryptocol::number::LongUnion;
    use cryptocol::symmetric::DES;

    let a_des = DES::new();
    let mut indices = LongUnion::new();
    indices.set_ubyte_(0, 0b_111111_00);
    indices.set_ubyte_(1, 0b_0000_1010);
    indices.set_ubyte_(2, 0b_10_100100);
    indices.set_ubyte_(3, 0b_010101_00);
    indices.set_ubyte_(4, 0b_1001_1101);
    indices.set_ubyte_(5, 0b_10_011011);

    let mut index = [0_usize; 8];
    a_des.slice_indices(indices.get(), &mut index);
    for i in 0..8
        { println!("idx({}) = {:06b}", i, index[i]); }
    assert_eq!(index[0], 0b111111);
    assert_eq!(index[1], 0b000000);
    assert_eq!(index[2], 0b101010);
    assert_eq!(index[3], 0b100100);
    assert_eq!(index[4], 0b010101);
    assert_eq!(index[5], 0b001001);
    assert_eq!(index[6], 0b110110);
    assert_eq!(index[7], 0b011011);

    let mut collector = 0_u32;
    let small = [0b1111_u32, 0b0101, 0b1000, 0b0111, 0b1100, 0b0011, 0b0001, 0b1001];
    let piece = [(small[0] << 4) | small[1], (small[2] << 4) | small[3],
                            (small[4] << 4) | small[5], (small[6] << 4) | small[7]];
    for i in 0..8
        { println!("{:04b} ", small[i]); }
    
    a_des.combine(&mut collector, piece[0]);
    a_des.combine(&mut collector, piece[1]);
    a_des.combine(&mut collector, piece[2]);
    a_des.combine(&mut collector, piece[3]);
    let col = IntUnion::new_with(collector);
    for i in 0..4
        { println!("{:08b} ", col.get_ubyte_(i)); }
    assert_eq!(col.get_ubyte_(0), 0b11110101);
    assert_eq!(col.get_ubyte_(1), 0b10000111);
    assert_eq!(col.get_ubyte_(2), 0b11000011);
    assert_eq!(col.get_ubyte_(3), 0b00011001);
    println!("-------------------------------");
}

fn des_f()
{
    println!("des_f");
    use cryptocol::number::IntUnion;
    use cryptocol::symmetric::DES;

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for i in 0..8
        { print!("{:08b} ", key[i]); }
    println!();

    let mut right = IntUnion::new();
    right.set_ubyte_(0, 0b_1111_0000_u8);
    right.set_ubyte_(1, 0b_1010_1010_u8);
    right.set_ubyte_(2, 0b_1111_0000_u8);
    right.set_ubyte_(3, 0b_1010_1010_u8);
    print!("R =\t");
    for i in 0..4
        { print!("{:08b} ", right.get_ubyte_(i)); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let c = a_des.f(0, right.get());
    let cipher = IntUnion::new_with(c);

    print!("F =\t");
    for i in 0..4
        { print!("{:08b} ", cipher.get_ubyte_(i)); }
    println!();
    println!("-------------------------------");
}
*/

// use std::io::Write;

// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]
// #[allow(non_camel_case_types)]
// #[allow(dead_code)]


pub fn main()
{
    des_quick_start_main();
    des_basic_operation_main();
    des_encrypt_decrypt_u64_array_u64_main();
}

fn des_quick_start_main()
{
    des_quick_start_instantiation_with_key();
    des_quick_start_instantiation_without_key();
    des_quick_start_encryption_decryption_16_rounds();
    des_quick_start_encryption_decryption_256_rounds();
}

fn des_quick_start_instantiation_with_key()
{
    println!("des_quick_start_instantiation_with_key()");
    use cryptocol::symmetric::DES;

    let key = 0x_1234567890ABCDEF_u64;
    let mut _a_des = DES::new_with_key_u64(key);

    let key = [0xEFu8, 0xCDu8, 0xABu8, 0x90u8, 0x78u8, 0x56u8, 0x34u8, 0x12u8];
    let mut _a_des = DES::new_with_key(key);
    println!("-------------------------------");
}

fn des_quick_start_instantiation_without_key()
{
    println!("des_quick_start_instantiation_without_key()");
    use cryptocol::symmetric::DES;

    let mut a_des = DES::new();
    let key = 0x_1234567890ABCDEF_u64;
    a_des.set_key_u64(key);

    let mut a_des = DES::new();
    let key = [0xEFu8, 0xCDu8, 0xABu8, 0x90u8, 0x78u8, 0x56u8, 0x34u8, 0x12u8];
    a_des.set_key(key);
    println!("-------------------------------");
}

fn des_quick_start_encryption_decryption_16_rounds()
{
    println!("des_quick_start_encryption_decryption_16_rounds()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ CBC_PKCS7, DES };
    
    let mut a_des = DES::new_with_key([0xEFu8, 0xCDu8, 0xABu8, 0x90u8, 0x78u8, 0x56u8, 0x34u8, 0x12u8]);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =\t{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_str_into_vec(iv, message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 C5 9E EF 01 A9 4A FD 5B ");
    
    let mut recovered = String::new();
    a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B (16 rounds) =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn des_quick_start_encryption_decryption_256_rounds()
{
    println!("des_quick_start_encryption_decryption_256_rounds()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ CBC_PKCS7, DES_Expanded };

    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key([0xEFu8, 0xCDu8, 0xABu8, 0x90u8, 0x78u8, 0x56u8, 0x34u8, 0x12u8]);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =\t{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 C6 9D C6 BD A6 9B 58 8B ");

    let mut recovered = String::new();
    a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn des_basic_operation_main()
{
    des_new();
    des_new_with_key();
    des_new_with_key_u64();
    des_encryptor_with_key();
    des_encryptor_with_key_u64();
    des_decryptor_with_key();
    des_decryptor_with_key_u64();
    des_get_key();
    des_get_key_u64();
    des_set_key();
    des_set_key_u64();
    des_turn_inverse();
    des_turn_encryptor();
    des_turn_decryptor();
}

fn des_new()
{
    println!("des_new()");
    use cryptocol::symmetric::DES;

    let mut des = DES::new();   // The default key is 0x0000000000000000 which is a weak key.
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x1E32B46B44C69201_u64);

    let cipher_cipher_text = des.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);  // So, you can't use the default key!!!

    #[cfg(test)]
    des_compile_fail_new();
    println!("-------------------------------");
}

#[test]
fn des_compile_fail_new()
{
    use cryptocol::symmetric::DES;
    let des = DES::new();
    // It cannot be compiled!
    #[cfg(compile_fail)]    des.encrypt_u64(0x1E32B46B44C69201_u64);
}

fn des_new_with_key()
{
    println!("des_new_with_key()");
    use cryptocol::symmetric::DES;

    // Normal case
    let mut des = DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x3B6041D76AF28F23_u64);

    let cipher_cipher_text = des.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x7C5AAE491DC1310D_u64);
    assert_ne!(cipher_cipher_text, plaintext);
    println!();

    // Weak key case 1 for [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    // The key [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] is the same key as the key
    // [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01] because of parity bits.
    let mut des1 = DES::new_with_key([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    let mut des2 = DES::new_with_key([0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext1 = des1.encrypt_u64(plaintext);
    let ciphertext2 = des2.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext1:\t\t{:#016X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#016X}", ciphertext2);
    assert_eq!(ciphertext1, 0x1E32B46B44C69201_u64);
    assert_eq!(ciphertext2, 0x1E32B46B44C69201_u64);
    assert_eq!(ciphertext1, ciphertext2);

    let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    println!("Cipher-ciphertext1:\t{:#016X}\n", cipher_cipher_text1);
    println!("Cipher-ciphertext2:\t{:#016X}\n", cipher_cipher_text2);
    assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text1, plaintext);
    assert_eq!(cipher_cipher_text2, plaintext);
    println!();
    // So, you can't use the weak key [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    // and [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]!!!


    // Weak key case 2 for [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
    // The key [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF] is the same key as the key
    // [0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE] because of parity bits.
    let mut des1 = DES::new_with_key([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    let mut des2 = DES::new_with_key([0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE]);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext1 = des1.encrypt_u64(plaintext);
    let ciphertext2 = des2.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext1:\t\t{:#016X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#016X}", ciphertext2);
    assert_eq!(ciphertext1, 0xA5997AB38BC07250_u64);
    assert_eq!(ciphertext2, 0xA5997AB38BC07250_u64);
    assert_eq!(ciphertext1, ciphertext2);

    let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    println!("Cipher-ciphertext1:\t{:#016X}\n", cipher_cipher_text1);
    println!("Cipher-ciphertext2:\t{:#016X}\n", cipher_cipher_text2);
    assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text1, plaintext);
    assert_eq!(cipher_cipher_text2, plaintext);
    println!();
    // So, you can't use the weak key [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
    // and [0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE]!!!

    // Weak key case 3 for [0xE0, 0xE0, 0xE0, 0xE0, 0xF1, 0xF1, 0xF1, 0xF1]
    // The key [0xE0, 0xE0, 0xE0, 0xE0, 0xF1, 0xF1, 0xF1, 0xF1] is the same key as the key
    // [0xE1, 0xE1, 0xE1, 0xE1, 0xF0, 0xF0, 0xF0, 0xF0] because of parity bits.
    let mut des1 = DES::new_with_key([0xE0, 0xE0, 0xE0, 0xE0, 0xF1, 0xF1, 0xF1, 0xF1]);
    let mut des2 = DES::new_with_key([0xE1, 0xE1, 0xE1, 0xE1, 0xF0, 0xF0, 0xF0, 0xF0]);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext1 = des1.encrypt_u64(plaintext);
    let ciphertext2 = des2.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext1:\t\t{:#016X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#016X}", ciphertext2);
    assert_eq!(ciphertext1, 0x94CCA0201F033101_u64);
    assert_eq!(ciphertext2, 0x94CCA0201F033101_u64);
    assert_eq!(ciphertext1, ciphertext2);

    let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    println!("Cipher-ciphertext1:\t{:#016X}\n", cipher_cipher_text1);
    println!("Cipher-ciphertext2:\t{:#016X}\n", cipher_cipher_text2);
    assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text1, plaintext);
    assert_eq!(cipher_cipher_text2, plaintext);
    println!();
    // So, you can't use the weak key [0xE0, 0xE0, 0xE0, 0xE0, 0xF1, 0xF1, 0xF1, 0xF1]
    // and [0xE1, 0xE1, 0xE1, 0xE1, 0xE1, 0xE1, 0xE1, 0xE1]!!!

    // Weak key case 4 for [0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E]
    // The key [0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E] is the same key as the key
    // [0x1E, 0x1E, 0x1E, 0x1E, 0x0F, 0x0F, 0x0F, 0x0F] because of parity bits.
    let mut des1 = DES::new_with_key([0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E]);
    let mut des2 = DES::new_with_key([0x1E, 0x1E, 0x1E, 0x1E, 0x0F, 0x0F, 0x0F, 0x0F]);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext1 = des1.encrypt_u64(plaintext);
    let ciphertext2 = des2.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext1:\t\t{:#016X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#016X}", ciphertext2);
    assert_eq!(ciphertext1, 0x4FB6397B5352DB0C_u64);
    assert_eq!(ciphertext2, 0x4FB6397B5352DB0C_u64);
    assert_eq!(ciphertext1, ciphertext2);

    let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    println!("Cipher-ciphertext1:\t{:#016X}\n", cipher_cipher_text1);
    println!("Cipher-ciphertext2:\t{:#016X}\n", cipher_cipher_text2);
    assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text1, plaintext);
    assert_eq!(cipher_cipher_text2, plaintext);
    println!();
    // So, you can't use the weak key [0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E]
    // and [0x1E, 0x1E, 0x1E, 0x1E, 0x0F, 0x0F, 0x0F, 0x0F]!!!

    // Semi-Weak key case 1 for [0x01, 0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E] and [0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E, 0x01]
    let mut des1 = DES::new_with_key([0x01, 0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E]);
    let mut des2 = DES::new_with_key([0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E, 0x01]);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0xC2C71D736E97876C_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x063A6E55466423D2_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys [0x01, 0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E] and [0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E, 0x01]!!!

    // Semi-Weak key case 2 for [0x01, 0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1] and [0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1, 0x01]
    let mut des1 = DES::new_with_key([0x01, 0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1]);
    let mut des2 = DES::new_with_key([0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1, 0x01]);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x85A63690E79AAA15_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x15B721BBB44A12F5_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys [0x01, 0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1] and [0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1, 0x01]!!!

    // Semi-Weak key case 3 for [0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE] and [0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01]
    let mut des1 = DES::new_with_key([0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE]);
    let mut des2 = DES::new_with_key([0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01]);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0xAE38CC9D9FA48581_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x7EE95658A653960D_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys [0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE] and [0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01]!!!

    // Semi-Weak key case 4 for [0x1F, 0xE0, 0x1F, 0xE0, 0x0E, 0xF1, 0x0E, 0xF1] and [0xE0, 0x1F, 0xE0, 0x1F, 0xF1, 0x0E, 0xF1, 0x0E]
    let mut des1 = DES::new_with_key([0x1F, 0xE0, 0x1F, 0xE0, 0x0E, 0xF1, 0x0E, 0xF1]);
    let mut des2 = DES::new_with_key([0xE0, 0x1F, 0xE0, 0x1F, 0xF1, 0x0E, 0xF1, 0x0E]);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x81ECC05B173F793E_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x4D0AD4DC147E4BDF_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys [0x1F, 0xE0, 0x1F, 0xE0, 0x0E, 0xF1, 0x0E, 0xF1] and [0xE0, 0x1F, 0xE0, 0x1F, 0xF1, 0x0E, 0xF1, 0x0E]!!!

    // Semi-Weak key case 5 for [0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E, 0xFE] and [0xFE, 0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E]
    let mut des1 = DES::new_with_key([0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E, 0xFE]);
    let mut des2 = DES::new_with_key([0xFE, 0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E]);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x59735490F84A0AD0_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x79FD3CBFE57F4B0B_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys [0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E, 0xFE] and [0xFE, 0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E]!!!

    // Semi-Weak key case 6 for [0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1, 0xFE] and [0xFE, 0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1]
    let mut des1 = DES::new_with_key([0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1, 0xFE]);
    let mut des2 = DES::new_with_key([0xFE, 0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1]);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x27C83AAE29571889_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0xDE76DF630C033919_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    // So, you can't use the semi-weak keys [0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1, 0xFE] and [0xFE, 0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1]!!!

    #[cfg(test)]
    des_compile_fail_new_with_key();
    println!("-------------------------------");
}

#[test]
fn des_compile_fail_new_with_key()
{
    use cryptocol::symmetric::DES;
    let des = DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    // It cannot be compiled!
    #[cfg(compile_fail)]    des.encrypt_u64(0x1E32B46B44C69201_u64);
}

fn des_new_with_key_u64()
{
    println!("des_new_with_key_u64");
    use cryptocol::symmetric::DES;

    // Normal case
    let mut des = DES::new_with_key_u64(0xEFCDAB9078563412);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x3B6041D76AF28F23_u64);

    let cipher_cipher_text = des.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x7C5AAE491DC1310D_u64);
    assert_ne!(cipher_cipher_text, plaintext);
    println!();

    // Weak key case 1 for 0x0000000000000000
    // The key 0x0000000000000000 is the same key as the key 0x0101010101010101 because of parity bits.
    let mut des1 = DES::new_with_key_u64(0x0000000000000000);
    let mut des2 = DES::new_with_key_u64(0x0101010101010101);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext1 = des1.encrypt_u64(plaintext);
    let ciphertext2 = des2.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext1:\t\t{:#016X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#016X}", ciphertext2);
    assert_eq!(ciphertext1, 0x1E32B46B44C69201_u64);
    assert_eq!(ciphertext2, 0x1E32B46B44C69201_u64);
    assert_eq!(ciphertext1, ciphertext2);

    let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    println!("Cipher-ciphertext1:\t{:#016X}\n", cipher_cipher_text1);
    println!("Cipher-ciphertext2:\t{:#016X}\n", cipher_cipher_text2);
    assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text1, plaintext);
    assert_eq!(cipher_cipher_text2, plaintext);
    println!();
    // So, you can't use the weak key 0x0000000000000000 and 0x0101010101010101!!!

    // Weak key case 2 for 0xFFFFFFFFFFFFFFFF
    // The key 0xFFFFFFFFFFFFFFFF is the same key as the key 0xFEFEFEFEFEFEFEFE because of parity bits.
    let mut des1 = DES::new_with_key_u64(0xFFFFFFFFFFFFFFFF);
    let mut des2 = DES::new_with_key_u64(0xFEFEFEFEFEFEFEFE);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext1 = des1.encrypt_u64(plaintext);
    let ciphertext2 = des2.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext1:\t\t{:#016X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#016X}", ciphertext2);
    assert_eq!(ciphertext1, 0xA5997AB38BC07250_u64);
    assert_eq!(ciphertext2, 0xA5997AB38BC07250_u64);
    assert_eq!(ciphertext1, ciphertext2);

    let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    println!("Cipher-ciphertext1:\t{:#016X}\n", cipher_cipher_text1);
    println!("Cipher-ciphertext2:\t{:#016X}\n", cipher_cipher_text2);
    assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text1, plaintext);
    assert_eq!(cipher_cipher_text2, plaintext);
    println!();
    // So, you can't use the weak key 0xFFFFFFFFFFFFFFFF and 0xFEFEFEFEFEFEFEFE!!!

    // Weak key case 3 for 0xF1F1F1F1E0E0E0E0 in little-endianness
    // The key 0xF1F1F1F1E0E0E0E0 is the same key as the key 0xF0F0F0F0E1E1E1E1 because of parity bits.
    let mut des1 = DES::new_with_key_u64(0xF1F1F1F1E0E0E0E0);
    let mut des2 = DES::new_with_key_u64(0xF0F0F0F0E1E1E1E1);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext1 = des1.encrypt_u64(plaintext);
    let ciphertext2 = des2.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext1:\t\t{:#016X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#016X}", ciphertext2);
    assert_eq!(ciphertext1, 0x94CCA0201F033101_u64);
    assert_eq!(ciphertext2, 0x94CCA0201F033101_u64);
    assert_eq!(ciphertext1, ciphertext2);

    let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    println!("Cipher-ciphertext1:\t{:#016X}\n", cipher_cipher_text1);
    println!("Cipher-ciphertext2:\t{:#016X}\n", cipher_cipher_text2);
    assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text1, plaintext);
    assert_eq!(cipher_cipher_text2, plaintext);
    println!();
    // So, you can't use the weak key 0xF1F1F1F1E0E0E0E0 and 0xF0F0F0F0E1E1E1E1!!!

    // Weak key case 4 for 0x0E0E0E0E1F1F1F1F in little-endianness
    // The key 0x0E0E0E0E1F1F1F1F is the same key as the key 0x0F0F0F0F1E1E1E1E because of parity bits.
    let mut des1 = DES::new_with_key_u64(0x0E0E0E0E1F1F1F1F);
    let mut des2 = DES::new_with_key_u64(0x0F0F0F0F1E1E1E1E);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext1 = des1.encrypt_u64(plaintext);
    let ciphertext2 = des2.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext1:\t\t{:#016X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#016X}", ciphertext2);
    assert_eq!(ciphertext1, 0x4FB6397B5352DB0C_u64);
    assert_eq!(ciphertext2, 0x4FB6397B5352DB0C_u64);
    assert_eq!(ciphertext1, ciphertext2);

    let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    println!("Cipher-ciphertext1:\t{:#016X}\n", cipher_cipher_text1);
    println!("Cipher-ciphertext2:\t{:#016X}\n", cipher_cipher_text2);
    assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text1, plaintext);
    assert_eq!(cipher_cipher_text2, plaintext);
    println!();
    // So, you can't use the weak key 0x0E0E0E0E1F1F1F1F and 0x0F0F0F0F1E1E1E1E!!!

    // Semi-Weak key case 1 for 0x0E010E011F011F01 and 0x010E010E011F011F in little-endianness
    let mut des1 = DES::new_with_key_u64(0x0E010E011F011F01);
    let mut des2 = DES::new_with_key_u64(0x010E010E011F011F);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0xC2C71D736E97876C_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x063A6E55466423D2_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys 0x0E010E011F011F01 and 0x010E010E011F011F!!!

    // Semi-Weak key case 2 for 0xF101F101E001E001 and 0x01F101F101E001E0 in little-endianness
    let mut des1 = DES::new_with_key_u64(0xF101F101E001E001);
    let mut des2 = DES::new_with_key_u64(0x01F101F101E001E0);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x85A63690E79AAA15_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x15B721BBB44A12F5_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys 0xF101F101E001E001 and 0x01F101F101E001E0!!!

    // Semi-Weak key case 3 for 0xFE01FE01FE01FE01 and 0x01FE01FE01FE01FE in little-endianness
    let mut des1 = DES::new_with_key_u64(0xFE01FE01FE01FE01);
    let mut des2 = DES::new_with_key_u64(0x01FE01FE01FE01FE);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0xAE38CC9D9FA48581_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x7EE95658A653960D_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys 0xFE01FE01FE01FE01 and 0x01FE01FE01FE01FE!!!

    // Semi-Weak key case 4 for 0xF10EF10EE01FE01F and 0x0EF10EF11FE01FE0 in little-endianness
    let mut des1 = DES::new_with_key_u64(0xF10EF10EE01FE01F);
    let mut des2 = DES::new_with_key_u64(0x0EF10EF11FE01FE0);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x81ECC05B173F793E_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x4D0AD4DC147E4BDF_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys 0xF10EF10EE01FE01F and 0x0EF10EF11FE01FE0!!!

    // Semi-Weak key case 5 for 0xFE0EFE0EFE1FFE1F and 0x0EFE0EFE1FFE1FFE in little-endianness
    let mut des1 = DES::new_with_key_u64(0xFE0EFE0EFE1FFE1F);
    let mut des2 = DES::new_with_key_u64(0x0EFE0EFE1FFE1FFE);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x59735490F84A0AD0_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x79FD3CBFE57F4B0B_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys 0xFE0EFE0EFE1FFE1F and 0x0EFE0EFE1FFE1FFE!!!

    // Semi-Weak key case 6 for 0xFEF1FEF1FEE0FEE0 and 0xF1FEF1FEE0FEE0FE in little-endianness 
    let mut des1 = DES::new_with_key_u64(0xFEF1FEF1FEE0FEE0);
    let mut des2 = DES::new_with_key_u64(0xF1FEF1FEE0FEE0FE);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x27C83AAE29571889_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0xDE76DF630C033919_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    // So, you can't use the semi-weak keys 0xFEF1FEF1FEE0FEE0 and 0xF1FEF1FEE0FEE0FE!!!
    println!("-------------------------------");
}

fn des_encryptor_with_key()
{
    println!("des_encryptor_with_key");
    use cryptocol::symmetric::{ DES, BigCryptor64, SmallCryptor };
    
    let keys: [Box<dyn SmallCryptor<u64, 8>>; 3]
            = [ Box::new(DES::encryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])),
                Box::new(DES::decryptor_with_key([0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])),
                Box::new(DES::encryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])) ];
    let mut tdes = BigCryptor64::new_with_small_cryptor_array(keys);
    let plaintext = 0x_1234567890ABCDEF_u64;
    let ciphertext = tdes.encrypt_u64(plaintext);
    
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x272A2AC7B4E66748_u64);
    
    let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();

    // Operators
    let mut tdes = BigCryptor64::new()
                    + DES::encryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])
                    - DES::encryptor_with_key([0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])
                    + DES::encryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
    let plaintext = 0x_1234567890ABCDEF_u64;
    let ciphertext = tdes.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);

    let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!("-------------------------------");
}

fn des_encryptor_with_key_u64()
{
    println!("des_encryptor_with_key_u64");
    use cryptocol::symmetric::{ BigCryptor64, DES };

    let mut tdes = BigCryptor64::new_with_small_cryptor_array(
                [Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)),
                Box::new(DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)),
                Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64))]
    );
    let plaintext = 0x_1234567890ABCDEF_u64;
    let ciphertext = tdes.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);

    let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    

    // Operators
    let mut tdes = BigCryptor64::new()
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    - DES::encryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let plaintext = 0x_1234567890ABCDEF_u64;
    let ciphertext = tdes.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);

    let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!("-------------------------------");
}

fn des_decryptor_with_key()
{
    println!("des_decryptor_with_key_u64");
    use cryptocol::symmetric::{ DES, BigCryptor64, SmallCryptor };
    
    let keys: [Box<dyn SmallCryptor<u64, 8>>; 3]
            = [ Box::new(DES::encryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])),
                Box::new(DES::decryptor_with_key([0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])),
                Box::new(DES::encryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])) ];
    let mut tdes = BigCryptor64::new_with_small_cryptor_array(keys);
    let plaintext = 0x_1234567890ABCDEF_u64;
    let ciphertext = tdes.encrypt_u64(plaintext);
    
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x272A2AC7B4E66748_u64);
    
    let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();

    // Operators
    let mut tdes = BigCryptor64::new()
                    - DES::decryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])
                    - DES::encryptor_with_key([0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])
                    - DES::decryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
    let plaintext = 0x_1234567890ABCDEF_u64;
    let ciphertext = tdes.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);

    let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!("-------------------------------");
}

fn des_decryptor_with_key_u64()
{
    println!("des_decryptor_with_key_u64");
    use cryptocol::symmetric::{ BigCryptor64, DES };

    let mut tdes = BigCryptor64::new_with_small_cryptor_array(
                    [ Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)),
                                    Box::new(DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)),
                                    Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)) ] );
    let plaintext = 0x_1234567890ABCDEF_u64;
    let ciphertext = tdes.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x272A2AC7B4E66748_u64);

    let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    

    // Operators
    let mut tdes = BigCryptor64::new()
                    - DES::decryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    - DES::decryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let plaintext = 0x_1234567890ABCDEF_u64;
    let ciphertext = tdes.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);

    let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!("-------------------------------");
}

fn des_get_key()
{
    println!("des_get_key");
    use cryptocol::symmetric::DES;

    let mut des = DES::new();
    des.set_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let key = des.get_key();
    print!("K = ");
    for k in key
        { print!("{:#02X} ", k); }
    assert_eq!(key, [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
}

fn des_get_key_u64()
{
    println!("des_get_key_u64");
    use cryptocol::symmetric::DES;

    let mut des = DES::new();
    des.set_key_u64(0xEFCDAB9078563412);
    let key = des.get_key_u64();
    println!("Key = {}", key);
    assert_eq!(key, 0xEFCDAB9078563412_u64);
}

fn des_set_key()
{
    println!("des_set_key");
    use cryptocol::symmetric::DES;

    // Normal case
    let mut des = DES::new();
    des.set_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x3B6041D76AF28F23_u64);

    let cipher_cipher_text = des.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x7C5AAE491DC1310D_u64);
    assert_ne!(cipher_cipher_text, plaintext);
    println!();

    // Weak key case 1 for [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    // The key [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] is the same key as the key
    // [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01] because of parity bits.
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    des2.set_key([0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext1 = des1.encrypt_u64(plaintext);
    let ciphertext2 = des2.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext1:\t\t{:#016X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#016X}", ciphertext2);
    assert_eq!(ciphertext1, 0x1E32B46B44C69201_u64);
    assert_eq!(ciphertext2, 0x1E32B46B44C69201_u64);
    assert_eq!(ciphertext1, ciphertext2);

    let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    println!("Cipher-ciphertext1:\t{:#016X}\n", cipher_cipher_text1);
    println!("Cipher-ciphertext2:\t{:#016X}\n", cipher_cipher_text2);
    assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text1, plaintext);
    assert_eq!(cipher_cipher_text2, plaintext);
    println!();
    // So, you can't use the weak key [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    // and [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]!!!


    // Weak key case 2 for [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
    // The key [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF] is the same key as the key
    // [0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE] because of parity bits.
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    des2.set_key([0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE]);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext1 = des1.encrypt_u64(plaintext);
    let ciphertext2 = des2.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext1:\t\t{:#016X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#016X}", ciphertext2);
    assert_eq!(ciphertext1, 0xA5997AB38BC07250_u64);
    assert_eq!(ciphertext2, 0xA5997AB38BC07250_u64);
    assert_eq!(ciphertext1, ciphertext2);

    let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    println!("Cipher-ciphertext1:\t{:#016X}\n", cipher_cipher_text1);
    println!("Cipher-ciphertext2:\t{:#016X}\n", cipher_cipher_text2);
    assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text1, plaintext);
    assert_eq!(cipher_cipher_text2, plaintext);
    println!();
    // So, you can't use the weak key [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
    // and [0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE]!!!

    // Weak key case 3 for [0xE0, 0xE0, 0xE0, 0xE0, 0xF1, 0xF1, 0xF1, 0xF1]
    // The key [0xE0, 0xE0, 0xE0, 0xE0, 0xE0, 0xE0, 0xE0, 0xE0] is the same key as the key
    // [0xE1, 0xE1, 0xE1, 0xE1, 0xF0, 0xF0, 0xF0, 0xF0] because of parity bits.
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key([0xE0, 0xE0, 0xE0, 0xE0, 0xF1, 0xF1, 0xF1, 0xF1]);
    des2.set_key([0xE1, 0xE1, 0xE1, 0xE1, 0xF0, 0xF0, 0xF0, 0xF0]);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext1 = des1.encrypt_u64(plaintext);
    let ciphertext2 = des2.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext1:\t\t{:#016X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#016X}", ciphertext2);
    assert_eq!(ciphertext1, 0x94CCA0201F033101_u64);
    assert_eq!(ciphertext2, 0x94CCA0201F033101_u64);
    assert_eq!(ciphertext1, ciphertext2);

    let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    println!("Cipher-ciphertext1:\t{:#016X}\n", cipher_cipher_text1);
    println!("Cipher-ciphertext2:\t{:#016X}\n", cipher_cipher_text2);
    assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text1, plaintext);
    assert_eq!(cipher_cipher_text2, plaintext);
    println!();
    // So, you can't use the weak key [0xE0, 0xE0, 0xE0, 0xE0, 0xF1, 0xF1, 0xF1, 0xF1]
    // and [0xE1, 0xE1, 0xE1, 0xE1, 0xE1, 0xE1, 0xE1, 0xE1]!!!

    // Weak key case 4 for [0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E]
    // The key [0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E] is the same key as the key
    // [0x1E, 0x1E, 0x1E, 0x1E, 0x0F, 0x0F, 0x0F, 0x0F] because of parity bits.
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key([0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E]);
    des2.set_key([0x1E, 0x1E, 0x1E, 0x1E, 0x0F, 0x0F, 0x0F, 0x0F]);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext1 = des1.encrypt_u64(plaintext);
    let ciphertext2 = des2.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext1:\t\t{:#016X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#016X}", ciphertext2);
    assert_eq!(ciphertext1, 0x4FB6397B5352DB0C_u64);
    assert_eq!(ciphertext2, 0x4FB6397B5352DB0C_u64);
    assert_eq!(ciphertext1, ciphertext2);

    let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    println!("Cipher-ciphertext1:\t{:#016X}\n", cipher_cipher_text1);
    println!("Cipher-ciphertext2:\t{:#016X}\n", cipher_cipher_text2);
    assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text1, plaintext);
    assert_eq!(cipher_cipher_text2, plaintext);
    println!();
    // So, you can't use the weak key [0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E]
    // and [0x1E, 0x1E, 0x1E, 0x1E, 0x0F, 0x0F, 0x0F, 0x0F]!!!

    // Semi-Weak key case 1 for [0x01, 0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E] and [0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E, 0x01]
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key([0x01, 0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E]);
    des2.set_key([0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E, 0x01]);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0xC2C71D736E97876C_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x063A6E55466423D2_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys [0x01, 0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E]
    // and [0x1F, 0x01, 0x1F, 0x01, 0x0E, 0x01, 0x0E, 0x01]!!!

    // Semi-Weak key case 2 for [0x01, 0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1] and [0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1, 0x01]
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key([0x01, 0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1]);
    des2.set_key([0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1, 0x01]);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x85A63690E79AAA15_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x15B721BBB44A12F5_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys [0x01, 0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1]
    // and [0xE0, 0x01, 0xE0, 0x01, 0xF1, 0x01, 0xF1, 0x01]!!!

    // Semi-Weak key case 3 for [0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE] and [0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01]
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key([0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE]);
    des2.set_key([0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01]);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0xAE38CC9D9FA48581_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x7EE95658A653960D_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys [0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE]
    // and [0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01]!!!

    // Semi-Weak key case 4 for [0x1F, 0xE0, 0x1F, 0xE0, 0x0E, 0xF1, 0x0E, 0xF1] and [0xE0, 0x1F, 0xE0, 0x1F, 0xF1, 0x0E, 0xF1, 0x0E]
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key([0x1F, 0xE0, 0x1F, 0xE0, 0x0E, 0xF1, 0x0E, 0xF1]);
    des2.set_key([0xE0, 0x1F, 0xE0, 0x1F, 0xF1, 0x0E, 0xF1, 0x0E]);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x81ECC05B173F793E_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x4D0AD4DC147E4BDF_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys [0x1F, 0xE0, 0x1F, 0xE0, 0x0E, 0xF1, 0x0E, 0xF1]
    // and [0xE0, 0x1F, 0xE0, 0x1F, 0xF1, 0x0E, 0xF1, 0x0E]!!!

    // Semi-Weak key case 5 for [0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E, 0xFE] and [0xFE, 0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E]
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key([0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E, 0xFE]);
    des2.set_key([0xFE, 0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E]);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x59735490F84A0AD0_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x79FD3CBFE57F4B0B_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys [0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E, 0xFE]
    // and [0xFE, 0x1F, 0xFE, 0x1F, 0xFE, 0x0E, 0xFE, 0x0E]!!!

    // Semi-Weak key case 6 for [0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1, 0xFE] and [0xFE, 0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1]
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key([0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1, 0xFE]);
    des2.set_key([0xFE, 0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1]);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x27C83AAE29571889_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0xDE76DF630C033919_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    // So, you can't use the semi-weak keys [0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1, 0xFE]
    // and [0xFE, 0xE0, 0xFE, 0xE0, 0xFE, 0xF1, 0xFE, 0xF1]!!!
    println!("-------------------------------");
}

fn des_set_key_u64()
{
    println!("des_set_key_u64");
    use cryptocol::symmetric::DES;

    // Normal case
    let mut des = DES::new();
    des.set_key_u64(0xEFCDAB9078563412);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x3B6041D76AF28F23_u64);

    let cipher_cipher_text = des.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x7C5AAE491DC1310D_u64);
    assert_ne!(cipher_cipher_text, plaintext);
    println!();

    // Weak key case 1 for 0x0000000000000000
    // The key 0x0000000000000000 is the same key as the key 0x0101010101010101 because of parity bits.
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key_u64(0x0000000000000000);
    des2.set_key_u64(0x0101010101010101);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext1 = des1.encrypt_u64(plaintext);
    let ciphertext2 = des2.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext1:\t\t{:#016X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#016X}", ciphertext2);
    assert_eq!(ciphertext1, 0x1E32B46B44C69201_u64);
    assert_eq!(ciphertext2, 0x1E32B46B44C69201_u64);
    assert_eq!(ciphertext1, ciphertext2);

    let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    println!("Cipher-ciphertext1:\t{:#016X}\n", cipher_cipher_text1);
    println!("Cipher-ciphertext2:\t{:#016X}\n", cipher_cipher_text2);
    assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text1, plaintext);
    assert_eq!(cipher_cipher_text2, plaintext);
    println!();
    // So, you can't use the weak key 0x0000000000000000 and 0x0101010101010101!!!

    // Weak key case 2 for 0xFFFFFFFFFFFFFFFF
    // The key 0xFFFFFFFFFFFFFFFF is the same key as the key 0xFEFEFEFEFEFEFEFE because of parity bits.
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key_u64(0xFFFFFFFFFFFFFFFF);
    des2.set_key_u64(0xFEFEFEFEFEFEFEFE);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext1 = des1.encrypt_u64(plaintext);
    let ciphertext2 = des2.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext1:\t\t{:#016X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#016X}", ciphertext2);
    assert_eq!(ciphertext1, 0xA5997AB38BC07250_u64);
    assert_eq!(ciphertext2, 0xA5997AB38BC07250_u64);
    assert_eq!(ciphertext1, ciphertext2);

    let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    println!("Cipher-ciphertext1:\t{:#016X}\n", cipher_cipher_text1);
    println!("Cipher-ciphertext2:\t{:#016X}\n", cipher_cipher_text2);
    assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text1, plaintext);
    assert_eq!(cipher_cipher_text2, plaintext);
    println!();
    // So, you can't use the weak key 0xFFFFFFFFFFFFFFFF and 0xFEFEFEFEFEFEFEFE!!!

    // Weak key case 3 for 0xF1F1F1F1E0E0E0E0 in little-endianness
    // The key 0xF1F1F1F1E0E0E0E0 is the same key as the key 0xF0F0F0F0E1E1E1E1 because of parity bits.
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key_u64(0xF1F1F1F1E0E0E0E0);
    des2.set_key_u64(0xF0F0F0F0E1E1E1E1);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext1 = des1.encrypt_u64(plaintext);
    let ciphertext2 = des2.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext1:\t\t{:#016X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#016X}", ciphertext2);
    assert_eq!(ciphertext1, 0x94CCA0201F033101_u64);
    assert_eq!(ciphertext2, 0x94CCA0201F033101_u64);
    assert_eq!(ciphertext1, ciphertext2);

    let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    println!("Cipher-ciphertext1:\t{:#016X}\n", cipher_cipher_text1);
    println!("Cipher-ciphertext2:\t{:#016X}\n", cipher_cipher_text2);
    assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text1, plaintext);
    assert_eq!(cipher_cipher_text2, plaintext);
    println!();
    // So, you can't use the weak key 0xF1F1F1F1E0E0E0E0 and 0xF0F0F0F0E1E1E1E1!!!

    // Weak key case 4 for 0x0E0E0E0E1F1F1F1F in little-endianness
    // The key 0x0E0E0E0E1F1F1F1F is the same key as the key 0x0F0F0F0F1E1E1E1E because of parity bits.
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key_u64(0x0E0E0E0E1F1F1F1F);
    des2.set_key_u64(0x0F0F0F0F1E1E1E1E);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext1 = des1.encrypt_u64(plaintext);
    let ciphertext2 = des2.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext1:\t\t{:#016X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#016X}", ciphertext2);
    assert_eq!(ciphertext1, 0x4FB6397B5352DB0C_u64);
    assert_eq!(ciphertext2, 0x4FB6397B5352DB0C_u64);
    assert_eq!(ciphertext1, ciphertext2);

    let cipher_cipher_text1 = des1.encrypt_u64(ciphertext1);
    let cipher_cipher_text2 = des2.encrypt_u64(ciphertext2);
    println!("Cipher-ciphertext1:\t{:#016X}\n", cipher_cipher_text1);
    println!("Cipher-ciphertext2:\t{:#016X}\n", cipher_cipher_text2);
    assert_eq!(cipher_cipher_text1, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text2, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text1, plaintext);
    assert_eq!(cipher_cipher_text2, plaintext);
    println!();
    // So, you can't use the weak key 0x0E0E0E0E1F1F1F1F and 0x0F0F0F0F1E1E1E1E!!!

    // Semi-Weak key case 1 for 0x0E010E011F011F01 and 0x010E010E011F011F in little-endianness
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key_u64(0x0E010E011F011F01);
    des2.set_key_u64(0x010E010E011F011F);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0xC2C71D736E97876C_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x063A6E55466423D2_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys 0x0E010E011F011F01 and 0x010E010E011F011F!!!

    // Semi-Weak key case 2 for 0xF101F101E001E001 and 0x01F101F101E001E0 in little-endianness
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key_u64(0xF101F101E001E001);
    des2.set_key_u64(0x01F101F101E001E0);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x85A63690E79AAA15_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x15B721BBB44A12F5_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys 0xF101F101E001E001 and 0x01F101F101E001E0!!!

    // Semi-Weak key case 3 for 0xFE01FE01FE01FE01 and 0x01FE01FE01FE01FE in little-endianness
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key_u64(0xFE01FE01FE01FE01);
    des2.set_key_u64(0x01FE01FE01FE01FE);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0xAE38CC9D9FA48581_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x7EE95658A653960D_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys 0xFE01FE01FE01FE01 and 0x01FE01FE01FE01FE!!!

    // Semi-Weak key case 4 for 0xF10EF10EE01FE01F and 0x0EF10EF11FE01FE0 in little-endianness
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key_u64(0xF10EF10EE01FE01F);
    des2.set_key_u64(0x0EF10EF11FE01FE0);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x81ECC05B173F793E_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x4D0AD4DC147E4BDF_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys 0xF10EF10EE01FE01F and 0x0EF10EF11FE01FE0!!!

    // Semi-Weak key case 5 for 0xFE0EFE0EFE1FFE1F and 0x0EFE0EFE1FFE1FFE in little-endianness
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key_u64(0xFE0EFE0EFE1FFE1F);
    des2.set_key_u64(0x0EFE0EFE1FFE1FFE);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x59735490F84A0AD0_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x79FD3CBFE57F4B0B_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    // So, you can't use the semi-weak keys 0xFE0EFE0EFE1FFE1F and 0x0EFE0EFE1FFE1FFE!!!

    // Semi-Weak key case 6 for 0xFEF1FEF1FEE0FEE0 and 0xF1FEF1FEE0FEE0FE in little-endianness 
    let mut des1 = DES::new();
    let mut des2 = DES::new();
    des1.set_key_u64(0xFEF1FEF1FEE0FEE0);
    des2.set_key_u64(0xF1FEF1FEE0FEE0FE);

    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = des1.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x27C83AAE29571889_u64);

    let cipher_cipher_text = des2.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);

    let ciphertext = des2.encrypt_u64(plaintext);
    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0xDE76DF630C033919_u64);

    let cipher_cipher_text = des1.encrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}\n", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    // So, you can't use the semi-weak keys 0xFEF1FEF1FEE0FEE0 and 0xF1FEF1FEE0FEE0FE!!!
    println!("-------------------------------");
}

fn des_turn_inverse()
{
    println!("des_turn_inverse");
    use cryptocol::symmetric::{ BigCryptor64, DES, SmallCryptor };

    let mut keys: [Box<dyn SmallCryptor<u64, 8>>; 3]
                = [ Box::new(DES::new_with_key_u64(0x_1234567890ABCDEF_u64)),
                    Box::new(DES::new_with_key_u64(0x_FEDCBA0987654321_u64)),
                    Box::new(DES::new_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    keys[1].turn_inverse();

    let mut tdes = BigCryptor64::new_with_small_cryptor_array(keys);
    let plaintext = 0x_1234567890ABCDEF_u64;
    let ciphertext = tdes.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);

    let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    

    // Operators
    let des1 = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    let mut des2 = DES::new_with_key_u64(0x_FEDCBA0987654321_u64);
    let des3 = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    des2.turn_inverse();

    let mut tdes = BigCryptor64::new() + des1 + des2 + des3; 
    let plaintext = 0x_1234567890ABCDEF_u64;
    let ciphertext = tdes.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);

    let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!("-------------------------------");
}

fn des_turn_encryptor()
{
    println!("des_turn_encryptor");
    use cryptocol::symmetric::{ BigCryptor64, DES, SmallCryptor };

    let mut keys: [Box<dyn SmallCryptor<u64, 8>>; 3]
            = [ Box::new(DES::new_with_key_u64(0x_1234567890ABCDEF_u64)),
                Box::new(DES::new_with_key_u64(0x_FEDCBA0987654321_u64)),
                Box::new(DES::new_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    keys[0].turn_encryptor();

    let mut tdes = BigCryptor64::new_with_small_cryptor_array(keys);
    let plaintext = 0x_1234567890ABCDEF_u64;
    let ciphertext = tdes.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x_CDAC175F3B7EAA2B_u64);

    let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    

    // Operators
    let mut des1 = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    let des2 = DES::new_with_key_u64(0x_FEDCBA0987654321_u64);
    let des3 = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    des1.turn_encryptor();

    let mut tdes = BigCryptor64::new() + des1 + des2 + des3; 
    let plaintext = 0x_1234567890ABCDEF_u64;
    let ciphertext = tdes.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x_CDAC175F3B7EAA2B_u64);

    let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!("-------------------------------");
}

fn des_turn_decryptor()
{
    println!("des_turn_decryptor");
    use cryptocol::symmetric::{ BigCryptor64, DES, SmallCryptor };

    let mut keys: [Box<dyn SmallCryptor<u64, 8>>; 3]
                = [ Box::new(DES::new_with_key_u64(0x_1234567890ABCDEF_u64)),
                    Box::new(DES::new_with_key_u64(0x_FEDCBA0987654321_u64)),
                    Box::new(DES::new_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    keys[1].turn_decryptor();

    let mut tdes = BigCryptor64::new_with_small_cryptor_array(keys);
    let plaintext = 0x_1234567890ABCDEF_u64;
    let ciphertext = tdes.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);

    let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    

    // Operators
    let des1 = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    let mut des2 = DES::new_with_key_u64(0x_FEDCBA0987654321_u64);
    let des3 = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    des2.turn_decryptor();

    let mut tdes = BigCryptor64::new() + des1 + des2 + des3; 
    let plaintext = 0x_1234567890ABCDEF_u64;
    let ciphertext = tdes.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u64);

    let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);
    println!("-------------------------------");
}

fn des_encrypt_decrypt_u64_array_u64_main()
{
    des_encrypt_u64();
    des_decrypt_u64();
    des__encrypt();
    des__decrypt();
    des_encrypt_array_u64();
    des_decrypt_array_u64();
    des_is_successful();
    des_is_failed();
    des_set_successful();
    des_set_failed();
    des_has_weak_key();
}

fn des_encrypt_u64()
{
    println!("des_encrypt_u64");
    use cryptocol::symmetric::{ DES, DES_Expanded };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    
    let message = 0x_1234567890ABCDEF_u64;
    println!("M_u64 =\t{:#016X}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let cipher = a_des.encrypt_u64(message);
    println!("C_u64 (16 rounds) =\t{:#016X}", cipher);
    assert_eq!(cipher, 0x_1BC4896735BBE206_u64);
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    
    let message = 0x_1234567890ABCDEF_u64;
    println!("M_u64 =\t{:#016X}", message);

    let mut b_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    let cipher = b_des.encrypt_u64(message);
    println!("C_u64 (128 rounds) =\t{:#016X}", cipher);
    assert_eq!(cipher, 0x_21F25F81CE4D4AA3_u64);
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    println!("K1 =\t{:#016x}", key1);
    
    let message = 0x_1234567890ABCDEF_u64;
    println!("M_u64 =\t{:#016X}", message);

    let cipher1 = c_des.encrypt_u64(message);
    let cipher2 = d_des.encrypt_u64(message);
    println!("C_u64 (0 rounds) =\t{:#016X}", cipher1);
    assert_eq!(cipher1, 0x_2138A9B46057CEDF_u64);

    println!("D_u64 (0 rounds) =\t{:#016X}", cipher);
    assert_eq!(cipher2, 0x_2138A9B46057CEDF_u64);
    assert_eq!(cipher1, cipher2);
    println!("-------------------------------");
}

fn des_decrypt_u64()
{
    println!("des_decrypt_u64");
    use cryptocol::symmetric::{ DES, DES_Expanded };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    
    let message = 0x_1234567890ABCDEF_u64;
    println!("M_u64 =\t{:#016X}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let cipher = a_des.encrypt_u64(message);
    println!("C_u64 (16 rounds) =\t{:#016X}", cipher);
    assert_eq!(cipher, 0x_1BC4896735BBE206_u64);

    let recovered = a_des.decrypt_u64(cipher);
    println!("B_u64 (16 rounds) =\t{:#016X}", recovered);
    assert_eq!(recovered, 0x_1234567890ABCDEF_u64);
    assert_eq!(recovered, message);
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    
    let message = 0x_1234567890ABCDEF_u64;
    println!("M_u64 =\t{:#016X}", message);

    let mut b_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    let cipher = b_des.encrypt_u64(message);
    println!("C_u64 (128 rounds) =\t{:#016X}", cipher);
    assert_eq!(cipher, 0x_21F25F81CE4D4AA3_u64);

    let recovered = b_des.decrypt_u64(cipher);
    println!("B_u64 (16 rounds) =\t{:#016X}", recovered);
    assert_eq!(recovered, 0x_1234567890ABCDEF_u64);
    assert_eq!(recovered, message);
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    println!("K =\t{:#016X}", key);
    
    let message = 0x_1234567890ABCDEF_u64;
    println!("M_u64 =\t{:#016X}", message);

    let cipher1 = c_des.encrypt_u64(message);
    let cipher2 = d_des.encrypt_u64(message);
    println!("C_u64 (0 rounds) =\t{:#016X}", cipher1);
    assert_eq!(cipher1, 0x_2138A9B46057CEDF_u64);

    println!("D_u64 (0 rounds) =\t{:#016X}", cipher);
    assert_eq!(cipher2, 0x_2138A9B46057CEDF_u64);
    assert_eq!(cipher1, cipher2);

    let recovered1 = c_des.decrypt_u64(cipher1);
    let recovered2 = d_des.decrypt_u64(cipher2);
    println!("B1_u64 (0 rounds) =\t{:#016X}", recovered1);
    println!("B2_u64 (0 rounds) =\t{:#016X}", recovered2);
    assert_eq!(recovered1, 0x_1234567890ABCDEF_u64);
    assert_eq!(recovered1, message);
    assert_eq!(recovered2, 0x_1234567890ABCDEF_u64);
    assert_eq!(recovered2, message);
    assert_eq!(recovered1, recovered2);
    println!("-------------------------------");
}

fn des__encrypt()
{
    println!("des__encrypt");
    use cryptocol::symmetric::{ DES, DES_Expanded };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    
    let message = 0x_1234567890ABCDEF_u64;
    println!("M_u64 =\t{:#016X}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let cipher = a_des._encrypt(message);
    println!("C_u64 (16 rounds) =\t{:#016X}", cipher);
    assert_eq!(cipher, 0x_1BC4896735BBE206_u64);
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    
    let message = 0x_1234567890ABCDEF_u64;
    println!("M_u64 =\t{:#016X}", message);

    let mut b_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    let cipher = b_des._encrypt(message);
    println!("C_u64 (128 rounds) =\t{:#016X}", cipher);
    assert_eq!(cipher, 0x_21F25F81CE4D4AA3_u64);
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    println!("K1 =\t{:#016x}", key1);
    
    let message = 0x_1234567890ABCDEF_u64;
    println!("M_u64 =\t{:#016X}", message);

    let cipher1 = c_des._encrypt(message);
    let cipher2 = d_des._encrypt(message);
    println!("C_u64 (0 rounds) =\t{:#016X}", cipher1);
    assert_eq!(cipher1, 0x_2138A9B46057CEDF_u64);

    println!("D_u64 (0 rounds) =\t{:#016X}", cipher);
    assert_eq!(cipher2, 0x_2138A9B46057CEDF_u64);
    assert_eq!(cipher1, cipher2);
    println!("-------------------------------");
}

fn des__decrypt()
{
    println!("des__decrypt");
    use cryptocol::symmetric::{ DES, DES_Expanded };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    
    let message = 0x_1234567890ABCDEF_u64;
    println!("M_u64 =\t{:#016X}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let cipher = a_des._encrypt(message);
    println!("C_u64 (16 rounds) =\t{:#016X}", cipher);
    assert_eq!(cipher, 0x_1BC4896735BBE206_u64);

    let recovered = a_des._decrypt(cipher);
    println!("B_u64 (16 rounds) =\t{:#016X}", recovered);
    assert_eq!(recovered, 0x_1234567890ABCDEF_u64);
    assert_eq!(recovered, message);
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    
    let message = 0x_1234567890ABCDEF_u64;
    println!("M_u64 =\t{:#016X}", message);

    let mut b_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    let cipher = b_des._encrypt(message);
    println!("C_u64 (128 rounds) =\t{:#016X}", cipher);
    assert_eq!(cipher, 0x_21F25F81CE4D4AA3_u64);

    let recovered = b_des._decrypt(cipher);
    println!("B_u64 (16 rounds) =\t{:#016X}", recovered);
    assert_eq!(recovered, 0x_1234567890ABCDEF_u64);
    assert_eq!(recovered, message);
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    println!("K =\t{:#016X}", key);
    
    let message = 0x_1234567890ABCDEF_u64;
    println!("M_u64 =\t{:#016X}", message);

    let cipher1 = c_des._encrypt(message);
    let cipher2 = d_des._encrypt(message);
    println!("C_u64 (0 rounds) =\t{:#016X}", cipher1);
    assert_eq!(cipher1, 0x_2138A9B46057CEDF_u64);

    println!("D_u64 (0 rounds) =\t{:#016X}", cipher);
    assert_eq!(cipher2, 0x_2138A9B46057CEDF_u64);
    assert_eq!(cipher1, cipher2);

    let recovered1 = c_des._decrypt(cipher1);
    let recovered2 = d_des._decrypt(cipher2);
    println!("B1_u64 (0 rounds) =\t{:#016X}", recovered1);
    println!("B2_u64 (0 rounds) =\t{:#016X}", recovered2);
    assert_eq!(recovered1, 0x_1234567890ABCDEF_u64);
    assert_eq!(recovered1, message);
    assert_eq!(recovered2, 0x_1234567890ABCDEF_u64);
    assert_eq!(recovered2, message);
    assert_eq!(recovered1, recovered2);
    println!("-------------------------------");
}

fn des_encrypt_array_u64()
{
    println!("des_encrypt_array_u64");
    use cryptocol::symmetric::{ DES, DES_Expanded };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    
    let message = [0x_1234567890ABCDEF_u64, 0xEFCDAB9078563412, 0xFEDCBA0987654321 ];
    print!("M =\t");
    for m in message
        { print!("{:#016X} ", m); }
    println!();
    let mut a_des = DES::new_with_key_u64(key);

    let mut cipher = [0; 3];
    a_des.encrypt_array_u64(&message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher
        { print!("{:#016X} ", c); }
    println!();
    assert_eq!(cipher[0], 0x_1BC4896735BBE206_u64);
    assert_eq!(cipher[1], 0x_1D8A61E5E62226A4_u64);
    assert_eq!(cipher[2], 0x_2990D69525C17067_u64);
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    
    let message = [0x_1234567890ABCDEF_u64, 0xEFCDAB9078563412, 0xFEDCBA0987654321 ];
    print!("M =\t");
    for m in message
        { print!("{:#016X} ", m); }
    println!();
    let mut b_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let mut cipher = [0; 3];
    b_des.encrypt_array_u64(&message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher
        { print!("{:#016X} ", c); }
    println!();
    assert_eq!(cipher[0], 0x_21F25F81CE4D4AA3_u64);
    assert_eq!(cipher[1], 0x_352F391A1482A504_u64);
    assert_eq!(cipher[2], 0x_F793546957AFDE50_u64);
    println!();
    

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    println!("K =\t{:#016X}", key);

    let message = [0x_1234567890ABCDEF_u64, 0xEFCDAB9078563412, 0xFEDCBA0987654321 ];
    print!("M =\t");
    for m in message
        { print!("{:#016X} ", m); }
    println!();

    let mut cipher1 = [0; 3];
    let mut cipher2 = [0; 3];
    c_des.encrypt_array_u64(&message, &mut cipher1);
    d_des.encrypt_array_u64(&message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1
        { print!("{:#016X} ", c); }
    println!();
    print!("D (0 rounds) =\t");
    for c in cipher2
        { print!("{:#016X} ", c); }
    println!();
    assert_eq!(cipher1[0], 0x_2138A9B46057CEDF_u64);
    assert_eq!(cipher1[1], 0x_DFCE5760B4A93821_u64);
    assert_eq!(cipher1[2], 0x_FDEC75064B9A8312_u64);
    assert_eq!(cipher2[0], 0x_2138A9B46057CEDF_u64);
    assert_eq!(cipher2[1], 0x_DFCE5760B4A93821_u64);
    assert_eq!(cipher2[2], 0x_FDEC75064B9A8312_u64);
    assert_eq!(cipher1[0], cipher2[0]);
    assert_eq!(cipher1[1], cipher2[1]);
    assert_eq!(cipher1[2], cipher2[2]);
    println!("-------------------------------");
}

fn des_decrypt_array_u64()
{
    println!("des_decrypt_array_u64");
    use cryptocol::symmetric::{ DES, DES_Expanded };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    
    let message = [0x_1234567890ABCDEF_u64, 0xEFCDAB9078563412, 0xFEDCBA0987654321 ];
    print!("M =\t");
    for m in message
        { print!("{:#016X} ", m); }
    println!();
    let mut a_des = DES::new_with_key_u64(key);

    let mut cipher = [0; 3];
    a_des.encrypt_array_u64(&message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher
        { print!("{:#016X} ", c); }
    println!();
    assert_eq!(cipher[0], 0x_1BC4896735BBE206_u64);
    assert_eq!(cipher[1], 0x_1D8A61E5E62226A4_u64);
    assert_eq!(cipher[2], 0x_2990D69525C17067_u64);

    let mut recovered = [0; 3];
    a_des.decrypt_array_u64(&cipher, &mut recovered);
    print!("B (16 rounds) =\t");
    for r in recovered
        { print!("{:#016X} ", r); }
    println!();
    assert_eq!(recovered[0], 0x_1234567890ABCDEF_u64);
    assert_eq!(recovered[1], 0x_EFCDAB9078563412_u64);
    assert_eq!(recovered[2], 0x_FEDCBA0987654321_u64);
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    
    let message = [0x_1234567890ABCDEF_u64, 0xEFCDAB9078563412, 0xFEDCBA0987654321 ];
    print!("M =\t");
    for m in message
        { print!("{:#016X} ", m); }
    println!();
    let mut b_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let mut cipher = [0; 3];
    b_des.encrypt_array_u64(&message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher
        { print!("{:#016X} ", c); }
    println!();
    assert_eq!(cipher[0], 0x_21F25F81CE4D4AA3_u64);
    assert_eq!(cipher[1], 0x_352F391A1482A504_u64);
    assert_eq!(cipher[2], 0x_F793546957AFDE50_u64);

    let mut recovered = [0; 3];
    b_des.decrypt_array_u64(&cipher, &mut recovered);
    print!("B (128 rounds) =\t");
    for r in recovered
        { print!("{:#016X} ", r); }
    println!();
    assert_eq!(recovered[0], 0x_1234567890ABCDEF_u64);
    assert_eq!(recovered[1], 0x_EFCDAB9078563412_u64);
    assert_eq!(recovered[2], 0x_FEDCBA0987654321_u64);

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    println!("K =\t{:#016X}", key);

    let message = [0x_1234567890ABCDEF_u64, 0xEFCDAB9078563412, 0xFEDCBA0987654321 ];
    print!("M =\t");
    for m in message
        { print!("{:#016X} ", m); }
    println!();

    let mut cipher1 = [0; 3];
    let mut cipher2 = [0; 3];
    c_des.encrypt_array_u64(&message, &mut cipher1);
    d_des.encrypt_array_u64(&message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1
        { print!("{:#016X} ", c); }
    println!();
    print!("D (0 rounds) =\t");
    for c in cipher2
        { print!("{:#016X} ", c); }
    println!();
    assert_eq!(cipher1[0], 0x_2138A9B46057CEDF_u64);
    assert_eq!(cipher1[1], 0x_DFCE5760B4A93821_u64);
    assert_eq!(cipher1[2], 0x_FDEC75064B9A8312_u64);
    assert_eq!(cipher2[0], 0x_2138A9B46057CEDF_u64);
    assert_eq!(cipher2[1], 0x_DFCE5760B4A93821_u64);
    assert_eq!(cipher2[2], 0x_FDEC75064B9A8312_u64);
    assert_eq!(cipher1[0], cipher2[0]);
    assert_eq!(cipher1[1], cipher2[1]);
    assert_eq!(cipher1[2], cipher2[2]);

    let mut recovered1 = [0; 3];
    let mut recovered2 = [0; 3];
    c_des.decrypt_array_u64(&cipher1, &mut recovered1);
    d_des.decrypt_array_u64(&cipher2, &mut recovered2);
    print!("B1 (0 rounds) =\t");
    for r in recovered1
        { print!("{:#016X} ", r); }
    println!();
    print!("B2 (0 rounds) =\t");
    for r in recovered2
        { print!("{:#016X} ", r); }
    println!();
    assert_eq!(recovered1[0], 0x_1234567890ABCDEF_u64);
    assert_eq!(recovered1[1], 0x_EFCDAB9078563412_u64);
    assert_eq!(recovered1[2], 0x_FEDCBA0987654321_u64);
    assert_eq!(recovered2[0], 0x_1234567890ABCDEF_u64);
    assert_eq!(recovered2[1], 0x_EFCDAB9078563412_u64);
    assert_eq!(recovered2[2], 0x_FEDCBA0987654321_u64);
    assert_eq!(recovered1[0], recovered2[0]);
    assert_eq!(recovered1[1], recovered2[1]);
    assert_eq!(recovered1[2], recovered2[2]);
    println!("-------------------------------");
}

fn des_is_successful()
{
    println!("des_is_successful");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::DES;

    {
        // Normal case for the message of 0 bytes
        let key = 0x_1234567890ABCDEF_u64;
        println!("K =\t{:#016X}", key);
        let mut a_des = DES::new_with_key_u64(key);
        use cryptocol::symmetric::ECB_PKCS7;
        let message = "";
        println!("M =\t{}", message);
        let mut cipher = [0_u8; 8];
        let len = a_des.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
        println!("The length of ciphertext = {}", len);
        assert_eq!(len, 8);
        let success = a_des.is_successful();
        assert_eq!(success, true);
        print!("C =\t");
        for c in cipher.clone()
            { print!("{:02X} ", c); }
        println!();
        let mut txt = String::new();
        for c in cipher.clone()
            { write!(txt, "{:02X} ", c); }
        assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");
    
    
        // Normal case for the original message of 0 bytes
        let key = 0x_1234567890ABCDEF_u64;
        println!("K =\t{:#016X}", key);
        let mut a_des = DES::new_with_key_u64(key);
    
        let cipher = [0x41u8, 0x7F, 0x89, 0x79, 0x08, 0xCD, 0xA1, 0x4C];
        print!("C =\t");
        for c in cipher.clone()
            { print!("{:02X} ", c); }
        println!();
        let mut recovered = [0u8; 8];
        let len = a_des.decrypt_array_into_array(&cipher, &mut recovered);
        println!("The length of plaintext = {}", len);
        assert_eq!(len, 0);
        let success = a_des.is_successful();
        assert_eq!(success, true);
        print!("Ba =\t");
        for b in recovered.clone()
            { print!("{:02X} ", b); }
        println!();
        let mut txt = String::new();
        for c in recovered.clone()
            { write!(txt, "{:02X} ", c); }
        assert_eq!(txt, "00 00 00 00 00 00 00 00 ");
    
        let mut converted = String::new();
        unsafe { converted.as_mut_vec() }.write(&recovered);
        unsafe { converted.as_mut_vec() }.truncate(len as usize);
        println!("Bb =\t{}", converted);
        assert_eq!(converted, "");
        assert_eq!(converted, message);
        println!();
    }

    {
        // Failed case for encryption
        use cryptocol::symmetric::CFB;
        
        let key = 0x_1234567890ABCDEF_u64;
        println!("K =\t{:#016X}", key);
        let mut a_des = DES::new_with_key_u64(key);

        let message = "In the beginning God created the heavens and the earth.";
        println!("M =\t{}", message);
        let iv = 0x_FEDCBA0987654321_u64;
        println!("IV =	{}", iv);
        let mut cipher = [0_u8; 40];
        let len = a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
        println!("The length of ciphertext = {}", len);
        assert_eq!(len, 0);
        let success = a_des.is_successful();
        assert_eq!(success, false);

        // Failed case for decryption
        let key = 0x_1234567890ABCDEF_u64;
        println!("K =\t{:#016X}", key);
        let mut a_des = DES::new_with_key_u64(key);

        let cipher = [0x2Eu8, 0x1E, 0xE1, 0x51, 0xFD, 0xB3, 0xB0, 0x4B, 0x79, 0x3A, 0xA1, 0x78, 0xEC, 0xCD, 0x02, 0x72, 0x6A, 0xC4, 0x41, 0x7C, 0x25, 0xA4, 0x2C, 0x07, 0xFC, 0x77, 0x25, 0x49, 0x12, 0x55, 0x0F, 0x8A, 0xED, 0x44, 0xC3, 0xE4, 0xDC, 0x91, 0x69, 0x0F, 0x40, 0x72, 0x7F, 0xF2, 0xD9, 0xB7, 0x54, 0x9F, 0x36, 0x91, 0xC5, 0x85, 0x4F, 0x9B, 0x30];
        print!("C =\t");
        for c in cipher.clone()
            { print!("{:02X} ", c); }
        println!();
        let mut recovered = [0u8; 40];
        let len = a_des.decrypt_array_into_array(iv, &cipher, &mut recovered);
        println!("The length of plaintext = {}", len);
        assert_eq!(len, 0);
        let success = a_des.is_successful();
        assert_eq!(success, false);
    }
    println!("-------------------------------");
}

fn des_is_failed()
{
    println!("des_is_failed");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::DES;

    {
        use cryptocol::symmetric::ECB_PKCS7;

        // Normal case for the message of 0 bytes
        let key = 0x_1234567890ABCDEF_u64;
        println!("K =\t{:#016X}", key);
        let mut a_des = DES::new_with_key_u64(key);
    
        let message = "";
        println!("M =\t{}", message);
        let mut cipher = [0_u8; 8];
        let len = a_des.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
        println!("The length of ciphertext = {}", len);
        assert_eq!(len, 8);
        let failure = a_des.is_failed();
        assert_eq!(failure, false);
        print!("C =\t");
        for c in cipher.clone()
            { print!("{:02X} ", c); }
        println!();
        let mut txt = String::new();
        for c in cipher.clone()
            { write!(txt, "{:02X} ", c); }
        assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");
    
    
        // Normal case for the original message of 0 bytes
        let cipher = [0x41u8, 0x7F, 0x89, 0x79, 0x08, 0xCD, 0xA1, 0x4C];
        print!("C =\t");
        for c in cipher.clone()
            { print!("{:02X} ", c); }
        let mut recovered = [0u8; 8];
        let len = a_des.decrypt_array_into_array(&cipher, &mut recovered);
        println!("The length of plaintext = {}", len);
        assert_eq!(len, 0);
        let failure = a_des.is_failed();
        assert_eq!(failure, false);
        print!("Ba =\t");
        for b in recovered.clone()
            { print!("{:02X} ", b); }
        println!();
        let mut txt = String::new();
        for c in recovered.clone()
            { write!(txt, "{:02X} ", c); }
        assert_eq!(txt, "00 00 00 00 00 00 00 00 ");
    
        let mut converted = String::new();
        unsafe { converted.as_mut_vec() }.write(&recovered);
        unsafe { converted.as_mut_vec() }.truncate(len as usize);
        println!("Bb =\t{}", converted);
        assert_eq!(converted, "");
        assert_eq!(converted, message);
        println!();
    }

    {
        // Failed case for encryption
        use cryptocol::symmetric::CFB;
    
        let key = 0x_1234567890ABCDEF_u64;
        println!("K =\t{:#016X}", key);
        let mut a_des = DES::new_with_key_u64(key);
    
        let message = "In the beginning God created the heavens and the earth.";
        println!("M =\t{}", message);
        let iv = 0x_FEDCBA0987654321_u64;
        println!("IV =	{}", iv);
        let mut cipher = [0_u8; 40];
        let len = a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
        println!("The length of ciphertext = {}", len);
        assert_eq!(len, 0);
        let failure = a_des.is_failed();
        assert_eq!(failure, true);
    
        // Failed case for decryption
        let key = 0x_1234567890ABCDEF_u64;
        println!("K =\t{:#016X}", key);
        let mut a_des = DES::new_with_key_u64(key);
    
        let cipher = [0x2Eu8, 0x1E, 0xE1, 0x51, 0xFD, 0xB3, 0xB0, 0x4B, 0x79, 0x3A, 0xA1, 0x78, 0xEC, 0xCD, 0x02, 0x72, 0x6A, 0xC4, 0x41, 0x7C, 0x25, 0xA4, 0x2C, 0x07, 0xFC, 0x77, 0x25, 0x49, 0x12, 0x55, 0x0F, 0x8A, 0xED, 0x44, 0xC3, 0xE4, 0xDC, 0x91, 0x69, 0x0F, 0x40, 0x72, 0x7F, 0xF2, 0xD9, 0xB7, 0x54, 0x9F, 0x36, 0x91, 0xC5, 0x85, 0x4F, 0x9B, 0x30];
        print!("C =\t");
        for c in cipher.clone()
            { print!("{:02X} ", c); }
        let mut recovered = [0u8; 40];
        let len = a_des.decrypt_array_into_array(iv, &cipher, &mut recovered);
        println!("The length of plaintext = {}", len);
        assert_eq!(len, 0);
        let failure = a_des.is_failed();
        assert_eq!(failure, true);
    }
    println!("-------------------------------");
}

fn des_set_successful()
{
    println!("des_set_successful");
    use cryptocol::symmetric::DES;
    let mut a_des = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    assert_eq!(a_des.is_successful(), false);

    a_des.set_successful();
    assert_eq!(a_des.is_successful(), true);
    println!("-------------------------------");
}

fn des_set_failed()
{
    println!("des_set_failed");
    use cryptocol::symmetric::DES;
    let mut a_des = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    a_des.encrypt_u64(0x1234567890ABCDEF_u64);
    assert_eq!(a_des.is_failed(), false);

    a_des.set_failed();
    assert_eq!(a_des.is_failed(), true);
    println!("-------------------------------");
}

fn des_has_weak_key()
{
    println!("des_has_weak_key");
    use cryptocol::symmetric::DES;

    let mut a_des = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    let weak_key = a_des.has_weak_key();
    assert_eq!(weak_key, false);

    a_des.set_key_u64(0x_0000000000000000_u64);
    let weak_key = a_des.has_weak_key();
    assert_eq!(weak_key, true);
    println!("-------------------------------");
}