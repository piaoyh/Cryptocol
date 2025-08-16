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
    bigcryptor64_quick_start_main();
    bigcryptor64_basic_operation_main();
}

fn bigcryptor64_quick_start_main()
{
    bigcryptor64_import_modules();
    bigcryptor64_instantiation_with_keys_u64();
    bigcryptor64_instantiation_with_keys();
    bigcryptor64_set_keys_u64_later();
    bigcryptor64_set_keys_later();
    bigcryptor64_cbc_pkcs7();
}

fn bigcryptor64_import_modules()
{
    println!("bigcryptor64_import_modules()");

    use cryptocol::symmetric::BigCryptor64;

    println!("-------------------------------");
}

fn bigcryptor64_instantiation_with_keys_u64()
{
    println!("bigcryptor64_instantiation_with_keys_u64()");
    use cryptocol::symmetric::{ BigCryptor64, DES };
    let mut _tdes = BigCryptor64::new()
                                + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
                                + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
                                + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    println!("-------------------------------");
}

fn bigcryptor64_instantiation_with_keys()
{
    println!("bigcryptor64_instantiation_with_keys()");
    use cryptocol::symmetric::{ BigCryptor64, DES };
    let mut _tdes = BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    println!("-------------------------------");
}

fn bigcryptor64_set_keys_u64_later()
{
    println!("bigcryptor64_instantiation_with_keys_u64()");
    use cryptocol::symmetric::{ BigCryptor64, DES };
    let mut tdes = BigCryptor64::new();
    let des1 = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let des2 = DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64);
    let des3 = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    tdes.push_small_cryptor(des1);
    tdes.push_small_cryptor(des2);
    tdes.push_small_cryptor(des3);
    println!("-------------------------------");
}

fn bigcryptor64_set_keys_later()
{
    println!("bigcryptor64_instantiation_with_keys()");
    use cryptocol::symmetric::{ BigCryptor64, DES };
    let mut tdes = BigCryptor64::new();
    let des1 = DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let des2 = DES::decryptor_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    let des3 = DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    tdes.push_small_cryptor(des1);
    tdes.push_small_cryptor(des2);
    tdes.push_small_cryptor(des3);
    println!("-------------------------------");
}

fn bigcryptor64_cbc_pkcs7()
{
    println!("bigcryptor64_cbc_pkcs7()");
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };
    let mut tdes = BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    
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
    assert_eq!(txt, "86 2B D7 BF 00 2E CD 70 ED 0C E3 8D 75 18 CE 0F BD A7 AE AF E5 19 46 F8 15 7A 24 0E CB 20 91 C0 03 B9 56 C5 77 01 33 E8 8E 84 CA B9 F2 99 63 AC 3A 3D 1F EF CA CA CB 67 ");

    let mut recovered = String::new();
    tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn bigcryptor64_basic_operation_main()
{
    bigcryptor64_new();
    bigcryptor64_new_with_small_cryptor_array();
    bigcryptor64_new_with_small_cryptor_vec();
    bigcryptor64_push_small_cryptor();
    bigcryptor64_push_small_cryptor_array();
    bigcryptor64_push_small_cryptor_vec();
    bigcryptor64_encrypt_u64();
    bigcryptor64_decrypt_u64();
    bigcryptor64_encrypt_array_u64();
    bigcryptor64_decrypt_array_u64();
    bigcryptor64_is_successful();
    bigcryptor64_is_failed();
}

fn bigcryptor64_new()
{
    println!("bigcryptor64_new()");
    use cryptocol::symmetric::{ BigCryptor64, DES };

    // Case 1
    let mut tdes = BigCryptor64::new();
    tdes.push_small_cryptor(DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]));
    tdes.push_small_cryptor(DES::decryptor_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]));
    tdes.push_small_cryptor(DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]));

    // Case 2
    let mut _tdes = BigCryptor64::new()
                                + DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                + DES::decryptor_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    println!("-------------------------------");
}

fn bigcryptor64_new_with_small_cryptor_array()
{
    println!("bigcryptor64_new_with_small_cryptor_array()");
    use cryptocol::symmetric::{ BigCryptor64, SmallCryptor, DES };

    // Case 1
    let cryptors: [Box<dyn SmallCryptor<u64, 8>>; 3] = [ Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)),
                                            Box::new(DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)),
                                            Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    let mut _tdes = BigCryptor64::new_with_small_cryptor_array(cryptors);
    println!("-------------------------------");
}

fn bigcryptor64_new_with_small_cryptor_vec()
{
    println!("bigcryptor64_new_with_small_cryptor_vec()");
    use cryptocol::symmetric::{ BigCryptor64, SmallCryptor, DES };

    // Case 1
    let cryptors: Vec<Box<dyn SmallCryptor<u64, 8>>> = vec![ Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)),
                                            Box::new(DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)),
                                            Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    let mut _tdes = BigCryptor64::new_with_small_cryptor_vec(cryptors);
    println!("-------------------------------");
}

fn bigcryptor64_push_small_cryptor()
{
    println!("bigcryptor64_new_with_small_cryptor_vec()");
    use cryptocol::symmetric::{ BigCryptor64, DES };

    // Case 1
    let mut tdes = BigCryptor64::new();
    let des1 = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    let des2 = DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64);
    let des3 = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    tdes.push_small_cryptor(des1);
    tdes.push_small_cryptor(des2);
    tdes.push_small_cryptor(des3);
    println!("-------------------------------");
}

fn bigcryptor64_push_small_cryptor_array()
{
    println!("bigcryptor64_push_small_cryptor_array()");
    use cryptocol::symmetric::{ BigCryptor64, SmallCryptor, DES };

    // Case 1
    let mut tdes = BigCryptor64::new();
    let cryptors: [Box<dyn SmallCryptor<u64, 8>>; 3] = [ Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)),
                                            Box::new(DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)),
                                            Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    tdes.push_small_cryptor_array(cryptors);
    println!("-------------------------------");
}

fn bigcryptor64_push_small_cryptor_vec()
{
    println!("bigcryptor64_push_small_cryptor_vec()");
    use cryptocol::symmetric::{ BigCryptor64, SmallCryptor, DES };

    // Case 1
    let mut tdes = BigCryptor64::new();
    let cryptors: Vec<Box<dyn SmallCryptor<u64, 8>>> = vec![ Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)),
                                            Box::new(DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)),
                                            Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    tdes.push_small_cryptor_vec(cryptors);
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_u64()
{
    println!("bigcryptor64_encrypt_u64()");
    use cryptocol::symmetric::{ BigCryptor64, DES };

    // Case 1
    let mut tdes = BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let message = 0x1234567890ABCDEF_u64;
    println!("M = {:#018X}", message);
    let cipher = tdes.encrypt_u64(message);
    println!("C = {:#018X}", cipher);
    assert_eq!(cipher, 0x_CA61814E7AE964BA_u64);
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_u64()
{
    println!("bigcryptor64_decrypt_u64()");
    use cryptocol::symmetric::{ BigCryptor64, DES };

    // Case 1
    let mut tdes = BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let message = 0x1234567890ABCDEF_u64;
    println!("M = {:#018X}", message);
    let cipher = tdes.encrypt_u64(message);
    println!("C = {:#018X}", cipher);
    assert_eq!(cipher, 0x_CA61814E7AE964BA_u64);

    let recovered = tdes.decrypt_u64(cipher);
    println!("B = {:#018X}", recovered);
    assert_eq!(recovered, 0x1234567890ABCDEF_u64);
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn bigcryptor64_encrypt_array_u64()
{
    println!("bigcryptor64_encrypt_array_u64()");
    use cryptocol::symmetric::{ BigCryptor64, DES };

    // Case 1
    let mut tdes = BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let message = [0x1234567890ABCDEF_u64, 0x1122334455667788, 0x9900AABBCCDDEEFF];
    print!("M = ");
    for msg in message.clone()
        { print!("{:#018X} ", msg); }
    println!();

    let mut cipher = [0_u64; 3];
    tdes.encrypt_array_u64(&message, &mut cipher);
    print!("C = ");
    for c in cipher.clone()
        { print!("{:#018X} ", c); }
    println!();
    assert_eq!(cipher[0], 0x_CA61814E7AE964BA_u64);
    assert_eq!(cipher[1], 0x_073450DF82262B1B_u64);
    assert_eq!(cipher[2], 0x_51712805A458A102_u64);
    println!("-------------------------------");
}

fn bigcryptor64_decrypt_array_u64()
{
    println!("bigcryptor64_decrypt_array_u64()");
    use cryptocol::symmetric::{ BigCryptor64, DES };

    // Case 1
    let mut tdes = BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let message = [0x1234567890ABCDEF_u64, 0x1122334455667788, 0x9900AABBCCDDEEFF];
    print!("M = ");
    for msg in message.clone()
        { print!("{:#018X} ", msg); }
    println!();

    let mut cipher = [0_u64; 3];
    tdes.encrypt_array_u64(&message, &mut cipher);
    print!("C = ");
    for c in cipher.clone()
        { print!("{:#018X} ", c); }
    println!();
    assert_eq!(cipher[0], 0x_CA61814E7AE964BA_u64);
    assert_eq!(cipher[1], 0x_073450DF82262B1B_u64);
    assert_eq!(cipher[2], 0x_51712805A458A102_u64);

    let mut recovered = [0_u64; 3];
    tdes.decrypt_array_u64(&cipher, &mut recovered);
    print!("B = ");
    for r in recovered.clone()
        { print!("{:#018X} ", r); }
    println!();
    assert_eq!(recovered[0], 0x_1234567890ABCDEF_u64);
    assert_eq!(recovered[1], 0x_1122334455667788_u64);
    assert_eq!(recovered[2], 0x_9900AABBCCDDEEFF_u64);
    assert_eq!(recovered[0], message[0]);
    assert_eq!(recovered[1], message[1]);
    assert_eq!(recovered[2], message[2]);
    println!("-------------------------------");
}

fn bigcryptor64_is_successful()
{
    println!("bigcryptor64_is_successful");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // Successful case for encryption
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut tdes = BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);

    let message = "";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 8];
    let len = tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    println!("The length of ciphertext = {}", len);
    assert_eq!(len, 8);
    let success = tdes.is_successful();
    assert_eq!(success, true);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "17 C8 15 48 EE 85 42 43 ");
    println!();

    // Successful case for decryption
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut tdes = BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);

    let cipher = [0x17_u8, 0xC8, 0x15, 0x48, 0xEE, 0x85, 0x42, 0x43];
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut recovered = [0u8; 8];
    let len = tdes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    println!("The length of plaintext = {}", len);
    assert_eq!(len, 0);
    let success = tdes.is_successful();
    assert_eq!(success, true);
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


    // Failure case for encryption
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut tdes = BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);

    let message = "";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 4];
    let len = tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    println!("The length of ciphertext = {}", len);
    assert_eq!(len, 0);
    let success = tdes.is_successful();
    assert_eq!(success, false);
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "00 00 00 00 ");
    println!();

    // Failure case for decryption
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut tdes = BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);

    let cipher = [0x17_u8, 0xC8, 0x15, 0x48];
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut recovered = [0u8; 8];
    let len = tdes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    println!("The length of plaintext = {}", len);
    assert_eq!(len, 0);
    let success = tdes.is_successful();
    assert_eq!(success, false);
    println!("-------------------------------");
}

fn bigcryptor64_is_failed()
{
    println!("bigcryptor64_is_failed");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };

    // Successful case for encryption
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut tdes = BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);

    let message = "";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 8];
    let len = tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    println!("The length of ciphertext = {}", len);
    assert_eq!(len, 8);
    let failure = tdes.is_failed();
    assert_eq!(failure, false);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "17 C8 15 48 EE 85 42 43 ");
    println!();

    // Successful case for decryption
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut tdes = BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);

    let cipher = [0x17_u8, 0xC8, 0x15, 0x48, 0xEE, 0x85, 0x42, 0x43];
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut recovered = [0u8; 8];
    let len = tdes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    println!("The length of plaintext = {}", len);
    assert_eq!(len, 0);
    let failure = tdes.is_failed();
    assert_eq!(failure, false);
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


    // Failure case for encryption
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut tdes = BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);

    let message = "";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 4];
    let len = tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    println!("The length of ciphertext = {}", len);
    assert_eq!(len, 0);
    let failure = tdes.is_failed();
    assert_eq!(failure, true);
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "00 00 00 00 ");
    println!();

    // Failure case for decryption
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{}", iv);
    let mut tdes = BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);

    let cipher = [0x17_u8, 0xC8, 0x15, 0x48];
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();

    let mut recovered = [0u8; 8];
    let len = tdes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    println!("The length of plaintext = {}", len);
    assert_eq!(len, 0);
    let failure = tdes.is_failed();
    assert_eq!(failure, true);
    println!("-------------------------------");
}
