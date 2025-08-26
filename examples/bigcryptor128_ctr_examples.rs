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
    bigcryptor128_encrypt_ctr();
    bigcryptor128_encrypt_ctr_into_vec();
    bigcryptor128_encrypt_ctr_into_array();
    bigcryptor128_encrypt_str_ctr();
    bigcryptor128_encrypt_str_ctr_into_vec();
    bigcryptor128_encrypt_str_ctr_into_array();
    bigcryptor128_encrypt_string_ctr();
    bigcryptor128_encrypt_string_ctr_into_vec();
    bigcryptor128_encrypt_string_ctr_into_array();
    bigcryptor128_encrypt_vec_ctr();
    bigcryptor128_encrypt_vec_ctr_into_vec();
    bigcryptor128_encrypt_vec_ctr_into_array();
    bigcryptor128_encrypt_array_ctr();
    bigcryptor128_encrypt_array_ctr_into_vec();
    bigcryptor128_encrypt_array_ctr_into_array();

    bigcryptor128_decrypt_ctr();
    bigcryptor128_decrypt_ctr_into_vec();
    bigcryptor128_decrypt_ctr_into_array();
    bigcryptor128_decrypt_ctr_into_string();
    bigcryptor128_decrypt_vec_ctr();
    bigcryptor128_decrypt_vec_ctr_into_vec();
    bigcryptor128_decrypt_vec_ctr_into_array();
    bigcryptor128_decrypt_vec_ctr_into_string();
    bigcryptor128_decrypt_array_ctr();
    bigcryptor128_decrypt_array_ctr_into_vec();
    bigcryptor128_decrypt_array_ctr_into_array();
    bigcryptor128_decrypt_array_ctr_into_string();
}

fn bigcryptor128_encrypt_ctr()
{
    println!("bigcryptor128_encrypt_ctr()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    taes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_ctr_into_vec()
{
    println!("bigcryptor128_encrypt_ctr_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_ctr_into_array()
{
    println!("bigcryptor128_encrypt_ctr_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    taes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_str_ctr()
{
    println!("bigcryptor128_encrypt_str_ctr()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    taes.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_str_ctr_into_vec()
{
    println!("bigcryptor128_encrypt_str_ctr_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_str_ctr_into_array()
{
    println!("bigcryptor128_encrypt_str_ctr_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    let mut cipher = [0_u8; 55];
    taes.encrypt_str_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_string_ctr()
{
    println!("bigcryptor128_encrypt_string_ctr()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.".to_string();
    let mut cipher = [0_u8; 55];
    taes.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_string_ctr_into_vec()
{
    println!("bigcryptor128_encrypt_string_ctr_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.".to_string();
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_string_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_string_ctr_into_array()
{
    println!("bigcryptor128_encrypt_string_ctr_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.".to_string();
    let mut cipher = [0_u8; 55];
    taes.encrypt_string_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_vec_ctr()
{
    println!("bigcryptor128_encrypt_vec_ctr()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 55];
    taes.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_vec_ctr_into_vec()
{
    println!("bigcryptor128_encrypt_vec_ctr_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_vec_ctr_into_array()
{
    println!("bigcryptor128_encrypt_vec_ctr_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 55];
    taes.encrypt_vec_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_array_ctr()
{
    println!("bigcryptor128_encrypt_array_ctr()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = [0_u8; 55];
    taes.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_array_ctr_into_vec()
{
    println!("bigcryptor128_encrypt_array_ctr_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_array_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_array_ctr_into_array()
{
    println!("bigcryptor128_encrypt_array_ctr_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = [0_u8; 55];
    taes.encrypt_array_into_array(nonce, &message, &mut cipher);
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    println!("-------------------------------");
}

fn bigcryptor128_decrypt_ctr()
{
    println!("bigcryptor128_decrypt_ctr()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");

    let mut recovered = vec![0; 55];
    taes.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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

fn bigcryptor128_decrypt_ctr_into_vec()
{
    println!("bigcryptor128_decrypt_ctr_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");

    let mut recovered = Vec::<u8>::new();
    taes.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

fn bigcryptor128_decrypt_ctr_into_array()
{
    println!("bigcryptor128_decrypt_ctr_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");

    let mut recovered = [0u8; 56];
    let len = taes.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

fn bigcryptor128_decrypt_ctr_into_string()
{
    println!("bigcryptor128_decrypt_ctr_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");

    let mut recovered = String::new();
    taes.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn bigcryptor128_decrypt_vec_ctr()
{
    println!("bigcryptor128_decrypt_vec_ctr()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");

    let mut recovered = vec![0; 55];
    taes.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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

fn bigcryptor128_decrypt_vec_ctr_into_vec()
{
    println!("bigcryptor128_decrypt_vec_ctr_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");

    let mut recovered = Vec::<u8>::new();
    taes.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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

fn bigcryptor128_decrypt_vec_ctr_into_array()
{
    println!("bigcryptor128_decrypt_vec_ctr_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");

    let mut recovered = [0u8; 56];
    let len = taes.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
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

fn bigcryptor128_decrypt_vec_ctr_into_string()
{
    println!("bigcryptor128_decrypt_vec_ctr_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");

    let mut recovered = String::new();
    taes.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn bigcryptor128_decrypt_array_ctr()
{
    println!("bigcryptor128_decrypt_array_ctr()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);    let mut cipher = [0_u8; 55];
    taes.encrypt_str_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");

    let mut recovered = vec![0; 55];
    let len = taes.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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

fn bigcryptor128_decrypt_array_ctr_into_vec()
{
    println!("bigcryptor128_decrypt_array_ctr_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);    let mut cipher = [0_u8; 55];
    taes.encrypt_str_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");

    let mut recovered = Vec::<u8>::new();
    taes.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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

fn bigcryptor128_decrypt_array_ctr_into_array()
{
    println!("bigcryptor128_decrypt_array_ctr_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);    let mut cipher = [0_u8; 55];
    taes.encrypt_str_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");

    let mut recovered = [0u8; 56];
    let len = taes.decrypt_array_into_array(nonce, &cipher, &mut recovered);
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

fn bigcryptor128_decrypt_array_ctr_into_string()
{
    println!("bigcryptor128_decrypt_array_ctr_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("Nonce = {:#034X}", nonce);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);    let mut cipher = [0_u8; 55];
    taes.encrypt_str_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");

    let mut recovered = String::new();
    taes.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}
