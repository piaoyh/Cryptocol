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
    aes_encrypt_with_padding_iso_ecb();
    aes_encrypt_with_padding_iso_ecb_into_vec();
    aes_encrypt_with_padding_iso_ecb_into_array();
    aes_encrypt_str_with_padding_iso_ecb();
    aes_encrypt_str_with_padding_iso_ecb_into_vec();
    aes_encrypt_str_with_padding_iso_ecb_into_array();
    aes_encrypt_string_with_padding_iso_ecb();
    aes_encrypt_string_with_padding_iso_ecb_into_vec();
    aes_encrypt_string_with_padding_iso_ecb_into_array();
    aes_encrypt_vec_with_padding_iso_ecb();
    aes_encrypt_vec_with_padding_iso_ecb_into_vec();
    aes_encrypt_vec_with_padding_iso_ecb_into_array();
    aes_encrypt_array_with_padding_iso_ecb();
    aes_encrypt_array_with_padding_iso_ecb_into_vec();
    aes_encrypt_array_with_padding_iso_ecb_into_array();

    aes_decrypt_with_padding_iso_ecb();
    aes_decrypt_with_padding_iso_ecb_into_vec();
    aes_decrypt_with_padding_iso_ecb_into_array();
    aes_decrypt_with_padding_iso_ecb_into_string();
    aes_decrypt_vec_with_padding_iso_ecb();
    aes_decrypt_vec_with_padding_iso_ecb_into_vec();
    aes_decrypt_vec_with_padding_iso_ecb_into_array();
    aes_decrypt_vec_with_padding_iso_ecb_into_string();
    aes_decrypt_array_with_padding_iso_ecb();
    aes_decrypt_array_with_padding_iso_ecb_into_vec();
    aes_decrypt_array_with_padding_iso_ecb_into_array();
    aes_decrypt_array_with_padding_iso_ecb_into_string();
}

fn aes_encrypt_with_padding_iso_ecb()
{
    println!("aes_encrypt_with_padding_iso_ecb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = Rijndael_256_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    println!("-------------------------------");
}

fn aes_encrypt_with_padding_iso_ecb_into_vec()
{
    println!("aes_encrypt_with_padding_iso_ecb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    println!("-------------------------------");
}

fn aes_encrypt_with_padding_iso_ecb_into_array()
{
    println!("aes_encrypt_with_padding_iso_ecb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    println!("-------------------------------");
}

fn aes_encrypt_str_with_padding_iso_ecb()
{
    println!("aes_encrypt_str_with_padding_iso_ecb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    println!("-------------------------------");
}

fn aes_encrypt_str_with_padding_iso_ecb_into_vec()
{
    println!("aes_encrypt_str_with_padding_iso_ecb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    println!("-------------------------------");
}

fn aes_encrypt_str_with_padding_iso_ecb_into_array()
{
    println!("aes_encrypt_str_with_padding_iso_ecb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    println!("-------------------------------");
}

fn aes_encrypt_string_with_padding_iso_ecb()
{
    println!("aes_encrypt_string_with_padding_iso_ecb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_string(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_string(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_string(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_string(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_string(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    println!("-------------------------------");
}

fn aes_encrypt_string_with_padding_iso_ecb_into_vec()
{
    println!("aes_encrypt_string_with_padding_iso_ecb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_string_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_string_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    println!("-------------------------------");
}

fn aes_encrypt_string_with_padding_iso_ecb_into_array()
{
    println!("aes_encrypt_string_with_padding_iso_ecb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_string_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_string_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_string_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_string_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_string_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_with_padding_iso_ecb()
{
    println!("aes_encrypt_vec_with_padding_iso_ecb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_vec(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_vec(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_vec(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_vec(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_vec(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_with_padding_iso_ecb_into_vec()
{
    println!("aes_encrypt_vec_with_padding_iso_ecb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_vec_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_vec_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_with_padding_iso_ecb_into_array()
{
    println!("aes_encrypt_vec_with_padding_iso_ecb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_vec_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_vec_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_vec_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_vec_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_vec_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    println!("-------------------------------");
}

fn aes_encrypt_array_with_padding_iso_ecb()
{
    println!("aes_encrypt_array_with_padding_iso_ecb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    println!("-------------------------------");
}

fn aes_encrypt_array_with_padding_iso_ecb_into_vec()
{
    println!("aes_encrypt_array_with_padding_iso_ecb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_array_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_array_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    println!("-------------------------------");
}

fn aes_encrypt_array_with_padding_iso_ecb_into_array()
{
    println!("aes_encrypt_array_with_padding_iso_ecb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_array_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_array_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_array_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_array_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_array_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    println!("-------------------------------");
}

fn aes_decrypt_with_padding_iso_ecb()
{
    println!("aes_decrypt_with_padding_iso_ecb");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    
    let mut recovered = vec![0; 55];
    a_rijndael.decrypt(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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


fn aes_decrypt_with_padding_iso_ecb_into_vec()
{
    println!("aes_decrypt_with_padding_iso_ecb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");
    println!();

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");
    println!();

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");
    println!();

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");
    println!();

    let mut recovered = Vec::<u8>::new();
    a_rijndael.decrypt_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    
    let mut recovered = Vec::<u8>::new();
    a_rijndael.decrypt_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

fn aes_decrypt_with_padding_iso_ecb_into_array()
{
    println!("aes_decrypt_with_padding_iso_ecb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");

    let mut recovered = [0; 64];
    let len = a_aes.decrypt_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");

    let mut recovered = [0; 64];
    a_aes.decrypt_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");

    let mut recovered = [0; 64];
    a_aes.decrypt_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");

    let mut recovered = [0; 64];
    a_rijndael.decrypt_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    
    let mut recovered = [0; 64];
    a_rijndael.decrypt_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

fn aes_decrypt_with_padding_iso_ecb_into_string()
{
    println!("aes_decrypt_with_padding_iso_ecb_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");

    let mut converted= String::new();
    a_aes.decrypt_into_string(cipher.as_ptr(), cipher.len() as u64, &mut converted);
    println!("B =\t{}", converted);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");

    let mut converted= String::new();
    a_aes.decrypt_into_string(cipher.as_ptr(), cipher.len() as u64, &mut converted);
    println!("B =\t{}", converted);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");

    let mut converted= String::new();
    a_aes.decrypt_into_string(cipher.as_ptr(), cipher.len() as u64, &mut converted);
    println!("B =\t{}", converted);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");

    let mut converted= String::new();
    a_rijndael.decrypt_into_string(cipher.as_ptr(), cipher.len() as u64, &mut converted);
    println!("B =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    
    let mut converted= String::new();
    a_rijndael.decrypt_into_string(cipher.as_ptr(), cipher.len() as u64, &mut converted);
    println!("B =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}

fn aes_decrypt_vec_with_padding_iso_ecb()
{
    println!("aes_decrypt_vec_with_padding_iso_ecb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_vec(&cipher, recovered.as_mut_ptr());
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_vec(&cipher, recovered.as_mut_ptr());
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_vec(&cipher, recovered.as_mut_ptr());
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_vec(&cipher, recovered.as_mut_ptr());
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

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_vec(&cipher, recovered.as_mut_ptr());
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

fn aes_decrypt_vec_with_padding_iso_ecb_into_vec()
{
    println!("aes_decrypt_vec_with_padding_iso_ecb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(&cipher, &mut recovered);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(&cipher, &mut recovered);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(&cipher, &mut recovered);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");

    let mut recovered = Vec::<u8>::new();
    a_rijndael.decrypt_vec_into_vec(&cipher, &mut recovered);
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

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    
    let mut recovered = Vec::<u8>::new();
    a_rijndael.decrypt_vec_into_vec(&cipher, &mut recovered);
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

fn aes_decrypt_vec_with_padding_iso_ecb_into_array()
{
    println!("aes_decrypt_vec_with_padding_iso_ecb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");

    let mut recovered = [0; 64];
    let len = a_aes.decrypt_vec_into_array(&cipher, &mut recovered);
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
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");

    let mut recovered = [0; 64];
    a_aes.decrypt_vec_into_array(&cipher, &mut recovered);
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
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");

    let mut recovered = [0; 64];
    a_aes.decrypt_vec_into_array(&cipher, &mut recovered);
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
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");

    let mut recovered = [0; 64];
    a_rijndael.decrypt_vec_into_array(&cipher, &mut recovered);
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
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    
    let mut recovered = [0; 64];
    a_rijndael.decrypt_vec_into_array(&cipher, &mut recovered);
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

fn aes_decrypt_vec_with_padding_iso_ecb_into_string()
{
    println!("aes_decrypt_vec_with_padding_iso_ecb_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");

    let mut converted= String::new();
    a_aes.decrypt_vec_into_string(&cipher, &mut converted);
    println!("B =\t{}", converted);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");

    let mut converted= String::new();
    a_aes.decrypt_vec_into_string(&cipher, &mut converted);
    println!("B =\t{}", converted);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");

    let mut converted= String::new();
    a_aes.decrypt_vec_into_string(&cipher, &mut converted);
    println!("B =\t{}", converted);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");

    let mut converted= String::new();
    a_rijndael.decrypt_vec_into_string(&cipher, &mut converted);
    println!("B =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    
    let mut converted= String::new();
    a_rijndael.decrypt_vec_into_string(&cipher, &mut converted);
    println!("B =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}

fn aes_decrypt_array_with_padding_iso_ecb()
{
    println!("aes_decrypt_array_with_padding_iso_ecb()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array(&cipher, recovered.as_mut_ptr());
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array(&cipher, recovered.as_mut_ptr());
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array(&cipher, recovered.as_mut_ptr());
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_array(&cipher, recovered.as_mut_ptr());
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

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");
    
    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_array(&cipher, recovered.as_mut_ptr());
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

fn aes_decrypt_array_with_padding_iso_ecb_into_vec()
{
    println!("aes_decrypt_array_with_padding_iso_ecb_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array_into_vec(&cipher, &mut recovered);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array_into_vec(&cipher, &mut recovered);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array_into_vec(&cipher, &mut recovered);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_array_into_vec(&cipher, &mut recovered);
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

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_array_into_vec(&cipher, &mut recovered);
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

fn aes_decrypt_array_with_padding_iso_ecb_into_array()
{
    println!("aes_decrypt_array_with_padding_iso_ecb_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");

    let mut recovered = [0; 64];
    let len = a_aes.decrypt_array_into_array(&cipher, &mut recovered);
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
    println!();

    // Normal case for AES-192
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..24
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_192::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");

    let mut recovered = [0; 64];
    let len = a_aes.decrypt_array_into_array(&cipher, &mut recovered);
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
    println!();

    // Normal case for AES-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_aes = AES_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");

    let mut recovered = [0; 64];
    let len = a_aes.decrypt_array_into_array(&cipher, &mut recovered);
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
    println!();

    // Normal case for Rijndael-256-256
    let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    print!("K =\t");
    for i in 0..32
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");

    let mut recovered = [0; 64];
    let len = a_rijndael.decrypt_array_into_array(&cipher, &mut recovered);
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
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");

    let mut recovered = [0; 64];
    let len = a_rijndael.decrypt_array_into_array(&cipher, &mut recovered);
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

fn aes_decrypt_array_with_padding_iso_ecb_into_string()
{
    println!("aes_decrypt_array_with_padding_iso_ecb_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 3F C4 ED AB CE EE B1 9A FF 05 29 FF 65 BD 25 93 ");

    let mut converted= String::new();
    a_aes.decrypt_array_into_string(&cipher, &mut converted);
    println!("B =\t{}", converted);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B 70 E3 6D 5B F7 85 2D BD FB 3B BE E3 6E A3 56 25 ");

    let mut converted= String::new();
    a_aes.decrypt_array_into_string(&cipher, &mut converted);
    println!("B =\t{}", converted);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 BE 7B 8D 4A 52 E8 03 1D 63 0D 5E F1 82 3A 28 A3 ");

    let mut converted= String::new();
    a_aes.decrypt_array_into_string(&cipher, &mut converted);
    println!("B =\t{}", converted);
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

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 53 E1 83 F5 55 E5 86 3E 21 C8 DF DE C8 9C 06 C5 41 DE 7C 88 FE E8 F3 A0 64 8B 52 DA 99 D9 BF F6 ");

    let mut converted= String::new();
    a_rijndael.decrypt_array_into_string(&cipher, &mut converted);
    println!("B =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!();

    // Normal case for Rijndael-512-512 for post-quantum
    use cryptocol::number::SharedArrays;
    use cryptocol::hash::SHA3_512;
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("Post-quantum");
    let key: [u8; 64] = sha3.get_hash_value_in_array();
    print!("K =\t");
    for i in 0..64
        { print!("{:02X}", key[i]); }
    println!();
    let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "F8 B7 BD C0 65 7B DC D8 7F 90 FC 2E 6B A6 D9 8C A7 0A D5 CF 9A 2D A4 99 4C 86 1F 67 56 88 2C E7 06 6C F2 95 FE 9E AF BF 9F 73 88 70 C7 A5 84 3F 82 27 76 C8 B7 CC 3B 44 07 CD AD F5 72 8E 73 72 ");

    let mut converted= String::new();
    a_rijndael.decrypt_array_into_string(&cipher, &mut converted);
    println!("B =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}
