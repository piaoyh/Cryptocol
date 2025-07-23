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
    aes_encrypt_ctr();
    // aes_encrypt_ctr_into_vec();
    // aes_encrypt_ctr_into_array();
    // aes_encrypt_str_ctr();
    // aes_encrypt_str_ctr_into_vec();
    // aes_encrypt_str_ctr_into_array();
    // aes_encrypt_string_ctr();
    // aes_encrypt_string_ctr_into_vec();
    // aes_encrypt_string_ctr_into_array();
    // aes_encrypt_vec_ctr();
    // aes_encrypt_vec_ctr_into_vec();
    // aes_encrypt_vec_ctr_into_array();
    // aes_encrypt_array_ctr();
    // aes_encrypt_array_ctr_into_vec();
    // aes_encrypt_array_ctr_into_array();

    aes_decrypt_ctr();
    // aes_decrypt_ctr_into_vec();
    // aes_decrypt_ctr_into_array();
    // aes_decrypt_ctr_into_string();
    // aes_decrypt_vec_ctr();
    // aes_decrypt_vec_ctr_into_vec();
    // aes_decrypt_vec_ctr_into_array();
    // aes_decrypt_vec_ctr_into_string();
    // aes_decrypt_array_ctr();
    // aes_decrypt_array_ctr_into_vec();
    // aes_decrypt_array_ctr_into_array();
    // aes_decrypt_array_ctr_into_string();
}

fn aes_encrypt_ctr()
{
    println!("aes_encrypt_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 9D 7C 52 E3 90 02 C5 7B EE BB 50 EE 3D 2C 3A 81 33 B7 54 77 35 FB 9F 48 6F 63 17 DC 5E 64 0E 29 4B 48 6D 53 24 CF D3 ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 4C DB 3F E9 F3 83 1E 7D D4 F7 5F 3D 14 28 B9 CE B0 11 61 53 FD E0 8F BC 2D 55 45 7E 24 C1 6A 9B 1A 82 94 4A 34 27 43 ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 79 17 99 06 B0 DD 3E 41 2B FF AD 70 F5 17 F4 65 D7 B0 AC FA 83 69 BF 8D C6 E6 6C 62 1C 5C 90 EA 45 D8 34 45 02 BE 33 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = Rijndael_256_256::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");
    println!();

    // // Normal case for the message of 0 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_aes = DES::new_with_key_u64(key);

    // let message = "";
    // println!("M =\t{}", message);
    // let nonce = 0x_FEDCBA0987654321_u64;
    // println!("Nonce =	{}", nonce);
    // let mut cipher = [0_u8; 0];
    // a_aes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "");
    // println!();

    // // Normal case for the message shorter than 8 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_aes = DES::new_with_key_u64(key);

    // let message = "7 bytes";
    // println!("M =\t{}", message);
    // let nonce = 0x_FEDCBA0987654321_u64;
    // println!("Nonce =	{}", nonce);
    // let mut cipher = [0_u8; 7];
    // a_aes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    // println!();

    // // Normal case for the message of 8 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_aes = DES::new_with_key_u64(key);

    // let message = "I am OK.";
    // println!("M =\t{}", message);
    // let nonce = 0x_FEDCBA0987654321_u64;
    // println!("Nonce =	{}", nonce);
    // let mut cipher = [0_u8; 8];
    // a_aes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    // println!();

    // // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_aes = DES::new_with_key_u64(key);

    // let message = "PARK Youngho";
    // println!("M =\t{}", message);
    // let nonce = 0x_FEDCBA0987654321_u64;
    // println!("Nonce =	{}", nonce);
    // let mut cipher = [0_u8; 12];
    // a_aes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    // println!();

    // // Normal case for the message of 16 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_aes = DES::new_with_key_u64(key);

    // let message = "고맙습니다.";
    // println!("M =\t{}", message);
    // let nonce = 0x_FEDCBA0987654321_u64;
    // println!("Nonce =	{}", nonce);
    // let mut cipher = [0_u8; 16];
    // a_aes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

/*
fn aes_encrypt_ctr_into_vec()
{
    println!("aes_encrypt_ctr_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    println!("K =\t{:#016X}", key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn aes_encrypt_ctr_into_array()
{
    println!("aes_encrypt_ctr_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn aes_encrypt_str_ctr()
{
    println!("aes_encrypt_str_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_str(nonce, &message, cipher1.as_mut_ptr());
    d_aes.encrypt_str(nonce, &message, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn aes_encrypt_str_ctr_into_vec()
{
    println!("aes_encrypt_str_ctr_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_str_into_vec(nonce, &message, &mut cipher1);
    d_aes.encrypt_str_into_vec(nonce, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn aes_encrypt_str_ctr_into_array()
{
    println!("aes_encrypt_str_ctr_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_str_into_array(nonce, &message, &mut cipher1);
    d_aes.encrypt_str_into_array(nonce, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_str_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_str_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_str_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_str_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_str_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn aes_encrypt_string_ctr()
{
    println!("aes_encrypt_string_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_string(nonce, &message, cipher1.as_mut_ptr());
    d_aes.encrypt_string(nonce, &message, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn aes_encrypt_string_ctr_into_vec()
{
    println!("aes_encrypt_string_ctr_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(nonce, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(nonce, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_string_into_vec(nonce, &message, &mut cipher1);
    d_aes.encrypt_string_into_vec(nonce, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn aes_encrypt_string_ctr_into_array()
{
    println!("aes_encrypt_string_ctr_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_string_into_array(nonce, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_string_into_array(nonce, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_string_into_array(nonce, &message, &mut cipher1);
    d_aes.encrypt_string_into_array(nonce, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_string_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_string_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_string_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_string_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.".to_string();
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_string_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_ctr()
{
    println!("aes_encrypt_vec_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K1 =\t{:#016X}", key1);
    println!("K2 =\t{:#016X}", key2);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_vec(nonce, &message, cipher1.as_mut_ptr());
    d_aes.encrypt_vec(nonce, &message, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_ctr_into_vec()
{
    println!("aes_encrypt_vec_ctr_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K1 =\t{:#016X}", key1);
    println!("K2 =\t{:#016X}", key2);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };

    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_vec_into_vec(nonce, &message, &mut cipher1);
    d_aes.encrypt_vec_into_vec(nonce, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_ctr_into_array()
{
    println!("aes_encrypt_vec_ctr_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_vec_into_array(nonce, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_vec_into_array(nonce, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K1 =\t{:#016X}", key1);
    println!("K2 =\t{:#016X}", key2);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_vec_into_array(nonce, &message, &mut cipher1);
    d_aes.encrypt_vec_into_array(nonce, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_vec_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_vec_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_vec_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_vec_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);
 
    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_vec_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn aes_encrypt_array_ctr()
{
    println!("aes_encrypt_array_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K1 =\t{:#016X}", key1);
    println!("K2 =\t{:#016X}", key2);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_array(nonce, &message, cipher1.as_mut_ptr());
    d_aes.encrypt_array(nonce, &message, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 0];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "7 bytes";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 7];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "I am OK.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 8];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "PARK Youngho";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 12];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "고맙습니다.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 16];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn aes_encrypt_array_ctr_into_vec()
{
    println!("aes_encrypt_array_ctr_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(nonce, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(nonce, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K1 =\t{:#016X}", key1);
    println!("K2 =\t{:#016X}", key2);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });

    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_array_into_vec(nonce, &message, &mut cipher1);
    d_aes.encrypt_array_into_vec(nonce, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 0];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "7 bytes";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 7];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "I am OK.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 8];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "PARK Youngho";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 12];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "고맙습니다.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 16];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn aes_encrypt_array_ctr_into_array()
{
    println!("aes_encrypt_array_ctr_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_array_into_array(nonce, &message, &mut cipher);
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_array_into_array(nonce, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K1 =\t{:#016X}", key1);
    println!("K2 =\t{:#016X}", key2);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_array_into_array(nonce, &message, &mut cipher1);
    d_aes.encrypt_array_into_array(nonce, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 0];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_array_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "7 bytes";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 7];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_array_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "I am OK.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 8];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_array_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "PARK Youngho";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 12];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_array_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);
 
    let mes = "고맙습니다.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 16];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_array_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}
*/

fn aes_decrypt_ctr()
{
    println!("aes_decrypt_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0], nonce[1], nonce[2], nonce[3]);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 9D 7C 52 E3 90 02 C5 7B EE BB 50 EE 3D 2C 3A 81 33 B7 54 77 35 FB 9F 48 6F 63 17 DC 5E 64 0E 29 4B 48 6D 53 24 CF D3 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 4C DB 3F E9 F3 83 1E 7D D4 F7 5F 3D 14 28 B9 CE B0 11 61 53 FD E0 8F BC 2D 55 45 7E 24 C1 6A 9B 1A 82 94 4A 34 27 43 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 79 17 99 06 B0 DD 3E 41 2B FF AD 70 F5 17 F4 65 D7 B0 AC FA 83 69 BF 8D C6 E6 6C 62 1C 5C 90 EA 45 D8 34 45 02 BE 33 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    // let mut a_aes = DES::new_with_key_u64(key);

    // let message = "";
    // println!("M =\t{}", message);
    // let nonce = 0x_FEDCBA0987654321_u64;
    // println!("Nonce =	{}", nonce);
    // let mut cipher = Vec::<u8>::new();
    // a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "");

    // let mut recovered = vec![0; 8];
    // let len = a_aes.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    // let mut a_aes = DES::new_with_key_u64(key);

    // let message = "7 bytes";
    // println!("M =\t{}", message);
    // let nonce = 0x_FEDCBA0987654321_u64;
    // println!("Nonce =	{}", nonce);
    // let mut cipher = Vec::<u8>::new();
    // a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    
    // let mut recovered = vec![0; 8];
    // let len = a_aes.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    // let mut a_aes = DES::new_with_key_u64(key);

    // let message = "I am OK.";
    // println!("M =\t{}", message);
    // let nonce = 0x_FEDCBA0987654321_u64;
    // println!("Nonce =	{}", nonce);
    // let mut cipher = Vec::<u8>::new();
    // a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    
    // let mut recovered = vec![0; 16];
    // let len = a_aes.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    // let mut a_aes = DES::new_with_key_u64(key);

    // let message = "PARK Youngho";
    // println!("M =\t{}", message);
    // let nonce = 0x_FEDCBA0987654321_u64;
    // println!("Nonce =	{}", nonce);
    // let mut cipher = Vec::<u8>::new();
    // a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

    // let mut recovered = vec![0; 16];
    // let len = a_aes.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    // let mut a_aes = DES::new_with_key_u64(key);

    // let message = "고맙습니다.";
    // println!("M =\t{}", message);
    // let nonce = 0x_FEDCBA0987654321_u64;
    // println!("Nonce =	{}", nonce);
    // let mut cipher = Vec::<u8>::new();
    // a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

    // let mut recovered = vec![0; 24];
    // let len = a_aes.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
fn aes_decrypt_ctr_into_vec()
{
    println!("aes_decrypt_ctr_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");

    let mut recovered1 = Vec::<u8>::new();
    let mut recovered2 = Vec::<u8>::new();
    c_aes.decrypt_into_vec(nonce, cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    d_aes.decrypt_into_vec(nonce, cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    
    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    
    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

fn aes_decrypt_ctr_into_array()
{
    println!("aes_decrypt_ctr_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

    let mut recovered = [0u8; 56];
    let len = a_aes.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

    let mut recovered = [0u8; 56];
    let len = a_aes.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");

    let mut recovered1 = [0u8; 56];
    let mut recovered2 = [0u8; 56];
    let len1 = c_aes.decrypt_into_array(nonce, cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    let len2 = d_aes.decrypt_into_array(nonce, cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = [0u8; 8];
    let len = a_aes.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);

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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");

    let mut recovered = [0u8; 8];
    let len = a_aes.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);

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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");

    let mut recovered = [0u8; 16];
    let len = a_aes.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);

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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

    let mut recovered = [0u8; 16];
    let len = a_aes.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);

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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

    let mut recovered = [0u8; 24];
    let len = a_aes.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);

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

fn aes_decrypt_ctr_into_string()
{
    println!("aes_decrypt_ctr_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

    let mut recovered = String::new();
    a_aes.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B (16 rounds) =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

    let mut recovered = String::new();
    a_aes.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B (128 rounds) =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");

    let mut recovered1 = String::new();
    let mut recovered2 = String::new();
    c_aes.decrypt_into_string(nonce, cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    d_aes.decrypt_into_string(nonce, cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = String::new();
    a_aes.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");

    let mut recovered = String::new();
    a_aes.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "7 bytes");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");

    let mut recovered = String::new();
    a_aes.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "I am OK.");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

    let mut recovered = String::new();
    a_aes.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "PARK Youngho");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

    let mut recovered = String::new();
    a_aes.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn aes_decrypt_vec_ctr()
{
    println!("aes_decrypt_vec_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");

    let mut recovered1 = vec![0; 55];
    let mut recovered2 = vec![0; 55];
    c_aes.decrypt_vec(nonce, &cipher1, recovered1.as_mut_ptr());
    d_aes.decrypt_vec(nonce, &cipher2, recovered2.as_mut_ptr());
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = vec![0; 8];
    let len = a_aes.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    
    let mut recovered = vec![0; 8];
    let len = a_aes.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    
    let mut recovered = vec![0; 16];
    let len = a_aes.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

    let mut recovered = vec![0; 16];
    let len = a_aes.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

    let mut recovered = vec![0; 24];
    let len = a_aes.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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

fn aes_decrypt_vec_ctr_into_vec()
{
    println!("aes_decrypt_vec_ctr_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");

    let mut recovered1 = Vec::<u8>::new();
    let mut recovered2 = Vec::<u8>::new();
    c_aes.decrypt_vec_into_vec(nonce, &cipher1, &mut recovered1);
    d_aes.decrypt_vec_into_vec(nonce, &cipher2, &mut recovered2);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    
    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    
    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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

fn aes_decrypt_vec_ctr_into_array()
{
    println!("aes_decrypt_vec_ctr_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

    let mut recovered = [0u8; 56];
    let len = a_aes.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
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
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

    let mut recovered = [0u8; 56];
    let len = a_aes.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
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
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");

    let mut recovered1 = [0u8; 56];
    let mut recovered2 = [0u8; 56];
    let len1 = c_aes.decrypt_vec_into_array(nonce, &cipher1, &mut recovered1);
    let len2 = d_aes.decrypt_vec_into_array(nonce, &cipher2, &mut recovered2);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = [0u8; 8];
    let len = a_aes.decrypt_vec_into_array(nonce, &cipher, &mut recovered);

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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");

    let mut recovered = [0u8; 8];
    let len = a_aes.decrypt_vec_into_array(nonce, &cipher, &mut recovered);

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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");

    let mut recovered = [0u8; 16];
    let len = a_aes.decrypt_vec_into_array(nonce, &cipher, &mut recovered);

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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

    let mut recovered = [0u8; 16];
    let len = a_aes.decrypt_vec_into_array(nonce, &cipher, &mut recovered);

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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

    let mut recovered = [0u8; 24];
    let len = a_aes.decrypt_vec_into_array(nonce, &cipher, &mut recovered);

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

fn aes_decrypt_vec_ctr_into_string()
{
    println!("aes_decrypt_vec_ctr_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

    let mut recovered = String::new();
    a_aes.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    println!("B (16 rounds) =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

    let mut recovered = String::new();
    a_aes.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    println!("B (128 rounds) =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");

    let mut recovered1 = String::new();
    let mut recovered2 = String::new();
    c_aes.decrypt_vec_into_string(nonce, &cipher1, &mut recovered1);
    d_aes.decrypt_vec_into_string(nonce, &cipher2, &mut recovered2);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = String::new();
    a_aes.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");

    let mut recovered = String::new();
    a_aes.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "7 bytes");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");

    let mut recovered = String::new();
    a_aes.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "I am OK.");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

    let mut recovered = String::new();
    a_aes.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "PARK Youngho");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

    let mut recovered = String::new();
    a_aes.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn aes_decrypt_array_ctr()
{
    println!("aes_decrypt_array_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

    let mut recovered = vec![0; 55];
    let len = a_aes.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

    let mut recovered = vec![0; 55];
    let len = a_aes.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");

    let mut recovered1 = vec![0; 55];
    let mut recovered2 = vec![0; 55];
    let len1 = c_aes.decrypt_array(nonce, &cipher1, recovered1.as_mut_ptr());
    let len2 = d_aes.decrypt_array(nonce, &cipher2, recovered2.as_mut_ptr());
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = vec![0; 8];
    let len = a_aes.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    
    let mut recovered = vec![0; 8];
    let len = a_aes.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    
    let mut recovered = vec![0; 16];
    let len = a_aes.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

    let mut recovered = vec![0; 16];
    let len = a_aes.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

    let mut recovered = vec![0; 24];
    let len = a_aes.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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

fn aes_decrypt_array_ctr_into_vec()
{
    println!("aes_decrypt_array_ctr_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");

    let mut recovered1 = Vec::<u8>::new();
    let mut recovered2 = Vec::<u8>::new();
    c_aes.decrypt_array_into_vec(nonce, &cipher1, &mut recovered1);
    d_aes.decrypt_array_into_vec(nonce, &cipher2, &mut recovered2);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    
    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    
    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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

fn aes_decrypt_array_ctr_into_array()
{
    println!("aes_decrypt_array_ctr_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

    let mut recovered = [0u8; 56];
    let len = a_aes.decrypt_array_into_array(nonce, &cipher, &mut recovered);
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
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

    let mut recovered = [0u8; 56];
    let len = a_aes.decrypt_array_into_array(nonce, &cipher, &mut recovered);
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
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");

    let mut recovered1 = [0u8; 56];
    let mut recovered2 = [0u8; 56];
    let len1 = c_aes.decrypt_array_into_array(nonce, &cipher1, &mut recovered1);
    let len2 = d_aes.decrypt_array_into_array(nonce, &cipher2, &mut recovered2);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = [0u8; 8];
    let len = a_aes.decrypt_array_into_array(nonce, &cipher, &mut recovered);

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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");

    let mut recovered = [0u8; 8];
    let len = a_aes.decrypt_array_into_array(nonce, &cipher, &mut recovered);

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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");

    let mut recovered = [0u8; 16];
    let len = a_aes.decrypt_array_into_array(nonce, &cipher, &mut recovered);

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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

    let mut recovered = [0u8; 16];
    let len = a_aes.decrypt_array_into_array(nonce, &cipher, &mut recovered);

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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

    let mut recovered = [0u8; 24];
    let len = a_aes.decrypt_array_into_array(nonce, &cipher, &mut recovered);

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

fn aes_decrypt_array_ctr_into_string()
{
    println!("aes_decrypt_array_ctr_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

    let mut recovered = String::new();
    a_aes.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    println!("B (16 rounds) =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

    let mut recovered = String::new();
    a_aes.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    println!("B (128 rounds) =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");

    let mut recovered1 = String::new();
    let mut recovered2 = String::new();
    c_aes.decrypt_array_into_string(nonce, &cipher1, &mut recovered1);
    d_aes.decrypt_array_into_string(nonce, &cipher2, &mut recovered2);
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
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = String::new();
    a_aes.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message shorter than 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");

    let mut recovered = String::new();
    a_aes.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "7 bytes");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");

    let mut recovered = String::new();
    a_aes.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "I am OK.");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

    let mut recovered = String::new();
    a_aes.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "PARK Youngho");
    assert_eq!(recovered, message);
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let nonce = 0x_FEDCBA0987654321_u64;
    println!("Nonce =	{}", nonce);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

    let mut recovered = String::new();
    a_aes.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}
*/
