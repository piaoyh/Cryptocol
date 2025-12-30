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

// #![allow(rustdoc::missing_doc_code_examples)]
// #[allow(non_camel_case_types)]
pub fn main()
{
    aes_encrypt_ctr();
    aes_encrypt_ctr_into_vec();
    aes_encrypt_ctr_into_array();
    aes_encrypt_str_ctr();
    aes_encrypt_str_ctr_into_vec();
    aes_encrypt_str_ctr_into_array();
    aes_encrypt_string_ctr();
    aes_encrypt_string_ctr_into_vec();
    aes_encrypt_string_ctr_into_array();
    aes_encrypt_vec_ctr();
    aes_encrypt_vec_ctr_into_vec();
    aes_encrypt_vec_ctr_into_array();
    aes_encrypt_array_ctr();
    aes_encrypt_array_ctr_into_vec();
    aes_encrypt_array_ctr_into_array();

    aes_decrypt_ctr();
    aes_decrypt_ctr_into_vec();
    aes_decrypt_ctr_into_array();
    aes_decrypt_ctr_into_string();
    aes_decrypt_vec_ctr();
    aes_decrypt_vec_ctr_into_vec();
    aes_decrypt_vec_ctr_into_array();
    aes_decrypt_vec_ctr_into_string();
    aes_decrypt_array_ctr();
    aes_decrypt_array_ctr_into_vec();
    aes_decrypt_array_ctr_into_array();
    aes_decrypt_array_ctr_into_string();
}

fn aes_encrypt_ctr()
{
    println!("aes_encrypt_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();

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
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    println!("-------------------------------");
}

fn aes_encrypt_ctr_into_vec()
{
    println!("aes_encrypt_ctr_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    println!("-------------------------------");
}

fn aes_encrypt_ctr_into_array()
{
    println!("aes_encrypt_ctr_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
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
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
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
    a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
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
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    println!("-------------------------------");
}

fn aes_encrypt_str_ctr()
{
    println!("aes_encrypt_str_ctr()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str(nonce, &message, cipher.as_mut_ptr());
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
    a_aes.encrypt_str(nonce, &message, cipher.as_mut_ptr());
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
    a_aes.encrypt_str(nonce, &message, cipher.as_mut_ptr());
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
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    println!("-------------------------------");
}

fn aes_encrypt_str_ctr_into_vec()
{
    println!("aes_encrypt_str_ctr_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
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
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
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
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
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
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    println!("-------------------------------");
}

fn aes_encrypt_str_ctr_into_array()
{
    println!("aes_encrypt_str_ctr_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce, &message, &mut cipher);
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
    a_aes.encrypt_str_into_array(nonce, &message, &mut cipher);
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
    a_aes.encrypt_str_into_array(nonce, &message, &mut cipher);
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
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_str_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_str_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    println!("-------------------------------");
}

fn aes_encrypt_string_ctr()
{
    println!("aes_encrypt_string_ctr()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_string(nonce, &message, cipher.as_mut_ptr());
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

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_string(nonce, &message, cipher.as_mut_ptr());
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

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_string(nonce, &message, cipher.as_mut_ptr());
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
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    println!("-------------------------------");
}

fn aes_encrypt_string_ctr_into_vec()
{
    println!("aes_encrypt_string_ctr_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(nonce, &message, &mut cipher);
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

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(nonce, &message, &mut cipher);
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

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_string_into_vec(nonce, &message, &mut cipher);
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
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_string_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_string_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    println!("-------------------------------");
}

fn aes_encrypt_string_ctr_into_array()
{
    println!("aes_encrypt_string_ctr_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_string_into_array(nonce, &message, &mut cipher);
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

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_string_into_array(nonce, &message, &mut cipher);
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

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_string_into_array(nonce, &message, &mut cipher);
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
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_string_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_string_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_ctr()
{
    println!("aes_encrypt_vec_ctr()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
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
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
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
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
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
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_ctr_into_vec()
{
    println!("aes_encrypt_vec_ctr_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(nonce, &message, &mut cipher);
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
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(nonce, &message, &mut cipher);
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
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_vec_into_vec(nonce, &message, &mut cipher);
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
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    println!("-------------------------------");
}

fn aes_encrypt_vec_ctr_into_array()
{
    println!("aes_encrypt_vec_ctr_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_vec_into_array(nonce, &message, &mut cipher);
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
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_vec_into_array(nonce, &message, &mut cipher);
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
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_vec_into_array(nonce, &message, &mut cipher);
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
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_vec_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let message = unsafe { message.to_string().as_mut_vec().clone() };
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_vec_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    println!("-------------------------------");
}

fn aes_encrypt_array_ctr()
{
    println!("aes_encrypt_array_ctr()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
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

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
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

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
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
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    println!("-------------------------------");
}

fn aes_encrypt_array_ctr_into_vec()
{
    println!("aes_encrypt_array_ctr_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(nonce, &message, &mut cipher);
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

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(nonce, &message, &mut cipher);
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

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_array_into_vec(nonce, &message, &mut cipher);
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
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_array_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_array_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    println!("-------------------------------");
}

fn aes_encrypt_array_ctr_into_array()
{
    println!("aes_encrypt_array_ctr_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_array_into_array(nonce, &message, &mut cipher);
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

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_array_into_array(nonce, &message, &mut cipher);
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

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_array_into_array(nonce, &message, &mut cipher);
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
    let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_array_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let mes = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", mes);
    let mut message = [0_u8; 55];
    message.copy_from_slice(mes.as_bytes());
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_array_into_array(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    println!("-------------------------------");
}


fn aes_decrypt_ctr()
{
    println!("aes_decrypt_ctr");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0], nonce[1], nonce[2], nonce[3]);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
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
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
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
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
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
    a_rijndael.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt(nonce.clone(), message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    
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
    println!("-------------------------------");
}


fn aes_decrypt_ctr_into_vec()
{
    println!("aes_decrypt_ctr_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 9D 7C 52 E3 90 02 C5 7B EE BB 50 EE 3D 2C 3A 81 33 B7 54 77 35 FB 9F 48 6F 63 17 DC 5E 64 0E 29 4B 48 6D 53 24 CF D3 ");
    println!();

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 4C DB 3F E9 F3 83 1E 7D D4 F7 5F 3D 14 28 B9 CE B0 11 61 53 FD E0 8F BC 2D 55 45 7E 24 C1 6A 9B 1A 82 94 4A 34 27 43 ");
    println!();

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 79 17 99 06 B0 DD 3E 41 2B FF AD 70 F5 17 F4 65 D7 B0 AC FA 83 69 BF 8D C6 E6 6C 62 1C 5C 90 EA 45 D8 34 45 02 BE 33 ");
    println!();

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    a_rijndael.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");
    println!();

    let mut recovered = Vec::<u8>::new();
    a_rijndael.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    
    let mut recovered = Vec::<u8>::new();
    a_rijndael.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

fn aes_decrypt_ctr_into_array()
{
    println!("aes_decrypt_ctr_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 9D 7C 52 E3 90 02 C5 7B EE BB 50 EE 3D 2C 3A 81 33 B7 54 77 35 FB 9F 48 6F 63 17 DC 5E 64 0E 29 4B 48 6D 53 24 CF D3 ");

    let mut recovered = [0; 64];
    let len = a_aes.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 4C DB 3F E9 F3 83 1E 7D D4 F7 5F 3D 14 28 B9 CE B0 11 61 53 FD E0 8F BC 2D 55 45 7E 24 C1 6A 9B 1A 82 94 4A 34 27 43 ");

    let mut recovered = [0; 64];
    a_aes.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 79 17 99 06 B0 DD 3E 41 2B FF AD 70 F5 17 F4 65 D7 B0 AC FA 83 69 BF 8D C6 E6 6C 62 1C 5C 90 EA 45 D8 34 45 02 BE 33 ");

    let mut recovered = [0; 64];
    a_aes.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");

    let mut recovered = [0; 64];
    a_rijndael.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    
    let mut recovered = [0; 64];
    a_rijndael.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

fn aes_decrypt_ctr_into_string()
{
    println!("aes_decrypt_ctr_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 9D 7C 52 E3 90 02 C5 7B EE BB 50 EE 3D 2C 3A 81 33 B7 54 77 35 FB 9F 48 6F 63 17 DC 5E 64 0E 29 4B 48 6D 53 24 CF D3 ");

    let mut converted= String::new();
    a_aes.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut converted);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 4C DB 3F E9 F3 83 1E 7D D4 F7 5F 3D 14 28 B9 CE B0 11 61 53 FD E0 8F BC 2D 55 45 7E 24 C1 6A 9B 1A 82 94 4A 34 27 43 ");

    let mut converted= String::new();
    a_aes.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut converted);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 79 17 99 06 B0 DD 3E 41 2B FF AD 70 F5 17 F4 65 D7 B0 AC FA 83 69 BF 8D C6 E6 6C 62 1C 5C 90 EA 45 D8 34 45 02 BE 33 ");

    let mut converted= String::new();
    a_aes.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut converted);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");

    let mut converted= String::new();
    a_rijndael.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut converted);
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    
    let mut converted= String::new();
    a_rijndael.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut converted);
    println!("B =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}

fn aes_decrypt_vec_ctr()
{
    println!("aes_decrypt_vec_ctr()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 9D 7C 52 E3 90 02 C5 7B EE BB 50 EE 3D 2C 3A 81 33 B7 54 77 35 FB 9F 48 6F 63 17 DC 5E 64 0E 29 4B 48 6D 53 24 CF D3 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 4C DB 3F E9 F3 83 1E 7D D4 F7 5F 3D 14 28 B9 CE B0 11 61 53 FD E0 8F BC 2D 55 45 7E 24 C1 6A 9B 1A 82 94 4A 34 27 43 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 79 17 99 06 B0 DD 3E 41 2B FF AD 70 F5 17 F4 65 D7 B0 AC FA 83 69 BF 8D C6 E6 6C 62 1C 5C 90 EA 45 D8 34 45 02 BE 33 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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

fn aes_decrypt_vec_ctr_into_vec()
{
    println!("aes_decrypt_vec_ctr_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 9D 7C 52 E3 90 02 C5 7B EE BB 50 EE 3D 2C 3A 81 33 B7 54 77 35 FB 9F 48 6F 63 17 DC 5E 64 0E 29 4B 48 6D 53 24 CF D3 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 4C DB 3F E9 F3 83 1E 7D D4 F7 5F 3D 14 28 B9 CE B0 11 61 53 FD E0 8F BC 2D 55 45 7E 24 C1 6A 9B 1A 82 94 4A 34 27 43 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 79 17 99 06 B0 DD 3E 41 2B FF AD 70 F5 17 F4 65 D7 B0 AC FA 83 69 BF 8D C6 E6 6C 62 1C 5C 90 EA 45 D8 34 45 02 BE 33 ");

    let mut recovered = Vec::<u8>::new();
    a_aes.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");

    let mut recovered = Vec::<u8>::new();
    a_rijndael.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    
    let mut recovered = Vec::<u8>::new();
    a_rijndael.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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

fn aes_decrypt_vec_ctr_into_array()
{
    println!("aes_decrypt_vec_ctr_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 9D 7C 52 E3 90 02 C5 7B EE BB 50 EE 3D 2C 3A 81 33 B7 54 77 35 FB 9F 48 6F 63 17 DC 5E 64 0E 29 4B 48 6D 53 24 CF D3 ");

    let mut recovered = [0; 64];
    let len = a_aes.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 4C DB 3F E9 F3 83 1E 7D D4 F7 5F 3D 14 28 B9 CE B0 11 61 53 FD E0 8F BC 2D 55 45 7E 24 C1 6A 9B 1A 82 94 4A 34 27 43 ");

    let mut recovered = [0; 64];
    a_aes.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 79 17 99 06 B0 DD 3E 41 2B FF AD 70 F5 17 F4 65 D7 B0 AC FA 83 69 BF 8D C6 E6 6C 62 1C 5C 90 EA 45 D8 34 45 02 BE 33 ");

    let mut recovered = [0; 64];
    a_aes.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");

    let mut recovered = [0; 64];
    a_rijndael.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    
    let mut recovered = [0; 64];
    a_rijndael.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
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

fn aes_decrypt_vec_ctr_into_string()
{
    println!("aes_decrypt_vec_ctr_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 9D 7C 52 E3 90 02 C5 7B EE BB 50 EE 3D 2C 3A 81 33 B7 54 77 35 FB 9F 48 6F 63 17 DC 5E 64 0E 29 4B 48 6D 53 24 CF D3 ");

    let mut converted= String::new();
    a_aes.decrypt_vec_into_string(nonce, &cipher, &mut converted);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 4C DB 3F E9 F3 83 1E 7D D4 F7 5F 3D 14 28 B9 CE B0 11 61 53 FD E0 8F BC 2D 55 45 7E 24 C1 6A 9B 1A 82 94 4A 34 27 43 ");

    let mut converted= String::new();
    a_aes.decrypt_vec_into_string(nonce, &cipher, &mut converted);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 79 17 99 06 B0 DD 3E 41 2B FF AD 70 F5 17 F4 65 D7 B0 AC FA 83 69 BF 8D C6 E6 6C 62 1C 5C 90 EA 45 D8 34 45 02 BE 33 ");

    let mut converted= String::new();
    a_aes.decrypt_vec_into_string(nonce, &cipher, &mut converted);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");

    let mut converted= String::new();
    a_rijndael.decrypt_vec_into_string(nonce, &cipher, &mut converted);
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(nonce, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    
    let mut converted= String::new();
    a_rijndael.decrypt_vec_into_string(nonce, &cipher, &mut converted);
    println!("B =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}

fn aes_decrypt_array_ctr()
{
    println!("aes_decrypt_array_ctr()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 9D 7C 52 E3 90 02 C5 7B EE BB 50 EE 3D 2C 3A 81 33 B7 54 77 35 FB 9F 48 6F 63 17 DC 5E 64 0E 29 4B 48 6D 53 24 CF D3 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 4C DB 3F E9 F3 83 1E 7D D4 F7 5F 3D 14 28 B9 CE B0 11 61 53 FD E0 8F BC 2D 55 45 7E 24 C1 6A 9B 1A 82 94 4A 34 27 43 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 79 17 99 06 B0 DD 3E 41 2B FF AD 70 F5 17 F4 65 D7 B0 AC FA 83 69 BF 8D C6 E6 6C 62 1C 5C 90 EA 45 D8 34 45 02 BE 33 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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
    a_rijndael.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    
    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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

fn aes_decrypt_array_ctr_into_vec()
{
    println!("aes_decrypt_array_ctr_into_vec()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 9D 7C 52 E3 90 02 C5 7B EE BB 50 EE 3D 2C 3A 81 33 B7 54 77 35 FB 9F 48 6F 63 17 DC 5E 64 0E 29 4B 48 6D 53 24 CF D3 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 4C DB 3F E9 F3 83 1E 7D D4 F7 5F 3D 14 28 B9 CE B0 11 61 53 FD E0 8F BC 2D 55 45 7E 24 C1 6A 9B 1A 82 94 4A 34 27 43 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 79 17 99 06 B0 DD 3E 41 2B FF AD 70 F5 17 F4 65 D7 B0 AC FA 83 69 BF 8D C6 E6 6C 62 1C 5C 90 EA 45 D8 34 45 02 BE 33 ");

    let mut recovered = vec![0; 55];
    a_aes.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    a_rijndael.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");

    let mut recovered = vec![0; 55];
    a_rijndael.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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

fn aes_decrypt_array_ctr_into_array()
{
    println!("aes_decrypt_array_ctr_into_array()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 9D 7C 52 E3 90 02 C5 7B EE BB 50 EE 3D 2C 3A 81 33 B7 54 77 35 FB 9F 48 6F 63 17 DC 5E 64 0E 29 4B 48 6D 53 24 CF D3 ");

    let mut recovered = [0; 64];
    let len = a_aes.decrypt_array_into_array(nonce, &cipher, &mut recovered);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 4C DB 3F E9 F3 83 1E 7D D4 F7 5F 3D 14 28 B9 CE B0 11 61 53 FD E0 8F BC 2D 55 45 7E 24 C1 6A 9B 1A 82 94 4A 34 27 43 ");

    let mut recovered = [0; 64];
    let len = a_aes.decrypt_array_into_array(nonce, &cipher, &mut recovered);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 79 17 99 06 B0 DD 3E 41 2B FF AD 70 F5 17 F4 65 D7 B0 AC FA 83 69 BF 8D C6 E6 6C 62 1C 5C 90 EA 45 D8 34 45 02 BE 33 ");

    let mut recovered = [0; 64];
    let len = a_aes.decrypt_array_into_array(nonce, &cipher, &mut recovered);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");

    let mut recovered = [0; 64];
    let len = a_rijndael.decrypt_array_into_array(nonce, &cipher, &mut recovered);
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");

    let mut recovered = [0; 64];
    let len = a_rijndael.decrypt_array_into_array(nonce, &cipher, &mut recovered);
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

fn aes_decrypt_array_ctr_into_string()
{
    println!("aes_decrypt_array_ctr_into_string()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };

    // Normal case for AES-128
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    println!("K =\t{:#016X}", key);
    let mut a_aes = AES_128::new_with_key_u128(key);
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 9D 7C 52 E3 90 02 C5 7B EE BB 50 EE 3D 2C 3A 81 33 B7 54 77 35 FB 9F 48 6F 63 17 DC 5E 64 0E 29 4B 48 6D 53 24 CF D3 ");

    let mut converted= String::new();
    a_aes.decrypt_array_into_string(nonce, &cipher, &mut converted);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 4C DB 3F E9 F3 83 1E 7D D4 F7 5F 3D 14 28 B9 CE B0 11 61 53 FD E0 8F BC 2D 55 45 7E 24 C1 6A 9B 1A 82 94 4A 34 27 43 ");

    let mut converted= String::new();
    a_aes.decrypt_array_into_string(nonce, &cipher, &mut converted);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 79 17 99 06 B0 DD 3E 41 2B FF AD 70 F5 17 F4 65 D7 B0 AC FA 83 69 BF 8D C6 E6 6C 62 1C 5C 90 EA 45 D8 34 45 02 BE 33 ");

    let mut converted= String::new();
    a_aes.decrypt_array_into_string(nonce, &cipher, &mut converted);
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
    let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be(), nonce[4].to_be(), nonce[5].to_be(), nonce[6].to_be(), nonce[7].to_be());

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC 43 1C FB 34 AF CC 14 37 E5 F8 11 07 37 2B D0 B1 4E D6 6F E0 68 C2 9A ");

    let mut converted= String::new();
    a_rijndael.decrypt_array_into_string(nonce, &cipher, &mut converted);
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
    let mut nonce = SharedArrays::<u32, 16, u8, 64>::new();
    nonce.src = sha3.get_hash_value_in_array();
    let nonce = unsafe { nonce.des };
    print!("Nonce =\t");
    for i in 0..16
        { print!("{:08X}", nonce[i].to_be()); }
    println!();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 55];
    a_rijndael.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");

    let mut converted= String::new();
    a_rijndael.decrypt_array_into_string(nonce, &cipher, &mut converted);
    println!("B =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!("-------------------------------");
}
