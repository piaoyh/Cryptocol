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
    des_crypt_cfb_main();
    des_crypt_ofb_main();
    des_crypt_ctr_main();
}

fn des_crypt_cfb_main()
{
    des_encrypt_cfb();
    des_encrypt_cfb_into_vec();
    des_encrypt_cfb_into_array();
    des_encrypt_str_cfb();
    des_encrypt_str_cfb_into_vec();
    des_encrypt_str_cfb_into_array();
    des_encrypt_string_cfb();
    des_encrypt_string_cfb_into_vec();
    des_encrypt_string_cfb_into_array();
    des_encrypt_vec_cfb();
    des_encrypt_vec_cfb_into_vec();
    des_encrypt_vec_cfb_into_array();
    des_encrypt_array_cfb();
    des_encrypt_array_cfb_into_vec();
    des_encrypt_array_cfb_into_array();

    des_decrypt_cfb();
    des_decrypt_cfb_into_vec();
    des_decrypt_cfb_into_array();
    des_decrypt_cfb_into_string();
    des_decrypt_vec_cfb();
    des_decrypt_vec_cfb_into_vec();
    des_decrypt_vec_cfb_into_array();
    des_decrypt_vec_cfb_into_string();
    des_decrypt_array_cfb();
    des_decrypt_array_cfb_into_vec();
    des_decrypt_array_cfb_into_array();
    des_decrypt_array_cfb_into_string();
}

fn des_encrypt_cfb()
{
    println!("des_encrypt_cfb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");
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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher1.as_mut_ptr());
    d_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");
    println!("-------------------------------");
}

fn des_encrypt_cfb_into_vec()
{
    println!("des_encrypt_cfb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");
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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");
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
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
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
    assert_eq!(txt, "");
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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");
    println!("-------------------------------");
}

fn des_encrypt_cfb_into_array()
{
    println!("des_encrypt_cfb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");
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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");
    println!("-------------------------------");
}

fn des_encrypt_str_cfb()
{
    println!("des_encrypt_str_cfb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");
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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_str(iv, &message, cipher1.as_mut_ptr());
    d_des.encrypt_str(iv, &message, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");
    println!("-------------------------------");
}

fn des_encrypt_str_cfb_into_vec()
{
    println!("des_encrypt_str_cfb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");
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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");
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
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
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
    assert_eq!(txt, "");
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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");
    println!("-------------------------------");
}

fn des_encrypt_str_cfb_into_array()
{
    println!("des_encrypt_str_cfb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");
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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_str_into_array(iv, &message, &mut cipher1);
    d_des.encrypt_str_into_array(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");
    println!("-------------------------------");
}

fn des_encrypt_string_cfb()
{
    println!("des_encrypt_string_cfb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");
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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_string(iv, &message, cipher1.as_mut_ptr());
    d_des.encrypt_string(iv, &message, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.".to_string();
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.".to_string();
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");
    println!("-------------------------------");
}

fn des_encrypt_string_cfb_into_vec()
{
    println!("des_encrypt_string_cfb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");
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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");
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
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
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
    assert_eq!(txt, "");
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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");
    println!("-------------------------------");
}

fn des_encrypt_string_cfb_into_array()
{
    println!("des_encrypt_string_cfb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");
    println!();

    // Expanded case for 128 rounds
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");
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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_string_into_array(iv, &message, &mut cipher1);
    d_des.encrypt_string_into_array(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    println!();

    // Normal case for the message of 0 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.".to_string();
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");
    println!();

    // Normal case for the message of 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.".to_string();
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");
    println!("-------------------------------");
}

fn des_encrypt_vec_cfb()
{
    println!("des_encrypt_vec_cfb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");
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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");
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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_vec(iv, &message, cipher1.as_mut_ptr());
    d_des.encrypt_vec(iv, &message, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
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
    let mut cipher = [0_u8; 0];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");
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
    let mut cipher = [0_u8; 16];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");
    println!("-------------------------------");
}

fn des_encrypt_vec_cfb_into_vec()
{
    println!("des_encrypt_vec_cfb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");
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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");
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
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
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
    assert_eq!(txt, "");
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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");
    println!("-------------------------------");
}

fn des_encrypt_vec_cfb_into_array()
{
    println!("des_encrypt_vec_cfb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");
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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");
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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_vec_into_array(iv, &message, &mut cipher1);
    d_des.encrypt_vec_into_array(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
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
    let mut cipher = [0_u8; 0];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");
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
    let mut cipher = [0_u8; 16];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");
    println!("-------------------------------");
}

fn des_encrypt_array_cfb()
{
    println!("des_encrypt_array_cfb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");
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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");
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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_array(iv, &message, cipher1.as_mut_ptr());
    d_des.encrypt_array(iv, &message, cipher2.as_mut_ptr());
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
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
    let mut cipher = [0_u8; 0];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "7 bytes";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 7];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "I am OK.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 8];
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");
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
    let mut cipher = [0_u8; 16];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");
    println!("-------------------------------");
}

fn des_encrypt_array_cfb_into_vec()
{
    println!("des_encrypt_array_cfb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");
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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");
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
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
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
    assert_eq!(txt, "");
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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");
    println!("-------------------------------");
}

fn des_encrypt_array_cfb_into_array()
{
    println!("des_encrypt_array_cfb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");
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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");
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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_array_into_array(iv, &message, &mut cipher1);
    d_des.encrypt_array_into_array(iv, &message, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
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
    let mut cipher = [0_u8; 0];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "7 bytes";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 7];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "I am OK.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 8];
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");
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
    let mut cipher = [0_u8; 16];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");
    println!("-------------------------------");
}

fn des_decrypt_cfb()
{
    println!("des_decrypt_cfb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");

    let mut recovered = vec![0; 55];
    a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");

    let mut recovered = vec![0; 55];
    a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");

    let mut recovered1 = vec![0; 55];
    let mut recovered2 = vec![0; 55];
    c_des.decrypt(iv, cipher1.as_ptr(), cipher1.len() as u64, recovered1.as_mut_ptr());
    d_des.decrypt(iv, cipher2.as_ptr(), cipher2.len() as u64, recovered2.as_mut_ptr());
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
    assert_eq!(txt, "");

    let mut recovered = vec![0; 8];
    let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
    let mut recovered = vec![0; 8];
    let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
    let mut recovered = vec![0; 16];
    let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");

    let mut recovered = vec![0; 16];
    let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");

    let mut recovered = vec![0; 24];
    let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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

fn des_decrypt_cfb_into_vec()
{
    println!("des_decrypt_cfb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");

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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");

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
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");

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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");

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

fn des_decrypt_cfb_into_array()
{
    println!("des_decrypt_cfb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");

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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");

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
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");

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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");

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

fn des_decrypt_cfb_into_string()
{
    println!("des_decrypt_cfb_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");

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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");

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
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");

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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");

    let mut recovered = String::new();
    a_des.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn des_decrypt_vec_cfb()
{
    println!("des_decrypt_vec_cfb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");

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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");

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
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");

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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");

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

fn des_decrypt_vec_cfb_into_vec()
{
    println!("des_decrypt_vec_cfb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");

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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");

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
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");

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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");

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

fn des_decrypt_vec_cfb_into_array()
{
    println!("des_decrypt_vec_cfb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");

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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");

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
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");

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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");

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

fn des_decrypt_vec_cfb_into_string()
{
    println!("des_decrypt_vec_cfb_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");

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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");

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
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");

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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");

    let mut recovered = String::new();
    a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn des_decrypt_array_cfb()
{
    println!("des_decrypt_array_cfb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");

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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");

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
    let mut cipher = [0_u8; 0];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

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
    let mut cipher = [0_u8; 7];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
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
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");

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
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");

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

fn des_decrypt_array_cfb_into_vec()
{
    println!("des_decrypt_array_cfb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");

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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");

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
    let mut cipher = [0_u8; 0];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

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
    let mut cipher = [0_u8; 7];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
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
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");

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
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");

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

fn des_decrypt_array_cfb_into_array()
{
    println!("des_decrypt_array_cfb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");

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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");

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
    let mut cipher = [0_u8; 0];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

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
    let mut cipher = [0_u8; 7];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

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
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");

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
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");

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

fn des_decrypt_array_cfb_into_string()
{
    println!("des_decrypt_array_cfb_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CFB};

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 79 3A A1 78 EC CD 02 72 6A C4 41 7C 25 A4 2C 07 FC 77 25 49 12 55 0F 8A ED 44 C3 E4 DC 91 69 0F 40 72 7F F2 D9 B7 54 9F 36 91 C5 85 4F 9B 30 ");

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 F2 9A 70 44 D9 78 F5 E3 CF 55 0F EF BA F0 4A 7E BE 79 C9 B4 68 F4 99 09 48 93 00 D2 22 9D 29 6C 20 74 FF E3 E2 01 0C D3 7E 8D 4D 30 8F EC D6 ");

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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    print!("C (0 rounds) =\t");
    for c in cipher1.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher1.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");
    print!("D (0 rounds) =\t");
    for c in cipher2.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher2.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5B ED BA 3F 6E 10 CC 9F C2 B9 1C 51 F3 49 A2 08 E1 31 43 C6 D3 E5 23 61 B3 46 E6 AD C3 AE 7B F7 53 E1 BC 3F B5 38 D9 88 83 B3 12 5B 5A 40 8E 21 63 16 40 D5 D1 E8 63 ");

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
    let mut cipher = [0_u8; 0];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

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
    let mut cipher = [0_u8; 7];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

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
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 74 F9 B6 4F ");

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
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 F4 FC 94 B5 94 1F B9 BD ");

    let mut recovered = String::new();
    a_des.decrypt_array_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn des_crypt_ofb_main()
{
    des_encrypt_ofb();
    des_encrypt_ofb_into_vec();
    des_encrypt_ofb_into_array();
    des_encrypt_str_ofb();
    des_encrypt_str_ofb_into_vec();
    des_encrypt_str_ofb_into_array();
    des_encrypt_string_ofb();
    des_encrypt_string_ofb_into_vec();
    des_encrypt_string_ofb_into_array();
    des_encrypt_vec_ofb();
    des_encrypt_vec_ofb_into_vec();
    des_encrypt_vec_ofb_into_array();
    des_encrypt_array_ofb();
    des_encrypt_array_ofb_into_vec();
    des_encrypt_array_ofb_into_array();

    des_decrypt_ofb();
    des_decrypt_ofb_into_vec();
    des_decrypt_ofb_into_array();
    des_decrypt_ofb_into_string();
    des_decrypt_vec_ofb();
    des_decrypt_vec_ofb_into_vec();
    des_decrypt_vec_ofb_into_array();
    des_decrypt_vec_ofb_into_string();
    des_decrypt_array_ofb();
    des_decrypt_array_ofb_into_vec();
    des_decrypt_array_ofb_into_array();
    des_decrypt_array_ofb_into_string();
}

fn des_encrypt_ofb()
{
    println!("des_encrypt_ofb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher1.as_mut_ptr());
    d_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher2.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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

fn des_encrypt_ofb_into_vec()
{
    println!("des_encrypt_ofb_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
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
    assert_eq!(txt, "");
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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn des_encrypt_ofb_into_array()
{
    println!("des_encrypt_ofb_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn des_encrypt_str_ofb()
{
    println!("des_encrypt_str_ofb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_str(iv, &message, cipher1.as_mut_ptr());
    d_des.encrypt_str(iv, &message, cipher2.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn des_encrypt_str_ofb_into_vec()
{
    println!("des_encrypt_str_ofb_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
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
    assert_eq!(txt, "");
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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn des_encrypt_str_ofb_into_array()
{
    println!("des_encrypt_str_ofb_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_str_into_array(iv, &message, &mut cipher1);
    d_des.encrypt_str_into_array(iv, &message, &mut cipher2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn des_encrypt_string_ofb()
{
    println!("des_encrypt_string_ofb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_string(iv, &message, cipher1.as_mut_ptr());
    d_des.encrypt_string(iv, &message, cipher2.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.".to_string();
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.".to_string();
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn des_encrypt_string_ofb_into_vec()
{
    println!("des_encrypt_string_ofb_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
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
    assert_eq!(txt, "");
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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn des_encrypt_string_ofb_into_array()
{
    println!("des_encrypt_string_ofb_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_string_into_array(iv, &message, &mut cipher1);
    d_des.encrypt_string_into_array(iv, &message, &mut cipher2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.".to_string();
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.".to_string();
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn des_encrypt_vec_ofb()
{
    println!("des_encrypt_vec_ofb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_vec(iv, &message, cipher1.as_mut_ptr());
    d_des.encrypt_vec(iv, &message, cipher2.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn des_encrypt_vec_ofb_into_vec()
{
    println!("des_encrypt_vec_ofb_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
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
    assert_eq!(txt, "");
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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn des_encrypt_vec_ofb_into_array()
{
    println!("des_encrypt_vec_ofb_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_vec_into_array(iv, &message, &mut cipher1);
    d_des.encrypt_vec_into_array(iv, &message, &mut cipher2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);
 
    let message = "고맙습니다.";
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn des_encrypt_array_ofb()
{
    println!("des_encrypt_array_ofb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_array(iv, &message, cipher1.as_mut_ptr());
    d_des.encrypt_array(iv, &message, cipher2.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 0];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "7 bytes";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 7];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "I am OK.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 8];
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "고맙습니다.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 16];
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn des_encrypt_array_ofb_into_vec()
{
    println!("des_encrypt_array_ofb_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");
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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");
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
    assert_eq!(txt, "");
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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn des_encrypt_array_ofb_into_array()
{
    println!("des_encrypt_array_ofb_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_array_into_array(iv, &message, &mut cipher1);
    d_des.encrypt_array_into_array(iv, &message, &mut cipher2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 0];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "7 bytes";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 7];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "I am OK.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 8];
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);
 
    let mes = "고맙습니다.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 16];
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");
    println!("-------------------------------");
}

fn des_decrypt_ofb()
{
    println!("des_decrypt_ofb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

    let mut recovered = vec![0; 55];
    a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

    let mut recovered = vec![0; 55];
    a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    c_des.decrypt(iv, cipher1.as_ptr(), cipher1.len() as u64, recovered1.as_mut_ptr());
    d_des.decrypt(iv, cipher2.as_ptr(), cipher2.len() as u64, recovered2.as_mut_ptr());
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
    assert_eq!(txt, "");

    let mut recovered = vec![0; 8];
    let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
    let mut recovered = vec![0; 8];
    let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
    let mut recovered = vec![0; 16];
    let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

    let mut recovered = vec![0; 16];
    let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

    let mut recovered = vec![0; 24];
    let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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

fn des_decrypt_ofb_into_vec()
{
    println!("des_decrypt_ofb_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

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

fn des_decrypt_ofb_into_array()
{
    println!("des_decrypt_ofb_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

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

fn des_decrypt_ofb_into_string()
{
    println!("des_decrypt_ofb_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

    let mut recovered = String::new();
    a_des.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn des_decrypt_vec_ofb()
{
    println!("des_decrypt_vec_ofb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

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

fn des_decrypt_vec_ofb_into_vec()
{
    println!("des_decrypt_vec_ofb_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

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

fn des_decrypt_vec_ofb_into_array()
{
    println!("des_decrypt_vec_ofb_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

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

fn des_decrypt_vec_ofb_into_string()
{
    println!("des_decrypt_vec_ofb_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

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
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

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
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

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
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

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
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

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
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

    let mut recovered = String::new();
    a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn des_decrypt_array_ofb()
{
    println!("des_decrypt_array_ofb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
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
    let mut cipher = [0_u8; 0];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

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
    let mut cipher = [0_u8; 7];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
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
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

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
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

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

fn des_decrypt_array_ofb_into_vec()
{
    println!("des_decrypt_array_ofb_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
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
    let mut cipher = [0_u8; 0];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

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
    let mut cipher = [0_u8; 7];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");
    
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
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");
    
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

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
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

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

fn des_decrypt_array_ofb_into_array()
{
    println!("des_decrypt_array_ofb_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
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
    let mut cipher = [0_u8; 0];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

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
    let mut cipher = [0_u8; 7];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

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
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

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
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

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

fn des_decrypt_array_ofb_into_string()
{
    println!("des_decrypt_array_ofb_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, OFB };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 1E E1 51 FD B3 B0 4B 2A EF BC 49 21 FA C0 27 FB 9F DD BB 17 8D 21 3B 49 66 A2 94 AB 4D 08 8E B9 8D D6 7F 9F 8B 8D 0E E3 E7 5D F4 57 BB 96 2D 63 C3 2F 9E 71 8C 72 ");

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "19 B0 8F 23 01 31 B3 95 5F 0F 7C D5 96 55 DB 98 76 6C C7 23 49 6D 35 9B FD DE B7 1A 6B B2 8A EF B6 63 DB A5 50 F6 07 30 8C 75 28 32 25 32 33 77 B0 46 FE 96 1C F7 6B ");

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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
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
    let mut cipher = [0_u8; 0];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

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
    let mut cipher = [0_u8; 7];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "50 50 A3 5C E1 B3 E3 ");

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
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "2E 50 A0 48 B5 99 DB 07 ");

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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "37 31 93 6E B5 8F FF 5C 21 EF BD 48 ");

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
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "8D C3 61 CE 32 4F 7C A3 FA 63 5E AF A4 18 0A 6E ");

    let mut recovered = String::new();
    a_des.decrypt_array_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn des_crypt_ctr_main()
{
    des_encrypt_ctr();
    des_encrypt_ctr_into_vec();
    des_encrypt_ctr_into_array();
    des_encrypt_str_ctr();
    des_encrypt_str_ctr_into_vec();
    des_encrypt_str_ctr_into_array();
    des_encrypt_string_ctr();
    des_encrypt_string_ctr_into_vec();
    des_encrypt_string_ctr_into_array();
    des_encrypt_vec_ctr();
    des_encrypt_vec_ctr_into_vec();
    des_encrypt_vec_ctr_into_array();
    des_encrypt_array_ctr();
    des_encrypt_array_ctr_into_vec();
    des_encrypt_array_ctr_into_array();

    des_decrypt_ctr();
    des_decrypt_ctr_into_vec();
    des_decrypt_ctr_into_array();
    des_decrypt_ctr_into_string();
    des_decrypt_vec_ctr();
    des_decrypt_vec_ctr_into_vec();
    des_decrypt_vec_ctr_into_array();
    des_decrypt_vec_ctr_into_string();
    des_decrypt_array_ctr();
    des_decrypt_array_ctr_into_vec();
    des_decrypt_array_ctr_into_array();
    des_decrypt_array_ctr_into_string();
}

fn des_encrypt_ctr()
{
    println!("des_encrypt_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher1.as_mut_ptr());
    d_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher2.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 8];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 16];
    a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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

fn des_encrypt_ctr_into_vec()
{
    println!("des_encrypt_ctr_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

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
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
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
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
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
    assert_eq!(txt, "");
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
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
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
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn des_encrypt_ctr_into_array()
{
    println!("des_encrypt_ctr_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn des_encrypt_str_ctr()
{
    println!("des_encrypt_str_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_str(iv, &message, cipher1.as_mut_ptr());
    d_des.encrypt_str(iv, &message, cipher2.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn des_encrypt_str_ctr_into_vec()
{
    println!("des_encrypt_str_ctr_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

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
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
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
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
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
    assert_eq!(txt, "");
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
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
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
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn des_encrypt_str_ctr_into_array()
{
    println!("des_encrypt_str_ctr_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_str_into_array(iv, &message, &mut cipher1);
    d_des.encrypt_str_into_array(iv, &message, &mut cipher2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt_str_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn des_encrypt_string_ctr()
{
    println!("des_encrypt_string_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_string(iv, &message, cipher1.as_mut_ptr());
    d_des.encrypt_string(iv, &message, cipher2.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.".to_string();
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.".to_string();
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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn des_encrypt_string_ctr_into_vec()
{
    println!("des_encrypt_string_ctr_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

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
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
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
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
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
    assert_eq!(txt, "");
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
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
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
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn des_encrypt_string_ctr_into_array()
{
    println!("des_encrypt_string_ctr_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_string_into_array(iv, &message, &mut cipher1);
    d_des.encrypt_string_into_array(iv, &message, &mut cipher2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.".to_string();
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    println!();

    // Normal case for the message longer than 8 bytes and shorter than 16 bytes
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "PARK Youngho".to_string();
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 12];
    a_des.encrypt_string_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.".to_string();
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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn des_encrypt_vec_ctr()
{
    println!("des_encrypt_vec_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_vec(iv, &message, cipher1.as_mut_ptr());
    d_des.encrypt_vec(iv, &message, cipher2.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "고맙습니다.";
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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn des_encrypt_vec_ctr_into_vec()
{
    println!("des_encrypt_vec_ctr_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

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
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
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
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
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
    assert_eq!(txt, "");
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
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
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
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn des_encrypt_vec_ctr_into_array()
{
    println!("des_encrypt_vec_ctr_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_vec_into_array(iv, &message, &mut cipher1);
    d_des.encrypt_vec_into_array(iv, &message, &mut cipher2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "7 bytes";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let message = "I am OK.";
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);
 
    let message = "고맙습니다.";
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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn des_encrypt_array_ctr()
{
    println!("des_encrypt_array_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_array(iv, &message, cipher1.as_mut_ptr());
    d_des.encrypt_array(iv, &message, cipher2.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 0];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "7 bytes";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 7];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "I am OK.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 8];
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "고맙습니다.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 16];
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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn des_encrypt_array_ctr_into_vec()
{
    println!("des_encrypt_array_ctr_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

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
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
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
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
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
    assert_eq!(txt, "");
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
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
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
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn des_encrypt_array_ctr_into_array()
{
    println!("des_encrypt_array_ctr_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
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
    let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_array_into_array(iv, &message, &mut cipher1);
    d_des.encrypt_array_into_array(iv, &message, &mut cipher2);
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 0];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 0];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "7 bytes";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 7];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 7];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);

    let mes = "I am OK.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 8];
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_array_into_array(iv, &message, &mut cipher);
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
    let mut a_des = DES::new_with_key_u64(key);
 
    let mes = "고맙습니다.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 16];
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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    println!("-------------------------------");
}

fn des_decrypt_ctr()
{
    println!("des_decrypt_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

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
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

    let mut recovered = vec![0; 55];
    a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

    let mut recovered = vec![0; 55];
    a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    c_des.decrypt(iv, cipher1.as_ptr(), cipher1.len() as u64, recovered1.as_mut_ptr());
    d_des.decrypt(iv, cipher2.as_ptr(), cipher2.len() as u64, recovered2.as_mut_ptr());
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
    assert_eq!(txt, "");

    let mut recovered = vec![0; 8];
    let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    
    let mut recovered = vec![0; 8];
    let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    
    let mut recovered = vec![0; 16];
    let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

    let mut recovered = vec![0; 16];
    let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

    let mut recovered = vec![0; 24];
    let len = a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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

fn des_decrypt_ctr_into_vec()
{
    println!("des_decrypt_ctr_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

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
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

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
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    
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
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

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

fn des_decrypt_ctr_into_array()
{
    println!("des_decrypt_ctr_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

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
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

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
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");

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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");

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
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

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

fn des_decrypt_ctr_into_string()
{
    println!("des_decrypt_ctr_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

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
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

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
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");

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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");

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
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

    let mut recovered = String::new();
    a_des.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn des_decrypt_vec_ctr()
{
    println!("des_decrypt_vec_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

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
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

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
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    
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
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

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

fn des_decrypt_vec_ctr_into_vec()
{
    println!("des_decrypt_vec_ctr_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

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
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

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
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    
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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    
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
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

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

fn des_decrypt_vec_ctr_into_array()
{
    println!("des_decrypt_vec_ctr_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

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
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

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
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");

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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");

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
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

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

fn des_decrypt_vec_ctr_into_string()
{
    println!("des_decrypt_vec_ctr_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

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
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

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
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

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
    assert_eq!(txt, "");

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
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");

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
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");

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
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

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
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

    let mut recovered = String::new();
    a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn des_decrypt_array_ctr()
{
    println!("des_decrypt_array_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
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
    let mut cipher = [0_u8; 0];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

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
    let mut cipher = [0_u8; 7];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    
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
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

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
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

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

fn des_decrypt_array_ctr_into_vec()
{
    println!("des_decrypt_array_ctr_into_vec");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
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
    let mut cipher = [0_u8; 0];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

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
    let mut cipher = [0_u8; 7];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    
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
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    
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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

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
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

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

fn des_decrypt_array_ctr_into_array()
{
    println!("des_decrypt_array_ctr_into_array");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
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
    let mut cipher = [0_u8; 0];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

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
    let mut cipher = [0_u8; 7];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");

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
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");

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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

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
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

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

fn des_decrypt_array_ctr_into_string()
{
    println!("des_decrypt_array_ctr_into_string");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DES, DES_Expanded, CTR };

    // Normal case
    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#016X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (16 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");

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
    let mut cipher = [0_u8; 55];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C (128 rounds) =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");

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
    let mut cipher1 = [0_u8; 55];
    let mut cipher2 = [0_u8; 55];
    c_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    d_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
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
    let mut cipher = [0_u8; 0];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "");

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
    let mut cipher = [0_u8; 7];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");

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
    let mut cipher = [0_u8; 8];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");

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
    let mut cipher = [0_u8; 12];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");

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
    let mut cipher = [0_u8; 16];
    a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");

    let mut recovered = String::new();
    a_des.decrypt_array_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "고맙습니다.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}
