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
// #![allow(rustdoc::missing_doc_code_examples)]
// #[allow(non_camel_case_types)]

pub fn main()
{
    aes_encrypt_with_padding_iso_cbc();
    aes_encrypt_with_padding_iso_cbc_into_vec();
    aes_encrypt_with_padding_iso_cbc_into_array();
    aes_encrypt_str_with_padding_iso_cbc();
    aes_encrypt_str_with_padding_iso_cbc_into_vec();
    aes_encrypt_str_with_padding_iso_cbc_into_array();
    aes_encrypt_string_with_padding_iso_cbc();
    aes_encrypt_string_with_padding_iso_cbc_into_vec();
    aes_encrypt_string_with_padding_iso_cbc_into_array();
    aes_encrypt_vec_with_padding_iso_cbc();
    aes_encrypt_vec_with_padding_iso_cbc_into_vec();
    aes_encrypt_vec_with_padding_iso_cbc_into_array();
    aes_encrypt_array_with_padding_iso_cbc();
    aes_encrypt_array_with_padding_iso_cbc_into_vec();
    aes_encrypt_array_with_padding_iso_cbc_into_array();

    aes_decrypt_with_padding_iso_cbc();
    aes_decrypt_with_padding_iso_cbc_into_vec();
    aes_decrypt_with_padding_iso_cbc_into_array();
    aes_decrypt_with_padding_iso_cbc_into_string();
    aes_decrypt_vec_with_padding_iso_cbc();
    aes_decrypt_vec_with_padding_iso_cbc_into_vec();
    aes_decrypt_vec_with_padding_iso_cbc_into_array();
    aes_decrypt_vec_with_padding_iso_cbc_into_string();
    aes_decrypt_array_with_padding_iso_cbc();
    aes_decrypt_array_with_padding_iso_cbc_into_vec();
    aes_decrypt_array_with_padding_iso_cbc_into_array();
    aes_decrypt_array_with_padding_iso_cbc_into_string();
}

fn aes_encrypt_with_padding_iso_cbc()
{
    println!("aes_encrypt_with_padding_iso_cbc()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();

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
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    println!("-------------------------------");
}

fn aes_encrypt_with_padding_iso_cbc_into_vec()
{
    println!("aes_encrypt_with_padding_iso_cbc_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
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
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
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
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
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
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    println!("-------------------------------");
}

fn aes_encrypt_with_padding_iso_cbc_into_array()
{
    println!("aes_encrypt_with_padding_iso_cbc_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
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
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
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
    a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
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
    a_rijndael.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    println!("-------------------------------");
}

fn aes_encrypt_str_with_padding_iso_cbc()
{
    println!("aes_encrypt_str_with_padding_iso_cbc()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
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
    a_aes.encrypt_str(iv, &message, cipher.as_mut_ptr());
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
    a_aes.encrypt_str(iv, &message, cipher.as_mut_ptr());
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
    a_rijndael.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    println!("-------------------------------");
}

fn aes_encrypt_str_with_padding_iso_cbc_into_vec()
{
    println!("aes_encrypt_str_with_padding_iso_cbc_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
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
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
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
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
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
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    println!("-------------------------------");
}

fn aes_encrypt_str_with_padding_iso_cbc_into_array()
{
    println!("aes_encrypt_str_with_padding_iso_cbc_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
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
    a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
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
    a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
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
    a_rijndael.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    println!("-------------------------------");
}

fn aes_encrypt_string_with_padding_iso_cbc()
{
    println!("aes_encrypt_string_with_padding_iso_cbc()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
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

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_string(iv, &message, cipher.as_mut_ptr());
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

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_string(iv, &message, cipher.as_mut_ptr());
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

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_string(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    println!("-------------------------------");
}

fn aes_encrypt_string_with_padding_iso_cbc_into_vec()
{
    println!("aes_encrypt_string_with_padding_iso_cbc_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
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

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(iv, &message, &mut cipher);
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

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(iv, &message, &mut cipher);
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

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_string_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    println!("-------------------------------");
}

fn aes_encrypt_string_with_padding_iso_cbc_into_array()
{
    println!("aes_encrypt_string_with_padding_iso_cbc_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
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

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_string_into_array(iv, &message, &mut cipher);
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

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_string_into_array(iv, &message, &mut cipher);
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

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_string_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_with_padding_iso_cbc()
{
    println!("aes_encrypt_vec_with_padding_iso_cbc()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
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
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
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
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
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
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_with_padding_iso_cbc_into_vec()
{
    println!("aes_encrypt_vec_with_padding_iso_cbc_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
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
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(iv, &message, &mut cipher);
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
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(iv, &message, &mut cipher);
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
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_vec_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_with_padding_iso_cbc_into_array()
{
    println!("aes_encrypt_vec_with_padding_iso_cbc_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
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
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_vec_into_array(iv, &message, &mut cipher);
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
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_vec_into_array(iv, &message, &mut cipher);
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
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_vec_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    println!("-------------------------------");
}

fn aes_encrypt_array_with_padding_iso_cbc()
{
    println!("aes_encrypt_array_with_padding_iso_cbc()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = [0_u8; 64];
    a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
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

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
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

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
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

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    println!("-------------------------------");
}

fn aes_encrypt_array_with_padding_iso_cbc_into_vec()
{
    println!("aes_encrypt_array_with_padding_iso_cbc_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
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

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(iv, &message, &mut cipher);
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

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(iv, &message, &mut cipher);
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

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_array_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    println!("-------------------------------");
}

fn aes_encrypt_array_with_padding_iso_cbc_into_array()
{
    println!("aes_encrypt_array_with_padding_iso_cbc_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
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

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_array_into_array(iv, &message, &mut cipher);
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

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_array_into_array(iv, &message, &mut cipher);
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

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_array_into_array(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    println!("-------------------------------");
}


fn aes_decrypt_with_padding_iso_cbc()
{
    println!("aes_decrypt_with_padding_iso_cbc()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
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
    a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
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
    a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
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
    a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt(iv.clone(), message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    
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
    println!("-------------------------------");
}


fn aes_decrypt_with_padding_iso_cbc_into_vec()
{
    println!("aes_decrypt_with_padding_iso_cbc_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    println!();

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    println!();

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    println!();

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    println!();

    let mut recovered = Vec::<u8>::new();
    a_rijndael.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt(iv.clone(), message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    
    let mut recovered = Vec::<u8>::new();
    a_rijndael.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

fn aes_decrypt_with_padding_iso_cbc_into_array()
{
    println!("aes_decrypt_with_padding_iso_cbc_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");

    let mut recovered = [0; 64];
    let len = a_aes.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");

    let mut recovered = [0; 64];
    a_aes.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");

    let mut recovered = [0; 64];
    a_aes.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");

    let mut recovered = [0; 64];
    a_rijndael.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    
    let mut recovered = [0; 64];
    a_rijndael.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

fn aes_decrypt_with_padding_iso_cbc_into_string()
{
    println!("aes_decrypt_with_padding_iso_cbc_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");

    let mut converted= String::new();
    a_aes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut converted);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");

    let mut converted= String::new();
    a_aes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut converted);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");

    let mut converted= String::new();
    a_aes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut converted);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");

    let mut converted= String::new();
    a_rijndael.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut converted);
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    
    let mut converted= String::new();
    a_rijndael.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut converted);
    println!("B =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}

fn aes_decrypt_vec_with_padding_iso_cbc()
{
    println!("aes_decrypt_vec_with_padding_iso_cbc()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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

fn aes_decrypt_vec_with_padding_iso_cbc_into_vec()
{
    println!("aes_decrypt_vec_with_padding_iso_cbc_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");

    let mut recovered = Vec::<u8>::new();
    a_rijndael.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    
    let mut recovered = Vec::<u8>::new();
    a_rijndael.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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

fn aes_decrypt_vec_with_padding_iso_cbc_into_array()
{
    println!("aes_decrypt_vec_with_padding_iso_cbc_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");

    let mut recovered = [0; 64];
    let len = a_aes.decrypt_vec_into_array(iv, &cipher, &mut recovered);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");

    let mut recovered = [0; 64];
    a_aes.decrypt_vec_into_array(iv, &cipher, &mut recovered);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");

    let mut recovered = [0; 64];
    a_aes.decrypt_vec_into_array(iv, &cipher, &mut recovered);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");

    let mut recovered = [0; 64];
    a_rijndael.decrypt_vec_into_array(iv, &cipher, &mut recovered);
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    
    let mut recovered = [0; 64];
    a_rijndael.decrypt_vec_into_array(iv, &cipher, &mut recovered);
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

fn aes_decrypt_vec_with_padding_iso_cbc_into_string()
{
    println!("aes_decrypt_vec_with_padding_iso_cbc_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");

    let mut converted= String::new();
    a_aes.decrypt_vec_into_string(iv, &cipher, &mut converted);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");

    let mut converted= String::new();
    a_aes.decrypt_vec_into_string(iv, &cipher, &mut converted);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");

    let mut converted= String::new();
    a_aes.decrypt_vec_into_string(iv, &cipher, &mut converted);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");

    let mut converted= String::new();
    a_rijndael.decrypt_vec_into_string(iv, &cipher, &mut converted);
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    
    let mut converted= String::new();
    a_rijndael.decrypt_vec_into_string(iv, &cipher, &mut converted);
    println!("B =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}

fn aes_decrypt_array_with_padding_iso_cbc()
{
    println!("aes_decrypt_array_with_padding_iso_cbc()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
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
    a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
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
    a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
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
    a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    
    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
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

fn aes_decrypt_array_with_padding_iso_cbc_into_vec()
{
    println!("aes_decrypt_array_with_padding_iso_cbc_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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

fn aes_decrypt_array_with_padding_iso_cbc_into_array()
{
    println!("aes_decrypt_array_with_padding_iso_cbc_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");

    let mut recovered = [0; 64];
    let len = a_aes.decrypt_array_into_array(iv, &cipher, &mut recovered);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");

    let mut recovered = [0; 64];
    let len = a_aes.decrypt_array_into_array(iv, &cipher, &mut recovered);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");

    let mut recovered = [0; 64];
    let len = a_aes.decrypt_array_into_array(iv, &cipher, &mut recovered);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");

    let mut recovered = [0; 64];
    let len = a_rijndael.decrypt_array_into_array(iv, &cipher, &mut recovered);
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");

    let mut recovered = [0; 64];
    let len = a_rijndael.decrypt_array_into_array(iv, &cipher, &mut recovered);
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

fn aes_decrypt_array_with_padding_iso_cbc_into_string()
{
    println!("aes_decrypt_array_with_padding_iso_cbc_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");

    let mut converted= String::new();
    a_aes.decrypt_array_into_string(iv, &cipher, &mut converted);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");

    let mut converted= String::new();
    a_aes.decrypt_array_into_string(iv, &cipher, &mut converted);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");

    let mut converted= String::new();
    a_aes.decrypt_array_into_string(iv, &cipher, &mut converted);
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
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");

    let mut converted= String::new();
    a_rijndael.decrypt_array_into_string(iv, &cipher, &mut converted);
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
    sha3.absorb_str("Initialize");
    let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    iv.src = sha3.get_hash_value_in_array();
    let iv = unsafe { iv.des };
    print!("IV =\t");
    for i in 0..16
        { print!("{:08X}", iv[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 64];
    a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");

    let mut converted= String::new();
    a_rijndael.decrypt_array_into_string(iv, &cipher, &mut converted);
    println!("B =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}
