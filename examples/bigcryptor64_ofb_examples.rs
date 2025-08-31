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
    bigcryptor64_encrypt_ofb();
    bigcryptor64_encrypt_ofb_into_vec();
    bigcryptor64_encrypt_ofb_into_array();
    bigcryptor64_encrypt_str_ofb();
    bigcryptor64_encrypt_str_ofb_into_vec();
    bigcryptor64_encrypt_str_ofb_into_array();
    bigcryptor64_encrypt_string_ofb();
    bigcryptor64_encrypt_string_ofb_into_vec();
    bigcryptor64_encrypt_string_ofb_into_array();
    bigcryptor64_encrypt_vec_ofb();
    bigcryptor64_encrypt_vec_ofb_into_vec();
    bigcryptor64_encrypt_vec_ofb_into_array();
    bigcryptor64_encrypt_array_ofb();
    bigcryptor64_encrypt_array_ofb_into_vec();
    bigcryptor64_encrypt_array_ofb_into_array();

    bigcryptor64_decrypt_ofb();
    bigcryptor64_decrypt_ofb_into_vec();
    bigcryptor64_decrypt_ofb_into_array();
    bigcryptor64_decrypt_ofb_into_string();
    bigcryptor64_decrypt_vec_ofb();
    bigcryptor64_decrypt_vec_ofb_into_vec();
    bigcryptor64_decrypt_vec_ofb_into_array();
    bigcryptor64_decrypt_vec_ofb_into_string();
    bigcryptor64_decrypt_array_ofb();
    bigcryptor64_decrypt_array_ofb_into_vec();
    bigcryptor64_decrypt_array_ofb_into_array();
    bigcryptor64_decrypt_array_ofb_into_string();
}

fn bigcryptor64_encrypt_ofb()
{
    println!("bigcryptor64_encrypt_ofb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    tdes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_ofb_into_vec()
{
    println!("bigcryptor64_encrypt_ofb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    tdes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_ofb_into_array()
{
    println!("bigcryptor64_encrypt_ofb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_str_ofb()
{
    println!("bigcryptor64_encrypt_str_ofb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    tdes.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_str_ofb_into_vec()
{
    println!("bigcryptor64_encrypt_str_ofb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    let mut cipher = Vec::<u8>::new();
    tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_str_ofb_into_array()
{
    println!("bigcryptor64_encrypt_str_ofb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    let mut cipher = [0_u8; 55];
    tdes.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_string_ofb()
{
    println!("bigcryptor64_encrypt_string_ofb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.".to_string();
    let mut cipher = [0_u8; 55];
    tdes.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_string_ofb_into_vec()
{
    println!("bigcryptor64_encrypt_string_ofb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.".to_string();
    let mut cipher = Vec::<u8>::new();
    tdes.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_string_ofb_into_array()
{
    println!("bigcryptor64_encrypt_string_ofb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.".to_string();
    let mut cipher = [0_u8; 55];
    tdes.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_vec_ofb()
{
    println!("bigcryptor64_encrypt_vec_ofb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 55];
    tdes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_vec_ofb_into_vec()
{
    println!("bigcryptor64_encrypt_vec_ofb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    tdes.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_vec_ofb_into_array()
{
    println!("bigcryptor64_encrypt_vec_ofb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 55];
    tdes.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_array_ofb()
{
    println!("bigcryptor64_encrypt_array_ofb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = [0_u8; 55];
    tdes.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_array_ofb_into_vec()
{
    println!("bigcryptor64_encrypt_array_ofb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = Vec::<u8>::new();
    tdes.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_array_ofb_into_array()
{
    println!("bigcryptor64_encrypt_array_ofb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = [0_u8; 55];
    tdes.encrypt_array_into_array(iv, &message, &mut cipher);
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_ofb()
{
    println!("bigcryptor64_decrypt_ofb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");

    let mut recovered = vec![0; 55];
    tdes.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_ofb_into_vec()
{
    println!("bigcryptor64_decrypt_ofb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");

    let mut recovered = Vec::<u8>::new();
    tdes.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_ofb_into_array()
{
    println!("bigcryptor64_decrypt_ofb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");

    let mut recovered = [0u8; 56];
    let len = tdes.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    print!("Ba =\t");
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
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_ofb_into_string()
{
    println!("bigcryptor64_decrypt_ofb_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");

    let mut recovered = String::new();
    tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_vec_ofb()
{
    println!("bigcryptor64_decrypt_vec_ofb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");

    let mut recovered = vec![0; 55];
    tdes.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_vec_ofb_into_vec()
{
    println!("bigcryptor64_decrypt_vec_ofb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");

    let mut recovered = Vec::<u8>::new();
    tdes.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_vec_ofb_into_array()
{
    println!("bigcryptor64_decrypt_vec_ofb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");

    let mut recovered = [0u8; 56];
    let len = tdes.decrypt_vec_into_array(iv, &cipher, &mut recovered);
    print!("Ba =\t");
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
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_vec_ofb_into_string()
{
    println!("bigcryptor64_decrypt_vec_ofb_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");

    let mut recovered = String::new();
    tdes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_array_ofb()
{
    println!("bigcryptor64_decrypt_array_ofb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);    let mut cipher = [0_u8; 55];
    tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");

    let mut recovered = vec![0; 55];
    let len = tdes.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
    recovered.truncate(len as usize);
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
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_array_ofb_into_vec()
{
    println!("bigcryptor64_decrypt_array_ofb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);    let mut cipher = [0_u8; 55];
    tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");

    let mut recovered = Vec::<u8>::new();
    tdes.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_array_ofb_into_array()
{
    println!("bigcryptor64_decrypt_array_ofb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);    let mut cipher = [0_u8; 55];
    tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");

    let mut recovered = [0u8; 56];
    let len = tdes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    print!("Ba =\t");
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
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_array_ofb_into_string()
{
    println!("bigcryptor64_decrypt_array_ofb_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, OFB };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);    let mut cipher = [0_u8; 55];
    tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A8 E8 A8 5C 2E 0C B6 68 B2 9A 04 35 11 DD F7 C6 8E 4E 9E EB D0 9B 97 1C F0 19 3F 0D 24 57 78 0F 84 2B F8 8C 22 26 B8 D3 AF D4 9C 69 86 1E 6D 2B 31 B4 10 49 29 0A 7A ");

    let mut recovered = String::new();
    tdes.decrypt_array_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}
