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


///// For test during implementation //////
pub fn main()
{
    // aes_devel();
    aes_quick_start();
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
