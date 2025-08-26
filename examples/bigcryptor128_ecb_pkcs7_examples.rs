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
    bigcryptor128_encrypt_with_padding_pkcs7_ecb();
    bigcryptor128_encrypt_with_padding_pkcs7_ecb_into_vec();
    bigcryptor128_encrypt_with_padding_pkcs7_ecb_into_array();
    bigcryptor128_encrypt_str_with_padding_pkcs7_ecb();
    bigcryptor128_encrypt_str_with_padding_pkcs7_ecb_into_vec();
    bigcryptor128_encrypt_str_with_padding_pkcs7_ecb_into_array();
    bigcryptor128_encrypt_string_with_padding_pkcs7_ecb();
    bigcryptor128_encrypt_string_with_padding_pkcs7_ecb_into_vec();
    bigcryptor128_encrypt_string_with_padding_pkcs7_ecb_into_array();
    bigcryptor128_encrypt_vec_with_padding_pkcs7_ecb();
    bigcryptor128_encrypt_vec_with_padding_pkcs7_ecb_into_vec();
    bigcryptor128_encrypt_vec_with_padding_pkcs7_ecb_into_array();
    bigcryptor128_encrypt_array_with_padding_pkcs7_ecb();
    bigcryptor128_encrypt_array_with_padding_pkcs7_ecb_into_vec();
    bigcryptor128_encrypt_array_with_padding_pkcs7_ecb_into_array();

    bigcryptor128_decrypt_with_padding_pkcs7_ecb();
    bigcryptor128_decrypt_with_padding_pkcs7_ecb_into_vec();
    bigcryptor128_decrypt_with_padding_pkcs7_ecb_into_array();
    bigcryptor128_decrypt_with_padding_pkcs7_ecb_into_string();
    bigcryptor128_decrypt_vec_with_padding_pkcs7_ecb();
    bigcryptor128_decrypt_vec_with_padding_pkcs7_ecb_into_vec();
    bigcryptor128_decrypt_vec_with_padding_pkcs7_ecb_into_array();
    bigcryptor128_decrypt_vec_with_padding_pkcs7_ecb_into_string();
    bigcryptor128_decrypt_array_with_padding_pkcs7_ecb();
    bigcryptor128_decrypt_array_with_padding_pkcs7_ecb_into_vec();
    bigcryptor128_decrypt_array_with_padding_pkcs7_ecb_into_array();
    bigcryptor128_decrypt_array_with_padding_pkcs7_ecb_into_string();
}

fn bigcryptor128_encrypt_with_padding_pkcs7_ecb()
{
    println!("bigcryptor128_encrypt_with_padding_pkcs7_ecb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    taes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_with_padding_pkcs7_ecb_into_vec()
{
    println!("bigcryptor128_encrypt_with_padding_pkcs7_ecb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_with_padding_pkcs7_ecb_into_array()
{
    println!("bigcryptor128_encrypt_with_padding_pkcs7_ecb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    taes.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_str_with_padding_pkcs7_ecb()
{
    println!("bigcryptor128_encrypt_str_with_padding_pkcs7_ecb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    taes.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_str_with_padding_pkcs7_ecb_into_vec()
{
    println!("bigcryptor128_encrypt_str_with_padding_pkcs7_ecb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_str_with_padding_pkcs7_ecb_into_array()
{
    println!("bigcryptor128_encrypt_str_with_padding_pkcs7_ecb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    let mut cipher = [0_u8; 64];
    taes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_string_with_padding_pkcs7_ecb()
{
    println!("bigcryptor128_encrypt_string_with_padding_pkcs7_ecb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.".to_string();
    let mut cipher = [0_u8; 64];
    taes.encrypt_string(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_string_with_padding_pkcs7_ecb_into_vec()
{
    println!("bigcryptor128_encrypt_string_with_padding_pkcs7_ecb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.".to_string();
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_string_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_string_with_padding_pkcs7_ecb_into_array()
{
    println!("bigcryptor128_encrypt_string_with_padding_pkcs7_ecb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.".to_string();
    let mut cipher = [0_u8; 64];
    taes.encrypt_string_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_vec_with_padding_pkcs7_ecb()
{
    println!("bigcryptor128_encrypt_vec_with_padding_pkcs7_ecb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    taes.encrypt_vec(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_vec_with_padding_pkcs7_ecb_into_vec()
{
    println!("bigcryptor128_encrypt_vec_with_padding_pkcs7_ecb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_vec_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_vec_with_padding_pkcs7_ecb_into_array()
{
    println!("bigcryptor128_encrypt_vec_with_padding_pkcs7_ecb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    taes.encrypt_vec_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_array_with_padding_pkcs7_ecb()
{
    println!("bigcryptor128_encrypt_array_with_padding_pkcs7_ecb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = [0_u8; 64];
    taes.encrypt_array(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_array_with_padding_pkcs7_ecb_into_vec()
{
    println!("bigcryptor128_encrypt_array_with_padding_pkcs7_ecb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_array_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_array_with_padding_pkcs7_ecb_into_array()
{
    println!("bigcryptor128_encrypt_array_with_padding_pkcs7_ecb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = [0_u8; 64];
    taes.encrypt_array_into_array(&message, &mut cipher);
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");
    println!("-------------------------------");
}

fn bigcryptor128_decrypt_with_padding_pkcs7_ecb()
{
    println!("bigcryptor128_decrypt_with_padding_pkcs7_ecb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");

    let mut recovered = vec![0; 55];
    taes.decrypt(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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

fn bigcryptor128_decrypt_with_padding_pkcs7_ecb_into_vec()
{
    println!("bigcryptor128_decrypt_with_padding_pkcs7_ecb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");

    let mut recovered = Vec::<u8>::new();
    taes.decrypt_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

fn bigcryptor128_decrypt_with_padding_pkcs7_ecb_into_array()
{
    println!("bigcryptor128_decrypt_with_padding_pkcs7_ecb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");

    let mut recovered = [0u8; 64];
    let len = taes.decrypt_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 00 00 00 00 00 00 00 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.write(&recovered);
    unsafe { converted.as_mut_vec() }.truncate(len as usize);
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}

fn bigcryptor128_decrypt_with_padding_pkcs7_ecb_into_string()
{
    println!("bigcryptor128_decrypt_with_padding_pkcs7_ecb_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");

    let mut recovered = String::new();
    taes.decrypt_into_string(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn bigcryptor128_decrypt_vec_with_padding_pkcs7_ecb()
{
    println!("bigcryptor128_decrypt_vec_with_padding_pkcs7_ecb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");

    let mut recovered = vec![0; 55];
    taes.decrypt_vec(&cipher, recovered.as_mut_ptr());
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

fn bigcryptor128_decrypt_vec_with_padding_pkcs7_ecb_into_vec()
{
    println!("bigcryptor128_decrypt_vec_with_padding_pkcs7_ecb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");

    let mut recovered = Vec::<u8>::new();
    taes.decrypt_vec_into_vec(&cipher, &mut recovered);
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

fn bigcryptor128_decrypt_vec_with_padding_pkcs7_ecb_into_array()
{
    println!("bigcryptor128_decrypt_vec_with_padding_pkcs7_ecb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");

    let mut recovered = [0u8; 64];
    let len = taes.decrypt_vec_into_array(&cipher, &mut recovered);
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 00 00 00 00 00 00 00 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.write(&recovered);
    unsafe { converted.as_mut_vec() }.truncate(len as usize);
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}

fn bigcryptor128_decrypt_vec_with_padding_pkcs7_ecb_into_string()
{
    println!("bigcryptor128_decrypt_vec_with_padding_pkcs7_ecb_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");

    let mut recovered = String::new();
    taes.decrypt_vec_into_string(&cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn bigcryptor128_decrypt_array_with_padding_pkcs7_ecb()
{
    println!("bigcryptor128_decrypt_array_with_padding_pkcs7_ecb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);    let mut cipher = [0_u8; 64];
    taes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");

    let mut recovered = vec![0; 55];
    let len = taes.decrypt_array(&cipher, recovered.as_mut_ptr());
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

fn bigcryptor128_decrypt_array_with_padding_pkcs7_ecb_into_vec()
{
    println!("bigcryptor128_decrypt_array_with_padding_pkcs7_ecb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);    let mut cipher = [0_u8; 64];
    taes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");

    let mut recovered = Vec::<u8>::new();
    taes.decrypt_array_into_vec(&cipher, &mut recovered);
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

fn bigcryptor128_decrypt_array_with_padding_pkcs7_ecb_into_array()
{
    println!("bigcryptor128_decrypt_array_with_padding_pkcs7_ecb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);    let mut cipher = [0_u8; 64];
    taes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");

    let mut recovered = [0u8; 64];
    let len = taes.decrypt_array_into_array(&cipher, &mut recovered);
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 00 00 00 00 00 00 00 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.write(&recovered);
    unsafe { converted.as_mut_vec() }.truncate(len as usize);
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}

fn bigcryptor128_decrypt_array_with_padding_pkcs7_ecb_into_string()
{
    println!("bigcryptor128_decrypt_array_with_padding_pkcs7_ecb_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, ECB_PKCS7 };

    // TAES_128 case
    let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);    let mut cipher = [0_u8; 64];
    taes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "5A 29 46 8D EE CE 8C C2 03 FE 08 AE 41 30 82 7A 03 BF 55 05 02 AE C0 4A 70 12 31 0E F6 8F 86 28 2B F9 0C 3F F4 84 D9 C9 3A 6B 68 4D E2 A3 DC F7 B7 D6 E3 0F 06 18 09 67 C7 DE 28 63 2B CE F0 B6 ");

    let mut recovered = String::new();
    taes.decrypt_array_into_string(&cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}
