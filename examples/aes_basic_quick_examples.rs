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

use cryptocol::symmetric::documentation::rijndael_basic::Rijndael_Generic;


///// For test during implementation //////
pub fn main()
{
    // aes_devel();
    aes_quick_start();
    aes_basic();
}

// fn aes_devel()
// {
//     aes_sbox();
//     aes_invsbox();
//     aes_mc();
//     aes_invmc();
//     aes_rc();
//     aes_test();
// }

// fn aes_sbox()
// {
//     println!("aes_sbox");
//     use cryptocol::symmetric::AES_128;
//     AES_128::show_SBox();
//     println!("-------------------------------");
// }

// fn aes_invsbox()
// {
//     println!("aes_invsbox");
//     use cryptocol::symmetric::AES_128;
//     AES_128::show_InvSBox();
//     println!("-------------------------------");
// }

// fn aes_mc()
// {
//     println!("aes_mc");
//     use cryptocol::symmetric::AES_128;
//     AES_128::show_MC();
//     println!("-------------------------------");
// }

// fn aes_invmc()
// {
//     println!("aes_invmc");
//     use cryptocol::symmetric::AES_128;
//     AES_128::show_InvMC();
//     println!("-------------------------------");
// }

// fn aes_rc()
// {
//     println!("aes_rc");
//     use cryptocol::symmetric::AES_128;
//     AES_128::show_RC();
//     println!("-------------------------------");
// }

// fn aes_test()
// {
//     println!("aes_test");
//     use cryptocol::number::LongerUnion;
//     use cryptocol::symmetric::{ AES_128, AES_192, AES_256 };

//     // AES-128
//     println!("AES-128");
//     let mut aes = AES_128::new();
//     let msg = LongerUnion::new_with_ubytes([0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
//     print!("Message =\t");
//     for i in 0..16
//         { print!("{:02x}", msg.get_ubyte_(i)); }
//     println!();
//     let cipher = LongerUnion::new_with(aes.encrypt_u128(msg.get()));
//     print!("Cipher =\t");
//     for i in 0..16
//         { print!("{:02x}", cipher.get_ubyte_(i)); }
//     println!();
//     assert_eq!(cipher.get(), 0x3ad78e726c1ec02b7ebfe92b23d9ec34_u128.to_be());
//     let restored = LongerUnion::new_with(aes.decrypt_u128(cipher.get()));
//     print!("Restored =\t");
//     for i in 0..16
//         { print!("{:02x}", restored.get_ubyte_(i)); }
//     println!("\n");
//     assert_eq!(restored.get(), 0x80000000000000000000000000000000_u128.to_be());
    
//     let mut aes = AES_128::new_with_key_u128(0x10a58869d74be5a374cf867cfb473859_u128.to_be());
//     let msg = LongerUnion::new();
//     print!("Message =\t");
//     for i in 0..16
//         { print!("{:02x}", msg.get_ubyte_(i)); }
//     println!();
//     let cipher = LongerUnion::new_with(aes.encrypt_u128(msg.get()));
//     print!("Cipher =\t");
//     for i in 0..16
//         { print!("{:02x}", cipher.get_ubyte_(i)); }
//     println!();
//     assert_eq!(cipher.get(), 0x6d251e6944b051e04eaa6fb4dbf78465_u128.to_be());
//     let restored = LongerUnion::new_with(aes.decrypt_u128(cipher.get()));
//     print!("Restored =\t");
//     for i in 0..16
//         { print!("{:02x}", restored.get_ubyte_(i)); }
//     println!("\n");
//     assert_eq!(restored.get(), 0x00000000000000000000000000000000_u128.to_be());

//     let mut aes = AES_128::new_with_key_u128(0x2b7e151628aed2a6abf7158809cf4f3c_u128.to_be());
//     let msg = LongerUnion::new_with_ubytes([0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34]);
//     print!("Message =\t");
//     for i in 0..16
//         { print!("{:02x}", msg.get_ubyte_(i)); }
//     println!();
//     let cipher = LongerUnion::new_with(aes.encrypt_u128(msg.get()));
//     print!("Cipher =\t");
//     for i in 0..16
//         { print!("{:02x}", cipher.get_ubyte_(i)); }
//     println!();
//     assert_eq!(cipher.get(), 0x3925841d02dc09fbdc118597196a0b32_u128.to_be());
//     let restored = LongerUnion::new_with(aes.decrypt_u128(cipher.get()));
//     print!("Restored =\t");
//     for i in 0..16
//         { print!("{:02x}", restored.get_ubyte_(i)); }
//     println!("\n\n");
//     assert_eq!(restored.get(), 0x3243f6a8885a308d313198a2e0370734_u128.to_be());


//     // AES-192
//     println!("AES-192");
//     let mut aes = AES_192::new();
//     let msg = LongerUnion::new_with_ubytes([0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
//     print!("Message =\t");
//     for i in 0..16
//         { print!("{:02x}", msg.get_ubyte_(i)); }
//     println!();
//     let cipher = LongerUnion::new_with(aes.encrypt_u128(msg.get()));
//     print!("Cipher =\t");
//     for i in 0..16
//         { print!("{:02x}", cipher.get_ubyte_(i)); }
//     println!();
//     assert_eq!(cipher.get(), 0x6cd02513e8d4dc986b4afe087a60bd0c_u128.to_be());
//     let restored = LongerUnion::new_with(aes.decrypt_u128(cipher.get()));
//     print!("Restored =\t");
//     for i in 0..16
//         { print!("{:02x}", restored.get_ubyte_(i)); }
//     println!("\n");
//     assert_eq!(restored.get(), 0x80000000000000000000000000000000_u128.to_be());
    
//     let mut aes = AES_192::new_with_key([0xe9, 0xf0, 0x65, 0xd7, 0xc1, 0x35, 0x73, 0x58, 0x7f, 0x78, 0x75, 0x35, 0x7d, 0xfb, 0xb1, 0x6c, 0x53, 0x48, 0x9f, 0x6a, 0x4b, 0xd0, 0xf7, 0xcd]);
//     let msg = LongerUnion::new();
//     print!("Message =\t");
//     for i in 0..16
//         { print!("{:02x}", msg.get_ubyte_(i)); }
//     println!();
//     let cipher = LongerUnion::new_with(aes.encrypt_u128(msg.get()));
//     print!("Cipher =\t");
//     for i in 0..16
//         { print!("{:02x}", cipher.get_ubyte_(i)); }
//     println!();
//     assert_eq!(cipher.get(), 0x0956259c9cd5cfd0181cca53380cde06_u128.to_be());
//     let restored = LongerUnion::new_with(aes.decrypt_u128(cipher.get()));
//     print!("Restored =\t");
//     for i in 0..16
//         { print!("{:02x}", restored.get_ubyte_(i)); }
//     println!("\n");
//     assert_eq!(restored.get(), 0x00000000000000000000000000000000_u128.to_be());

//     let mut aes = AES_192::new_with_key([0x8E, 0x73, 0xB0, 0xF7, 0xDA, 0x0E, 0x64, 0x52, 0xC8, 0x10, 0xF3, 0x2B, 0x80, 0x90, 0x79, 0xE5, 0x62, 0xF8, 0xEA, 0xD2, 0x52, 0x2C, 0x6B, 0x7B]);
//     let msg = LongerUnion::new_with(0x6BC1BEE22E409F96E93D7E117393172A_u128.to_be());
//     print!("Message =\t");
//     for i in 0..16
//         { print!("{:02x}", msg.get_ubyte_(i)); }
//     println!();
//     let cipher = LongerUnion::new_with(aes.encrypt_u128(msg.get()));
//     print!("Cipher =\t");
//     for i in 0..16
//         { print!("{:02x}", cipher.get_ubyte_(i)); }
//     println!();
//     assert_eq!(cipher.get(), 0xBD334F1D6E45F25FF712A214571FA5CC_u128.to_be());
//     let restored = LongerUnion::new_with(aes.decrypt_u128(cipher.get()));
//     print!("Restored =\t");
//     for i in 0..16
//         { print!("{:02x}", restored.get_ubyte_(i)); }
//     println!("\n");
//     assert_eq!(restored.get(), 0x6BC1BEE22E409F96E93D7E117393172A_u128.to_be());


//     // AES-256
//     println!("AES-256");
//     let mut aes = AES_256::new();
//     let msg = LongerUnion::new_with_ubytes([0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
//     print!("Message =\t");
//     for i in 0..16
//         { print!("{:02x}", msg.get_ubyte_(i)); }
//     println!();
//     let cipher = LongerUnion::new_with(aes.encrypt_u128(msg.get()));
//     print!("Cipher =\t");
//     for i in 0..16
//         { print!("{:02x}", cipher.get_ubyte_(i)); }
//     println!();
//     assert_eq!(cipher.get(), 0xddc6bf790c15760d8d9aeb6f9a75fd4e_u128.to_be());
//     let restored = LongerUnion::new_with(aes.decrypt_u128(cipher.get()));
//     print!("Restored =\t");
//     for i in 0..16
//         { print!("{:02x}", restored.get_ubyte_(i)); }
//     println!("\n");
//     assert_eq!(restored.get(), 0x80000000000000000000000000000000_u128.to_be());
    
//     let mut aes = AES_256::new_with_key([0xc4, 0x7b, 0x02, 0x94, 0xdb, 0xbb, 0xee, 0x0f, 0xec, 0x47, 0x57, 0xf2, 0x2f, 0xfe, 0xee, 0x35, 0x87, 0xca, 0x47, 0x30, 0xc3, 0xd3, 0x3b, 0x69, 0x1d, 0xf3, 0x8b, 0xab, 0x07, 0x6b, 0xc5, 0x58]);
//     let msg = LongerUnion::new();
//     print!("Message =\t");
//     for i in 0..16
//         { print!("{:02x}", msg.get_ubyte_(i)); }
//     println!();
//     let cipher = LongerUnion::new_with(aes.encrypt_u128(msg.get()));
//     print!("Cipher =\t");
//     for i in 0..16
//         { print!("{:02x}", cipher.get_ubyte_(i)); }
//     println!();
//     assert_eq!(cipher.get(), 0x46f2fb342d6f0ab477476fc501242c5f_u128.to_be());
//     let restored = LongerUnion::new_with(aes.decrypt_u128(cipher.get()));
//     print!("Restored =\t");
//     for i in 0..16
//         { print!("{:02x}", restored.get_ubyte_(i)); }
//     println!("\n");
//     assert_eq!(restored.get(), 0x00000000000000000000000000000000_u128.to_be());

//     let mut aes = AES_256::new_with_key([0x60, 0x3D, 0xEB, 0x10, 0x15, 0xCA, 0x71, 0xBE, 0x2B, 0x73, 0xAE, 0xF0, 0x85, 0x7D, 0x77, 0x81, 0x1F, 0x35, 0x2C, 0x07, 0x3B, 0x61, 0x08, 0xD7, 0x2D, 0x98, 0x10, 0xA3, 0x09, 0x14, 0xDF, 0xF4]);
//     let msg = LongerUnion::new_with(0x6BC1BEE22E409F96E93D7E117393172A_u128.to_be());
//     print!("Message =\t");
//     for i in 0..16
//         { print!("{:02x}", msg.get_ubyte_(i)); }
//     println!();
//     let cipher = LongerUnion::new_with(aes.encrypt_u128(msg.get()));
//     print!("Cipher =\t");
//     for i in 0..16
//         { print!("{:02x}", cipher.get_ubyte_(i)); }
//     println!();
//     assert_eq!(cipher.get(), 0xF3EED1BDB5D2A03C064B5A7E3DB181F8_u128.to_be());
//     let restored = LongerUnion::new_with(aes.decrypt_u128(cipher.get()));
//     print!("Restored =\t");
//     for i in 0..16
//         { print!("{:02x}", restored.get_ubyte_(i)); }
//     println!("\n");
//     assert_eq!(restored.get(), 0x6BC1BEE22E409F96E93D7E117393172A_u128.to_be());
//     println!("-------------------------------");
// }

fn aes_quick_start()
{
    aes_import_modules();
    aes_instantiation_with_keys();
    aes_instantiation_with_no_key();
    aes_cbc_pkcs7();
    aes_expanded_rijndael();
}

fn aes_import_modules()
{
    println!("aes_import_modules()");
    use cryptocol::symmetric::AES_128;
    use cryptocol::symmetric::AES_192;
    use cryptocol::symmetric::AES_256;

    use cryptocol::symmetric::Rijndael_256_256;
    use cryptocol::symmetric::Rijndael_256_192;
    use cryptocol::symmetric::Rijndael_256_128;
    use cryptocol::symmetric::Rijndael_192_256;
    use cryptocol::symmetric::Rijndael_192_192;
    use cryptocol::symmetric::Rijndael_192_128;
    use cryptocol::symmetric::Rijndael_128_256;
    use cryptocol::symmetric::Rijndael_128_192;
    use cryptocol::symmetric::Rijndael_128_128;

    use cryptocol::symmetric::Rijndael_32_32;
    use cryptocol::symmetric::Rijndael_64_64;
    println!("-------------------------------");
}

fn aes_instantiation_with_keys()
{
    println!("aes_instantiation_with_keys()");
    use cryptocol::symmetric::AES_128;
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    let mut _a_aes = AES_128::new_with_key_u128(key);

    let key = [0xEFu8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12];
    let mut _a_aes = AES_128::new_with_key(&key);
    println!("-------------------------------");
}

fn aes_instantiation_with_no_key()
{
    println!("aes_instantiation_with_no_key()");
    use cryptocol::symmetric::AES_128;

    let mut a_aes = AES_128::new();
    let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    a_aes.set_key_u128(key);

    let mut a_aes = AES_128::new();
    let key = [0xEFu8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12];
    a_aes.set_key(&key);
    println!("-------------------------------");
}

fn aes_cbc_pkcs7()
{
    println!("aes_cbc_pkcs7()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ AES_128, CBC_PKCS7 };

    let mut a_aes = AES_128::new_with_key(&[0xEFu8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    let mut cipher = Vec::<u8>::new();
    a_aes.encrypt_str_into_vec(iv, message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 A3 89 67 0C 21 29 3E 4D DC AD B6 44 09 D4 3B 02 ");

    let mut recovered = String::new();
    a_aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn aes_expanded_rijndael()
{
    println!("aes_cbc_pkcs7()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ Rijndael_Generic, CBC_PKCS7 };

    let mut a_rijndael = Rijndael_Generic::<22, 16, 16>::new_with_key(&[0xEFu8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let iv = [0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32];
    println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());

    let mut cipher = Vec::<u8>::new();
    a_rijndael.encrypt_str_into_vec(iv, message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B1 C0 1F 84 17 46 35 12 D9 16 52 44 5F 40 A1 7F 3B 55 F7 E6 42 E5 1F 42 57 43 AD E4 00 19 54 1D B6 F3 1B 20 C8 D3 08 92 B7 C4 0C E2 77 73 A5 E4 0D E7 0F F4 B0 38 FE 78 30 56 E4 A7 9E CE 0E 50 ");

    let mut recovered = String::new();
    a_rijndael.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn aes_basic()
{
    aes_new();
    aes_new_with_key();
    aes_new_with_key_u128();
    // aes_encryptor_with_key();
    // aes_encryptor_with_key_u128();
    // aes_decryptor_with_key();
    // aes_decryptor_with_key_u128();
    aes_get_key();
    aes_get_key_u128();
    aes_set_key();
    aes_set_key_u128();
    // aes_turn_inverse();
    // aes_turn_encryptor();
    // aes_turn_decryptor();
    aes_encrypt_unit();
    aes_encrypt_u128();
    aes_decrypt_unit();
    aes_decrypt_u128();
    // aes__encrypt();
    // aes__decrypt();
    aes_encrypt_array_unit();
    aes_encrypt_array_u128();
    aes_decrypt_array_unit();
    aes_decrypt_array_u128();
    aes_is_successful();
    aes_is_failed();
    aes_get_desirable_round();
}

fn aes_new()
{
    println!("aes_new()");
    use cryptocol::symmetric::AES_128;
    let mut _a_aes = AES_128::new();
    let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    let ciphertext = _a_aes.encrypt_u128(plaintext);
    
    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0xE2C8CD3BFD4D72366A4806B100659867);
    
    let recovered_cipher_text = _a_aes.decrypt_u128(ciphertext);
    println!("Recovered-ciphertext:\t{:#034X}", recovered_cipher_text);
    assert_eq!(recovered_cipher_text, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    assert_eq!(recovered_cipher_text, plaintext);

    #[cfg(test)]
    aes_compile_fail_new();
    println!("-------------------------------");
}

#[test]
fn aes_compile_fail_new()
{
    use cryptocol::symmetric::AES_128;
    let _aes = AES_128::new();
    // It cannot be compiled!
    #[cfg(compile_fail)]    _aes.encrypt_u128(0x1234567890ABCDEF1234567890ABCDEF_u128);
}

fn aes_new_with_key()
{
    println!("aes_new_with_key()");
    use cryptocol::symmetric::AES_128;

    // Normal case
    let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    let ciphertext = aes.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x01CCF8264AECB5D644E2BAE927584D87_u128);

    let recovered_cipher_text = aes.decrypt_u128(ciphertext);
    println!("Recovered-ciphertext:\t{:#034X}", recovered_cipher_text);
    assert_eq!(recovered_cipher_text, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    assert_eq!(recovered_cipher_text, plaintext);
    println!();

    // The case for the necessary key is longer than the given key.
    // The key [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF] is the same as
    // the key [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x00, 0x00,
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] for AES_128
    let mut aes1 = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    let mut aes2 = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);

    let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    let ciphertext1 = aes1.encrypt_u128(plaintext);
    let ciphertext2 = aes1.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext1:\t\t{:#034X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#034X}", ciphertext2);
    assert_eq!(ciphertext1, 0x07A3B9744B517ADB37FC90ADE4410D62_u128);
    assert_eq!(ciphertext2, 0x07A3B9744B517ADB37FC90ADE4410D62_u128);
    assert_eq!(ciphertext1, ciphertext2);

    let recovered_cipher_text1 = aes1.decrypt_u128(ciphertext1);
    let recovered_cipher_text2 = aes2.decrypt_u128(ciphertext2);
    println!("Recovered-ciphertext1:\t{:#034X}", recovered_cipher_text1);
    println!("Recovered-ciphertext2:\t{:#034X}", recovered_cipher_text2);
    assert_eq!(recovered_cipher_text1, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    assert_eq!(recovered_cipher_text2, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    assert_eq!(recovered_cipher_text1, plaintext);
    assert_eq!(recovered_cipher_text2, plaintext);
    println!();

    // The case for the necessary key is shorter than the given key.
    // The key [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34,
    // 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB,
    // 0xCD, 0xEF] is the same as the key [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB,
    // 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF] for AES_128
    let mut aes1 = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let mut aes2 = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);

    let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    let ciphertext1 = aes1.encrypt_u128(plaintext);
    let ciphertext2 = aes1.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext1:\t\t{:#034X}", ciphertext1);
    println!("Ciphertext2:\t\t{:#034X}", ciphertext2);
    assert_eq!(ciphertext1, 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    assert_eq!(ciphertext2, 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    assert_eq!(ciphertext1, ciphertext2);

    let recovered_cipher_text1 = aes1.decrypt_u128(ciphertext1);
    let recovered_cipher_text2 = aes2.decrypt_u128(ciphertext2);
    println!("Recovered-ciphertext1:\t{:#034X}", recovered_cipher_text1);
    println!("Recovered-ciphertext2:\t{:#034X}", recovered_cipher_text2);
    assert_eq!(recovered_cipher_text1, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    assert_eq!(recovered_cipher_text2, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    assert_eq!(recovered_cipher_text1, plaintext);
    assert_eq!(recovered_cipher_text2, plaintext);

    #[cfg(test)]
    aes_compile_fail_new_with_key();
    println!("-------------------------------");
}

#[test]
fn aes_compile_fail_new_with_key()
{
    use cryptocol::symmetric::AES_128;
    let aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    // It cannot be compiled!
    #[cfg(compile_fail)]    aes.encrypt_u128(0x1234567890ABCDEF1234567890ABCDEF_u128);
}

fn aes_new_with_key_u128()
{
    println!("aes_new_with_key_u128");
    use cryptocol::symmetric::{ AES_128, AES_256, Rijndael_Generic };

    // Normal case
    let mut aes = AES_128::new_with_key_u128(0xEFCDAB9078563412EFCDAB9078563412);
    let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    let ciphertext = aes.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x01CCF8264AECB5D644E2BAE927584D87_u128);

    let recovered_cipher_text = aes.decrypt_u128(ciphertext);
    println!("Recovered-ciphertext:\t{:#034X}\n", recovered_cipher_text);
    assert_eq!(recovered_cipher_text, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    assert_eq!(recovered_cipher_text, plaintext);

    // The case for the necessary key longer than 128 bits
    let mut aes = AES_256::new_with_key_u128(0xEFCDAB9078563412EFCDAB9078563412);
    let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    let ciphertext = aes.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x39293C7C8FF7F9B5F76038FDCE5E3470_u128);

    let recovered_cipher_text = aes.decrypt_u128(ciphertext);
    println!("Recovered-ciphertext:\t{:#034X}\n", recovered_cipher_text);
    assert_eq!(recovered_cipher_text, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    assert_eq!(recovered_cipher_text, plaintext);

    // The case for the necessary key shorter than 128 bits
    let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key_u128(0xEFCDAB9078563412EFCDAB9078563412);
    let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    let ciphertext = rijndael.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x9D6C20BD28996D5570E7E05DBF20110F_u128);

    let recovered_cipher_text = rijndael.decrypt_u128(ciphertext);
    println!("Recovered-ciphertext:\t{:#034X}\n", recovered_cipher_text);
    assert_eq!(recovered_cipher_text, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    assert_eq!(recovered_cipher_text, plaintext);
    println!("-------------------------------");
}
/*
fn aes_encryptor_with_key()
{
    println!("aes_encryptor_with_key");
    use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    
    let keys: [Box<dyn SmallCryptor<u64, 8>>; 3]
            = [ Box::new(AES_128::encryptor_with_key(&[0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])),
                Box::new(AES_128::decryptor_with_key(&[0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])),
                Box::new(AES_128::encryptor_with_key(&[0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])) ];
    let mut tdes = BigCryptor64::new_with_small_cryptor_array(keys);
    let plaintext = 0x_1234567890ABCDEF_u128;
    let ciphertext = tdes.encrypt_u128(plaintext);
    
    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x272A2AC7B4E66748_u128);
    
    let cipher_cipher_text = tdes.decrypt_u128(ciphertext);
    println!("Cipher-ciphertext:\t{:#034X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u128);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();

    // Operators
    let mut tdes = BigCryptor64::new()
                    + AES_128::encryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])
                    - AES_128::encryptor_with_key([0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])
                    + AES_128::encryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
    let plaintext = 0x_1234567890ABCDEF_u128;
    let ciphertext = tdes.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u128);

    let cipher_cipher_text = tdes.decrypt_u128(ciphertext);
    println!("Cipher-ciphertext:\t{:#034X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u128);
    assert_eq!(cipher_cipher_text, plaintext);
    println!("-------------------------------");
}

fn aes_encryptor_with_key_u128()
{
    println!("aes_encryptor_with_key_u128");
    use cryptocol::symmetric::{ BigCryptor64, AES_128 };

    let mut tdes = BigCryptor64::new_with_small_cryptor_array(
                [Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEF_u128)),
                Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA0987654321_u128)),
                Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEF_u128))]
    );
    let plaintext = 0x_1234567890ABCDEF_u128;
    let ciphertext = tdes.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u128);

    let cipher_cipher_text = tdes.decrypt_u128(ciphertext);
    println!("Cipher-ciphertext:\t{:#034X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u128);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    

    // Operators
    let mut tdes = BigCryptor64::new()
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEF_u128)
                    - AES_128::encryptor_with_key_u128(0x_FEDCBA0987654321_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEF_u128);
    let plaintext = 0x_1234567890ABCDEF_u128;
    let ciphertext = tdes.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u128);

    let cipher_cipher_text = tdes.decrypt_u128(ciphertext);
    println!("Cipher-ciphertext:\t{:#034X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u128);
    assert_eq!(cipher_cipher_text, plaintext);
    println!("-------------------------------");
}

fn aes_decryptor_with_key()
{
    println!("aes_decryptor_with_key_u128");
    use cryptocol::symmetric::{ AES_128, BigCryptor64, SmallCryptor };
    
    let keys: [Box<dyn SmallCryptor<u64, 8>>; 3]
            = [ Box::new(AES_128::encryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])),
                Box::new(AES_128::decryptor_with_key([0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])),
                Box::new(AES_128::encryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])) ];
    let mut tdes = BigCryptor64::new_with_small_cryptor_array(keys);
    let plaintext = 0x_1234567890ABCDEF_u128;
    let ciphertext = tdes.encrypt_u128(plaintext);
    
    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x272A2AC7B4E66748_u128);
    
    let cipher_cipher_text = tdes.decrypt_u128(ciphertext);
    println!("Cipher-ciphertext:\t{:#034X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u128);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();

    // Operators
    let mut tdes = BigCryptor64::new()
                    - AES_128::decryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])
                    - AES_128::encryptor_with_key([0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])
                    - AES_128::decryptor_with_key([0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
    let plaintext = 0x_1234567890ABCDEF_u128;
    let ciphertext = tdes.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u128);

    let cipher_cipher_text = tdes.decrypt_u128(ciphertext);
    println!("Cipher-ciphertext:\t{:#034X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u128);
    assert_eq!(cipher_cipher_text, plaintext);
    println!("-------------------------------");
}

fn aes_decryptor_with_key_u128()
{
    println!("aes_decryptor_with_key_u128");
    use cryptocol::symmetric::{ BigCryptor64, AES_128 };

    let mut tdes = BigCryptor64::new_with_small_cryptor_array(
                    [ Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEF_u128)),
                                    Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA0987654321_u128)),
                                    Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEF_u128)) ] );
    let plaintext = 0x_1234567890ABCDEF_u128;
    let ciphertext = tdes.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x272A2AC7B4E66748_u128);

    let cipher_cipher_text = tdes.decrypt_u128(ciphertext);
    println!("Cipher-ciphertext:\t{:#034X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u128);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    

    // Operators
    let mut tdes = BigCryptor64::new()
                    - AES_128::decryptor_with_key_u128(0x_1234567890ABCDEF_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA0987654321_u128)
                    - AES_128::decryptor_with_key_u128(0x_1234567890ABCDEF_u128);
    let plaintext = 0x_1234567890ABCDEF_u128;
    let ciphertext = tdes.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u128);

    let cipher_cipher_text = tdes.decrypt_u128(ciphertext);
    println!("Cipher-ciphertext:\t{:#034X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u128);
    assert_eq!(cipher_cipher_text, plaintext);
    println!("-------------------------------");
}
*/
fn aes_get_key()
{
    println!("aes_get_key()");
    use cryptocol::symmetric::{ AES_128, AES_192, Rijndael_Generic };

    // The case for AES_128
    let mut _aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let key = _aes.get_key();
    print!("K = ");
    for k in key
        { print!("{:#010X} ", k); }
    println!();
    assert_eq!(key, [0x78563412, 0xEFCDAB90, 0x78563412, 0xEFCDAB90]);

    // The case for AES_192
    let mut _aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let key = _aes.get_key();
    print!("K = ");
    for k in key
        { print!("{:#010X} ", k); }
    println!();
    assert_eq!(key, [0x78563412_u32, 0xEFCDAB90, 0x78563412_u32, 0xEFCDAB90, 0x78563412_u32, 0xEFCDAB90]);

    // The case for Rijndael_Generic::<10, 4, 2>
    let mut _rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let key = _rijndael.get_key();
    print!("K = ");
    for k in key
        { print!("{:#010X} ", k); }
    println!();
    assert_eq!(key, [0x78563412_u32, 0xEFCDAB90]);
    println!("-------------------------------");
}

fn aes_get_key_u128()
{
    println!("aes_get_key_u128()");
    use cryptocol::symmetric::{ AES_128, AES_192, Rijndael_Generic };

    // Normal case for 128-bit key
    let mut _aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let key = _aes.get_key_u128();
    println!("Key = {:#034X}", key);
    assert_eq!(key, 0xEFCDAB9078563412EFCDAB9078563412_u128);

    // The case for the key longer than 128 bits
    let mut _aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    let key = _aes.get_key_u128();
    println!("Key = {:#034X}", key);
    assert_eq!(key, 0x8877665544332211EFCDAB9078563412_u128);

    // The case for the key shorter than 128 bits
    let mut _rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let key = _rijndael.get_key_u128();
    println!("Key = {:#034X}", key);
    assert_eq!(key, 0x0000000000000000EFCDAB9078563412_u128);
    println!("-------------------------------");
}

fn aes_set_key()
{
    println!("aes_set_key");

    use cryptocol::symmetric::{ AES_128, AES_192, Rijndael_Generic };

    // Normal case
    let mut aes = AES_128::new();
    aes.set_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let key = aes.get_key();
    print!("K = ");
    for k in key
        { print!("{:#010X} ", k); }
    println!();
    assert_eq!(key, [0x78563412, 0xEFCDAB90, 0x78563412, 0xEFCDAB90]);

    // The case for the necessary key longer than the given key
    let mut aes = AES_192::new();
    aes.set_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let key = aes.get_key();
    print!("K = ");
    for k in key
        { print!("{:#010X} ", k); }
    println!();
    assert_eq!(key, [0x78563412_u32, 0xEFCDAB90, 0x78563412_u32, 0xEFCDAB90, 0x00000000, 0x00000000]);

    // The case for the necessary key shorter than the given key
    let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key_u128(0xEFCDAB9078563412EFCDAB9078563412);
    rijndael.set_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let key = rijndael.get_key();
    print!("K = ");
    for k in key
        { print!("{:#010X} ", k); }
    println!();
    assert_eq!(key, [0x78563412_u32, 0xEFCDAB90]);
    println!("-------------------------------");
}

fn aes_set_key_u128()
{
    println!("aes_set_key_u128");
    use cryptocol::symmetric::{ AES_128, AES_256, Rijndael_Generic };

    // Normal case
    let mut aes = AES_128::new();
    aes.set_key_u128(0xEFCDAB9078563412EFCDAB9078563412);
    let key = aes.get_key();
    print!("K = ");
    for k in key
        { print!("{:#010X} ", k); }
    println!();
    assert_eq!(key, [0x78563412, 0xEFCDAB90, 0x78563412, 0xEFCDAB90]);

    // The case for the necessary key longer than 128 bits
    let mut aes = AES_256::new();
    aes.set_key_u128(0xEFCDAB9078563412EFCDAB9078563412);
    let key = aes.get_key();
    print!("K = ");
    for k in key
        { print!("{:#010X} ", k); }
    println!();
    assert_eq!(key, [0x78563412_u32, 0xEFCDAB90, 0x78563412_u32, 0xEFCDAB90, 0x00000000, 0x00000000, 0x00000000, 0x00000000]);

    // The case for the necessary key shorter than 128 bits
    let mut rijndael = Rijndael_Generic::<10, 4, 2>::new();
    rijndael.set_key_u128(0xEFCDAB9078563412EFCDAB9078563412);
    let key = rijndael.get_key();
    print!("K = ");
    for k in key
        { print!("{:#010X} ", k); }
    println!();
    assert_eq!(key, [0x78563412_u32, 0xEFCDAB90]);
    println!("-------------------------------");
}
/*
fn aes_turn_inverse()
{
    println!("aes_turn_inverse");
    use cryptocol::symmetric::{ BigCryptor64, AES_128, SmallCryptor };

    let mut keys: [Box<dyn SmallCryptor<u64, 8>>; 3]
                = [ Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEF_u128)),
                    Box::new(AES_128::new_with_key_u128(0x_FEDCBA0987654321_u128)),
                    Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEF_u128)) ];
    keys[1].turn_inverse();

    let mut tdes = BigCryptor64::new_with_small_cryptor_array(keys);
    let plaintext = 0x_1234567890ABCDEF_u128;
    let ciphertext = tdes.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u128);

    let cipher_cipher_text = tdes.decrypt_u128(ciphertext);
    println!("Cipher-ciphertext:\t{:#034X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u128);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    

    // Operators
    let des1 = AES_128::new_with_key_u128(0x_1234567890ABCDEF_u128);
    let mut des2 = AES_128::new_with_key_u128(0x_FEDCBA0987654321_u128);
    let des3 = AES_128::new_with_key_u128(0x_1234567890ABCDEF_u128);
    des2.turn_inverse();

    let mut tdes = BigCryptor64::new() + des1 + des2 + des3; 
    let plaintext = 0x_1234567890ABCDEF_u128;
    let ciphertext = tdes.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u128);

    let cipher_cipher_text = tdes.decrypt_u128(ciphertext);
    println!("Cipher-ciphertext:\t{:#034X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u128);
    assert_eq!(cipher_cipher_text, plaintext);
    println!("-------------------------------");
}

fn aes_turn_encryptor()
{
    println!("aes_turn_encryptor");
    use cryptocol::symmetric::{ BigCryptor64, AES_128, SmallCryptor };

    let mut keys: [Box<dyn SmallCryptor<u64, 8>>; 3]
            = [ Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEF_u128)),
                Box::new(AES_128::new_with_key_u128(0x_FEDCBA0987654321_u128)),
                Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEF_u128)) ];
    keys[0].turn_encryptor();

    let mut tdes = BigCryptor64::new_with_small_cryptor_array(keys);
    let plaintext = 0x_1234567890ABCDEF_u128;
    let ciphertext = tdes.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x_CDAC175F3B7EAA2B_u128);

    let cipher_cipher_text = tdes.decrypt_u128(ciphertext);
    println!("Cipher-ciphertext:\t{:#034X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u128);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    

    // Operators
    let mut des1 = AES_128::new_with_key_u128(0x_1234567890ABCDEF_u128);
    let des2 = AES_128::new_with_key_u128(0x_FEDCBA0987654321_u128);
    let des3 = AES_128::new_with_key_u128(0x_1234567890ABCDEF_u128);
    des1.turn_encryptor();

    let mut tdes = BigCryptor64::new() + des1 + des2 + des3; 
    let plaintext = 0x_1234567890ABCDEF_u128;
    let ciphertext = tdes.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x_CDAC175F3B7EAA2B_u128);

    let cipher_cipher_text = tdes.decrypt_u128(ciphertext);
    println!("Cipher-ciphertext:\t{:#034X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u128);
    assert_eq!(cipher_cipher_text, plaintext);
    println!("-------------------------------");
}

fn aes_turn_decryptor()
{
    println!("aes_turn_decryptor");
    use cryptocol::symmetric::{ BigCryptor64, AES_128, SmallCryptor };

    let mut keys: [Box<dyn SmallCryptor<u64, 8>>; 3]
                = [ Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEF_u128)),
                    Box::new(AES_128::new_with_key_u128(0x_FEDCBA0987654321_u128)),
                    Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEF_u128)) ];
    keys[1].turn_decryptor();

    let mut tdes = BigCryptor64::new_with_small_cryptor_array(keys);
    let plaintext = 0x_1234567890ABCDEF_u128;
    let ciphertext = tdes.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u128);

    let cipher_cipher_text = tdes.decrypt_u128(ciphertext);
    println!("Cipher-ciphertext:\t{:#034X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u128);
    assert_eq!(cipher_cipher_text, plaintext);
    println!();
    

    // Operators
    let des1 = AES_128::new_with_key_u128(0x_1234567890ABCDEF_u128);
    let mut des2 = AES_128::new_with_key_u128(0x_FEDCBA0987654321_u128);
    let des3 = AES_128::new_with_key_u128(0x_1234567890ABCDEF_u128);
    des2.turn_decryptor();

    let mut tdes = BigCryptor64::new() + des1 + des2 + des3; 
    let plaintext = 0x_1234567890ABCDEF_u128;
    let ciphertext = tdes.encrypt_u128(plaintext);

    println!("Plaintext:\t\t{:#034X}", plaintext);
    println!("Ciphertext:\t\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x_272A2AC7B4E66748_u128);

    let cipher_cipher_text = tdes.decrypt_u128(ciphertext);
    println!("Cipher-ciphertext:\t{:#034X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x_1234567890ABCDEF_u128);
    assert_eq!(cipher_cipher_text, plaintext);
    println!("-------------------------------");
}
*/

fn aes_encrypt_unit()
{
    println!("aes_encrypt_unit()");
    use cryptocol::number::IntUnion;
    use cryptocol::symmetric::{ AES_128, AES_192, Rijndael_Generic };

    // The case for AES_128
    let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let plaintext = [IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678), IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678)];
    let ciphertext = aes.encrypt_unit(&plaintext);

    println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}", plaintext[0].get(), plaintext[1].get(), plaintext[2].get(), plaintext[3].get());
    println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}", ciphertext[0].get(), ciphertext[1].get(), ciphertext[2].get(), ciphertext[3].get());
    assert_eq!(ciphertext[0].get(), 0x27584D87);
    assert_eq!(ciphertext[1].get(), 0x44E2BAE9);
    assert_eq!(ciphertext[2].get(), 0x4AECB5D6);
    assert_eq!(ciphertext[3].get(), 0x01CCF826);

    // The case for AES_192
    let mut aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let plaintext = [IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678), IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678)];
    let ciphertext = aes.encrypt_unit(&plaintext);

    println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}", plaintext[0].get(), plaintext[1].get(), plaintext[2].get(), plaintext[3].get());
    println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}", ciphertext[0].get(), ciphertext[1].get(), ciphertext[2].get(), ciphertext[3].get());
    assert_eq!(ciphertext[0].get(), 0x77047F1E);
    assert_eq!(ciphertext[1].get(), 0x008E2C5B);
    assert_eq!(ciphertext[2].get(), 0x6E5EB091);
    assert_eq!(ciphertext[3].get(), 0x0DB5608E);

    // The case for Rijndael_Generic::<10, 4, 2>
    let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let plaintext = [IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678), IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678)];
    let ciphertext = rijndael.encrypt_unit(&plaintext);

    println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}", plaintext[0].get(), plaintext[1].get(), plaintext[2].get(), plaintext[3].get());
    println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}", ciphertext[0].get(), ciphertext[1].get(), ciphertext[2].get(), ciphertext[3].get());
    assert_eq!(ciphertext[0].get(), 0xBF20110F);
    assert_eq!(ciphertext[1].get(), 0x70E7E05D);
    assert_eq!(ciphertext[2].get(), 0x28996D55);
    assert_eq!(ciphertext[3].get(), 0x9D6C20BD);
    println!("-------------------------------");
}

fn aes_encrypt_u128()
{
    println!("aes_encrypt_u128()");
    use cryptocol::number::IntUnion;
    use cryptocol::symmetric::{ AES_128, AES_192, Rijndael_Generic };

    // The case for AES_128
    let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let plaintext = 0x1234567890ABCDEF1234567890ABCDEF;
    let ciphertext = aes.encrypt_u128(plaintext);

    println!("Plaintext:\t{:#034X}", plaintext);
    println!("Ciphertext:\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x01CCF8264AECB5D644E2BAE927584D87_u128);

    // The case for AES_192
    let mut aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let plaintext = 0x1234567890ABCDEF1234567890ABCDEF;
    let ciphertext = aes.encrypt_u128(plaintext);

    println!("Plaintext:\t{:#034X}", plaintext);
    println!("Ciphertext:\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x0DB5608E6E5EB091008E2C5B77047F1E_u128);

    // The case for Rijndael_Generic::<10, 4, 2>
    let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let plaintext = 0x1234567890ABCDEF1234567890ABCDEF;
    let ciphertext = rijndael.encrypt_u128(plaintext);

    println!("Plaintext:\t{:#034X}", plaintext);
    println!("Ciphertext:\t{:#034X}", ciphertext);
    assert_eq!(ciphertext, 0x9D6C20BD28996D5570E7E05DBF20110F_u128);
    println!("-------------------------------");
}

fn aes_decrypt_unit()
{
    println!("aes_decrypt_unit()");
    use cryptocol::number::IntUnion;
    use cryptocol::symmetric::{ AES_128, AES_192, Rijndael_Generic };

    // The case for AES_128
    let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let ciphertext = [IntUnion::new_with(0x27584D87), IntUnion::new_with(0x44E2BAE9), IntUnion::new_with(0x4AECB5D6), IntUnion::new_with(0x01CCF826)];
    let plaintext = aes.decrypt_unit(&ciphertext);

    println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}", ciphertext[0].get(), ciphertext[1].get(), ciphertext[2].get(), ciphertext[3].get());
    println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}", plaintext[0].get(), plaintext[1].get(), plaintext[2].get(), plaintext[3].get());
    assert_eq!(plaintext[0].get(), 0x90ABCDEF);
    assert_eq!(plaintext[1].get(), 0x12345678);
    assert_eq!(plaintext[2].get(), 0x90ABCDEF);
    assert_eq!(plaintext[3].get(), 0x12345678);

    // The case for AES_192
    let mut aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let ciphertext = [IntUnion::new_with(0x77047F1E), IntUnion::new_with(0x008E2C5B), IntUnion::new_with(0x6E5EB091), IntUnion::new_with(0x0DB5608E)];
    let plaintext = aes.decrypt_unit(&ciphertext);

    println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}", ciphertext[0].get(), ciphertext[1].get(), ciphertext[2].get(), ciphertext[3].get());
    println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}", plaintext[0].get(), plaintext[1].get(), plaintext[2].get(), plaintext[3].get());
    assert_eq!(plaintext[0].get(), 0x90ABCDEF);
    assert_eq!(plaintext[1].get(), 0x12345678);
    assert_eq!(plaintext[2].get(), 0x90ABCDEF);
    assert_eq!(plaintext[3].get(), 0x12345678);

    // The case for Rijndael_Generic::<10, 4, 2>
    let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let ciphertext = [IntUnion::new_with(0xBF20110F), IntUnion::new_with(0x70E7E05D), IntUnion::new_with(0x28996D55), IntUnion::new_with(0x9D6C20BD)];
    let plaintext = rijndael.decrypt_unit(&ciphertext);

    println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}", plaintext[0].get(), plaintext[1].get(), plaintext[2].get(), plaintext[3].get());
    println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}", ciphertext[0].get(), ciphertext[1].get(), ciphertext[2].get(), ciphertext[3].get());
    assert_eq!(plaintext[0].get(), 0x90ABCDEF);
    assert_eq!(plaintext[1].get(), 0x12345678);
    assert_eq!(plaintext[2].get(), 0x90ABCDEF);
    assert_eq!(plaintext[3].get(), 0x12345678);
    println!("-------------------------------");
}

fn aes_decrypt_u128()
{
    println!("aes_decrypt_u128()");
    use cryptocol::number::IntUnion;
    use cryptocol::symmetric::{ AES_128, AES_192, Rijndael_Generic };

    // The case for AES_128
    let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let ciphertext = 0x01CCF8264AECB5D644E2BAE927584D87_u128;
    let plaintext = aes.decrypt_u128(ciphertext);

    println!("Ciphertext:\t{:#034X}", ciphertext);
    println!("Plaintext:\t{:#034X}", plaintext);
    assert_eq!(plaintext, 0x1234567890ABCDEF1234567890ABCDEF);

    // The case for AES_192
    let mut aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let ciphertext = 0x0DB5608E6E5EB091008E2C5B77047F1E_u128;
    let plaintext = aes.decrypt_u128(ciphertext);

    println!("Ciphertext:\t{:#034X}", ciphertext);
    println!("Plaintext:\t{:#034X}", plaintext);
    assert_eq!(plaintext, 0x1234567890ABCDEF1234567890ABCDEF);

    // The case for Rijndael_Generic::<10, 4, 2>
    let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let ciphertext = 0x9D6C20BD28996D5570E7E05DBF20110F_u128;
    let plaintext = rijndael.decrypt_u128(ciphertext);

    println!("Ciphertext:\t{:#034X}", ciphertext);
    println!("Plaintext:\t{:#034X}", plaintext);
    assert_eq!(plaintext, 0x1234567890ABCDEF1234567890ABCDEF);
    println!("-------------------------------");
}

fn aes_encrypt_array_unit()
{
    println!("aes_encrypt_array_unit()");
    use cryptocol::number::IntUnion;
    use cryptocol::symmetric::{ AES_128, AES_192, Rijndael_Generic };

    // The case for AES_128
    let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let plaintext = [[IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678), IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678)]; 3];
    let mut ciphertext = [[IntUnion::new(); 4]; 3];
    aes.encrypt_array_unit(&plaintext, &mut ciphertext);

    println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
            plaintext[0][0].get(), plaintext[0][1].get(), plaintext[0][2].get(), plaintext[0][3].get(),
            plaintext[1][0].get(), plaintext[1][1].get(), plaintext[1][2].get(), plaintext[1][3].get(),
            plaintext[2][0].get(), plaintext[2][1].get(), plaintext[2][2].get(), plaintext[2][3].get());
    println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
            ciphertext[0][0].get(), ciphertext[0][1].get(), ciphertext[0][2].get(), ciphertext[0][3].get(),
            ciphertext[1][0].get(), ciphertext[1][1].get(), ciphertext[1][2].get(), ciphertext[1][3].get(),
            ciphertext[2][0].get(), ciphertext[2][1].get(), ciphertext[2][2].get(), ciphertext[2][3].get());
    assert_eq!(ciphertext[0][0].get(), 0x27584D87);
    assert_eq!(ciphertext[0][1].get(), 0x44E2BAE9);
    assert_eq!(ciphertext[0][2].get(), 0x4AECB5D6);
    assert_eq!(ciphertext[0][3].get(), 0x01CCF826);
    assert_eq!(ciphertext[1][0].get(), 0x27584D87);
    assert_eq!(ciphertext[1][1].get(), 0x44E2BAE9);
    assert_eq!(ciphertext[1][2].get(), 0x4AECB5D6);
    assert_eq!(ciphertext[1][3].get(), 0x01CCF826);
    assert_eq!(ciphertext[2][0].get(), 0x27584D87);
    assert_eq!(ciphertext[2][1].get(), 0x44E2BAE9);
    assert_eq!(ciphertext[2][2].get(), 0x4AECB5D6);
    assert_eq!(ciphertext[2][3].get(), 0x01CCF826);
    
    // The case for AES_192
    let mut aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let plaintext = [[IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678), IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678)]; 3];
    let mut ciphertext = [[IntUnion::new(); 4]; 3];
    aes.encrypt_array_unit(&plaintext, &mut ciphertext);

    println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
            plaintext[0][0].get(), plaintext[0][1].get(), plaintext[0][2].get(), plaintext[0][3].get(),
            plaintext[1][0].get(), plaintext[1][1].get(), plaintext[1][2].get(), plaintext[1][3].get(),
            plaintext[2][0].get(), plaintext[2][1].get(), plaintext[2][2].get(), plaintext[2][3].get());
    println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
            ciphertext[0][0].get(), ciphertext[0][1].get(), ciphertext[0][2].get(), ciphertext[0][3].get(),
            ciphertext[1][0].get(), ciphertext[1][1].get(), ciphertext[1][2].get(), ciphertext[1][3].get(),
            ciphertext[2][0].get(), ciphertext[2][1].get(), ciphertext[2][2].get(), ciphertext[2][3].get());
    assert_eq!(ciphertext[0][0].get(), 0x77047F1E);
    assert_eq!(ciphertext[0][1].get(), 0x008E2C5B);
    assert_eq!(ciphertext[0][2].get(), 0x6E5EB091);
    assert_eq!(ciphertext[0][3].get(), 0x0DB5608E);
    assert_eq!(ciphertext[1][0].get(), 0x77047F1E);
    assert_eq!(ciphertext[1][1].get(), 0x008E2C5B);
    assert_eq!(ciphertext[1][2].get(), 0x6E5EB091);
    assert_eq!(ciphertext[1][3].get(), 0x0DB5608E);
    assert_eq!(ciphertext[2][0].get(), 0x77047F1E);
    assert_eq!(ciphertext[2][1].get(), 0x008E2C5B);
    assert_eq!(ciphertext[2][2].get(), 0x6E5EB091);
    assert_eq!(ciphertext[2][3].get(), 0x0DB5608E);

    // The case for Rijndael_Generic::<10, 4, 2>
    let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let plaintext = [[IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678), IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678)]; 3];
    let mut ciphertext = [[IntUnion::new(); 4]; 3];
    rijndael.encrypt_array_unit(&plaintext, &mut ciphertext);

    println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
            plaintext[0][0].get(), plaintext[0][1].get(), plaintext[0][2].get(), plaintext[0][3].get(),
            plaintext[1][0].get(), plaintext[1][1].get(), plaintext[1][2].get(), plaintext[1][3].get(),
            plaintext[2][0].get(), plaintext[2][1].get(), plaintext[2][2].get(), plaintext[2][3].get());
    println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
            ciphertext[0][0].get(), ciphertext[0][1].get(), ciphertext[0][2].get(), ciphertext[0][3].get(),
            ciphertext[1][0].get(), ciphertext[1][1].get(), ciphertext[1][2].get(), ciphertext[1][3].get(),
            ciphertext[2][0].get(), ciphertext[2][1].get(), ciphertext[2][2].get(), ciphertext[2][3].get());
    assert_eq!(ciphertext[0][0].get(), 0xBF20110F);
    assert_eq!(ciphertext[0][1].get(), 0x70E7E05D);
    assert_eq!(ciphertext[0][2].get(), 0x28996D55);
    assert_eq!(ciphertext[0][3].get(), 0x9D6C20BD);
    assert_eq!(ciphertext[1][0].get(), 0xBF20110F);
    assert_eq!(ciphertext[1][1].get(), 0x70E7E05D);
    assert_eq!(ciphertext[1][2].get(), 0x28996D55);
    assert_eq!(ciphertext[1][3].get(), 0x9D6C20BD);
    assert_eq!(ciphertext[2][0].get(), 0xBF20110F);
    assert_eq!(ciphertext[2][1].get(), 0x70E7E05D);
    assert_eq!(ciphertext[2][2].get(), 0x28996D55);
    assert_eq!(ciphertext[2][3].get(), 0x9D6C20BD);
    println!("-------------------------------");
}

fn aes_encrypt_array_u128()
{
    println!("aes_encrypt_array_u128()");
    use cryptocol::number::IntUnion;
    use cryptocol::symmetric::{ AES_128, AES_192, Rijndael_Generic };

    // The case for AES_128
    let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let plaintext = [0x1234567890ABCDEF1234567890ABCDEF_u128, 0x1234567890ABCDEF1234567890ABCDEF, 0x1234567890ABCDEF1234567890ABCDEF];
    let mut ciphertext = [0_u128; 3];
    aes.encrypt_array_u128(&plaintext, &mut ciphertext);

    println!("Plaintext:\t{:#034X} {:#034X} {:#034X}", plaintext[0], plaintext[1], plaintext[2]);
    println!("Ciphertext:\t{:#034X} {:#034X} {:#034X}", ciphertext[0], ciphertext[1], ciphertext[2]);
    assert_eq!(ciphertext[0], 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    assert_eq!(ciphertext[1], 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    assert_eq!(ciphertext[2], 0x01CCF8264AECB5D644E2BAE927584D87_u128);

    // The case for AES_192
    let mut aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let plaintext = [0x1234567890ABCDEF1234567890ABCDEF_u128, 0x1234567890ABCDEF1234567890ABCDEF, 0x1234567890ABCDEF1234567890ABCDEF];
    let mut ciphertext = [0_u128; 3];
    aes.encrypt_array_u128(&plaintext, &mut ciphertext);

    println!("Plaintext:\t{:#034X} {:#034X} {:#034X}", plaintext[0], plaintext[1], plaintext[2]);
    println!("Ciphertext:\t{:#034X} {:#034X} {:#034X}", ciphertext[0], ciphertext[1], ciphertext[2]);
    assert_eq!(ciphertext[0], 0x0DB5608E6E5EB091008E2C5B77047F1E_u128);
    assert_eq!(ciphertext[1], 0x0DB5608E6E5EB091008E2C5B77047F1E_u128);
    assert_eq!(ciphertext[2], 0x0DB5608E6E5EB091008E2C5B77047F1E_u128);

    // The case for Rijndael_Generic::<10, 4, 2>
    let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let plaintext = [0x1234567890ABCDEF1234567890ABCDEF_u128, 0x1234567890ABCDEF1234567890ABCDEF, 0x1234567890ABCDEF1234567890ABCDEF];
    let mut ciphertext = [0_u128; 3];
    rijndael.encrypt_array_u128(&plaintext, &mut ciphertext);

    println!("Plaintext:\t{:#034X} {:#034X} {:#034X}", plaintext[0], plaintext[1], plaintext[2]);
    println!("Ciphertext:\t{:#034X} {:#034X} {:#034X}", ciphertext[0], ciphertext[1], ciphertext[2]);
    assert_eq!(ciphertext[0], 0x9D6C20BD28996D5570E7E05DBF20110F_u128);
    assert_eq!(ciphertext[1], 0x9D6C20BD28996D5570E7E05DBF20110F_u128);
    assert_eq!(ciphertext[2], 0x9D6C20BD28996D5570E7E05DBF20110F_u128);
    println!("-------------------------------");
}

fn aes_decrypt_array_unit()
{
    println!("aes_decrypt_array_unit()");
    use cryptocol::number::IntUnion;
    use cryptocol::symmetric::{ AES_128, AES_192, Rijndael_Generic };

    // The case for AES_128
    let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let ciphertext = [[IntUnion::new_with(0x27584D87), IntUnion::new_with(0x44E2BAE9), IntUnion::new_with(0x4AECB5D6), IntUnion::new_with(0x01CCF826)]; 3];
    let mut plaintext = [[IntUnion::new(); 4]; 3];
    aes.decrypt_array_unit(&ciphertext, &mut plaintext);

    println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
            ciphertext[0][0].get(), ciphertext[0][1].get(), ciphertext[0][2].get(), ciphertext[0][3].get(),
            ciphertext[1][0].get(), ciphertext[1][1].get(), ciphertext[1][2].get(), ciphertext[1][3].get(),
            ciphertext[2][0].get(), ciphertext[2][1].get(), ciphertext[2][2].get(), ciphertext[2][3].get());
    println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
            plaintext[0][0].get(), plaintext[0][1].get(), plaintext[0][2].get(), plaintext[0][3].get(),
            plaintext[1][0].get(), plaintext[1][1].get(), plaintext[1][2].get(), plaintext[1][3].get(),
            plaintext[2][0].get(), plaintext[2][1].get(), plaintext[2][2].get(), plaintext[2][3].get());
    assert_eq!(plaintext[0][0].get(), 0x90ABCDEF);
    assert_eq!(plaintext[0][1].get(), 0x12345678);
    assert_eq!(plaintext[0][2].get(), 0x90ABCDEF);
    assert_eq!(plaintext[0][3].get(), 0x12345678);
    assert_eq!(plaintext[1][0].get(), 0x90ABCDEF);
    assert_eq!(plaintext[1][1].get(), 0x12345678);
    assert_eq!(plaintext[1][2].get(), 0x90ABCDEF);
    assert_eq!(plaintext[1][3].get(), 0x12345678);
    assert_eq!(plaintext[2][0].get(), 0x90ABCDEF);
    assert_eq!(plaintext[2][1].get(), 0x12345678);
    assert_eq!(plaintext[2][2].get(), 0x90ABCDEF);
    assert_eq!(plaintext[2][3].get(), 0x12345678);
    
    // The case for AES_192
    let mut aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let ciphertext = [[IntUnion::new_with(0x77047F1E), IntUnion::new_with(0x008E2C5B), IntUnion::new_with(0x6E5EB091), IntUnion::new_with(0x0DB5608E)]; 3];
    let mut plaintext = [[IntUnion::new(); 4]; 3];
    aes.decrypt_array_unit(&ciphertext, &mut plaintext);

    println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
            ciphertext[0][0].get(), ciphertext[0][1].get(), ciphertext[0][2].get(), ciphertext[0][3].get(),
            ciphertext[1][0].get(), ciphertext[1][1].get(), ciphertext[1][2].get(), ciphertext[1][3].get(),
            ciphertext[2][0].get(), ciphertext[2][1].get(), ciphertext[2][2].get(), ciphertext[2][3].get());
    println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
            plaintext[0][0].get(), plaintext[0][1].get(), plaintext[0][2].get(), plaintext[0][3].get(),
            plaintext[1][0].get(), plaintext[1][1].get(), plaintext[1][2].get(), plaintext[1][3].get(),
            plaintext[2][0].get(), plaintext[2][1].get(), plaintext[2][2].get(), plaintext[2][3].get());
    assert_eq!(plaintext[0][0].get(), 0x90ABCDEF);
    assert_eq!(plaintext[0][1].get(), 0x12345678);
    assert_eq!(plaintext[0][2].get(), 0x90ABCDEF);
    assert_eq!(plaintext[0][3].get(), 0x12345678);
    assert_eq!(plaintext[1][0].get(), 0x90ABCDEF);
    assert_eq!(plaintext[1][1].get(), 0x12345678);
    assert_eq!(plaintext[1][2].get(), 0x90ABCDEF);
    assert_eq!(plaintext[1][3].get(), 0x12345678);
    assert_eq!(plaintext[2][0].get(), 0x90ABCDEF);
    assert_eq!(plaintext[2][1].get(), 0x12345678);
    assert_eq!(plaintext[2][2].get(), 0x90ABCDEF);
    assert_eq!(plaintext[2][3].get(), 0x12345678);

    // The case for Rijndael_Generic::<10, 4, 2>
    let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let ciphertext = [[IntUnion::new_with(0xBF20110F), IntUnion::new_with(0x70E7E05D), IntUnion::new_with(0x28996D55), IntUnion::new_with(0x9D6C20BD)]; 3];
    let mut plaintext = [[IntUnion::new(); 4]; 3];
    rijndael.decrypt_array_unit(&ciphertext, &mut plaintext);

    println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
            ciphertext[0][0].get(), ciphertext[0][1].get(), ciphertext[0][2].get(), ciphertext[0][3].get(),
            ciphertext[1][0].get(), ciphertext[1][1].get(), ciphertext[1][2].get(), ciphertext[1][3].get(),
            ciphertext[2][0].get(), ciphertext[2][1].get(), ciphertext[2][2].get(), ciphertext[2][3].get());
    println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
            plaintext[0][0].get(), plaintext[0][1].get(), plaintext[0][2].get(), plaintext[0][3].get(),
            plaintext[1][0].get(), plaintext[1][1].get(), plaintext[1][2].get(), plaintext[1][3].get(),
            plaintext[2][0].get(), plaintext[2][1].get(), plaintext[2][2].get(), plaintext[2][3].get());
    assert_eq!(plaintext[0][0].get(), 0x90ABCDEF);
    assert_eq!(plaintext[0][1].get(), 0x12345678);
    assert_eq!(plaintext[0][2].get(), 0x90ABCDEF);
    assert_eq!(plaintext[0][3].get(), 0x12345678);
    assert_eq!(plaintext[1][0].get(), 0x90ABCDEF);
    assert_eq!(plaintext[1][1].get(), 0x12345678);
    assert_eq!(plaintext[1][2].get(), 0x90ABCDEF);
    assert_eq!(plaintext[1][3].get(), 0x12345678);
    assert_eq!(plaintext[2][0].get(), 0x90ABCDEF);
    assert_eq!(plaintext[2][1].get(), 0x12345678);
    assert_eq!(plaintext[2][2].get(), 0x90ABCDEF);
    assert_eq!(plaintext[2][3].get(), 0x12345678);
    println!("-------------------------------");
}

fn aes_decrypt_array_u128()
{
    println!("aes_decrypt_array_u128()");

    use cryptocol::number::IntUnion;
    use cryptocol::symmetric::{ AES_128, AES_192, Rijndael_Generic };

    // The case for AES_128
    let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let ciphertext = [0x01CCF8264AECB5D644E2BAE927584D87_u128, 0x01CCF8264AECB5D644E2BAE927584D87_u128, 0x01CCF8264AECB5D644E2BAE927584D87_u128];
    let mut plaintext = [0_u128; 3];
    aes.decrypt_array_u128(&ciphertext, &mut plaintext);

    println!("Ciphertext:\t{:#034X} {:#034X} {:#034X}", ciphertext[0], ciphertext[1], ciphertext[2]);
    println!("Plaintext:\t{:#034X} {:#034X} {:#034X}", plaintext[0], plaintext[1], plaintext[2]);
    assert_eq!(plaintext[0], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    assert_eq!(plaintext[1], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    assert_eq!(plaintext[2], 0x1234567890ABCDEF1234567890ABCDEF_u128);

    // The case for AES_192
    let mut aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let ciphertext = [0x0DB5608E6E5EB091008E2C5B77047F1E_u128, 0x0DB5608E6E5EB091008E2C5B77047F1E_u128, 0x0DB5608E6E5EB091008E2C5B77047F1E_u128];
    let mut plaintext = [0_u128; 3];
    aes.decrypt_array_u128(&ciphertext, &mut plaintext);

    println!("Ciphertext:\t{:#034X} {:#034X} {:#034X}", ciphertext[0], ciphertext[1], ciphertext[2]);
    println!("Plaintext:\t{:#034X} {:#034X} {:#034X}", plaintext[0], plaintext[1], plaintext[2]);
    assert_eq!(plaintext[0], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    assert_eq!(plaintext[1], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    assert_eq!(plaintext[2], 0x1234567890ABCDEF1234567890ABCDEF_u128);

    // The case for Rijndael_Generic::<10, 4, 2>
    let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let ciphertext = [0x9D6C20BD28996D5570E7E05DBF20110F_u128, 0x9D6C20BD28996D5570E7E05DBF20110F_u128, 0x9D6C20BD28996D5570E7E05DBF20110F_u128];
    let mut plaintext = [0_u128; 3];
    rijndael.decrypt_array_u128(&ciphertext, &mut plaintext);

    println!("Ciphertext:\t{:#034X} {:#034X} {:#034X}", ciphertext[0], ciphertext[1], ciphertext[2]);
    println!("Plaintext:\t{:#034X} {:#034X} {:#034X}", plaintext[0], plaintext[1], plaintext[2]);
    assert_eq!(plaintext[0], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    assert_eq!(plaintext[1], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    assert_eq!(plaintext[2], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    println!("-------------------------------");
}

fn aes_is_successful()
{
    println!("aes_is_successful()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::AES_128;
    {
        use cryptocol::symmetric::ECB_PKCS7;

        // Normal case for the message of 0 bytes
        let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
        println!("K =\t{:#034X}", key);
        let mut a_aes = AES_128::new_with_key_u128(key);
        let message = "";
        println!("M =\t{}", message);
        let mut cipher = [0_u8; 16];
        let len = a_aes.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
        println!("The length of ciphertext = {}", len);
        assert_eq!(len, 16);
        let success = a_aes.is_successful();
        assert_eq!(success, true);
        print!("C =\t");
        for c in cipher.clone()
            { print!("{:02X} ", c); }
        println!();
        let mut txt = String::new();
        for c in cipher.clone()
            { write!(txt, "{:02X} ", c); }
        assert_eq!(txt, "26 F2 F8 B7 B7 FD 46 9A 97 97 F3 24 E7 51 99 47 ");
    
    
        // Normal case for the original message of 0 bytes
        let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
        println!("K =\t{:#034X}", key);
        let mut a_aes = AES_128::new_with_key_u128(key);
    
        let cipher = [0x26_u8, 0xF2, 0xF8, 0xB7, 0xB7, 0xFD, 0x46, 0x9A, 0x97, 0x97, 0xF3, 0x24, 0xE7, 0x51, 0x99, 0x47];
        print!("C =\t");
        for c in cipher.clone()
            { print!("{:02X} ", c); }
        println!();
        let mut recovered = [0u8; 16];
        let len = a_aes.decrypt_array_into_array(&cipher, &mut recovered);
        println!("The length of plaintext = {}", len);
        assert_eq!(len, 0);
        let success = a_aes.is_successful();
        assert_eq!(success, true);
        print!("Ba =\t");
        for b in recovered.clone()
            { print!("{:02X} ", b); }
        println!();
        let mut txt = String::new();
        for c in recovered.clone()
            { write!(txt, "{:02X} ", c); }
        assert_eq!(txt, "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ");
    
        let mut converted = String::new();
        unsafe { converted.as_mut_vec() }.write(&recovered);
        unsafe { converted.as_mut_vec() }.truncate(len as usize);
        println!("Bb =\t{}", converted);
        assert_eq!(converted, "");
        assert_eq!(converted, message);
        println!();
    }

    {
        use cryptocol::symmetric::CFB;

        // Failed case for encryption
        let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
        println!("K =\t{:#034X}", key);
        let mut a_aes = AES_128::new_with_key_u128(key);

        let message = "In the beginning God created the heavens and the earth.";
        println!("M =\t{}", message);
        let iv = [0xEFCDAB90_u32, 0x78563412, 0xEFCDAB90, 0x78563412];
        println!("IV = {} {} {} {}", iv[0], iv[1], iv[2], iv[3]);
        let mut cipher = [0_u8; 40];
        let len = a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
        println!("The length of ciphertext = {}", len);
        assert_eq!(len, 0);
        let success = a_aes.is_successful();
        assert_eq!(success, false);

        // Failed case for decryption
        let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
        println!("K =\t{:#034X}", key);
        let mut a_aes = AES_128::new_with_key_u128(key);

        let cipher = [0x2Eu8, 0x1E, 0xE1, 0x51, 0xFD, 0xB3, 0xB0, 0x4B, 0x79, 0x3A, 0xA1, 0x78, 0xEC, 0xCD, 0x02, 0x72, 0x6A, 0xC4, 0x41, 0x7C, 0x25, 0xA4, 0x2C, 0x07, 0xFC, 0x77, 0x25, 0x49, 0x12, 0x55, 0x0F, 0x8A, 0xED, 0x44, 0xC3, 0xE4, 0xDC, 0x91, 0x69, 0x0F, 0x40, 0x72, 0x7F, 0xF2, 0xD9, 0xB7, 0x54, 0x9F, 0x36, 0x91, 0xC5, 0x85, 0x4F, 0x9B, 0x30];
        print!("C =\t");
        for c in cipher.clone()
            { print!("{:02X} ", c); }
        println!();
        let mut recovered = [0u8; 40];
        let len = a_aes.decrypt_array_into_array(iv, &cipher, &mut recovered);
        println!("The length of plaintext = {}", len);
        assert_eq!(len, 0);
        let success = a_aes.is_successful();
        assert_eq!(success, false);
    }
    println!("-------------------------------");
}

fn aes_is_failed()
{
    println!("aes_is_failed()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::AES_128;
    {
        use cryptocol::symmetric::ECB_PKCS7;

        // Normal case for the message of 0 bytes
        let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
        println!("K =\t{:#034X}", key);
        let mut a_aes = AES_128::new_with_key_u128(key);
        let message = "";
        println!("M =\t{}", message);
        let mut cipher = [0_u8; 16];
        let len = a_aes.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
        println!("The length of ciphertext = {}", len);
        assert_eq!(len, 16);
        let failure = a_aes.is_failed();
        assert_eq!(failure, false);
        print!("C =\t");
        for c in cipher.clone()
            { print!("{:02X} ", c); }
        println!();
        let mut txt = String::new();
        for c in cipher.clone()
            { write!(txt, "{:02X} ", c); }
        assert_eq!(txt, "26 F2 F8 B7 B7 FD 46 9A 97 97 F3 24 E7 51 99 47 ");
    
    
        // Normal case for the original message of 0 bytes
        let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
        println!("K =\t{:#034X}", key);
        let mut a_aes = AES_128::new_with_key_u128(key);
    
        let cipher = [0x26_u8, 0xF2, 0xF8, 0xB7, 0xB7, 0xFD, 0x46, 0x9A, 0x97, 0x97, 0xF3, 0x24, 0xE7, 0x51, 0x99, 0x47];
        print!("C =\t");
        for c in cipher.clone()
            { print!("{:02X} ", c); }
        println!();
        let mut recovered = [0u8; 16];
        let len = a_aes.decrypt_array_into_array(&cipher, &mut recovered);
        println!("The length of plaintext = {}", len);
        assert_eq!(len, 0);
        let failure = a_aes.is_failed();
        assert_eq!(failure, false);
        print!("Ba =\t");
        for b in recovered.clone()
            { print!("{:02X} ", b); }
        println!();
        let mut txt = String::new();
        for c in recovered.clone()
            { write!(txt, "{:02X} ", c); }
        assert_eq!(txt, "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ");
    
        let mut converted = String::new();
        unsafe { converted.as_mut_vec() }.write(&recovered);
        unsafe { converted.as_mut_vec() }.truncate(len as usize);
        println!("Bb =\t{}", converted);
        assert_eq!(converted, "");
        assert_eq!(converted, message);
        println!();
    }

    {
        use cryptocol::symmetric::CFB;

        // Failed case for encryption
        let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
        println!("K =\t{:#034X}", key);
        let mut a_aes = AES_128::new_with_key_u128(key);

        let message = "In the beginning God created the heavens and the earth.";
        println!("M =\t{}", message);
        let iv = [0xEFCDAB90_u32, 0x78563412, 0xEFCDAB90, 0x78563412];
        println!("IV = {} {} {} {}", iv[0], iv[1], iv[2], iv[3]);
        let mut cipher = [0_u8; 40];
        let len = a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
        println!("The length of ciphertext = {}", len);
        assert_eq!(len, 0);
        let failure = a_aes.is_failed();
        assert_eq!(failure, true);

        // Failed case for decryption
        let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
        println!("K =\t{:#034X}", key);
        let mut a_aes = AES_128::new_with_key_u128(key);

        let cipher = [0x2Eu8, 0x1E, 0xE1, 0x51, 0xFD, 0xB3, 0xB0, 0x4B, 0x79, 0x3A, 0xA1, 0x78, 0xEC, 0xCD, 0x02, 0x72, 0x6A, 0xC4, 0x41, 0x7C, 0x25, 0xA4, 0x2C, 0x07, 0xFC, 0x77, 0x25, 0x49, 0x12, 0x55, 0x0F, 0x8A, 0xED, 0x44, 0xC3, 0xE4, 0xDC, 0x91, 0x69, 0x0F, 0x40, 0x72, 0x7F, 0xF2, 0xD9, 0xB7, 0x54, 0x9F, 0x36, 0x91, 0xC5, 0x85, 0x4F, 0x9B, 0x30];
        print!("C =\t");
        for c in cipher.clone()
            { print!("{:02X} ", c); }
        println!();
        let mut recovered = [0u8; 40];
        let len = a_aes.decrypt_array_into_array(iv, &cipher, &mut recovered);
        println!("The length of plaintext = {}", len);
        assert_eq!(len, 0);
        let failure = a_aes.is_failed();
        assert_eq!(failure, true);
    }
    println!("-------------------------------");
}

fn aes_get_desirable_round()
{
    println!("aes_get_desirable_round()");
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_64_64, Rijndael_Generic };
    let rounds = AES_128::get_desirable_round();
    println!("The desirable number of rounds of AES_128 is {}", rounds);
    assert_eq!(rounds, 10);

    let rounds = AES_192::get_desirable_round();
    println!("The desirable number of rounds of AES_192 is {}", rounds);
    assert_eq!(rounds, 12);

    let rounds = AES_256::get_desirable_round();
    println!("The desirable number of rounds of AES_256 is {}", rounds);
    assert_eq!(rounds, 14);

    let rounds = Rijndael_256_256::get_desirable_round();
    println!("The desirable number of rounds of Rijndael_256_256 is {}", rounds);
    assert_eq!(rounds, 14);

    let rounds = Rijndael_64_64::get_desirable_round();
    println!("The desirable number of rounds of Rijndael_64_64 is {}", rounds);
    assert_eq!(rounds, 8);

    let rounds = Rijndael_Generic::<0, 16, 16>::get_desirable_round();
    println!("The desirable number of rounds of Rijndael_Generic<0, 16, 16> is {}", rounds);
    assert_eq!(rounds, 22);
    println!("-------------------------------");
}