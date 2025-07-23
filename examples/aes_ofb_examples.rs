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


pub fn main()
{
    aes_encrypt_ofb();
    // aes_encrypt_ofb_into_vec();
    // aes_encrypt_ofb_into_array();
    // aes_encrypt_str_ofb();
    // aes_encrypt_str_ofb_into_vec();
    // aes_encrypt_str_ofb_into_array();
    // aes_encrypt_string_ofb();
    // aes_encrypt_string_ofb_into_vec();
    // aes_encrypt_string_ofb_into_array();
    // aes_encrypt_vec_ofb();
    // aes_encrypt_vec_ofb_into_vec();
    // aes_encrypt_vec_ofb_into_array();
    // aes_encrypt_array_ofb();
    // aes_encrypt_array_ofb_into_vec();
    // aes_encrypt_array_ofb_into_array();

    aes_decrypt_ofb();
    // aes_decrypt_ofb_into_vec();
    // aes_decrypt_ofb_into_array();
    // aes_decrypt_ofb_into_string();
    // aes_decrypt_vec_ofb();
    // aes_decrypt_vec_ofb_into_vec();
    // aes_decrypt_vec_ofb_into_array();
    // aes_decrypt_vec_ofb_into_string();
    // aes_decrypt_array_ofb();
    // aes_decrypt_array_ofb_into_vec();
    // aes_decrypt_array_ofb_into_array();
    // aes_decrypt_array_ofb_into_string();
}

fn aes_encrypt_ofb()
{
    println!("aes_encrypt_ofb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
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
    let mut cipher = [0_u8; 55];
    a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
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
    let mut cipher = [0_u8; 55];
    a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = Rijndael_256_256::new_with_key(&key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
    println!();

    // // Normal case for the message of 0 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_aes = DES::new_with_key_u64(key);

    // let message = "";
    // println!("M =\t{}", message);
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = [0_u8; 0];
    // a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = [0_u8; 7];
    // a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    // println!();

    // // Normal case for the message of 8 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_aes = DES::new_with_key_u64(key);

    // let message = "I am OK.";
    // println!("M =\t{}", message);
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = [0_u8; 8];
    // a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    // println!();

    // // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_aes = DES::new_with_key_u64(key);

    // let message = "PARK Youngho";
    // println!("M =\t{}", message);
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = [0_u8; 12];
    // a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
    // println!();

    // // Normal case for the message of 16 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_aes = DES::new_with_key_u64(key);

    // let message = "고맙습니다.";
    // println!("M =\t{}", message);
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = [0_u8; 16];
    // a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

/*
fn aes_encrypt_ofb_into_vec()
{
    println!("aes_encrypt_ofb_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    println!("K =\t{:#016X}", key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn aes_encrypt_ofb_into_array()
{
    println!("aes_encrypt_ofb_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn aes_encrypt_str_ofb()
{
    println!("aes_encrypt_str_ofb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_str(iv, &message, cipher1.as_mut_ptr());
    d_aes.encrypt_str(iv, &message, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_str(iv, &message, cipher.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn aes_encrypt_str_ofb_into_vec()
{
    println!("aes_encrypt_str_ofb_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_str_into_vec(iv, &message, &mut cipher1);
    d_aes.encrypt_str_into_vec(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn aes_encrypt_str_ofb_into_array()
{
    println!("aes_encrypt_str_ofb_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_str_into_array(iv, &message, &mut cipher1);
    d_aes.encrypt_str_into_array(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn aes_encrypt_string_ofb()
{
    println!("aes_encrypt_string_ofb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_string(iv, &message, cipher1.as_mut_ptr());
    d_aes.encrypt_string(iv, &message, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_string(iv, &message, cipher.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn aes_encrypt_string_ofb_into_vec()
{
    println!("aes_encrypt_string_ofb_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_string_into_vec(iv, &message, &mut cipher1);
    d_aes.encrypt_string_into_vec(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(iv, &message, &mut cipher);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn aes_encrypt_string_ofb_into_array()
{
    println!("aes_encrypt_string_ofb_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
    println!();

    // Expanded case for 0 rounds which means that key is meaningless
    let key1 = 0x_1234567890ABCDEF_u64;
    let key2 = 0_u64;
    println!("K =\t{:#016X}", key);
    let mut c_aes = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_aes = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_string_into_array(iv, &message, &mut cipher1);
    d_aes.encrypt_string_into_array(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_string_into_array(iv, &message, &mut cipher);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_ofb()
{
    println!("aes_encrypt_vec_ofb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_vec(iv, &message, cipher1.as_mut_ptr());
    d_aes.encrypt_vec(iv, &message, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_ofb_into_vec()
{
    println!("aes_encrypt_vec_ofb_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
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

    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_vec_into_vec(iv, &message, &mut cipher1);
    d_aes.encrypt_vec_into_vec(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(iv, &message, &mut cipher);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_ofb_into_array()
{
    println!("aes_encrypt_vec_ofb_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_vec_into_array(iv, &message, &mut cipher1);
    d_aes.encrypt_vec_into_array(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_vec_into_array(iv, &message, &mut cipher);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);
 
    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn aes_encrypt_array_ofb()
{
    println!("aes_encrypt_array_ofb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_array(iv, &message, cipher1.as_mut_ptr());
    d_aes.encrypt_array(iv, &message, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 0];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_array(iv, &message, cipher.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "I am OK.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 8];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "PARK Youngho";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 12];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "고맙습니다.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 16];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn aes_encrypt_array_ofb_into_vec()
{
    println!("aes_encrypt_array_ofb_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
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

    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_array_into_vec(iv, &message, &mut cipher1);
    d_aes.encrypt_array_into_vec(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 0];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(iv, &message, &mut cipher);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "I am OK.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 8];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "PARK Youngho";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 12];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "고맙습니다.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 16];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn aes_encrypt_array_ofb_into_array()
{
    println!("aes_encrypt_array_ofb_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_array_into_array(iv, &message, &mut cipher);
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_array_into_array(iv, &message, &mut cipher1);
    d_aes.encrypt_array_into_array(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 0];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_array_into_array(iv, &message, &mut cipher);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    println!();

    // Normal case for the message of 8 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "I am OK.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 8];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let mes = "PARK Youngho";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 12];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);
 
    let mes = "고맙습니다.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 16];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}
*/

fn aes_decrypt_ofb()
{
    println!("aes_decrypt_ofb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0], iv[1], iv[2], iv[3]);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");

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
    let mut cipher = [0_u8; 55];
    a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");

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
    let mut cipher = [0_u8; 55];
    a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");

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
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");

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

    // // Normal case for the message of 8 bytes
    // let key = 0x_1234567890ABCDEF_u64;
    // println!("K =\t{:#016X}", key);
    // let mut a_aes = DES::new_with_key_u64(key);

    // let message = "I am OK.";
    // println!("M =\t{}", message);
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = Vec::<u8>::new();
    // a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
    // let mut recovered = vec![0; 16];
    // let len = a_aes.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = Vec::<u8>::new();
    // a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

    // let mut recovered = vec![0; 16];
    // let len = a_aes.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    // let iv = 0x_FEDCBA0987654321_u64;
    // println!("IV =	{}", iv);
    // let mut cipher = Vec::<u8>::new();
    // a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    // print!("C =\t");
    // for c in cipher.clone()
    //     { print!("{:02X} ", c); }
    // println!();
    // let mut txt = String::new();
    // for c in cipher.clone()
    //     { write!(txt, "{:02X} ", c); }
    // assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

    // let mut recovered = vec![0; 24];
    // let len = a_aes.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
fn aes_decrypt_ofb_into_vec()
{
    println!("aes_decrypt_ofb_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");

    let mut recovered1 = Vec::<u8>::new();
    let mut recovered2 = Vec::<u8>::new();
    c_aes.decrypt_into_vec(iv, cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    d_aes.decrypt_into_vec(iv, cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

fn aes_decrypt_ofb_into_array()
{
    println!("aes_decrypt_ofb_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

    let mut recovered = [0u8; 56];
    let len = a_aes.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

    let mut recovered = [0u8; 56];
    let len = a_aes.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");

    let mut recovered1 = [0u8; 56];
    let mut recovered2 = [0u8; 56];
    let len1 = c_aes.decrypt_into_array(iv, cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    let len2 = d_aes.decrypt_into_array(iv, cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = [0u8; 8];
    let len = a_aes.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);

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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

    let mut recovered = [0u8; 8];
    let len = a_aes.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);

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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

    let mut recovered = [0u8; 16];
    let len = a_aes.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);

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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

    let mut recovered = [0u8; 16];
    let len = a_aes.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);

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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

    let mut recovered = [0u8; 24];
    let len = a_aes.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);

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

fn aes_decrypt_ofb_into_string()
{
    println!("aes_decrypt_ofb_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

    let mut recovered = String::new();
    a_aes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

    let mut recovered = String::new();
    a_aes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");

    let mut recovered1 = String::new();
    let mut recovered2 = String::new();
    c_aes.decrypt_into_string(iv, cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    d_aes.decrypt_into_string(iv, cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = String::new();
    a_aes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

    let mut recovered = String::new();
    a_aes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

    let mut recovered = String::new();
    a_aes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

    let mut recovered = String::new();
    a_aes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

    let mut recovered = String::new();
    a_aes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn aes_decrypt_vec_ofb()
{
    println!("aes_decrypt_vec_ofb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");

    let mut recovered1 = vec![0; 55];
    let mut recovered2 = vec![0; 55];
    c_aes.decrypt_vec(iv, &cipher1, recovered1.as_mut_ptr());
    d_aes.decrypt_vec(iv, &cipher2, recovered2.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = vec![0; 8];
    let len = a_aes.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
    let mut recovered = vec![0; 8];
    let len = a_aes.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
    let mut recovered = vec![0; 16];
    let len = a_aes.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

    let mut recovered = vec![0; 16];
    let len = a_aes.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

    let mut recovered = vec![0; 24];
    let len = a_aes.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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

fn aes_decrypt_vec_ofb_into_vec()
{
    println!("aes_decrypt_vec_ofb_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");

    let mut recovered1 = Vec::<u8>::new();
    let mut recovered2 = Vec::<u8>::new();
    c_aes.decrypt_vec_into_vec(iv, &cipher1, &mut recovered1);
    d_aes.decrypt_vec_into_vec(iv, &cipher2, &mut recovered2);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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

fn aes_decrypt_vec_ofb_into_array()
{
    println!("aes_decrypt_vec_ofb_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

    let mut recovered = [0u8; 56];
    let len = a_aes.decrypt_vec_into_array(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

    let mut recovered = [0u8; 56];
    let len = a_aes.decrypt_vec_into_array(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");

    let mut recovered1 = [0u8; 56];
    let mut recovered2 = [0u8; 56];
    let len1 = c_aes.decrypt_vec_into_array(iv, &cipher1, &mut recovered1);
    let len2 = d_aes.decrypt_vec_into_array(iv, &cipher2, &mut recovered2);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = [0u8; 8];
    let len = a_aes.decrypt_vec_into_array(iv, &cipher, &mut recovered);

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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

    let mut recovered = [0u8; 8];
    let len = a_aes.decrypt_vec_into_array(iv, &cipher, &mut recovered);

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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

    let mut recovered = [0u8; 16];
    let len = a_aes.decrypt_vec_into_array(iv, &cipher, &mut recovered);

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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

    let mut recovered = [0u8; 16];
    let len = a_aes.decrypt_vec_into_array(iv, &cipher, &mut recovered);

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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

    let mut recovered = [0u8; 24];
    let len = a_aes.decrypt_vec_into_array(iv, &cipher, &mut recovered);

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

fn aes_decrypt_vec_ofb_into_string()
{
    println!("aes_decrypt_vec_ofb_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

    let mut recovered = String::new();
    a_aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

    let mut recovered = String::new();
    a_aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = Vec::<u8>::new();
    let mut cipher2 = Vec::<u8>::new();
    c_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");

    let mut recovered1 = String::new();
    let mut recovered2 = String::new();
    c_aes.decrypt_vec_into_string(iv, &cipher1, &mut recovered1);
    d_aes.decrypt_vec_into_string(iv, &cipher2, &mut recovered2);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = String::new();
    a_aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

    let mut recovered = String::new();
    a_aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

    let mut recovered = String::new();
    a_aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

    let mut recovered = String::new();
    a_aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

    let mut recovered = String::new();
    a_aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn aes_decrypt_array_ofb()
{
    println!("aes_decrypt_array_ofb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

    let mut recovered = vec![0; 55];
    let len = a_aes.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

    let mut recovered = vec![0; 55];
    let len = a_aes.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");

    let mut recovered1 = vec![0; 55];
    let mut recovered2 = vec![0; 55];
    let len1 = c_aes.decrypt_array(iv, &cipher1, recovered1.as_mut_ptr());
    let len2 = d_aes.decrypt_array(iv, &cipher2, recovered2.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = vec![0; 8];
    let len = a_aes.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
    let mut recovered = vec![0; 8];
    let len = a_aes.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
    let mut recovered = vec![0; 16];
    let len = a_aes.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

    let mut recovered = vec![0; 16];
    let len = a_aes.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

    let mut recovered = vec![0; 24];
    let len = a_aes.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
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

fn aes_decrypt_array_ofb_into_vec()
{
    println!("aes_decrypt_array_ofb_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");

    let mut recovered1 = Vec::<u8>::new();
    let mut recovered2 = Vec::<u8>::new();
    c_aes.decrypt_array_into_vec(iv, &cipher1, &mut recovered1);
    d_aes.decrypt_array_into_vec(iv, &cipher2, &mut recovered2);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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

fn aes_decrypt_array_ofb_into_array()
{
    println!("aes_decrypt_array_ofb_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

    let mut recovered = [0u8; 56];
    let len = a_aes.decrypt_array_into_array(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

    let mut recovered = [0u8; 56];
    let len = a_aes.decrypt_array_into_array(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");

    let mut recovered1 = [0u8; 56];
    let mut recovered2 = [0u8; 56];
    let len1 = c_aes.decrypt_array_into_array(iv, &cipher1, &mut recovered1);
    let len2 = d_aes.decrypt_array_into_array(iv, &cipher2, &mut recovered2);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = [0u8; 8];
    let len = a_aes.decrypt_array_into_array(iv, &cipher, &mut recovered);

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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

    let mut recovered = [0u8; 8];
    let len = a_aes.decrypt_array_into_array(iv, &cipher, &mut recovered);

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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

    let mut recovered = [0u8; 16];
    let len = a_aes.decrypt_array_into_array(iv, &cipher, &mut recovered);

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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

    let mut recovered = [0u8; 16];
    let len = a_aes.decrypt_array_into_array(iv, &cipher, &mut recovered);

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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

    let mut recovered = [0u8; 24];
    let len = a_aes.decrypt_array_into_array(iv, &cipher, &mut recovered);

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

fn aes_decrypt_array_ofb_into_string()
{
    println!("aes_decrypt_array_ofb_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_aes = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

    let mut recovered = String::new();
    a_aes.decrypt_array_into_string(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

    let mut recovered = String::new();
    a_aes.decrypt_array_into_string(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F 44 24 0C E9 67 D3 B2 99 32 C4 F5 2F 26 16 9E 98 40 37 00 E3 29 CE B4 9B 32 EB FF 2A 70 10 82 8E 01 22 0B E3 29 CE B4 9B 32 E6 FB 39 72 1D C2 ");

    let mut recovered1 = String::new();
    let mut recovered2 = String::new();
    c_aes.decrypt_array_into_string(iv, &cipher1, &mut recovered1);
    d_aes.decrypt_array_into_string(iv, &cipher2, &mut recovered2);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

    let mut recovered = String::new();
    a_aes.decrypt_array_into_string(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

    let mut recovered = String::new();
    a_aes.decrypt_array_into_string(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

    let mut recovered = String::new();
    a_aes.decrypt_array_into_string(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

    let mut recovered = String::new();
    a_aes.decrypt_array_into_string(iv, &cipher, &mut recovered);
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
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

    let mut recovered = String::new();
    a_aes.decrypt_array_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}
*/
