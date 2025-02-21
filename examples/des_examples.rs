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

///// For test
// use cryptocol::symmetric::DES;
// use cryptocol::number::{ IntUnion };
//
// trait TestDes
// {
//     fn get_block(&self) -> u64;
//     fn set_block(&mut self, block: u64);
//     fn permutate_initially(&mut self);
//     fn permutate_finally(&mut self);
//     fn expand(&self, right: u32) -> u64;
//     fn compress_into_56bits(&self) -> u64;
//     fn split(&self) -> (IntUnion, IntUnion);
//     fn make_round_keys(&mut self);
//     fn get_round_key(&self, round: usize) -> u64;
//     fn slice_indices(&self, indices: u64, array: &mut [usize; 8]);
//     fn combine(&self, collector: &mut u32, piece: u32);
//     fn f(&mut self, round: usize, right: u32) -> u32;
// }
//
// impl TestDes for DES
// {
//     fn get_block(&self) -> u64          { self.test_get_block() }
//     fn set_block(&mut self, block: u64) { self.test_set_block(block); }
//     fn permutate_initially(&mut self)   { self.test_permutate_initially(); }
//     fn permutate_finally(&mut self)     { self.test_permutate_finally(); }
//     fn expand(&self, right: u32) -> u64     { self.test_expand(right) }
//     fn compress_into_56bits(&self) -> u64   { self.test_compress_into_56bits() }
//     fn split(&self) -> (IntUnion, IntUnion)     { self.test_split() }
//     fn make_round_keys(&mut self)    { self.test_make_round_keys(); }
//     fn get_round_key(&self, round: usize) -> u64  { self.test_get_round_key(round) }
//     fn slice_indices(&self, indices: u64, array: &mut [usize; 8])   { self.test_slice_indices(indices, array) }
//     fn combine(&self, collector: &mut u32, piece: u32) { self.test_combine(collector, piece); }
//     fn f(&mut self, round: usize, right: u32) -> u32   { self.test_f(round, right) }
// }

use std::io::Write;

// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]
// #[allow(non_camel_case_types)]
// #[allow(dead_code)]
pub fn main()
{
    // des_private_functions_main();
    des_basic_operation_main();
    des_encrypt_decrypt_u64_array_u64_main();
    des_crypt_with_padding_pkcs7_main();
    des_crypt_with_padding_iso_main();
    des_crypt_with_padding_pkcs7_ecb_main();
    des_crypt_with_padding_iso_ecb_main();
    des_crypt_with_padding_pkcs7_cbc_main();
    des_crypt_with_padding_iso_cbc_main();
    des_crypt_with_padding_pkcs7_pcbc_main();
    des_crypt_with_padding_iso_pcbc_main();
    des_crypt_cfb_main();
    des_crypt_ofb_main();
    des_crypt_ctr_main();
}

// fn des_private_functions_main()
// {
//     des_permutate_initially_finally();
//     des_permutate_expansion();
//     des_split();
//     des_make_round_keys();
//     des_slice_indices_combine();
//     des_f();
// }

// fn des_permutate_initially_finally()
// {
//     println!("des_permutate_initially_finally");
//     use std::fmt::Write;
//     use cryptocol::number::LongUnion;
//     use cryptocol::symmetric::DES;

//     let mut a_des = DES::new();
//     let block = (1_u64 << (8-2)) | (1_u64 << ((50-1) / 8 * 8 + (7 - (50-1) % 8)));
//     a_des.set_block(block);
//     a_des.permutate_initially();
//     let out = a_des.get_block();
//     let bu = LongUnion::new_with(block);
//     print!("block =\t");
//     for i in 0..8
//         { print!("{:08b} ", bu.get_ubyte_(i)); }
//     println!();
//     let mut txt = String::new();
//     for i in 0..8
//         { write!(txt, "{:08b} ", bu.get_ubyte_(i)); }
//     assert_eq!(txt, "01000000 00000000 00000000 00000000 00000000 00000000 01000000 00000000 ");

//     let ou = LongUnion::new_with(out);
//     print!("out =\t");
//     for i in 0..8
//         { print!("{:08b} ", ou.get_ubyte_(i)); }
//     println!();
//     let mut txt = String::new();
//     for i in 0..8
//         { write!(txt, "{:08b} ", ou.get_ubyte_(i)); }
//     assert_eq!(txt, "01000001 00000000 00000000 00000000 00000000 00000000 00000000 00000000 ");

//     a_des.permutate_finally();
//     let back = a_des.get_block();
//     let cu = LongUnion::new_with(back);
//     print!("back =\t");
//     for i in 0..8
//         { print!("{:08b} ", cu.get_ubyte_(i)); }
//     println!();
//     let mut txt = String::new();
//     for i in 0..8
//         { write!(txt, "{:08b} ", cu.get_ubyte_(i)); }
//     assert_eq!(txt, "01000000 00000000 00000000 00000000 00000000 00000000 01000000 00000000 ");
//     println!("-------------------------------");
// }

// fn des_permutate_expansion()
// {
//     println!("des_permutate_expansion");
//     use std::fmt::Write;
//     use cryptocol::number::{ IntUnion, LongUnion };
//     use cryptocol::symmetric::DES;
    
//     let mut right = IntUnion::new();
//     let mut i = 0;
//     for val in [0b_1111_0000_u8, 0b_1010_1010, 0b_1111_0000, 0b_1010_1010]
//     {
//         right.set_ubyte_(i, val);
//         i += 1;
//     }
//     print!("right =\t");
//     for i in 0..4
//         { print!("{:08b} ", right.get_ubyte_(i)); }
//     println!();

//     let a_des = DES::new();
//     let out = a_des.expand(right.get());

//     let ou = LongUnion::new_with(out);
//     print!("out =\t");
//     for i in 0..6
//         { print!("{:08b} ", ou.get_ubyte_(i)); }
//     println!();
//     let mut txt = String::new();
//     for i in 0..6
//         { write!(txt, "{:08b} ", ou.get_ubyte_(i)); }
//     assert_eq!(txt, "01111010 00010101 01010101 01111010 00010101 01010101 ");
//     println!("-------------------------------");
// }

// fn des_split()
// {
//     println!("des_split");
//     use std::fmt::Write;
//     use cryptocol::number::LongUnion;
//     use cryptocol::symmetric::DES;
    
//     let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
//     print!("K =\t");
//     for i in 0..8
//         { print!("{:08b} ", key[i]); }
//     println!();
//     let mut txt = String::new();
//     for i in 0..8
//         { write!(txt, "{:08b} ", key[i]); }
//     assert_eq!(txt, "00010011 00110100 01010111 01111001 10011011 10111100 11011111 11110001 ");

//     let a_des = DES::new_with_key(key.clone());
//     let key_56bit = LongUnion::new_with(a_des.compress_into_56bits());
//     print!("K+ =\t");
//     for i in 0..7
//         { print!("{:08b} ", key_56bit.get_ubyte_(i)); }
//     println!();
//     let mut txt = String::new();
//     for i in 0..7
//         { write!(txt, "{:08b} ", key_56bit.get_ubyte_(i)); }
//     assert_eq!(txt, "11110000 11001100 10101010 11110101 01010110 01100111 10001111 ");

//     let a_des = DES::new_with_key(key.clone());
//     let (left, right) = a_des.split();
//     print!("L =\t");
//     for i in 0..4
//         { print!("{:08b} ", left.get_ubyte_(i)); }
//     println!();
//     let mut txt = String::new();
//     for i in 0..4
//         { write!(txt, "{:08b} ", left.get_ubyte_(i)); }
//     assert_eq!(txt, "11110000 11001100 10101010 11110000 ");

//     print!("R =\t");
//     for i in 0..4
//         { print!("{:08b} ", right.get_ubyte_(i)); }
//     println!();
//     let mut txt = String::new();
//     for i in 0..4
//         { write!(txt, "{:08b} ", right.get_ubyte_(i)); }
//     assert_eq!(txt, "01010101 01100110 01111000 11110000 ");
//     println!("-------------------------------");
// }

// fn des_make_round_keys()
// {
//     println!("des_make_round_keys");
//     use std::fmt::Write;
//     use cryptocol::number::LongUnion;
//     use cryptocol::symmetric::DES;

//     let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
//     print!("K =\t");
//     for i in 0..8
//         { print!("{:08b} ", key[i]); }
//     println!();

//     let a_des = DES::new_with_key(key);
//     for i in 0..16
//     {
//         let round_key = LongUnion::new_with(a_des.get_round_key(i));
//         print!("K({}) =\t", i);
//         for j in 0..6
//             { print!("{:08b} ", round_key.get_ubyte_(j)); }
//         println!();
//     }

//     let round_key = LongUnion::new_with(a_des.get_round_key(0));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "00011011 00000010 11101111 11111100 01110000 01110010 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(1));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "01111001 10101110 11011001 11011011 11001001 11100101 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(2));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "01010101 11111100 10001010 01000010 11001111 10011001 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(3));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "01110010 10101101 11010110 11011011 00110101 00011101 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(4));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "01111100 11101100 00000111 11101011 01010011 10101000 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(5));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "01100011 10100101 00111110 01010000 01111011 00101111 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(6));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "11101100 10000100 10110111 11110110 00011000 10111100 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(7));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "11110111 10001010 00111010 11000001 00111011 11111011 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(8));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "11100000 11011011 11101011 11101101 11100111 10000001 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(9));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "10110001 11110011 01000111 10111010 01000110 01001111 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(10));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "00100001 01011111 11010011 11011110 11010011 10000110 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(11));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "01110101 01110001 11110101 10010100 01100111 11101001 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(12));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "10010111 11000101 11010001 11111010 10111010 01000001 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(13));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "01011111 01000011 10110111 11110010 11100111 00111010 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(14));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "10111111 10010001 10001101 00111101 00111111 00001010 ");

//     let round_key = LongUnion::new_with(a_des.get_round_key(15));
//     let mut txt = String::new();
//     for j in 0..6
//         { write!(txt, "{:08b} ", round_key.get_ubyte_(j)); }
//     assert_eq!(txt, "11001011 00111101 10001011 00001110 00010111 11110101 ");
//     println!("-------------------------------");
// }

// fn des_slice_indices_combine()
// {
//     println!("des_slice_indices_combine");
//     use cryptocol::number::LongUnion;
//     use cryptocol::symmetric::DES;

//     let a_des = DES::new();
//     let mut indices = LongUnion::new();
//     indices.set_ubyte_(0, 0b_111111_00);
//     indices.set_ubyte_(1, 0b_0000_1010);
//     indices.set_ubyte_(2, 0b_10_100100);
//     indices.set_ubyte_(3, 0b_010101_00);
//     indices.set_ubyte_(4, 0b_1001_1101);
//     indices.set_ubyte_(5, 0b_10_011011);

//     let mut index = [0_usize; 8];
//     a_des.slice_indices(indices.get(), &mut index);
//     for i in 0..8
//         { println!("idx({}) = {:06b}", i, index[i]); }
//     assert_eq!(index[0], 0b111111);
//     assert_eq!(index[1], 0b000000);
//     assert_eq!(index[2], 0b101010);
//     assert_eq!(index[3], 0b100100);
//     assert_eq!(index[4], 0b010101);
//     assert_eq!(index[5], 0b001001);
//     assert_eq!(index[6], 0b110110);
//     assert_eq!(index[7], 0b011011);

//     let mut collector = 0_u32;
//     let small = [0b1111_u32, 0b0101, 0b1000, 0b0111, 0b1100, 0b0011, 0b0001, 0b1001];
//     let piece = [(small[0] << 4) | small[1], (small[2] << 4) | small[3],
//                             (small[4] << 4) | small[5], (small[6] << 4) | small[7]];
//     for i in 0..8
//         { println!("{:04b} ", small[i]); }
    
//     a_des.combine(&mut collector, piece[0]);
//     a_des.combine(&mut collector, piece[1]);
//     a_des.combine(&mut collector, piece[2]);
//     a_des.combine(&mut collector, piece[3]);
//     let col = IntUnion::new_with(collector);
//     for i in 0..4
//         { println!("{:08b} ", col.get_ubyte_(i)); }
//     assert_eq!(col.get_ubyte_(0), 0b11110101);
//     assert_eq!(col.get_ubyte_(1), 0b10000111);
//     assert_eq!(col.get_ubyte_(2), 0b11000011);
//     assert_eq!(col.get_ubyte_(3), 0b00011001);
//     println!("-------------------------------");
// }

// fn des_f()
// {
//     println!("des_f");
//     use cryptocol::number::IntUnion;
//     use cryptocol::symmetric::DES;

//     let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
//     print!("K =\t");
//     for i in 0..8
//         { print!("{:08b} ", key[i]); }
//     println!();

//     let mut right = IntUnion::new();
//     right.set_ubyte_(0, 0b_1111_0000_u8);
//     right.set_ubyte_(1, 0b_1010_1010_u8);
//     right.set_ubyte_(2, 0b_1111_0000_u8);
//     right.set_ubyte_(3, 0b_1010_1010_u8);
//     print!("R =\t");
//     for i in 0..4
//         { print!("{:08b} ", right.get_ubyte_(i)); }
//     println!();

//     let mut a_des = DES::new_with_key(key);
//     let c = a_des.f(0, right.get());
//     let cipher = IntUnion::new_with(c);

//     print!("F =\t");
//     for i in 0..4
//         { print!("{:08b} ", cipher.get_ubyte_(i)); }
//     println!();
//     println!("-------------------------------");
// }

fn des_basic_operation_main()
{
    des_new();
    des_new_with_key();
    des_new_with_key_u64();
    des_set_key();
    des_set_key_u64();

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

fn des_encrypt_decrypt_u64_array_u64_main()
{
    des_encrypt_u64();
    des_decrypt_u64();
    des_encrypt_array_u64();
    des_decrypt_array_u64();
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

fn des_crypt_with_padding_pkcs7_main()
{
    des_encrypt_with_padding_pkcs7();
    des_encrypt_with_padding_pkcs7_into_vec();
    des_encrypt_with_padding_pkcs7_into_array();
    des_encrypt_str_with_padding_pkcs7();
    des_encrypt_str_with_padding_pkcs7_into_vec();
    des_encrypt_str_with_padding_pkcs7_into_array();
    des_encrypt_string_with_padding_pkcs7();
    des_encrypt_string_with_padding_pkcs7_into_vec();
    des_encrypt_string_with_padding_pkcs7_into_array();
    des_encrypt_vec_with_padding_pkcs7();
    des_encrypt_vec_with_padding_pkcs7_into_vec();
    des_encrypt_vec_with_padding_pkcs7_into_array();
    des_encrypt_array_with_padding_pkcs7();
    des_encrypt_array_with_padding_pkcs7_into_vec();
    des_encrypt_array_with_padding_pkcs7_into_array();

    des_decrypt_with_padding_pkcs7();
    des_decrypt_with_padding_pkcs7_into_vec();
    des_decrypt_with_padding_pkcs7_into_array();
    des_decrypt_with_padding_pkcs7_into_string();
    des_decrypt_vec_with_padding_pkcs7();
    des_decrypt_vec_with_padding_pkcs7_into_vec();
    des_decrypt_vec_with_padding_pkcs7_into_array();
    des_decrypt_vec_with_padding_pkcs7_into_string();
    des_decrypt_array_with_padding_pkcs7();
    des_decrypt_array_with_padding_pkcs7_into_vec();
    des_decrypt_array_with_padding_pkcs7_into_array();
    des_decrypt_array_with_padding_pkcs7_into_string();
}

fn des_encrypt_with_padding_pkcs7()
{
    println!("des_encrypt_with_padding_pkcs7");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_pkcs7(&message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C43FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA80808080808080808");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_pkcs7(&message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828297C753D2DBCC8C9");

    let mut back = Out { uu64: [0u64; 4] };
    unsafe { a_des.decrypt_array_u64(&cipher.uu64, &mut back.uu64); }
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD030303");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_with_padding_pkcs7_into_vec()
{
    println!("des_encrypt_with_padding_pkcs7_into_vec");
    use std::fmt::Write;
    use cryptocol::symmetric::{ DES, DES_Expanded };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 10 01 6D 99 BF 41 F8 BC 00 A8 1D 81 B7 4B 20 6F B5 30 0A 14 03 A9 8E 69 E7 A6 33 42 AF 97 59 ED 9D E0 95 35 DC DF 0D 99 58 FA 92 13 50 4D 50 D3 4E 76 9C C5 BB 9E CB ");
    println!();


    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut b_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    b_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "DD C6 D8 D1 B0 66 D9 AC F7 F3 B4 FD D6 6C ED 78 20 FB A6 8D 35 38 EA 65 B0 65 23 05 FF D4 53 B1 D1 E0 C5 52 36 1E AC E2 19 EF 94 B8 98 04 A9 69 CC 6A BC 81 7D 6B 29 C0 ");
    println!();


    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    println!("K =\t{:#016X}", key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");
    println!();


    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");
    println!();


    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "7 bytes";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F6 F0 41 DD 55 55 3B 35 ");
    println!();


    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "I am OK.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "27 F5 93 EE 76 DC 64 87 41 7F 89 79 08 CD A1 4C ");
    println!();


    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "PARK Youngho";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8E 52 20 47 78 78 51 B7 00 69 10 77 91 B7 52 36 ");
    println!();


    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "고맙습니다.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");
    println!("-------------------------------");
}

fn des_encrypt_with_padding_pkcs7_into_array()
{
    println!("des_encrypt_with_padding_pkcs7_into_array");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_pkcs7()
{
    println!("des_encrypt_str_with_padding_pkcs7");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_pkcs7(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D829019CBDE8EBC148CCCC235F0A771D771E9AE12233BB704A5E490E0328FB0F0085208B5D9D9BAEE224659910C15230FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 8] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E0808080808080808");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_pkcs7(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");
    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_pkcs7_into_vec()
{
    println!("des_encrypt_str_with_padding_pkcs7_into_vec");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_pkcs7_into_array()
{
    println!("des_encrypt_str_with_padding_pkcs7_into_array");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_pkcs7()
{
    println!("des_encrypt_string_with_padding_pkcs7");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_pkcs7(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D829019CBDE8EBC148CCCC235F0A771D771E9AE12233BB704A5E490E0328FB0F0085208B5D9D9BAEE224659910C15230FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 8] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E0808080808080808");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_pkcs7(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");
    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_pkcs7_into_vec()
{
    println!("des_encrypt_string_with_padding_pkcs7_into_vec");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_pkcs7_into_array()
{
    println!("des_encrypt_string_with_padding_pkcs7_into_array");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_pkcs7()
{
    println!("des_encrypt_vec_with_padding_pkcs7");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_pkcs7(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C43FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA80808080808080808");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_pkcs7(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828297C753D2DBCC8C9");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD030303");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_pkcs7_into_vec()
{
    println!("des_encrypt_vec_with_padding_pkcs7_into_vec");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_pkcs7_into_array()
{
    println!("des_encrypt_vec_with_padding_pkcs7_into_array");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_pkcs7()
{
    println!("des_encrypt_array_with_padding_pkcs7");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_pkcs7(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C43FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA80808080808080808");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_pkcs7(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828297C753D2DBCC8C9");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD030303");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_pkcs7_into_vec()
{
    println!("des_encrypt_array_with_padding_pkcs7_into_vec");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_pkcs7_into_array()
{
    println!("des_encrypt_array_with_padding_pkcs7_into_array");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_with_padding_pkcs7()
{
    println!("des_decrypt_with_padding_pkcs7");
    use std::fmt::Write;
    use cryptocol::symmetric::{ DES, DES_Expanded };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 10 01 6D 99 BF 41 F8 BC 00 A8 1D 81 B7 4B 20 6F B5 30 0A 14 03 A9 8E 69 E7 A6 33 42 AF 97 59 ED 9D E0 95 35 DC DF 0D 99 58 FA 92 13 50 4D 50 D3 4E 76 9C C5 BB 9E CB ");

    let mut recovered = vec![0; 55];
    a_des.decrypt_with_padding_pkcs7(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba (16 rounds) =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    
    println!("Bb (16 rounds) =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!();


    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "DD C6 D8 D1 B0 66 D9 AC F7 F3 B4 FD D6 6C ED 78 20 FB A6 8D 35 38 EA 65 B0 65 23 05 FF D4 53 B1 D1 E0 C5 52 36 1E AC E2 19 EF 94 B8 98 04 A9 69 CC 6A BC 81 7D 6B 29 C0 ");

    let mut recovered = vec![0; 55];
    a_des.decrypt_with_padding_pkcs7(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba (128 rounds) =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    
    println!("Bb (128 rounds) =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!();


    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    println!("K =\t{:#016X}", key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");

    let mut recovered1 = vec![0; 55];
    let mut recovered2 = vec![0; 55];
    c_des.decrypt_with_padding_pkcs7(cipher1.as_ptr(), cipher1.len() as u64, recovered1.as_mut_ptr());
    d_des.decrypt_with_padding_pkcs7(cipher2.as_ptr(), cipher2.len() as u64, recovered2.as_mut_ptr());
    print!("B1a (0 rounds) =\t");
    for b in recovered1.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");
    print!("B2a (0 rounds) =\t");
    for b in recovered2.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");

    let mut converted1 = String::new();
    let mut converted2 = String::new();
    unsafe { converted1.as_mut_vec() }.append(&mut recovered1);
    unsafe { converted2.as_mut_vec() }.append(&mut recovered2);
    
    println!("B1b (0 rounds) =\t{}", converted1);
    assert_eq!(converted1, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted1, message);
    println!("B2b (0 rounds) =\t{}", converted2);
    assert_eq!(converted2, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted2, message);
    assert_eq!(converted1, converted1);
    println!();


    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");

    let mut recovered = vec![0; 8];
    let len = a_des.decrypt_with_padding_pkcs7(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "00 00 00 00 00 00 00 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    converted.truncate(len as usize);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "");
    assert_eq!(converted, message);
    println!();


    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "7 bytes";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F6 F0 41 DD 55 55 3B 35 ");
    
    let mut recovered = vec![0; 8];
    let len = a_des.decrypt_with_padding_pkcs7(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 20 62 79 74 65 73 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    converted.truncate(len as usize);

    println!("Bb =\t{}", converted);
    assert_eq!(converted, "7 bytes");
    assert_eq!(converted, message);
    println!();


    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "I am OK.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "27 F5 93 EE 76 DC 64 87 41 7F 89 79 08 CD A1 4C ");
    
    let mut recovered = vec![0; 16];
    let len = a_des.decrypt_with_padding_pkcs7(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 20 61 6D 20 4F 4B 2E 00 00 00 00 00 00 00 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    converted.truncate(len as usize);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "I am OK.");
    assert_eq!(converted, message);
    println!();


    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "PARK Youngho";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8E 52 20 47 78 78 51 B7 00 69 10 77 91 B7 52 36 ");

    let mut recovered = vec![0; 16];
    let len = a_des.decrypt_with_padding_pkcs7(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 41 52 4B 20 59 6F 75 6E 67 68 6F 00 00 00 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    converted.truncate(len as usize);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "PARK Youngho");
    assert_eq!(converted, message);
    println!();



    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "고맙습니다.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");

    let mut recovered = vec![0; 24];
    let len = a_des.decrypt_with_padding_pkcs7(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "EA B3 A0 EB A7 99 EC 8A B5 EB 8B 88 EB 8B A4 2E 00 00 00 00 00 00 00 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    converted.truncate(len as usize);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "고맙습니다.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}

fn des_decrypt_with_padding_pkcs7_into_vec()
{
    println!("des_decrypt_with_padding_pkcs7_into_vec");
    use std::fmt::Write;
    use cryptocol::symmetric::{ DES, DES_Expanded };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 10 01 6D 99 BF 41 F8 BC 00 A8 1D 81 B7 4B 20 6F B5 30 0A 14 03 A9 8E 69 E7 A6 33 42 AF 97 59 ED 9D E0 95 35 DC DF 0D 99 58 FA 92 13 50 4D 50 D3 4E 76 9C C5 BB 9E CB ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    print!("Ba (16 rounds) =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    
    println!("Bb (16 rounds) =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!();


    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "DD C6 D8 D1 B0 66 D9 AC F7 F3 B4 FD D6 6C ED 78 20 FB A6 8D 35 38 EA 65 B0 65 23 05 FF D4 53 B1 D1 E0 C5 52 36 1E AC E2 19 EF 94 B8 98 04 A9 69 CC 6A BC 81 7D 6B 29 C0 ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    print!("Ba (128 rounds) =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    
    println!("Bb (128 rounds) =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!();


    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    println!("K =\t{:#016X}", key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");

    let mut recovered1 = Vec::<u8>::new();
    let mut recovered2 = Vec::<u8>::new();
    c_des.decrypt_with_padding_pkcs7_into_vec(cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    d_des.decrypt_with_padding_pkcs7_into_vec(cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
    print!("B1a (0 rounds) =\t");
    for b in recovered1.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");
    print!("B2a (0 rounds) =\t");
    for b in recovered2.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");

    let mut converted1 = String::new();
    let mut converted2 = String::new();
    unsafe { converted1.as_mut_vec() }.append(&mut recovered1);
    unsafe { converted2.as_mut_vec() }.append(&mut recovered2);
    
    println!("B1b (0 rounds) =\t{}", converted1);
    assert_eq!(converted1, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted1, message);
    println!("B2b (0 rounds) =\t{}", converted2);
    assert_eq!(converted2, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted2, message);
    assert_eq!(converted1, converted1);
    println!();


    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "");
    assert_eq!(converted, message);
    println!();


    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "7 bytes";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F6 F0 41 DD 55 55 3B 35 ");
    
    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 20 62 79 74 65 73 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "7 bytes");
    assert_eq!(converted, message);
    println!();


    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "I am OK.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "27 F5 93 EE 76 DC 64 87 41 7F 89 79 08 CD A1 4C ");
    
    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 20 61 6D 20 4F 4B 2E ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "I am OK.");
    assert_eq!(converted, message);
    println!();


    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "PARK Youngho";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8E 52 20 47 78 78 51 B7 00 69 10 77 91 B7 52 36 ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 41 52 4B 20 59 6F 75 6E 67 68 6F ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "PARK Youngho");
    assert_eq!(converted, message);
    println!();



    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "고맙습니다.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "EA B3 A0 EB A7 99 EC 8A B5 EB 8B 88 EB 8B A4 2E ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "고맙습니다.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}

fn des_decrypt_with_padding_pkcs7_into_array()
{
    println!("des_decrypt_with_padding_pkcs7_into_array");
    use std::fmt::Write;
    use cryptocol::symmetric::{ DES, DES_Expanded };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 10 01 6D 99 BF 41 F8 BC 00 A8 1D 81 B7 4B 20 6F B5 30 0A 14 03 A9 8E 69 E7 A6 33 42 AF 97 59 ED 9D E0 95 35 DC DF 0D 99 58 FA 92 13 50 4D 50 D3 4E 76 9C C5 BB 9E CB ");

    let mut recovered = [0u8; 56];
    let len = a_des.decrypt_with_padding_pkcs7_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    print!("Ba (16 rounds) =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.write(&recovered);
    unsafe { converted.as_mut_vec() }.truncate(len as usize);
    println!("Bb (16 rounds) =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!();



    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "DD C6 D8 D1 B0 66 D9 AC F7 F3 B4 FD D6 6C ED 78 20 FB A6 8D 35 38 EA 65 B0 65 23 05 FF D4 53 B1 D1 E0 C5 52 36 1E AC E2 19 EF 94 B8 98 04 A9 69 CC 6A BC 81 7D 6B 29 C0 ");

    let mut recovered = [0u8; 56];
    let len = a_des.decrypt_with_padding_pkcs7_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    print!("Ba (16 rounds) =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.write(&recovered);
    unsafe { converted.as_mut_vec() }.truncate(len as usize);
    println!("Bb (16 rounds) =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!();


    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    println!("K =\t{:#016X}", key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");

    let mut recovered1 = [0u8; 56];
    let mut recovered2 = [0u8; 56];
    let len1 = c_des.decrypt_with_padding_pkcs7_into_array(cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    let len2 = d_des.decrypt_with_padding_pkcs7_into_array(cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
    print!("B1a (0 rounds) =\t");
    for b in recovered1.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 ");
    print!("B2a (0 rounds) =\t");
    for b in recovered2.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 ");

    let mut converted1 = String::new();
    let mut converted2 = String::new();
    unsafe { converted1.as_mut_vec() }.write(&recovered1);
    unsafe { converted2.as_mut_vec() }.write(&recovered2);
    unsafe { converted1.as_mut_vec() }.truncate(len1 as usize);
    unsafe { converted2.as_mut_vec() }.truncate(len2 as usize);
    println!("B1b (0 rounds) =\t{}", converted1);
    println!("B2b (0 rounds) =\t{}", converted2);
    assert_eq!(converted1, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted2, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted1, message);
    assert_eq!(converted2, message);
    assert_eq!(converted1, converted2);
    println!();


    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");

    let mut recovered = [0u8; 8];
    let len = a_des.decrypt_with_padding_pkcs7_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);

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


    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "7 bytes";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F6 F0 41 DD 55 55 3B 35 ");

    let mut recovered = [0u8; 8];
    let len = a_des.decrypt_with_padding_pkcs7_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);

    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 20 62 79 74 65 73 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.write(&recovered);
    unsafe { converted.as_mut_vec() }.truncate(len as usize);
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "7 bytes");
    assert_eq!(converted, message);
    println!();


    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "I am OK.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "27 F5 93 EE 76 DC 64 87 41 7F 89 79 08 CD A1 4C ");

    let mut recovered = [0u8; 16];
    let len = a_des.decrypt_with_padding_pkcs7_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);

    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 20 61 6D 20 4F 4B 2E 00 00 00 00 00 00 00 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.write(&recovered);
    unsafe { converted.as_mut_vec() }.truncate(len as usize);
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "I am OK.");
    assert_eq!(converted, message);
    println!();


    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "PARK Youngho";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8E 52 20 47 78 78 51 B7 00 69 10 77 91 B7 52 36 ");

    let mut recovered = [0u8; 16];
    let len = a_des.decrypt_with_padding_pkcs7_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);

    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 41 52 4B 20 59 6F 75 6E 67 68 6F 00 00 00 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.write(&recovered);
    unsafe { converted.as_mut_vec() }.truncate(len as usize);
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "PARK Youngho");
    assert_eq!(converted, message);
    println!();


    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "고맙습니다.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");

    let mut recovered = [0u8; 24];
    let len = a_des.decrypt_with_padding_pkcs7_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);

    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "EA B3 A0 EB A7 99 EC 8A B5 EB 8B 88 EB 8B A4 2E 00 00 00 00 00 00 00 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.write(&recovered);
    unsafe { converted.as_mut_vec() }.truncate(len as usize);
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "고맙습니다.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}

fn des_decrypt_with_padding_pkcs7_into_string()
{
    println!("des_decrypt_with_padding_pkcs7_into_string");
    use std::fmt::Write;
    use cryptocol::symmetric::{ DES, DES_Expanded };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 10 01 6D 99 BF 41 F8 BC 00 A8 1D 81 B7 4B 20 6F B5 30 0A 14 03 A9 8E 69 E7 A6 33 42 AF 97 59 ED 9D E0 95 35 DC DF 0D 99 58 FA 92 13 50 4D 50 D3 4E 76 9C C5 BB 9E CB ");

    let mut recovered = String::new();
    a_des.decrypt_with_padding_pkcs7_into_string(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B (16 rounds) =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!();


    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut b_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    b_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "DD C6 D8 D1 B0 66 D9 AC F7 F3 B4 FD D6 6C ED 78 20 FB A6 8D 35 38 EA 65 B0 65 23 05 FF D4 53 B1 D1 E0 C5 52 36 1E AC E2 19 EF 94 B8 98 04 A9 69 CC 6A BC 81 7D 6B 29 C0 ");

    let mut recovered = String::new();
    b_des.decrypt_with_padding_pkcs7_into_string(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B (128 rounds) =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!();


    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    println!("K =\t{:#016X}", key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");

    let mut recovered1 = String::new();
    let mut recovered2 = String::new();
    c_des.decrypt_with_padding_pkcs7_into_string(cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    d_des.decrypt_with_padding_pkcs7_into_string(cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
    println!("B1 (0 rounds) =\t{}", recovered1);
    println!("B2 (0 rounds) =\t{}", recovered2);
    assert_eq!(recovered1, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered2, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered1, message);
    assert_eq!(recovered2, message);
    assert_eq!(recovered1, recovered2);
    println!();


    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");

    let mut recovered = String::new();
    a_des.decrypt_with_padding_pkcs7_into_string(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "");
    assert_eq!(recovered, message);
    println!();


    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "7 bytes";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F6 F0 41 DD 55 55 3B 35 ");

    let mut recovered = String::new();
    a_des.decrypt_with_padding_pkcs7_into_string(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "7 bytes");
    assert_eq!(recovered, message);
    println!();


    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "I am OK.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "27 F5 93 EE 76 DC 64 87 41 7F 89 79 08 CD A1 4C ");

    let mut recovered = String::new();
    a_des.decrypt_with_padding_pkcs7_into_string(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "I am OK.");
    assert_eq!(recovered, message);
    println!();


    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "PARK Youngho";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8E 52 20 47 78 78 51 B7 00 69 10 77 91 B7 52 36 ");

    let mut recovered = String::new();
    a_des.decrypt_with_padding_pkcs7_into_string(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "PARK Youngho");
    assert_eq!(recovered, message);
    println!();


    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "고맙습니다.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");

    let mut recovered = String::new();
    a_des.decrypt_with_padding_pkcs7_into_string(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn des_decrypt_vec_with_padding_pkcs7()
{
    println!("des_decrypt_vec_with_padding_pkcs7");
    use std::fmt::Write;
    use cryptocol::symmetric::{ DES, DES_Expanded };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 10 01 6D 99 BF 41 F8 BC 00 A8 1D 81 B7 4B 20 6F B5 30 0A 14 03 A9 8E 69 E7 A6 33 42 AF 97 59 ED 9D E0 95 35 DC DF 0D 99 58 FA 92 13 50 4D 50 D3 4E 76 9C C5 BB 9E CB ");

    let mut recovered = vec![0; 55];
    a_des.decrypt_vec_with_padding_pkcs7(&cipher, recovered.as_mut_ptr());
    print!("Ba (16 rounds) =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    
    println!("Bb (16 rounds) =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!();


    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "DD C6 D8 D1 B0 66 D9 AC F7 F3 B4 FD D6 6C ED 78 20 FB A6 8D 35 38 EA 65 B0 65 23 05 FF D4 53 B1 D1 E0 C5 52 36 1E AC E2 19 EF 94 B8 98 04 A9 69 CC 6A BC 81 7D 6B 29 C0 ");

    let mut recovered = vec![0; 55];
    a_des.decrypt_with_padding_pkcs7(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba (128 rounds) =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    
    println!("Bb (128 rounds) =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!();


    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    println!("K =\t{:#016X}", key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");

    let mut recovered1 = vec![0; 55];
    let mut recovered2 = vec![0; 55];
    c_des.decrypt_with_padding_pkcs7(cipher1.as_ptr(), cipher1.len() as u64, recovered1.as_mut_ptr());
    d_des.decrypt_with_padding_pkcs7(cipher2.as_ptr(), cipher2.len() as u64, recovered2.as_mut_ptr());
    print!("B1a (0 rounds) =\t");
    for b in recovered1.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");
    print!("B2a (0 rounds) =\t");
    for b in recovered2.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");

    let mut converted1 = String::new();
    let mut converted2 = String::new();
    unsafe { converted1.as_mut_vec() }.append(&mut recovered1);
    unsafe { converted2.as_mut_vec() }.append(&mut recovered2);
    
    println!("B1b (0 rounds) =\t{}", converted1);
    assert_eq!(converted1, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted1, message);
    println!("B2b (0 rounds) =\t{}", converted2);
    assert_eq!(converted2, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted2, message);
    assert_eq!(converted1, converted1);
    println!();


    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");

    let mut recovered = vec![0; 8];
    let len = a_des.decrypt_with_padding_pkcs7(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "00 00 00 00 00 00 00 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    converted.truncate(len as usize);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "");
    assert_eq!(converted, message);
    println!();


    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "7 bytes";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F6 F0 41 DD 55 55 3B 35 ");
    
    let mut recovered = vec![0; 8];
    let len = a_des.decrypt_with_padding_pkcs7(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 20 62 79 74 65 73 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    converted.truncate(len as usize);

    println!("Bb =\t{}", converted);
    assert_eq!(converted, "7 bytes");
    assert_eq!(converted, message);
    println!();


    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "I am OK.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "27 F5 93 EE 76 DC 64 87 41 7F 89 79 08 CD A1 4C ");
    
    let mut recovered = vec![0; 16];
    let len = a_des.decrypt_with_padding_pkcs7(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 20 61 6D 20 4F 4B 2E 00 00 00 00 00 00 00 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    converted.truncate(len as usize);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "I am OK.");
    assert_eq!(converted, message);
    println!();


    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "PARK Youngho";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8E 52 20 47 78 78 51 B7 00 69 10 77 91 B7 52 36 ");

    let mut recovered = vec![0; 16];
    let len = a_des.decrypt_with_padding_pkcs7(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 41 52 4B 20 59 6F 75 6E 67 68 6F 00 00 00 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    converted.truncate(len as usize);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "PARK Youngho");
    assert_eq!(converted, message);
    println!();



    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let message = "고맙습니다.";
    println!("M =\t{}", message);

    let mut a_des = DES::new_with_key_u64(key);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");

    let mut recovered = vec![0; 24];
    let len = a_des.decrypt_with_padding_pkcs7(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "EA B3 A0 EB A7 99 EC 8A B5 EB 8B 88 EB 8B A4 2E 00 00 00 00 00 00 00 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    converted.truncate(len as usize);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "고맙습니다.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}

fn des_decrypt_vec_with_padding_pkcs7_into_vec()
{
    println!("des_decrypt_vec_with_padding_pkcs7_into_vec");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_vec_with_padding_pkcs7_into_array()
{
    println!("des_decrypt_vec_with_padding_pkcs7_into_array");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_vec_with_padding_pkcs7_into_string()
{
    println!("des_decrypt_vec_with_padding_pkcs7_into_string");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_array_with_padding_pkcs7()
{
    println!("des_decrypt_array_with_padding_pkcs7");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_array_with_padding_pkcs7_into_vec()
{
    println!("des_decrypt_array_with_padding_pkcs7_into_vec");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_array_with_padding_pkcs7_into_array()
{
    println!("des_decrypt_array_with_padding_pkcs7_into_array");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_array_with_padding_pkcs7_into_string()
{
    println!("des_decrypt_array_with_padding_pkcs7_into_string");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}


fn des_crypt_with_padding_iso_main()
{
    des_encrypt_with_padding_iso();
    des_encrypt_with_padding_iso_into_vec();
    des_encrypt_with_padding_iso_into_array();
    des_encrypt_str_with_padding_iso();
    des_encrypt_str_with_padding_iso_into_vec();
    des_encrypt_str_with_padding_iso_into_array();
    des_encrypt_string_with_padding_iso();
    des_encrypt_string_with_padding_iso_into_vec();
    des_encrypt_string_with_padding_iso_into_array();
    des_encrypt_vec_with_padding_iso();
    des_encrypt_vec_with_padding_iso_into_vec();
    des_encrypt_vec_with_padding_iso_into_array();
    des_encrypt_array_with_padding_iso();
    des_encrypt_array_with_padding_iso_into_vec();
    des_encrypt_array_with_padding_iso_into_array();

    des_decrypt_with_padding_iso();
    des_decrypt_with_padding_iso_into_vec();
    des_decrypt_with_padding_iso_into_array();
    des_decrypt_with_padding_iso_into_string();
    des_decrypt_vec_with_padding_iso();
    des_decrypt_vec_with_padding_iso_into_vec();
    des_decrypt_vec_with_padding_iso_into_array();
    des_decrypt_vec_with_padding_iso_into_string();
    des_decrypt_array_with_padding_iso();
    des_decrypt_array_with_padding_iso_into_vec();
    des_decrypt_array_with_padding_iso_into_array();
    des_decrypt_array_with_padding_iso_into_string();
}

fn des_encrypt_with_padding_iso()
{
    println!("des_encrypt_with_padding_iso");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_iso(&message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA88000000000000000");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_iso(&message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD800000");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_with_padding_iso_into_vec()
{
    println!("des_encrypt_with_padding_iso_into_vec");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }
    union Out2 {
        pub uu8: [u8; 24],
        pub uu64: [u64; 3],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Vec::<u8>::new();
    let length = a_des.encrypt_with_padding_iso_into_vec(&message as *const u8, message.len() as u64, &mut cipher);

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", cipher[i]); }
    println!();

    let mut cipher2 = Out { uu64: [0u64; 4] };
    let mut i = 0_usize;
    for val in cipher.clone()
    {
        unsafe { cipher2.uu8[i] = val; }
        i += 1;
    }

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher2.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA88000000000000000");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso(cipher.as_ptr() as *mut u8, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Vec::<u8>::new();
    let length = a_des.encrypt_with_padding_iso_into_vec(&message as *const u8, message.len() as u64, &mut cipher);

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", cipher[i]); }
    println!();

    let mut cipher2 = Out2 { uu64: [0u64; 3] };
    let mut i = 0_usize;
    for val in cipher.clone()
    {
        unsafe { cipher2.uu8[i] = val; }
        i += 1;
    }

    let mut back = Out2 { uu64: [0u64; 3] };
    a_des.decrypt_array_u64(unsafe { &cipher2.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD800000");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso(cipher.as_ptr() as *const u8, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_with_padding_iso_into_array()
{
    println!("des_encrypt_with_padding_iso_into_array");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_iso()
{
    println!("des_encrypt_str_with_padding_iso");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_iso(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D829019CBDE8EBC148CCCC235F0A771D771E9AE12233BB704A5E490E0328FB0F0085208B5D9D9BAEE224659910C1523087AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 8] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E8000000000000000");
    
    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_iso(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");
    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_iso_into_vec()
{
    println!("des_encrypt_str_with_padding_iso_into_vec");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_iso_into_array()
{
    println!("des_encrypt_str_with_padding_iso_into_array");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_iso()
{
    println!("des_encrypt_string_with_padding_iso");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_iso(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D829019CBDE8EBC148CCCC235F0A771D771E9AE12233BB704A5E490E0328FB0F0085208B5D9D9BAEE224659910C1523087AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 8] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E8000000000000000");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_iso(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");
    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_iso_into_vec()
{
    println!("des_encrypt_string_with_padding_iso_into_vec");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_iso_into_array()
{
    println!("des_encrypt_string_with_padding_iso_into_array");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_iso()
{
    println!("des_encrypt_vec_with_padding_iso");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_iso(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C4387AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA88000000000000000");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_iso(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF82800D8D8F29761F19E");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD800000");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_iso_into_vec()
{
    println!("des_encrypt_vec_with_padding_iso_into_vec");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_iso_into_array()
{
    println!("des_encrypt_vec_with_padding_iso_into_array");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_iso()
{
    println!("des_encrypt_array_with_padding_iso");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_iso(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C4387AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA88000000000000000");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_iso(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF82800D8D8F29761F19E");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD800000");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_iso_into_vec()
{
    println!("des_encrypt_array_with_padding_iso_into_vec");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_iso_into_array()
{
    println!("des_encrypt_array_with_padding_iso_into_array");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_with_padding_iso()
{
    println!("des_decrypt_with_padding_iso");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let cipher = [0x85_u8, 0xE8, 0x13, 0x54, 0x0F, 0x0A, 0xB4, 0x05, 0x71, 0xD5, 0x28, 0xC6, 0x05, 0x0F, 0xF8, 0x28, 0xD9, 0x51, 0xFC, 0x58, 0xB6, 0x42, 0x2C, 0x43, 0x87, 0xAB, 0x78, 0xD1, 0x1E, 0x18, 0x8D, 0xF6];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso(&cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x85_u8, 0xE8, 0x13, 0x54, 0x0F, 0x0A, 0xB4, 0x05, 0x71, 0xD5, 0x28, 0xC6, 0x05, 0x0F, 0xF8, 0x28, 0x00, 0xD8, 0xD8, 0xF2, 0x97, 0x61, 0xF1, 0x9E];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = unsafe { a_des.decrypt_with_padding_iso(&cipher as *const u8, cipher.len() as u64, message.uu8.as_mut_ptr()) };

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_with_padding_iso_into_vec()
{
    println!("des_decrypt_with_padding_iso_into_vec");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_with_padding_iso_into_array()
{
    println!("des_decrypt_with_padding_iso_into_array");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_with_padding_iso_into_string()
{
    println!("des_decrypt_with_padding_iso_into_string");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_vec_with_padding_iso()
{
    println!("des_decrypt_vec_with_padding_iso");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_vec_with_padding_iso_into_vec()
{
    println!("des_decrypt_with_padding_iso_into_vec");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_vec_with_padding_iso_into_array()
{
    println!("des_decrypt_vec_with_padding_iso_into_array");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_vec_with_padding_iso_into_string()
{
    println!("des_decrypt_vec_with_padding_iso_into_string");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_array_with_padding_iso()
{
    println!("des_decrypt_array_with_padding_iso");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_array_with_padding_iso_into_vec()
{
    println!("des_decrypt_with_padding_iso_into_vec");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_array_with_padding_iso_into_array()
{
    println!("des_decrypt_array_with_padding_iso_into_array");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}

fn des_decrypt_array_with_padding_iso_into_string()
{
    println!("des_decrypt_array_with_padding_iso_into_string");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    println!("-------------------------------");
}


fn des_crypt_with_padding_pkcs7_ecb_main()
{
    des_encrypt_with_padding_pkcs7_ecb();
    des_encrypt_str_with_padding_pkcs7_ecb();
    des_encrypt_string_with_padding_pkcs7_ecb();
    des_encrypt_vec_with_padding_pkcs7_ecb();
    des_encrypt_array_with_padding_pkcs7_ecb();
    des_decrypt_with_padding_pkcs7_ecb();
}

fn des_encrypt_with_padding_pkcs7_ecb()
{
    println!("des_encrypt_with_padding_pkcs7_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_pkcs7_ecb(&message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C43FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA80808080808080808");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_pkcs7_ecb(&message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828297C753D2DBCC8C9");

    let mut back = Out { uu64: [0u64; 4] };
    unsafe { a_des.decrypt_array_u64(&cipher.uu64, &mut back.uu64); }
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD030303");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_with_padding_pkcs7_ecb()
{
    println!("des_decrypt_with_padding_pkcs7_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let cipher = [0x85_u8, 0xE8, 0x13, 0x54, 0x0F, 0x0A, 0xB4, 0x05, 0x71, 0xD5, 0x28, 0xC6, 0x05, 0x0F, 0xF8, 0x28, 0xD9, 0x51, 0xFC, 0x58, 0xB6, 0x42, 0x2C, 0x43, 0xFD, 0xF2, 0xE1, 0x74, 0x49, 0x29, 0x22, 0xF8];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(&cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x85_u8, 0xE8, 0x13, 0x54, 0x0F, 0x0A, 0xB4, 0x05, 0x71, 0xD5, 0x28, 0xC6, 0x05, 0x0F, 0xF8, 0x28, 0x29, 0x7C, 0x75, 0x3D, 0x2D, 0xBC, 0xC8, 0xC9];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(&cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_pkcs7_ecb()
{
    println!("des_encrypt_str_with_padding_pkcs7_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_pkcs7_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D829019CBDE8EBC148CCCC235F0A771D771E9AE12233BB704A5E490E0328FB0F0085208B5D9D9BAEE224659910C15230FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 8] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E0808080808080808");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");
    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_pkcs7_ecb()
{
    println!("des_encrypt_string_with_padding_pkcs7_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_pkcs7_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D829019CBDE8EBC148CCCC235F0A771D771E9AE12233BB704A5E490E0328FB0F0085208B5D9D9BAEE224659910C15230FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 8] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E0808080808080808");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");
    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_pkcs7_ecb()
{
    println!("des_encrypt_array_with_padding_pkcs7_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_pkcs7_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C43FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA80808080808080808");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_pkcs7_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828297C753D2DBCC8C9");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD030303");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_pkcs7_ecb()
{
    println!("des_encrypt_vec_with_padding_pkcs7_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_pkcs7_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C43FDF2E174492922F8");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA80808080808080808");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_pkcs7_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828297C753D2DBCC8C9");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD030303");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_with_padding_iso_ecb_main()
{
    des_encrypt_with_padding_iso_ecb();
    des_decrypt_with_padding_iso_ecb();
    des_encrypt_str_with_padding_iso_ecb();
    des_encrypt_string_with_padding_iso_ecb();
    des_encrypt_array_with_padding_iso_ecb();
    des_encrypt_vec_with_padding_iso_ecb();
}

fn des_encrypt_with_padding_iso_ecb()
{
    println!("des_encrypt_with_padding_iso_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_iso_ecb(&message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA88000000000000000");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_iso_ecb(&message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD800000");
    
    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_with_padding_iso_ecb()
{
    println!("des_decrypt_with_padding_iso_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let cipher = [0x85_u8, 0xE8, 0x13, 0x54, 0x0F, 0x0A, 0xB4, 0x05, 0x71, 0xD5, 0x28, 0xC6, 0x05, 0x0F, 0xF8, 0x28, 0xD9, 0x51, 0xFC, 0x58, 0xB6, 0x42, 0x2C, 0x43, 0x87, 0xAB, 0x78, 0xD1, 0x1E, 0x18, 0x8D, 0xF6];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_ecb(&cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x85_u8, 0xE8, 0x13, 0x54, 0x0F, 0x0A, 0xB4, 0x05, 0x71, 0xD5, 0x28, 0xC6, 0x05, 0x0F, 0xF8, 0x28, 0x00, 0xD8, 0xD8, 0xF2, 0x97, 0x61, 0xF1, 0x9E];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = unsafe { a_des.decrypt_with_padding_iso_ecb(&cipher as *const u8, cipher.len() as u64, message.uu8.as_mut_ptr()) };

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_iso_ecb()
{
    println!("des_encrypt_str_with_padding_iso_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_iso_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D829019CBDE8EBC148CCCC235F0A771D771E9AE12233BB704A5E490E0328FB0F0085208B5D9D9BAEE224659910C1523087AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 8] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E8000000000000000");
    
    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_iso_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");
    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_iso_ecb()
{
    println!("des_encrypt_string_with_padding_iso_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_iso_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D829019CBDE8EBC148CCCC235F0A771D771E9AE12233BB704A5E490E0328FB0F0085208B5D9D9BAEE224659910C1523087AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 8] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E8000000000000000");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_iso_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");
    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_iso_ecb()
{
    println!("des_encrypt_array_with_padding_iso_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_iso_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C4387AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA88000000000000000");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_iso_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF82800D8D8F29761F19E");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD800000");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_iso_ecb()
{
    println!("des_encrypt_vec_with_padding_iso_ecb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_iso_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF828D951FC58B6422C4387AB78D11E188DF6");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in unsafe { 0..back.uu8.len() }
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in unsafe { 0..back.uu8.len() }
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA88000000000000000");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_iso_ecb(&message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "85E813540F0AB40571D528C6050FF82800D8D8F29761F19E");

    let mut back = Out { uu64: [0u64; 4] };
    a_des.decrypt_array_u64(unsafe { &cipher.uu64 }, unsafe { &mut back.uu64 });
    print!("B =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { back.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { back.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD800000");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_ecb(unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_with_padding_pkcs7_cbc_main()
{
    des_encrypt_with_padding_pkcs7_cbc();
    des_decrypt_with_padding_pkcs7_cbc();
    des_encrypt_str_with_padding_pkcs7_cbc();
    des_encrypt_string_with_padding_pkcs7_cbc();
    des_encrypt_array_with_padding_pkcs7_cbc();
    des_encrypt_vec_with_padding_pkcs7_cbc();
}

fn des_encrypt_with_padding_pkcs7_cbc()
{
    println!("des_encrypt_with_padding_pkcs7_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_pkcs7_cbc(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7C9AFF7F59D81AE59B33EB4FDA247DB78E");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_pkcs7_cbc(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7CC5502C9C37698343");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_with_padding_pkcs7_cbc()
{
    println!("des_decrypt_with_padding_pkcs7_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let cipher = [0x72_u8, 0x2D, 0x01, 0x78, 0xF1, 0x79, 0x3C, 0x6D, 0x5F, 0xDB, 0xE3, 0xB6, 0xC1, 0x23, 0x7A, 0x7C, 0x9A, 0xFF, 0x7F, 0x59, 0xD8, 0x1A, 0xE5, 0x9B, 0x33, 0xEB, 0x4F, 0xDA, 0x24, 0x7D, 0xB7, 0x8E];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x72_u8, 0x2D, 0x01, 0x78, 0xF1, 0x79, 0x3C, 0x6D, 0x5F, 0xDB, 0xE3, 0xB6, 0xC1, 0x23, 0x7A, 0x7C, 0xC5, 0x50, 0x2C, 0x9C, 0x37, 0x69, 0x83, 0x43];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_pkcs7_cbc()
{
    println!("des_encrypt_str_with_padding_pkcs7_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_pkcs7_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "DCF20146C73267CF6EFA79CA0B36A33C73F03A3BF1F4C43384E0664148104650E4A5F44804CB853F7734D79583F8A8B0A4C55A1B41A67A88");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text.as_str());
    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_pkcs7_cbc()
{
    println!("des_encrypt_string_with_padding_pkcs7_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_pkcs7_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "DCF20146C73267CF6EFA79CA0B36A33C73F03A3BF1F4C43384E0664148104650E4A5F44804CB853F7734D79583F8A8B0A4C55A1B41A67A88");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text);
    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_pkcs7_cbc()
{
    println!("des_encrypt_array_with_padding_pkcs7_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_pkcs7_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7C9AFF7F59D81AE59B33EB4FDA247DB78E");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_pkcs7_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7CC5502C9C37698343");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_pkcs7_cbc()
{
    println!("des_encrypt_vec_with_padding_pkcs7_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_pkcs7_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7C9AFF7F59D81AE59B33EB4FDA247DB78E");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_pkcs7_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7CC5502C9C37698343");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_with_padding_iso_cbc_main()
{
    des_encrypt_with_padding_iso_cbc();
    des_decrypt_with_padding_iso_cbc();
    des_encrypt_str_with_padding_iso_cbc();
    des_encrypt_string_with_padding_iso_cbc();
    des_encrypt_array_with_padding_iso_cbc();
    des_encrypt_vec_with_padding_iso_cbc();
}

fn des_encrypt_with_padding_iso_cbc()
{
    println!("des_encrypt_with_padding_iso_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_iso_cbc(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7C9AFF7F59D81AE59B9F4B15B7DDDFE2B8");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_iso_cbc(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7CE80D8F56AA71F3D0");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_with_padding_iso_cbc()
{
    println!("des_decrypt_with_padding_iso_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let cipher = [0x72_u8, 0x2D, 0x01, 0x78, 0xF1, 0x79, 0x3C, 0x6D, 0x5F, 0xDB, 0xE3, 0xB6, 0xC1, 0x23, 0x7A, 0x7C, 0x9A, 0xFF, 0x7F, 0x59, 0xD8, 0x1A, 0xE5, 0x9B, 0x9F, 0x4B, 0x15, 0xB7, 0xDD, 0xDF, 0xE2, 0xB8];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x72_u8, 0x2D, 0x01, 0x78, 0xF1, 0x79, 0x3C, 0x6D, 0x5F, 0xDB, 0xE3, 0xB6, 0xC1, 0x23, 0x7A, 0x7C, 0xE8, 0x0D, 0x8F, 0x56, 0xAA, 0x71, 0xF3, 0xD0];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_iso_cbc()
{
    println!("des_encrypt_str_with_padding_iso_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_iso_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "DCF20146C73267CF6EFA79CA0B36A33C73F03A3BF1F4C43384E0664148104650E4A5F44804CB853F7734D79583F8A8B05278468F4BED756A");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text.as_str());
    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_iso_cbc()
{
    println!("des_encrypt_string_with_padding_iso_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_iso_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "DCF20146C73267CF6EFA79CA0B36A33C73F03A3BF1F4C43384E0664148104650E4A5F44804CB853F7734D79583F8A8B05278468F4BED756A");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text);
    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_iso_cbc()
{
    println!("des_encrypt_array_with_padding_iso_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_iso_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7C9AFF7F59D81AE59B9F4B15B7DDDFE2B8");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_iso_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7CE80D8F56AA71F3D0");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_iso_cbc()
{
    println!("des_encrypt_vec_with_padding_iso_cbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_iso_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7C9AFF7F59D81AE59B9F4B15B7DDDFE2B8");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_iso_cbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D5FDBE3B6C1237A7CE80D8F56AA71F3D0");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_cbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_with_padding_pkcs7_pcbc_main()
{
    des_encrypt_with_padding_pkcs7_pcbc();
    des_decrypt_with_padding_pkcs7_pcbc();
    des_encrypt_str_with_padding_pkcs7_pcbc();
    des_encrypt_string_with_padding_pkcs7_pcbc();
    des_encrypt_array_with_padding_pkcs7_pcbc();
    des_encrypt_vec_with_padding_pkcs7_pcbc();
}

fn des_encrypt_with_padding_pkcs7_pcbc()
{
    println!("des_encrypt_with_padding_pkcs7_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_pkcs7_pcbc(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61BAED1404114166B136D0AAB7F464DDE6");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_pkcs7_pcbc(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA613F0A58ED52F62A2E");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_with_padding_pkcs7_pcbc()
{
    println!("des_decrypt_with_padding_pkcs7_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let cipher = [0x72_u8, 0x2D, 0x01, 0x78, 0xF1, 0x79, 0x3C, 0x6D, 0x1F, 0x46, 0x3D, 0xE0, 0xDB, 0x24, 0xBA, 0x61, 0xBA, 0xED, 0x14, 0x04, 0x11, 0x41, 0x66, 0xB1, 0x36, 0xD0, 0xAA, 0xB7, 0xF4, 0x64, 0xDD, 0xE6];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x72_u8, 0x2D, 0x01, 0x78, 0xF1, 0x79, 0x3C, 0x6D, 0x1F, 0x46, 0x3D, 0xE0, 0xDB, 0x24, 0xBA, 0x61, 0x3F, 0x0A, 0x58, 0xED, 0x52, 0xF6, 0x2A, 0x2E];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_pkcs7_pcbc()
{
    println!("des_encrypt_str_with_padding_pkcs7_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_pkcs7_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "DCF20146C73267CF1F6DC62ED90971FE7A64C7CBD56B6A59AE88190BC5C49D1D4158351BFB086BDAC6F88D64FD054AEC4F64B13AE4FAA77E");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text.as_str());
    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_pkcs7_pcbc()
{
    println!("des_encrypt_string_with_padding_pkcs7_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_pkcs7_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "DCF20146C73267CF1F6DC62ED90971FE7A64C7CBD56B6A59AE88190BC5C49D1D4158351BFB086BDAC6F88D64FD054AEC4F64B13AE4FAA77E");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text);
    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_pkcs7_pcbc()
{
    println!("des_encrypt_array_with_padding_pkcs7_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_pkcs7_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61BAED1404114166B136D0AAB7F464DDE6");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_pkcs7_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA613F0A58ED52F62A2E");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_pkcs7_pcbc()
{
    println!("des_encrypt_vec_with_padding_pkcs7_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_pkcs7_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61BAED1404114166B136D0AAB7F464DDE6");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_pkcs7_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA613F0A58ED52F62A2E");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_pkcs7_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_with_padding_iso_pcbc_main()
{
    des_encrypt_with_padding_iso_pcbc();
    des_decrypt_with_padding_iso_pcbc();
    des_encrypt_str_with_padding_iso_pcbc();
    des_encrypt_string_with_padding_iso_pcbc();
    des_encrypt_array_with_padding_iso_pcbc();
    des_encrypt_vec_with_padding_iso_pcbc();
}

fn des_encrypt_with_padding_iso_pcbc()
{
    println!("des_encrypt_with_padding_iso_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_iso_pcbc(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61BAED1404114166B16FF807E714DB65C2");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_with_padding_iso_pcbc(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61EB9C35C902B7B4BC");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_with_padding_iso_pcbc()
{
    println!("des_decrypt_with_padding_iso_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let cipher = [0x72_u8, 0x2D, 0x01, 0x78, 0xF1, 0x79, 0x3C, 0x6D, 0x1F, 0x46, 0x3D, 0xE0, 0xDB, 0x24, 0xBA, 0x61, 0xBA, 0xED, 0x14, 0x04, 0x11, 0x41, 0x66, 0xB1, 0x6F, 0xF8, 0x07, 0xE7, 0x14, 0xDB, 0x65, 0xC2];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x72_u8, 0x2D, 0x01, 0x78, 0xF1, 0x79, 0x3C, 0x6D, 0x1F, 0x46, 0x3D, 0xE0, 0xDB, 0x24, 0xBA, 0x61, 0xEB, 0x9C, 0x35, 0xC9, 0x02, 0xB7, 0xB4, 0xBC];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_with_padding_iso_pcbc()
{
    println!("des_encrypt_str_with_padding_iso_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_iso_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "DCF20146C73267CF1F6DC62ED90971FE7A64C7CBD56B6A59AE88190BC5C49D1D4158351BFB086BDAC6F88D64FD054AEC348BC507FFF5613E");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text.as_str());
    println!("-------------------------------");
}

fn des_encrypt_string_with_padding_iso_pcbc()
{
    println!("des_encrypt_string_with_padding_iso_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_with_padding_iso_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "DCF20146C73267CF1F6DC62ED90971FE7A64C7CBD56B6A59AE88190BC5C49D1D4158351BFB086BDAC6F88D64FD054AEC348BC507FFF5613E");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text);
    println!("-------------------------------");
}

fn des_encrypt_array_with_padding_iso_pcbc()
{
    println!("des_encrypt_array_with_padding_iso_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_iso_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61BAED1404114166B16FF807E714DB65C2");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_with_padding_iso_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61EB9C35C902B7B4BC");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_with_padding_iso_pcbc()
{
    println!("des_encrypt_vec_with_padding_iso_pcbc");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_iso_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61BAED1404114166B16FF807E714DB65C2");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_with_padding_iso_pcbc(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "722D0178F1793C6D1F463DE0DB24BA61EB9C35C902B7B4BC");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_with_padding_iso_pcbc(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_cfb_main()
{
    des_encrypt_cfb();
    des_decrypt_cfb();
    des_encrypt_str_cfb();
    des_encrypt_string_cfb();
    des_encrypt_array_cfb();
    des_encrypt_vec_cfb();
}

fn des_encrypt_cfb()
{
    println!("des_encrypt_cfb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_cfb(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A849F4181D45E49A9AB03AF9F0D7A3BD8C");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_cfb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_cfb(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A849F4181D45E49A9AB03AF9F0D7");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_cfb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_cfb()
{
    println!("des_decrypt_cfb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let cipher = [0xA7_u8, 0x00, 0x00, 0x57, 0xF2, 0xA3, 0x84, 0xA8, 0x49, 0xF4, 0x18, 0x1D, 0x45, 0xE4, 0x9A, 0x9A, 0xB0, 0x3A, 0xF9, 0xF0, 0xD7, 0xA3, 0xBD, 0x8C];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_cfb(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0xA7_u8, 0x00, 0x00, 0x57, 0xF2, 0xA3, 0x84, 0xA8, 0x49, 0xF4, 0x18, 0x1D, 0x45, 0xE4, 0x9A, 0x9A, 0xB0, 0x3A, 0xF9, 0xF0, 0xD7];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_cfb(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_cfb()
{
    println!("des_encrypt_str_cfb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_cfb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "F24B2C435B613A67B952FFF079F4D0E32AD95AD79006FEFD7F9B62FE1ADBEBFF5FBD9EF56FCBBE30A21505F3A87DE1A4");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_cfb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text.as_str());
    println!("-------------------------------");
}

fn des_encrypt_string_cfb()
{
    println!("des_encrypt_string_cfb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_cfb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "F24B2C435B613A67B952FFF079F4D0E32AD95AD79006FEFD7F9B62FE1ADBEBFF5FBD9EF56FCBBE30A21505F3A87DE1A4");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_cfb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text);
    println!("-------------------------------");
}

fn des_encrypt_array_cfb()
{
    println!("des_encrypt_array_cfb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_cfb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A849F4181D45E49A9AB03AF9F0D7A3BD8C");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_cfb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_cfb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A849F4181D45E49A9AB03AF9F0D7");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_cfb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_cfb()
{
    println!("des_encrypt_vec_cfb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_cfb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A849F4181D45E49A9AB03AF9F0D7A3BD8C");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_cfb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_cfb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A849F4181D45E49A9AB03AF9F0D7");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_cfb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_ofb_main()
{
    des_encrypt_ofb();
    des_decrypt_ofb();
    des_encrypt_str_ofb();
    des_encrypt_string_ofb();
    des_encrypt_array_ofb();
    des_encrypt_vec_ofb();
}

fn des_encrypt_ofb()
{
    println!("des_encrypt_ofb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_ofb(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A88A749AF76E5BC84BD66FDDAA3B830434");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ofb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_ofb(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A88A749AF76E5BC84BD66FDDAA3B");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ofb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_ofb()
{
    println!("des_decrypt_ofb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let cipher = [0xA7_u8, 0x00, 0x00, 0x57, 0xF2, 0xA3, 0x84, 0xA8, 0x8A, 0x74, 0x9A, 0xF7, 0x6E, 0x5B, 0xC8, 0x4B, 0xD6, 0x6F, 0xDD, 0xAA, 0x3B, 0x83, 0x04, 0x34];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ofb(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0xA7_u8, 0x00, 0x00, 0x57, 0xF2, 0xA3, 0x84, 0xA8, 0x8A, 0x74, 0x9A, 0xF7, 0x6E, 0x5B, 0xC8, 0x4B, 0xD6, 0x6F, 0xDD, 0xAA, 0x3B];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ofb(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_ofb()
{
    println!("des_encrypt_str_ofb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_ofb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "F24B2C435B613A67FA76DDD648499FA520B74622A33ED5BCFF82DE7AB6984F36DE4C1FB6AF2F5556B1D1C5BAF9A7F9C4");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_ofb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text.as_str());
    println!("-------------------------------");
}

fn des_encrypt_string_ofb()
{
    println!("des_encrypt_string_ofb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_ofb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "F24B2C435B613A67FA76DDD648499FA520B74622A33ED5BCFF82DE7AB6984F36DE4C1FB6AF2F5556B1D1C5BAF9A7F9C4");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_ofb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text);
    println!("-------------------------------");
}

fn des_encrypt_array_ofb()
{
    println!("des_encrypt_array_ofb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_ofb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A88A749AF76E5BC84BD66FDDAA3B830434");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ofb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_ofb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A88A749AF76E5BC84BD66FDDAA3B");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ofb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_ofb()
{
    println!("des_encrypt_vec_ofb");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_ofb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A88A749AF76E5BC84BD66FDDAA3B830434");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ofb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_ofb(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "A7000057F2A384A88A749AF76E5BC84BD66FDDAA3B");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ofb(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_crypt_ctr_main()
{
    des_encrypt_ctr();
    des_decrypt_ctr();
    des_encrypt_str_ctr();
    des_encrypt_string_ctr();
    des_encrypt_array_ctr();
    des_encrypt_vec_ctr();
}

fn des_encrypt_ctr()
{
    println!("des_encrypt_ctr");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_ctr(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "86DCC11C303A6BF4C043CB5BCE420935BE310DC54761AA55");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ctr(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_ctr(iv, &message as *const u8, message.len() as u64, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "86DCC11C303A6BF4C043CB5BCE420935BE310DC547");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ctr(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_decrypt_ctr()
{
    println!("des_decrypt_ctr");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let cipher = [0x86_u8, 0xDC, 0xC1, 0x1C, 0x30, 0x3A, 0x6B, 0xF4, 0xC0, 0x43, 0xCB, 0x5B, 0xCE, 0x42, 0x09, 0x35, 0xBE, 0x31, 0x0D, 0xC5, 0x47, 0x61, 0xAA, 0x55];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ctr(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let cipher = [0x86_u8, 0xDC, 0xC1, 0x1C, 0x30, 0x3A, 0x6B, 0xF4, 0xC0, 0x43, 0xCB, 0x5B, 0xCE, 0x42, 0x09, 0x35, 0xBE, 0x31, 0x0D, 0xC5, 0x47];
    print!("C =\t");
    for val in cipher
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut message = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ctr(iv, &cipher as *const u8, cipher.len() as u64, unsafe { message.uu8.as_mut_ptr() });

    print!("M =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { message.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { message.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_str_ctr()
{
    println!("des_encrypt_str_ctr");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.";
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_ctr(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D397ED0899F8D53BB0418C7AE8505EDB48E9964DDFDC7BDD617347F37F85252F9EABABF5F1CF75FE7E88C352EA53B5F9");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_ctr(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text.as_str());
    println!("-------------------------------");
}

fn des_encrypt_string_ctr()
{
    println!("des_encrypt_string_ctr");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 64],
        pub uu64: [u64; 8],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    let message = "This is a test for DES. 잘 되는지 봅시다.".to_string();
    print!("M =\t");
    for val in message.as_bytes()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 8] };
    let length = a_des.encrypt_str_ctr(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "D397ED0899F8D53BB0418C7AE8505EDB48E9964DDFDC7BDD617347F37F85252F9EABABF5F1CF75FE7E88C352EA53B5F9");

    let mut decoded = Out { uu64: [0u64; 8] };
    let length = a_des.decrypt_ctr(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "546869732069732061207465737420666F72204445532E20EC9E9820EB9098EB8A94ECA78020EBB485EC8B9CEB8BA42E");

    let text = unsafe { String::from_utf8_unchecked(decoded.uu8.to_vec()) };
    println!("T =\t{}", text);
    println!("-------------------------------");
}

fn des_encrypt_array_ctr()
{
    println!("des_encrypt_array_ctr");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_ctr(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "86DCC11C303A6BF4C043CB5BCE420935BE310DC54761AA55");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ctr(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = [0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_array_ctr(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "86DCC11C303A6BF4C043CB5BCE420935BE310DC547");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ctr(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}

fn des_encrypt_vec_ctr()
{
    println!("des_encrypt_vec_ctr");
    use std::fmt::Write;
    use cryptocol::symmetric::DES;

    union Out {
        pub uu8: [u8; 32],
        pub uu64: [u64; 4],
    }

    let key = [0b00010011_u8, 0b00110100, 0b01010111, 0b01111001, 0b10011011, 0b10111100, 0b11011111, 0b11110001];
    print!("K =\t");
    for k in key
        { print!("{:08b} ", k); }
    println!();

    let iv = 12345678901234567890_u64;
    println!("IV =\t{:016X}", iv);
    
    // Fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xA8];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_ctr(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();
    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "86DCC11C303A6BF4C043CB5BCE420935BE310DC54761AA55");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ctr(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDDEEFFA8");

    // Not fit to block size
    let message = vec![0x01_u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD];
    print!("M =\t");
    for val in message.clone()
        { print!("{:02X}", val); }
    println!();

    let mut a_des = DES::new_with_key(key);
    let mut cipher = Out { uu64: [0u64; 4] };
    let length = a_des.encrypt_vec_ctr(iv, &message, unsafe { cipher.uu8.as_mut_ptr() });

    print!("C =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { cipher.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { cipher.uu8[i] }); }
    assert_eq!(txt, "86DCC11C303A6BF4C043CB5BCE420935BE310DC547");

    let mut decoded = Out { uu64: [0u64; 4] };
    let length = a_des.decrypt_ctr(iv, unsafe { cipher.uu8.as_ptr() }, length, unsafe { decoded.uu8.as_mut_ptr() });
    print!("D =\t");
    for i in 0..length as usize
        { print!("{:02X}", unsafe { decoded.uu8[i] }); }
    println!();

    let mut txt = String::new();
    for i in 0..length as usize
        { write!(&mut txt, "{:02X}", unsafe { decoded.uu8[i] }); }
    assert_eq!(txt, "0123456789ABCDEF112233445566778899AABBCCDD");
    println!("-------------------------------");
}
