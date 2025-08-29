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
    tdes_quick_start_main();
    ddes_quick_start_main();
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
    bigcryptor64_turn_inverse();
    bigcryptor64_turn_encryptor();
    bigcryptor64_turn_decryptor();
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

fn bigcryptor64_turn_inverse()
{
    println!("bigcryptor64_turn_inverse");
    use cryptocol::symmetric::{ BigCryptor64, DES, Rijndael_64_64, SmallCryptor };
    let mut tdes= BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let des = DES::new_with_key([0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
    let rijndael = Rijndael_64_64::new_with_key(&[0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    tdes.turn_inverse();
    let mut bigcryptor = des + rijndael + tdes;
    
    let plaintext = 0x_1234567890ABCDEF_u64;
    println!("Plaintext:\t\t{:#018X}", plaintext);
    let ciphertext = bigcryptor.encrypt_u64(plaintext);
    println!("Ciphertext:\t\t{:#018X}", ciphertext);
    assert_eq!(ciphertext, 0x_0036D446DF6D218F_u64);

    let recovered_text = bigcryptor.decrypt_u64(ciphertext);
    println!("Recovered text:\t{:#018X}", recovered_text);
    assert_eq!(recovered_text, 0x1234567890ABCDEF_u64);
    assert_eq!(recovered_text, plaintext);
    println!("-------------------------------");
}

fn bigcryptor64_turn_encryptor()
{
    println!("bigcryptor64_turn_encryptor");
    use cryptocol::symmetric::{ BigCryptor64, DES, Rijndael_64_64, SmallCryptor };
    let mut tdes= BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let des = DES::new_with_key([0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
    let rijndael = Rijndael_64_64::new_with_key(&[0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    tdes.turn_encryptor();
    let mut bigcryptor = des + rijndael + tdes;
    
    let plaintext = 0x_1234567890ABCDEF_u64;
    println!("Plaintext:\t\t{:#018X}", plaintext);
    let ciphertext = bigcryptor.encrypt_u64(plaintext);
    println!("Ciphertext:\t\t{:#018X}", ciphertext);
    assert_eq!(ciphertext, 0x_911ED9892E52BC7C_u64);

    let recovered_text = bigcryptor.decrypt_u64(ciphertext);
    println!("Recovered text:\t{:#018X}", recovered_text);
    assert_eq!(recovered_text, 0x1234567890ABCDEF_u64);
    assert_eq!(recovered_text, plaintext);
    println!("-------------------------------");
}

fn bigcryptor64_turn_decryptor()
{
    println!("bigcryptor64_turn_decryptor");
    use cryptocol::symmetric::{ BigCryptor64, DES, Rijndael_64_64, SmallCryptor };
    let mut tdes= BigCryptor64::new()
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let des = DES::new_with_key([0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
    let rijndael = Rijndael_64_64::new_with_key(&[0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    tdes.turn_decryptor();
    let mut bigcryptor = des + rijndael + tdes;
    
    let plaintext = 0x_1234567890ABCDEF_u64;
    println!("Plaintext:\t\t{:#018X}", plaintext);
    let ciphertext = bigcryptor.encrypt_u64(plaintext);
    println!("Ciphertext:\t\t{:#018X}", ciphertext);
    assert_eq!(ciphertext, 0x_0036D446DF6D218F_u64);

    let recovered_text = bigcryptor.decrypt_u64(ciphertext);
    println!("Recovered text:\t{:#018X}", recovered_text);
    assert_eq!(recovered_text, 0x1234567890ABCDEF_u64);
    assert_eq!(recovered_text, plaintext);
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

fn tdes_quick_start_main()
{
    tdes_new_with_2_keys_u64();
    tdes_new_with_3_keys_u64();
    tdes_new_with_2_keys();
    tdes_new_with_3_keys();
    tdes_new_with_keys_u128();
}

fn tdes_new_with_2_keys_u64()
{
    println!("tdes_new_with_2_keys_u64()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ TDES, CBC_PKCS7 };

    let mut tdes = TDES::new_with_2_keys_u64(0x_1234567890ABCDEF_u64, 0x_FEDCBA0987654321_u64);
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

fn tdes_new_with_3_keys_u64()
{
    println!("tdes_new_with_3_keys_u64()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ TDES, CBC_PKCS7 };

    let mut tdes = TDES::new_with_3_keys_u64(0x_1234567890ABCDEF_u64, 0x_1122334455667788_u64, 0x_9900AABBCCDDEEFF_u64);
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
    assert_eq!(txt, "B4 6C 2D 9A F5 67 60 45 A0 2B 68 E8 76 8E BD EF 7E 2D 7A 49 9A DE 43 0D 4C 3C 50 1E B6 8B 0A E8 A4 88 6E E7 FF 99 B9 2D 83 67 C9 3C 4A 2D C7 BA 40 F3 19 88 05 F2 2C D9 ");

    let mut recovered = String::new();
    tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn tdes_new_with_2_keys()
{
    println!("tdes_new_with_2_keys()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ TDES, CBC_PKCS7 };

    let key1 = [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    let key2 = [0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21];
    let mut tdes = TDES::new_with_2_keys(key1, key2);
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
    assert_eq!(txt, "86 2B D7 BF 00 2E CD 70 ED 0C E3 8D 75 18 CE 0F BD A7 AE AF E5 19 46 F8 15 7A 24 0E CB 20 91 C0 03 B9 56 C5 77 01 33 E8 8E 84 CA B9 F2 99 63 AC 3A 3D 1F EF CA CA CB 67 ");

    let mut recovered = String::new();
    tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn tdes_new_with_3_keys()
{
    println!("tdes_new_with_3_keys()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ TDES, CBC_PKCS7 };

    let key1 = [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    let key2 = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    let key3 = [0x99, 0x00, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
    let mut tdes = TDES::new_with_3_keys(key1, key2, key3);
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
    assert_eq!(txt, "93 8E 74 E3 51 84 23 B5 36 76 B6 82 D1 8B 7A A3 1F 87 D2 48 9A 75 BF 59 0D 93 6D 8D A7 86 4A CC 0F D8 0D E0 CD 0D F9 A8 B9 38 36 0C E7 24 73 3F 5F 4D 61 AB 92 D6 34 14 ");

    let mut recovered = String::new();
    tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn tdes_new_with_keys_u128()
{
    println!("tdes_new_with_keys_u128()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ TDES, CBC_PKCS7 };

    let mut tdes = TDES::new_with_keys_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
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
    assert_eq!(txt, "17 AE B5 A8 D2 77 21 0C 73 52 2F EB 5B 7C 9B 82 47 71 D2 7A F0 A9 F0 EA EC 0A D5 61 CB 63 86 33 8C 1F F2 F1 16 62 A0 55 22 9E 12 7A 91 88 D1 37 7B CB 43 32 19 0D AA B0 ");

    let mut recovered = String::new();
    tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn ddes_quick_start_main()
{
    ddes_new_with_2_keys_u64();
    ddes_new_with_2_keys();
    ddes_new_with_keys_u128();
}

fn ddes_new_with_2_keys_u64()
{
    println!("ddes_new_with_2_keys_u64()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DDES, CBC_PKCS7 };

    let mut ddes = DDES::new_with_2_keys_u64(0x_1234567890ABCDEF_u64, 0x_FEDCBA0987654321_u64);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    ddes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "B0 56 0C 03 A4 38 1F 70 C1 21 99 C6 23 06 05 30 DA 57 5F CD 17 FB 11 BF E1 05 92 B3 FD A8 70 15 24 3F A1 29 B9 D1 F6 7A 1D 58 86 BE 40 41 26 15 4A DA D3 EB 4E 84 85 60 ");

    let mut recovered = String::new();
    ddes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn ddes_new_with_2_keys()
{
    println!("ddes_new_with_2_keys()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DDES, CBC_PKCS7 };

    let key1 = [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    let key2 = [0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21];
    let mut ddes = DDES::new_with_2_keys(key1, key2);
    let iv = 0x_FEDCBA0987654321_u64;
    println!("IV =	{:#018X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    ddes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "26 FB A9 70 99 13 83 84 63 A7 17 79 DD C6 1D D0 3D 37 12 07 B3 79 FE 5F E7 E2 6A 06 FF EC 43 26 CB 38 D6 A0 74 05 C0 E8 18 86 3F AF EA 54 AC 12 64 B7 20 C5 7F 36 02 2F ");

    let mut recovered = String::new();
    ddes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn ddes_new_with_keys_u128()
{
    println!("ddes_new_with_keys_u128()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ DDES, CBC_PKCS7 };

    let mut tdes = DDES::new_with_keys_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
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
    assert_eq!(txt, "8E C3 47 07 38 6E 5F E1 BA 9C AF C1 41 B5 22 E3 55 22 DE 83 60 21 41 86 5D 94 06 1A 6C 54 32 F8 58 BE 43 23 FC 80 99 5B 1C 9D 4B 5D 1B E7 8B 9F 91 9D 83 1B C7 3D C0 55 ");

    let mut recovered = String::new();
    tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}
