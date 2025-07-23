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
    aes_encrypt_with_padding_pkcs7_pcbc();
    // aes_encrypt_with_padding_pkcs7_pcbc_into_vec();
    // aes_encrypt_with_padding_pkcs7_pcbc_into_array();
    // aes_encrypt_str_with_padding_pkcs7_pcbc();
    // aes_encrypt_str_with_padding_pkcs7_pcbc_into_vec();
    // aes_encrypt_str_with_padding_pkcs7_pcbc_into_array();
    // aes_encrypt_string_with_padding_pkcs7_pcbc();
    // aes_encrypt_string_with_padding_pkcs7_pcbc_into_vec();
    // aes_encrypt_string_with_padding_pkcs7_pcbc_into_array();
    // aes_encrypt_vec_with_padding_pkcs7_pcbc();
    // aes_encrypt_vec_with_padding_pkcs7_pcbc_into_vec();
    // aes_encrypt_vec_with_padding_pkcs7_pcbc_into_array();
    // aes_encrypt_array_with_padding_pkcs7_pcbc();
    // aes_encrypt_array_with_padding_pkcs7_pcbc_into_vec();
    // aes_encrypt_array_with_padding_pkcs7_pcbc_into_array();

    aes_decrypt_with_padding_pkcs7_pcbc();
    // aes_decrypt_with_padding_pkcs7_pcbc_into_vec();
    // aes_decrypt_with_padding_pkcs7_pcbc_into_array();
    // aes_decrypt_with_padding_pkcs7_pcbc_into_string();
    // aes_decrypt_vec_with_padding_pkcs7_pcbc();
    // aes_decrypt_vec_with_padding_pkcs7_pcbc_into_vec();
    // aes_decrypt_vec_with_padding_pkcs7_pcbc_into_array();
    // aes_decrypt_vec_with_padding_pkcs7_pcbc_into_string();
    // aes_decrypt_array_with_padding_pkcs7_pcbc();
    // aes_decrypt_array_with_padding_pkcs7_pcbc_into_vec();
    // aes_decrypt_array_with_padding_pkcs7_pcbc_into_array();
    // aes_decrypt_array_with_padding_pkcs7_pcbc_into_string();
}

fn aes_encrypt_with_padding_pkcs7_pcbc()
{
    println!("aes_encrypt_with_padding_pkcs7_pcbc");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 89 BB 67 77 F3 56 67 3A 6B DE 56 C0 63 78 7C 95 ");
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
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C DF 4B 11 43 90 8D 06 A0 07 52 17 31 8F CB 2F 7D EC A2 3F 20 15 3C 88 DD E7 0D 54 74 BC A2 AE 02 6C 73 8C E2 06 50 77 E9 A6 15 16 2C 34 27 C9 23 ");
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
    assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 DE 41 61 A8 40 B9 71 D8 33 CF 8D CD D8 3D ED D6 1B E4 9A 53 FD 6F 61 7E E2 7D C5 9B C0 14 4B FB AA 53 07 BD C3 D9 8D 9A 9D FE F5 1B EB 28 6E C9 ");
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
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 54 52 82 23 DF 87 C6 D9 07 56 63 64 6D 08 85 6A CB 63 AA C3 7D 16 D3 86 B2 A3 E3 85 AA BA 7B 37 ");
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
    // assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");
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
    // assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
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
    // assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
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
    // assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");
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
    // assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");
    println!("-------------------------------");
}

/*
fn aes_encrypt_with_padding_pkcs7_pcbc_into_vec()
{
    println!("aes_encrypt_with_padding_pkcs7_pcbc_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");
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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");
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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");
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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");
    println!("-------------------------------");
}

fn aes_encrypt_with_padding_pkcs7_pcbc_into_array()
{
    println!("aes_encrypt_with_padding_pkcs7_pcbc_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");
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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");
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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");
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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");
    println!("-------------------------------");
}

fn aes_encrypt_str_with_padding_pkcs7_pcbc()
{
    println!("aes_encrypt_str_with_padding_pkcs7_pcbc");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");
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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");
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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");
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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");
    println!("-------------------------------");
}

fn aes_encrypt_str_with_padding_pkcs7_pcbc_into_vec()
{
    println!("aes_encrypt_str_with_padding_pkcs7_pcbc_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");
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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");
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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");
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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");
    println!("-------------------------------");
}

fn aes_encrypt_str_with_padding_pkcs7_pcbc_into_array()
{
    println!("aes_encrypt_str_with_padding_pkcs7_pcbc_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");
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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");
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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");
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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");
    println!("-------------------------------");
}

fn aes_encrypt_string_with_padding_pkcs7_pcbc()
{
    println!("aes_encrypt_string_with_padding_pkcs7_pcbc");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");
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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");
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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");
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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");
    println!("-------------------------------");
}

fn aes_encrypt_string_with_padding_pkcs7_pcbc_into_vec()
{
    println!("aes_encrypt_string_with_padding_pkcs7_pcbc_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");
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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");
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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");
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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");
    println!("-------------------------------");
}

fn aes_encrypt_string_with_padding_pkcs7_pcbc_into_array()
{
    println!("aes_encrypt_string_with_padding_pkcs7_pcbc_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");
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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");
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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");
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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_with_padding_pkcs7_pcbc()
{
    println!("aes_encrypt_vec_with_padding_pkcs7_pcbc");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");
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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");
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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");
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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_with_padding_pkcs7_pcbc_into_vec()
{
    println!("aes_encrypt_vec_with_padding_pkcs7_pcbc_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");
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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");
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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");
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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_with_padding_pkcs7_pcbc_into_array()
{
    println!("aes_encrypt_vec_with_padding_pkcs7_pcbc_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");
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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");
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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");
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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");
    println!("-------------------------------");
}

fn aes_encrypt_array_with_padding_pkcs7_pcbc()
{
    println!("aes_encrypt_array_with_padding_pkcs7_pcbc");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");
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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");
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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");
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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");
    println!("-------------------------------");
}

fn aes_encrypt_array_with_padding_pkcs7_pcbc_into_vec()
{
    println!("aes_encrypt_array_with_padding_pkcs7_pcbc_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");
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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");
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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");
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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");
    println!("-------------------------------");
}

fn aes_encrypt_array_with_padding_pkcs7_pcbc_into_array()
{
    println!("aes_encrypt_array_with_padding_pkcs7_pcbc_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");
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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");
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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");
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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");
    println!("-------------------------------");
}
*/

fn aes_decrypt_with_padding_pkcs7_pcbc()
{
    println!("aes_decrypt_with_padding_pkcs7_pcbc");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0], iv[1], iv[2], iv[3]);

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
    assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 89 BB 67 77 F3 56 67 3A 6B DE 56 C0 63 78 7C 95 ");

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
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C DF 4B 11 43 90 8D 06 A0 07 52 17 31 8F CB 2F 7D EC A2 3F 20 15 3C 88 DD E7 0D 54 74 BC A2 AE 02 6C 73 8C E2 06 50 77 E9 A6 15 16 2C 34 27 C9 23 ");

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
    assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 DE 41 61 A8 40 B9 71 D8 33 CF 8D CD D8 3D ED D6 1B E4 9A 53 FD 6F 61 7E E2 7D C5 9B C0 14 4B FB AA 53 07 BD C3 D9 8D 9A 9D FE F5 1B EB 28 6E C9 ");

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
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 54 52 82 23 DF 87 C6 D9 07 56 63 64 6D 08 85 6A CB 63 AA C3 7D 16 D3 86 B2 A3 E3 85 AA BA 7B 37 ");

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
    // assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");

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
    // assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
    
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
    // assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
    
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
    // assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");

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
    // assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");

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
fn aes_decrypt_with_padding_pkcs7_pcbc_into_vec()
{
    println!("aes_decrypt_with_padding_pkcs7_pcbc_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");

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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");

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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");

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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");

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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
    
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
    
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");

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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");

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

fn aes_decrypt_with_padding_pkcs7_pcbc_into_array()
{
    println!("aes_decrypt_with_padding_pkcs7_pcbc_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");

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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");

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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");

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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");

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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");

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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");

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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");

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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");

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

fn aes_decrypt_with_padding_pkcs7_pcbc_into_string()
{
    println!("aes_decrypt_with_padding_pkcs7_pcbc_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");

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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");

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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");

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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");

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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");

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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");

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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");

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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");

    let mut recovered = String::new();
    a_des.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn aes_decrypt_vec_with_padding_pkcs7_pcbc()
{
    println!("aes_decrypt_vec_with_padding_pkcs7_pcbc");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");

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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");

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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");

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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");

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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
    
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
    
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");

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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");

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

fn aes_decrypt_vec_with_padding_pkcs7_pcbc_into_vec()
{
    println!("aes_decrypt_vec_with_padding_pkcs7_pcbc_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");

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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");

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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");

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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");

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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
    
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
    
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");

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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");

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

fn aes_decrypt_vec_with_padding_pkcs7_pcbc_into_array()
{
    println!("aes_decrypt_vec_with_padding_pkcs7_pcbc_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");

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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");

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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");

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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");

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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");

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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");

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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");

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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");

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

fn aes_decrypt_vec_with_padding_pkcs7_pcbc_into_string()
{
    println!("aes_decrypt_vec_with_padding_pkcs7_pcbc_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");

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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");

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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");

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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");

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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");

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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");

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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");

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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");

    let mut recovered = String::new();
    a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn aes_decrypt_array_with_padding_pkcs7_pcbc()
{
    println!("aes_decrypt_array_with_padding_pkcs7_pcbc");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");

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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");

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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");

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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");

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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
    
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
    
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");

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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");

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

fn aes_decrypt_array_with_padding_pkcs7_pcbc_into_vec()
{
    println!("aes_decrypt_array_with_padding_pkcs7_pcbc_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");

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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");

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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");

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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");

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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
    
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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");
    
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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");

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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");

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

fn aes_decrypt_array_with_padding_pkcs7_pcbc_into_array()
{
    println!("aes_decrypt_array_with_padding_pkcs7_pcbc_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");

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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");

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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");

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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");

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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");

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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");

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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");

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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");

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

fn aes_decrypt_array_with_padding_pkcs7_pcbc_into_string()
{
    println!("aes_decrypt_array_with_padding_pkcs7_pcbc_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, PCBC_PKCS7 };

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
    assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");

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
    assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 2C A9 4B DE 29 F7 D5 52 0E 20 AB 1D 9D D8 D3 5B 35 83 96 A4 94 EA DE A9 49 7D 52 5C 7C 27 38 86 F8 72 8B 4D 95 49 1F AC 50 1E 3C 13 15 4F CE 12 ");

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
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C 74 2B C3 D6 68 D3 71 96 32 07 CA EC 19 E6 9E 68 B3 38 C0 DC 26 F2 48 94 F1 18 C0 E6 B0 D3 8D 41 F2 22 C7 D3 D9 C1 47 AB F1 19 C4 CA 4E EE 02 CF ");

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
    assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");

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
    assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");

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
    assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 F3 26 1A F7 9E 57 19 5D ");

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
    assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B D4 1C 72 2B A2 5E E9 B1 ");

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
    assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E BA 3C 7F F6 D1 3B 80 23 DC 7E 78 E0 2E D2 53 18 ");

    let mut recovered = String::new();
    a_des.decrypt_array_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}
*/
