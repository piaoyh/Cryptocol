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
    bigcryptor64_encrypt_with_padding_pkcs7_cbc();
    bigcryptor64_encrypt_with_padding_pkcs7_cbc_into_vec();
    bigcryptor64_encrypt_with_padding_pkcs7_cbc_into_array();
    bigcryptor64_encrypt_str_with_padding_pkcs7_cbc();
    bigcryptor64_encrypt_str_with_padding_pkcs7_cbc_into_vec();
    bigcryptor64_encrypt_str_with_padding_pkcs7_cbc_into_array();
    bigcryptor64_encrypt_string_with_padding_pkcs7_cbc();
    bigcryptor64_encrypt_string_with_padding_pkcs7_cbc_into_vec();
    bigcryptor64_encrypt_string_with_padding_pkcs7_cbc_into_array();
    bigcryptor64_encrypt_vec_with_padding_pkcs7_cbc();
    bigcryptor64_encrypt_vec_with_padding_pkcs7_cbc_into_vec();
    bigcryptor64_encrypt_vec_with_padding_pkcs7_cbc_into_array();
    bigcryptor64_encrypt_array_with_padding_pkcs7_cbc();
    bigcryptor64_encrypt_array_with_padding_pkcs7_cbc_into_vec();
    bigcryptor64_encrypt_array_with_padding_pkcs7_cbc_into_array();

    bigcryptor64_decrypt_with_padding_pkcs7_cbc();
    bigcryptor64_decrypt_with_padding_pkcs7_cbc_into_vec();
    bigcryptor64_decrypt_with_padding_pkcs7_cbc_into_array();
    bigcryptor64_decrypt_with_padding_pkcs7_cbc_into_string();
    bigcryptor64_decrypt_vec_with_padding_pkcs7_cbc();
    bigcryptor64_decrypt_vec_with_padding_pkcs7_cbc_into_vec();
    bigcryptor64_decrypt_vec_with_padding_pkcs7_cbc_into_array();
    bigcryptor64_decrypt_vec_with_padding_pkcs7_cbc_into_string();
    bigcryptor64_decrypt_array_with_padding_pkcs7_cbc();
    bigcryptor64_decrypt_array_with_padding_pkcs7_cbc_into_vec();
    bigcryptor64_decrypt_array_with_padding_pkcs7_cbc_into_array();
    bigcryptor64_decrypt_array_with_padding_pkcs7_cbc_into_string();
}

fn bigcryptor64_encrypt_with_padding_pkcs7_cbc()
{
    println!("bigcryptor64_encrypt_with_padding_pkcs7_cbc()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 56];
    tdes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_with_padding_pkcs7_cbc_into_vec()
{
    println!("bigcryptor64_encrypt_with_padding_pkcs7_cbc_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

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
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_with_padding_pkcs7_cbc_into_array()
{
    println!("bigcryptor64_encrypt_with_padding_pkcs7_cbc_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 56];
    tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_str_with_padding_pkcs7_cbc()
{
    println!("bigcryptor64_encrypt_str_with_padding_pkcs7_cbc()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 56];
    tdes.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_str_with_padding_pkcs7_cbc_into_vec()
{
    println!("bigcryptor64_encrypt_str_with_padding_pkcs7_cbc_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

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
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_str_with_padding_pkcs7_cbc_into_array()
{
    println!("bigcryptor64_encrypt_str_with_padding_pkcs7_cbc_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    let mut cipher = [0_u8; 56];
    tdes.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_string_with_padding_pkcs7_cbc()
{
    println!("bigcryptor64_encrypt_string_with_padding_pkcs7_cbc()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.".to_string();
    let mut cipher = [0_u8; 56];
    tdes.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_string_with_padding_pkcs7_cbc_into_vec()
{
    println!("bigcryptor64_encrypt_string_with_padding_pkcs7_cbc_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

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
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_string_with_padding_pkcs7_cbc_into_array()
{
    println!("bigcryptor64_encrypt_string_with_padding_pkcs7_cbc_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.".to_string();
    let mut cipher = [0_u8; 56];
    tdes.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_vec_with_padding_pkcs7_cbc()
{
    println!("bigcryptor64_encrypt_vec_with_padding_pkcs7_cbc()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 56];
    tdes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_vec_with_padding_pkcs7_cbc_into_vec()
{
    println!("bigcryptor64_encrypt_vec_with_padding_pkcs7_cbc_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

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
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_vec_with_padding_pkcs7_cbc_into_array()
{
    println!("bigcryptor64_encrypt_vec_with_padding_pkcs7_cbc_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 56];
    tdes.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_array_with_padding_pkcs7_cbc()
{
    println!("bigcryptor64_encrypt_array_with_padding_pkcs7_cbc()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = [0_u8; 56];
    tdes.encrypt_array(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_array_with_padding_pkcs7_cbc_into_vec()
{
    println!("bigcryptor64_encrypt_array_with_padding_pkcs7_cbc_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = Vec::<u8>::new();
    tdes.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_array_with_padding_pkcs7_cbc_into_array()
{
    println!("bigcryptor64_encrypt_array_with_padding_pkcs7_cbc_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = [0_u8; 56];
    tdes.encrypt_array_into_array(iv, &message, &mut cipher);
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_with_padding_pkcs7_cbc()
{
    println!("bigcryptor64_decrypt_with_padding_pkcs7_cbc()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

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
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");

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

fn bigcryptor64_decrypt_with_padding_pkcs7_cbc_into_vec()
{
    println!("bigcryptor64_decrypt_with_padding_pkcs7_cbc_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

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
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");

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

fn bigcryptor64_decrypt_with_padding_pkcs7_cbc_into_array()
{
    println!("bigcryptor64_decrypt_with_padding_pkcs7_cbc_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

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
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");

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

fn bigcryptor64_decrypt_with_padding_pkcs7_cbc_into_string()
{
    println!("bigcryptor64_decrypt_with_padding_pkcs7_cbc_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

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
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");

    let mut recovered = String::new();
    tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_vec_with_padding_pkcs7_cbc()
{
    println!("bigcryptor64_decrypt_vec_with_padding_pkcs7_cbc()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

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
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");

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

fn bigcryptor64_decrypt_vec_with_padding_pkcs7_cbc_into_vec()
{
    println!("bigcryptor64_decrypt_vec_with_padding_pkcs7_cbc_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

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
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");

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

fn bigcryptor64_decrypt_vec_with_padding_pkcs7_cbc_into_array()
{
    println!("bigcryptor64_decrypt_vec_with_padding_pkcs7_cbc_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

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
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");

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

fn bigcryptor64_decrypt_vec_with_padding_pkcs7_cbc_into_string()
{
    println!("bigcryptor64_decrypt_vec_with_padding_pkcs7_cbc_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

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
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");

    let mut recovered = String::new();
    tdes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_array_with_padding_pkcs7_cbc()
{
    println!("bigcryptor64_decrypt_array_with_padding_pkcs7_cbc()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);    let mut cipher = [0_u8; 56];
    tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");

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

fn bigcryptor64_decrypt_array_with_padding_pkcs7_cbc_into_vec()
{
    println!("bigcryptor64_decrypt_array_with_padding_pkcs7_cbc_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);    let mut cipher = [0_u8; 56];
    tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");

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

fn bigcryptor64_decrypt_array_with_padding_pkcs7_cbc_into_array()
{
    println!("bigcryptor64_decrypt_array_with_padding_pkcs7_cbc_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);    let mut cipher = [0_u8; 56];
    tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");

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

fn bigcryptor64_decrypt_array_with_padding_pkcs7_cbc_into_string()
{
    println!("bigcryptor64_decrypt_array_with_padding_pkcs7_cbc_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // TDES case
    let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                    + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                    + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);    let mut cipher = [0_u8; 56];
    tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "01 A5 7E BC ED 83 28 FB 7A 06 61 60 7D 8B BA AF 5C A7 76 0B FE 94 1C C4 C2 BC DD 15 98 A9 98 6C 85 18 92 6B 00 72 FB 72 DB 9D 46 BD B1 3C 56 C0 DC 0E 37 04 69 4F 62 68 ");

    let mut recovered = String::new();
    tdes.decrypt_array_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}
