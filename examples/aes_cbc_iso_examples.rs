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


// use std::io::Write;

// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]
// #[allow(non_camel_case_types)]
// #[allow(dead_code)]
pub fn main()
{
    aes_encrypt_with_padding_iso_cbc();
    // aes_encrypt_with_padding_iso_cbc_into_vec();
    // aes_encrypt_with_padding_iso_cbc_into_array();
    // aes_encrypt_str_with_padding_iso_cbc();
    // aes_encrypt_str_with_padding_iso_cbc_into_vec();
    // aes_encrypt_str_with_padding_iso_cbc_into_array();
    // aes_encrypt_string_with_padding_iso_cbc();
    // aes_encrypt_string_with_padding_iso_cbc_into_vec();
    // aes_encrypt_string_with_padding_iso_cbc_into_array();
    // aes_encrypt_vec_with_padding_iso_cbc();
    // aes_encrypt_vec_with_padding_iso_cbc_into_vec();
    // aes_encrypt_vec_with_padding_iso_cbc_into_array();
    // aes_encrypt_array_with_padding_iso_cbc();
    // aes_encrypt_array_with_padding_iso_cbc_into_vec();
    // aes_encrypt_array_with_padding_iso_cbc_into_array();

    aes_decrypt_with_padding_iso_cbc();
    // aes_decrypt_with_padding_iso_cbc_into_vec();
    // aes_decrypt_with_padding_iso_cbc_into_array();
    // aes_decrypt_with_padding_iso_cbc_into_string();
    // aes_decrypt_vec_with_padding_iso_cbc();
    // aes_decrypt_vec_with_padding_iso_cbc_into_vec();
    // aes_decrypt_vec_with_padding_iso_cbc_into_array();
    // aes_decrypt_vec_with_padding_iso_cbc_into_string();
    // aes_decrypt_array_with_padding_iso_cbc();
    // aes_decrypt_array_with_padding_iso_cbc_into_vec();
    // aes_decrypt_array_with_padding_iso_cbc_into_array();
    // aes_decrypt_array_with_padding_iso_cbc_into_string();
}

fn aes_encrypt_with_padding_iso_cbc()
{
    println!("aes_encrypt_with_padding_iso_cbc");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X}", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    println!();

    // // Normal case for the message of 0 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_des = DES::new_with_key_u64(key);

    // let message = "";
    // println!("M =\t{}", message);
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = [0_u8; 8];
    // a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");
    // println!();

    // // Normal case for the message shorter than 8 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_des = DES::new_with_key_u64(key);

    // let message = "7 bytes";
    // println!("M =\t{}", message);
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = [0_u8; 8];
    // a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    // println!();

    // // Normal case for the message of 8 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_des = DES::new_with_key_u64(key);

    // let message = "I am OK.";
    // println!("M =\t{}", message);
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = [0_u8; 16];
    // a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    // println!();

    // // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_des = DES::new_with_key_u64(key);

    // let message = "PARK Youngho";
    // println!("M =\t{}", message);
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = [0_u8; 16];
    // a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");
    // println!();

    // // Normal case for the message of 16 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_des = DES::new_with_key_u64(key);

    // let message = "고맙습니다.";
    // println!("M =\t{}", message);
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = [0_u8; 24];
    // a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");
    println!("-------------------------------");
}

/*
fn aes_encrypt_with_padding_iso_cbc_into_vec()
{
    println!("aes_encrypt_with_padding_iso_cbc_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    println!("K =\t{:#016X}", key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");
    println!("-------------------------------");
}

fn aes_encrypt_with_padding_iso_cbc_into_array()
{
    println!("aes_encrypt_with_padding_iso_cbc_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 56];
    let mut cipher2 = [0_u8; 56];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");
    println!();


    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 24];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");
    println!("-------------------------------");
}

fn aes_encrypt_str_with_padding_iso_cbc()
{
    println!("aes_encrypt_str_with_padding_iso_cbc");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 56];
    let mut cipher2 = [0_u8; 56];
    c_des.encrypt_str(iv, &message, cipher1.as_mut_ptr());
    d_des.encrypt_str(iv, &message, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");
    println!();


    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 24];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");
    println!("-------------------------------");
}

fn aes_encrypt_str_with_padding_iso_cbc_into_vec()
{
    println!("aes_encrypt_str_with_padding_iso_cbc_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_str_into_vec(iv, &message, &mut cipher1);
    d_des.encrypt_str_into_vec(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");
    println!();


    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");
    println!("-------------------------------");
}

fn aes_encrypt_str_with_padding_iso_cbc_into_array()
{
    println!("aes_encrypt_str_with_padding_iso_cbc_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 56];
    let mut cipher2 = [0_u8; 56];
    c_des.encrypt_str_into_array(iv, &message, &mut cipher1);
    d_des.encrypt_str_into_array(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");
    println!();


    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 24];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");
    println!("-------------------------------");
}

fn aes_encrypt_string_with_padding_iso_cbc()
{
    println!("aes_encrypt_string_with_padding_iso_cbc");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 56];
    let mut cipher2 = [0_u8; 56];
    c_des.encrypt_string(iv, &message, cipher1.as_mut_ptr());
    d_des.encrypt_string(iv, &message, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");
    println!();


    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 24];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");
    println!("-------------------------------");
}

fn aes_encrypt_string_with_padding_iso_cbc_into_vec()
{
    println!("aes_encrypt_string_with_padding_iso_cbc_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_string_into_vec(iv, &message, &mut cipher1);
    d_des.encrypt_string_into_vec(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");
    println!();


    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");
    println!("-------------------------------");
}

fn aes_encrypt_string_with_padding_iso_cbc_into_array()
{
    println!("aes_encrypt_string_with_padding_iso_cbc_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 56];
    let mut cipher2 = [0_u8; 56];
    c_des.encrypt_string_into_array(iv, &message, &mut cipher1);
    d_des.encrypt_string_into_array(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 24];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_with_padding_iso_cbc()
{
    println!("aes_encrypt_vec_with_padding_iso_cbc");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K1 =\t{:#016X}", key1);
    println!("K2 =\t{:#016X}", key2);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 56];
    let mut cipher2 = [0_u8; 56];
    c_des.encrypt_vec(iv, &message, cipher1.as_mut_ptr());
    d_des.encrypt_vec(iv, &message, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");
    println!();


    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 24];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_with_padding_iso_cbc_into_vec()
{
    println!("aes_encrypt_vec_with_padding_iso_cbc_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K1 =\t{:#016X}", key1);
    println!("K2 =\t{:#016X}", key2);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };

    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_vec_into_vec(iv, &message, &mut cipher1);
    d_des.encrypt_vec_into_vec(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");
    println!();


    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_with_padding_iso_cbc_into_array()
{
    println!("aes_encrypt_vec_with_padding_iso_cbc_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K1 =\t{:#016X}", key1);
    println!("K2 =\t{:#016X}", key2);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 56];
    let mut cipher2 = [0_u8; 56];
    c_des.encrypt_vec_into_array(iv, &message, &mut cipher1);
    d_des.encrypt_vec_into_array(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");
    println!();


    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);
 
    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 24];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");
    println!("-------------------------------");
}

fn aes_encrypt_array_with_padding_iso_cbc()
{
    println!("aes_encrypt_array_with_padding_iso_cbc");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K1 =\t{:#016X}", key1);
    println!("K2 =\t{:#016X}", key2);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 56];
    let mut cipher2 = [0_u8; 56];
    c_des.encrypt_array(iv, &message, cipher1.as_mut_ptr());
    d_des.encrypt_array(iv, &message, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 0];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "7 bytes";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 7];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "I am OK.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 8];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "PARK Youngho";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 12];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");
    println!();


    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "고맙습니다.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 16];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 24];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");
    println!("-------------------------------");
}

fn aes_encrypt_array_with_padding_iso_cbc_into_vec()
{
    println!("aes_encrypt_array_with_padding_iso_cbc_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K1 =\t{:#016X}", key1);
    println!("K2 =\t{:#016X}", key2);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });

    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_array_into_vec(iv, &message, &mut cipher1);
    d_des.encrypt_array_into_vec(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 0];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "7 bytes";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 7];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "I am OK.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 8];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "PARK Youngho";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 12];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");
    println!();


    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "고맙습니다.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 16];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");
    println!("-------------------------------");
}

fn aes_encrypt_array_with_padding_iso_cbc_into_array()
{
    println!("aes_encrypt_array_with_padding_iso_cbc_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K1 =\t{:#016X}", key1);
    println!("K2 =\t{:#016X}", key2);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 56];
    let mut cipher2 = [0_u8; 56];
    c_des.encrypt_array_into_array(iv, &message, &mut cipher1);
    d_des.encrypt_array_into_array(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 0];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "7 bytes";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 7];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "I am OK.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 8];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "PARK Youngho";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 12];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");
    println!();


    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);
 
    let mes = "고맙습니다.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 16];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 24];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");
    println!("-------------------------------");
}
*/

fn aes_decrypt_with_padding_iso_cbc()
{
    println!("aes_decrypt_with_padding_iso_cbc");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X}", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!();

    // // Normal case for the message of 0 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_des = DES::new_with_key_u64(key);

    // let message = "";
    // println!("M =\t{}", message);
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = Vec::<u8>::new();
    // a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");

    // let mut recovered = vec![0; 8];
    // let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    // print!("Ba =\t");
    // for b in recovered.clone()
    //     { print!("{:02X} ", b); }
    // println!();
    // let mut txt = String::new();
    // for c in recovered.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "00 00 00 00 00 00 00 00 ");

    // let mut converted = String::new();
    // unsafe { converted.as_mut_vec() }.append(&mut recovered);
    // converted.truncate(len as usize);
    
    // println!("Bb =\t{}", converted);
    // assert_eq!(converted, "");
    // assert_eq!(converted, message);
    // println!();

    // // Normal case for the message shorter than 8 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_des = DES::new_with_key_u64(key);

    // let message = "7 bytes";
    // println!("M =\t{}", message);
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = Vec::<u8>::new();
    // a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    
    // let mut recovered = vec![0; 8];
    // let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    // print!("Ba =\t");
    // for b in recovered.clone()
    //     { print!("{:02X} ", b); }
    // println!();
    // let mut txt = String::new();
    // for c in recovered.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "37 20 62 79 74 65 73 00 ");

    // let mut converted = String::new();
    // unsafe { converted.as_mut_vec() }.append(&mut recovered);
    // converted.truncate(len as usize);

    // println!("Bb =\t{}", converted);
    // assert_eq!(converted, "7 bytes");
    // assert_eq!(converted, message);
    // println!();

    // // Normal case for the message of 8 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_des = DES::new_with_key_u64(key);

    // let message = "I am OK.";
    // println!("M =\t{}", message);
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = Vec::<u8>::new();
    // a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    
    // let mut recovered = vec![0; 16];
    // let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    // print!("Ba =\t");
    // for b in recovered.clone()
    //     { print!("{:02X} ", b); }
    // println!();
    // let mut txt = String::new();
    // for c in recovered.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "49 20 61 6D 20 4F 4B 2E 00 00 00 00 00 00 00 00 ");

    // let mut converted = String::new();
    // unsafe { converted.as_mut_vec() }.append(&mut recovered);
    // converted.truncate(len as usize);
    
    // println!("Bb =\t{}", converted);
    // assert_eq!(converted, "I am OK.");
    // assert_eq!(converted, message);
    // println!();

    // // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_des = DES::new_with_key_u64(key);

    // let message = "PARK Youngho";
    // println!("M =\t{}", message);
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = Vec::<u8>::new();
    // a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");

    // let mut recovered = vec![0; 16];
    // let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    // print!("Ba =\t");
    // for b in recovered.clone()
    //     { print!("{:02X} ", b); }
    // println!();
    // let mut txt = String::new();
    // for c in recovered.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "50 41 52 4B 20 59 6F 75 6E 67 68 6F 00 00 00 00 ");

    // let mut converted = String::new();
    // unsafe { converted.as_mut_vec() }.append(&mut recovered);
    // converted.truncate(len as usize);
    
    // println!("Bb =\t{}", converted);
    // assert_eq!(converted, "PARK Youngho");
    // assert_eq!(converted, message);
    // println!();


    // // Normal case for the message of 16 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_des = DES::new_with_key_u64(key);

    // let message = "고맙습니다.";
    // println!("M =\t{}", message);
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = Vec::<u8>::new();
    // a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");

    // let mut recovered = vec![0; 24];
    // let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    // print!("Ba =\t");
    // for b in recovered.clone()
    //     { print!("{:02X} ", b); }
    // println!();
    // let mut txt = String::new();
    // for c in recovered.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "EA B3 A0 EB A7 99 EC 8A B5 EB 8B 88 EB 8B A4 2E 00 00 00 00 00 00 00 00 ");

    // let mut converted = String::new();
    // unsafe { converted.as_mut_vec() }.append(&mut recovered);
    // converted.truncate(len as usize);
    
    // println!("Bb =\t{}", converted);
    // assert_eq!(converted, "고맙습니다.");
    // assert_eq!(converted, message);
    println!("-------------------------------");
}

/*
fn aes_decrypt_with_padding_iso_cbc_into_vec()
{
    println!("aes_decrypt_with_padding_iso_cbc_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");

    let mut recovered1 = Vec::<u8>::new();
    let mut recovered2 = Vec::<u8>::new();
    c_des.decrypt_into_vec(iv, cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    d_des.decrypt_into_vec(iv, cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    
    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    
    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

fn aes_decrypt_with_padding_iso_cbc_into_array()
{
    println!("aes_decrypt_with_padding_iso_cbc_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");

    let mut recovered = [0u8; 56];
    let len = a_des.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");

    let mut recovered = [0u8; 56];
    let len = a_des.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");

    let mut recovered1 = [0u8; 56];
    let mut recovered2 = [0u8; 56];
    let len1 = c_des.decrypt_into_array(iv, cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    let len2 = d_des.decrypt_into_array(iv, cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");

    let mut recovered = [0u8; 8];
    let len = a_des.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);

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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");

    let mut recovered = [0u8; 8];
    let len = a_des.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);

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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");

    let mut recovered = [0u8; 16];
    let len = a_des.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);

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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");

    let mut recovered = [0u8; 16];
    let len = a_des.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);

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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");

    let mut recovered = [0u8; 24];
    let len = a_des.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);

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

fn aes_decrypt_with_padding_iso_cbc_into_string()
{
    println!("aes_decrypt_with_padding_iso_cbc_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");

    let mut recovered = String::new();
    a_des.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B (16 rounds) =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");

    let mut recovered = String::new();
    a_des.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B (128 rounds) =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");

    let mut recovered1 = String::new();
    let mut recovered2 = String::new();
    c_des.decrypt_into_string(iv, cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    d_des.decrypt_into_string(iv, cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");

    let mut recovered = String::new();
    a_des.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");

    let mut recovered = String::new();
    a_des.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "7 bytes");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");

    let mut recovered = String::new();
    a_des.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "I am OK.");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");

    let mut recovered = String::new();
    a_des.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "PARK Youngho");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");

    let mut recovered = String::new();
    a_des.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn aes_decrypt_vec_with_padding_iso_cbc()
{
    println!("aes_decrypt_vec_with_padding_iso_cbc");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");

    let mut recovered = vec![0; 55];
    a_des.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");

    let mut recovered = vec![0; 55];
    a_des.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");

    let mut recovered1 = vec![0; 55];
    let mut recovered2 = vec![0; 55];
    c_des.decrypt_vec(iv, &cipher1, recovered1.as_mut_ptr());
    d_des.decrypt_vec(iv, &cipher2, recovered2.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");

    let mut recovered = vec![0; 8];
    let len = a_des.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    
    let mut recovered = vec![0; 8];
    let len = a_des.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    
    let mut recovered = vec![0; 16];
    let len = a_des.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");

    let mut recovered = vec![0; 16];
    let len = a_des.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");

    let mut recovered = vec![0; 24];
    let len = a_des.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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

fn aes_decrypt_vec_with_padding_iso_cbc_into_vec()
{
    println!("aes_decrypt_vec_with_padding_iso_cbc_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");

    let mut recovered1 = Vec::<u8>::new();
    let mut recovered2 = Vec::<u8>::new();
    c_des.decrypt_vec_into_vec(iv, &cipher1, &mut recovered1);
    d_des.decrypt_vec_into_vec(iv, &cipher2, &mut recovered2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    
    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    
    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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

fn aes_decrypt_vec_with_padding_iso_cbc_into_array()
{
    println!("aes_decrypt_vec_with_padding_iso_cbc_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");

    let mut recovered = [0u8; 56];
    let len = a_des.decrypt_vec_into_array(iv, &cipher, &mut recovered);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");

    let mut recovered = [0u8; 56];
    let len = a_des.decrypt_vec_into_array(iv, &cipher, &mut recovered);
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
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");

    let mut recovered1 = [0u8; 56];
    let mut recovered2 = [0u8; 56];
    let len1 = c_des.decrypt_vec_into_array(iv, &cipher1, &mut recovered1);
    let len2 = d_des.decrypt_vec_into_array(iv, &cipher2, &mut recovered2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");

    let mut recovered = [0u8; 8];
    let len = a_des.decrypt_vec_into_array(iv, &cipher, &mut recovered);

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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");

    let mut recovered = [0u8; 8];
    let len = a_des.decrypt_vec_into_array(iv, &cipher, &mut recovered);

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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");

    let mut recovered = [0u8; 16];
    let len = a_des.decrypt_vec_into_array(iv, &cipher, &mut recovered);

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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");

    let mut recovered = [0u8; 16];
    let len = a_des.decrypt_vec_into_array(iv, &cipher, &mut recovered);

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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");

    let mut recovered = [0u8; 24];
    let len = a_des.decrypt_vec_into_array(iv, &cipher, &mut recovered);

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

fn aes_decrypt_vec_with_padding_iso_cbc_into_string()
{
    println!("aes_decrypt_vec_with_padding_iso_cbc_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");

    let mut recovered = String::new();
    a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B (16 rounds) =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");

    let mut recovered = String::new();
    a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B (128 rounds) =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");

    let mut recovered1 = String::new();
    let mut recovered2 = String::new();
    c_des.decrypt_vec_into_string(iv, &cipher1, &mut recovered1);
    d_des.decrypt_vec_into_string(iv, &cipher2, &mut recovered2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");

    let mut recovered = String::new();
    a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");

    let mut recovered = String::new();
    a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "7 bytes");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");

    let mut recovered = String::new();
    a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "I am OK.");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");

    let mut recovered = String::new();
    a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "PARK Youngho");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");

    let mut recovered = String::new();
    a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn aes_decrypt_array_with_padding_iso_cbc()
{
    println!("aes_decrypt_array_with_padding_iso_cbc");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");

    let mut recovered = vec![0; 55];
    let len = a_des.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
    recovered.truncate(len as usize);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");

    let mut recovered = vec![0; 55];
    let len = a_des.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
    recovered.truncate(len as usize);
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
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 56];
    let mut cipher2 = [0_u8; 56];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");

    let mut recovered1 = vec![0; 55];
    let mut recovered2 = vec![0; 55];
    let len1 = c_des.decrypt_array(iv, &cipher1, recovered1.as_mut_ptr());
    let len2 = d_des.decrypt_array(iv, &cipher2, recovered2.as_mut_ptr());
    recovered1.truncate(len1 as usize);
    recovered2.truncate(len2 as usize);

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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");

    let mut recovered = vec![0; 8];
    let len = a_des.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
    recovered.truncate(len as usize);

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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    
    let mut recovered = vec![0; 8];
    let len = a_des.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
    recovered.truncate(len as usize);

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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    
    let mut recovered = vec![0; 16];
    let len = a_des.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
    recovered.truncate(len as usize);

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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");

    let mut recovered = vec![0; 16];
    let len = a_des.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
    recovered.truncate(len as usize);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 24];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");

    let mut recovered = vec![0; 24];
    let len = a_des.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
    recovered.truncate(len as usize);

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

fn aes_decrypt_array_with_padding_iso_cbc_into_vec()
{
    println!("aes_decrypt_array_with_padding_iso_cbc_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 56];
    let mut cipher2 = [0_u8; 56];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");

    let mut recovered1 = Vec::<u8>::new();
    let mut recovered2 = Vec::<u8>::new();
    c_des.decrypt_array_into_vec(iv, &cipher1, &mut recovered1);
    d_des.decrypt_array_into_vec(iv, &cipher2, &mut recovered2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");
    
    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");
    
    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 24];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");

    let mut recovered = Vec::<u8>::new();
    a_des.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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

fn aes_decrypt_array_with_padding_iso_cbc_into_array()
{
    println!("aes_decrypt_array_with_padding_iso_cbc_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");

    let mut recovered = [0u8; 56];
    let len = a_des.decrypt_array_into_array(iv, &cipher, &mut recovered);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");

    let mut recovered = [0u8; 56];
    let len = a_des.decrypt_array_into_array(iv, &cipher, &mut recovered);
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
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 56];
    let mut cipher2 = [0_u8; 56];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");

    let mut recovered1 = [0u8; 56];
    let mut recovered2 = [0u8; 56];
    let len1 = c_des.decrypt_array_into_array(iv, &cipher1, &mut recovered1);
    let len2 = d_des.decrypt_array_into_array(iv, &cipher2, &mut recovered2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");

    let mut recovered = [0u8; 8];
    let len = a_des.decrypt_array_into_array(iv, &cipher, &mut recovered);

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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");

    let mut recovered = [0u8; 8];
    let len = a_des.decrypt_array_into_array(iv, &cipher, &mut recovered);

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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");

    let mut recovered = [0u8; 16];
    let len = a_des.decrypt_array_into_array(iv, &cipher, &mut recovered);

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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");

    let mut recovered = [0u8; 16];
    let len = a_des.decrypt_array_into_array(iv, &cipher, &mut recovered);

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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 24];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");

    let mut recovered = [0u8; 24];
    let len = a_des.decrypt_array_into_array(iv, &cipher, &mut recovered);

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

fn aes_decrypt_array_with_padding_iso_cbc_into_string()
{
    println!("aes_decrypt_array_with_padding_iso_cbc_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CBC_ISO };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 11 78 69 FB 0B 32 CF 92 ");

    let mut recovered = String::new();
    a_des.decrypt_array_into_string(iv, &cipher, &mut recovered);
    println!("B (16 rounds) =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 56];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 64 5B 06 90 34 4F FE E0 ");

    let mut recovered = String::new();
    a_des.decrypt_array_into_string(iv, &cipher, &mut recovered);
    println!("B (128 rounds) =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 56];
    let mut cipher2 = [0_u8; 56];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 62 ");

    let mut recovered1 = String::new();
    let mut recovered2 = String::new();
    c_des.decrypt_array_into_string(iv, &cipher1, &mut recovered1);
    d_des.decrypt_array_into_string(iv, &cipher2, &mut recovered2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6A 9B 70 EA 3D 21 44 0D ");

    let mut recovered = String::new();
    a_des.decrypt_array_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D 67 D6 FC A6 8C 44 6C ");

    let mut recovered = String::new();
    a_des.decrypt_array_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "7 bytes");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 2F C4 F5 DB 1B 76 88 FD ");

    let mut recovered = String::new();
    a_des.decrypt_array_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "I am OK.");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 22 91 AF 6D B3 27 BB 70 ");

    let mut recovered = String::new();
    a_des.decrypt_array_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "PARK Youngho");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 24];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 69 95 9B 38 AF 07 33 CF ");

    let mut recovered = String::new();
    a_des.decrypt_array_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}
*/