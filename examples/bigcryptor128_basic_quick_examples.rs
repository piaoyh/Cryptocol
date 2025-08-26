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
    bigcryptor128_quick_start_main();
    bigcryptor128_basic_operation_main();
}

fn bigcryptor128_quick_start_main()
{
    bigcryptor128_import_modules();
    bigcryptor128_instantiation_with_keys_u128();
    bigcryptor128_instantiation_with_keys();
    bigcryptor128_set_keys_u64_later();
    bigcryptor128_set_keys_later();
    bigcryptor128_cbc_pkcs7();
}

fn bigcryptor128_import_modules()
{
    println!("bigcryptor128_import_modules()");

    use cryptocol::symmetric::BigCryptor128;

    println!("-------------------------------");
}

fn bigcryptor128_instantiation_with_keys_u128()
{
    println!("bigcryptor128_instantiation_with_keys_u128()");
    use cryptocol::symmetric::{ BigCryptor128, AES_128 };
    let mut _taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
                    + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
                    + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    println!("-------------------------------");
}

fn bigcryptor128_instantiation_with_keys()
{
    println!("bigcryptor128_instantiation_with_keys()");
    use cryptocol::symmetric::{ BigCryptor128, AES_128 };
    let mut _taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                    - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                    + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    println!("-------------------------------");
}

fn bigcryptor128_set_keys_u64_later()
{
    println!("bigcryptor128_instantiation_with_keys_u128()");
    use cryptocol::symmetric::{ BigCryptor128, AES_128 };
    let mut taes = BigCryptor128::new();
    let aes1 = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let aes2 = AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128);
    let aes3 = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    taes.push_small_cryptor(aes1);
    taes.push_small_cryptor(aes2);
    taes.push_small_cryptor(aes3);
    println!("-------------------------------");
}

fn bigcryptor128_set_keys_later()
{
    println!("bigcryptor128_instantiation_with_keys()");
    use cryptocol::symmetric::{ BigCryptor128, AES_128 };
    let mut taes = BigCryptor128::new();
    let aes1 = AES_128::encryptor_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    let aes2 = AES_128::decryptor_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let aes3 = AES_128::encryptor_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    taes.push_small_cryptor(aes1);
    taes.push_small_cryptor(aes2);
    taes.push_small_cryptor(aes3);
    println!("-------------------------------");
}

fn bigcryptor128_cbc_pkcs7()
{
    println!("bigcryptor128_cbc_pkcs7()");
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CBC_PKCS7 };
    let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                    - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                    + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    
    let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("IV =	{:#034X}", iv);
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    taes.encrypt_str_into_vec(iv, &message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "1E 24 26 AD 13 7F 6F 6A CD 22 3A 4F A5 24 D8 E0 E3 6A B2 39 0D 82 2B 6E 7B D6 95 09 6D EF C2 7B 30 53 87 B7 9C D3 21 7C C0 85 11 74 28 7B 98 7B 9F 02 54 81 23 96 6D F5 A1 39 C8 A2 4B 20 76 7A ");

    let mut recovered = String::new();
    taes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    println!("B =\t{}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn bigcryptor128_basic_operation_main()
{
    bigcryptor128_new();
    bigcryptor128_new_with_small_cryptor_array();
    bigcryptor128_new_with_small_cryptor_vec();
    bigcryptor128_push_small_cryptor();
    bigcryptor128_push_small_cryptor_array();
    bigcryptor128_push_small_cryptor_vec();
    bigcryptor128_encrypt_u128();
    bigcryptor128_decrypt_u128();
    bigcryptor128_encrypt_array_u128();
    bigcryptor128_decrypt_array_u128();
    bigcryptor128_is_successful();
    bigcryptor128_is_failed();
}

fn bigcryptor128_new()
{
    println!("bigcryptor128_new()");
    use cryptocol::symmetric::{ BigCryptor128, AES_128 };

    // Case 1
    let mut taes = BigCryptor128::new();
    taes.push_small_cryptor(AES_128::encryptor_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]));
    taes.push_small_cryptor(AES_128::decryptor_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]));
    taes.push_small_cryptor(AES_128::encryptor_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]));

    // Case 2
    let mut _taes = AES_128::encryptor_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                    + AES_128::decryptor_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                    + AES_128::encryptor_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    println!("-------------------------------");
}

fn bigcryptor128_new_with_small_cryptor_array()
{
    println!("bigcryptor128_new_with_small_cryptor_array()");
    use cryptocol::symmetric::{ BigCryptor128, SmallCryptor, AES_128 };

    // Case 1
    let cryptors: [Box<dyn SmallCryptor<u128, 16>>; 3] = [ Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)),
                                            Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
                                            Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)) ];
    let mut _taes = BigCryptor128::new_with_small_cryptor_array(cryptors);
    println!("-------------------------------");
}

fn bigcryptor128_new_with_small_cryptor_vec()
{
    println!("bigcryptor128_new_with_small_cryptor_vec()");
    use cryptocol::symmetric::{ BigCryptor128, SmallCryptor, AES_128 };

    // Case 1
    let cryptors: Vec<Box<dyn SmallCryptor<u128, 16>>> = vec![ Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)),
                                            Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
                                            Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)) ];
    let mut _taes = BigCryptor128::new_with_small_cryptor_vec(cryptors);
    println!("-------------------------------");
}

fn bigcryptor128_push_small_cryptor()
{
    println!("bigcryptor128_new_with_small_cryptor_vec()");
    use cryptocol::symmetric::{ BigCryptor128, AES_128 };

    // Case 1
    let mut taes = BigCryptor128::new();
    let aes1 = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    let aes2 = AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128);
    let aes3 = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    taes.push_small_cryptor(aes1);
    taes.push_small_cryptor(aes2);
    taes.push_small_cryptor(aes3);
    println!("-------------------------------");
}

fn bigcryptor128_push_small_cryptor_array()
{
    println!("bigcryptor128_push_small_cryptor_array()");
    use cryptocol::symmetric::{ BigCryptor128, SmallCryptor, AES_128 };

    // Case 1
    let mut taes = BigCryptor128::new();
    let cryptors: [Box<dyn SmallCryptor<u128, 16>>; 3] = [ Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)),
                                            Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
                                            Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)) ];
    taes.push_small_cryptor_array(cryptors);
    println!("-------------------------------");
}

fn bigcryptor128_push_small_cryptor_vec()
{
    println!("bigcryptor128_push_small_cryptor_vec()");
    use cryptocol::symmetric::{ BigCryptor128, SmallCryptor, AES_128 };

    // Case 1
    let mut taes = BigCryptor128::new();
    let cryptors: Vec<Box<dyn SmallCryptor<u128, 16>>> = vec![ Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)),
                                            Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
                                            Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)) ];
    taes.push_small_cryptor_vec(cryptors);
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_u128()
{
    println!("bigcryptor128_encrypt_u128()");
    use cryptocol::symmetric::{ BigCryptor128, AES_128 };

    // Case 1
    let mut taes = BigCryptor128::new()
                                + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    let message = 0x_1234567890ABCDEFFEDCBA0987654321_u128;
    println!("M = {:#034X}", message);
    let cipher = taes.encrypt_u128(message);
    println!("C = {:#034X}", cipher);
    assert_eq!(cipher, 0x_965C637ECAC29A9B0BE3F62C9593C04C_u128);
    println!("-------------------------------");
}

fn bigcryptor128_decrypt_u128()
{
    println!("bigcryptor128_decrypt_u128()");
    use cryptocol::symmetric::{ BigCryptor128, AES_128 };

    // Case 1
    let mut taes = BigCryptor128::new()
                                + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    let message = 0x_1234567890ABCDEFFEDCBA0987654321_u128;
    println!("M = {:#034X}", message);
    let cipher = taes.encrypt_u128(message);
    println!("C = {:#034X}", cipher);
    assert_eq!(cipher, 0x_965C637ECAC29A9B0BE3F62C9593C04C_u128);

    let recovered = taes.decrypt_u128(cipher);
    println!("B = {:#034X}", recovered);
    assert_eq!(recovered, 0x_1234567890ABCDEFFEDCBA0987654321_u128);
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn bigcryptor128_encrypt_array_u128()
{
    println!("bigcryptor128_encrypt_array_u128()");
    use cryptocol::symmetric::{ BigCryptor128, AES_128 };

    // Case 1
    let mut taes = BigCryptor128::new()
                                + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    let message = [0x_1234567890ABCDEFFEDCBA0987654321_u128, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211_u128];
    print!("M = ");
    for msg in message.clone()
        { print!("{:#034X} ", msg); }
    println!();

    let mut cipher = [0_u128; 3];
    taes.encrypt_array_u128(&message, &mut cipher);
    print!("C = ");
    for c in cipher.clone()
        { print!("{:#034X} ", c); }
    println!();
    assert_eq!(cipher[0], 0x_965C637ECAC29A9B0BE3F62C9593C04C_u128);
    assert_eq!(cipher[1], 0x_A397AABE9537829FABA0596B5D3EA8B9_u128);
    assert_eq!(cipher[2], 0x_85457798D08431CCB8A4A58517A5D452_u128);
    println!("-------------------------------");
}

fn bigcryptor128_decrypt_array_u128()
{
    println!("bigcryptor128_decrypt_array_u128()");
    use cryptocol::symmetric::{ BigCryptor128, AES_128 };

    // Case 1
    let mut taes = BigCryptor128::new()
                                + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                                - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                                + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    let message = [0x_1234567890ABCDEFFEDCBA0987654321_u128, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211];
    print!("M = ");
    for msg in message.clone()
        { print!("{:#034X} ", msg); }
    println!();

    let mut cipher = [0_u128; 3];
    taes.encrypt_array_u128(&message, &mut cipher);
    print!("C = ");
    for c in cipher.clone()
        { print!("{:#034X} ", c); }
    println!();
    assert_eq!(cipher[0], 0x_965C637ECAC29A9B0BE3F62C9593C04C_u128);
    assert_eq!(cipher[1], 0x_A397AABE9537829FABA0596B5D3EA8B9_u128);
    assert_eq!(cipher[2], 0x_85457798D08431CCB8A4A58517A5D452_u128);

    let mut recovered = [0_u128; 3];
    taes.decrypt_array_u128(&cipher, &mut recovered);
    print!("B = ");
    for r in recovered.clone()
        { print!("{:#034X} ", r); }
    println!();
    assert_eq!(recovered[0], 0x_1234567890ABCDEFFEDCBA0987654321_u128);
    assert_eq!(recovered[1], 0x_11223344556677889900AABBCCDDEEFF_u128);
    assert_eq!(recovered[2], 0x_FFEEDDCCBBAA00998877665544332211_u128);
    assert_eq!(recovered[0], message[0]);
    assert_eq!(recovered[1], message[1]);
    assert_eq!(recovered[2], message[2]);
    println!("-------------------------------");
}

fn bigcryptor128_is_successful()
{
    println!("bigcryptor128_is_successful");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CBC_PKCS7 };

    // Successful case for encryption
    let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("IV =	{}", iv);
    let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                    - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                    + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);

    let message = "";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 16];
    let len = taes.encrypt_str_into_array(iv, &message, &mut cipher);
    println!("The length of ciphertext = {}", len);
    assert_eq!(len, 16);
    let success = taes.is_successful();
    assert_eq!(success, true);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "D9 F7 43 4F 83 5D 3E 70 1F CD A1 4A 49 C1 78 83 ");
    println!();

    // Successful case for decryption
    let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("IV =	{}", iv);
    let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                    - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                    + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);

    let cipher = [0xD9_u8, 0xF7, 0x43, 0x4F, 0x83, 0x5D, 0x3E, 0x70, 0x1F, 0xCD, 0xA1, 0x4A, 0x49, 0xC1, 0x78, 0x83];
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut recovered = [0u8; 16];
    let len = taes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    println!("The length of plaintext = {}", len);
    assert_eq!(len, 0);
    let success = taes.is_successful();
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


    // Failure case for encryption
    let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("IV =	{}", iv);
    let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                    - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                    + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);

    let message = "";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 4];
    let len = taes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    println!("The length of ciphertext = {}", len);
    assert_eq!(len, 0);
    let success = taes.is_successful();
    assert_eq!(success, false);
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "00 00 00 00 ");
    println!();

    // Failure case for decryption
    let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("IV =	{}", iv);
    let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                    - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                    + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);

    let cipher = [0x17_u8, 0xC8, 0x15, 0x48];
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut recovered = [0u8; 16];
    let len = taes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    println!("The length of plaintext = {}", len);
    assert_eq!(len, 0);
    let success = taes.is_successful();
    assert_eq!(success, false);
    println!("-------------------------------");
}

fn bigcryptor128_is_failed()
{
    println!("bigcryptor128_is_failed");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ BigCryptor128, AES_128, CBC_PKCS7 };

    // Successful case for encryption
    let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("IV =	{}", iv);
    let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                    - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                    + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);

    let message = "";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 16];
    let len = taes.encrypt_str_into_array(iv, &message, &mut cipher);
    println!("The length of ciphertext = {}", len);
    assert_eq!(len, 16);
    let failure = taes.is_failed();
    assert_eq!(failure, false);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "D9 F7 43 4F 83 5D 3E 70 1F CD A1 4A 49 C1 78 83 ");
    println!();

    // Successful case for decryption
    let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("IV =	{}", iv);
    let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                    - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                    + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);

    let cipher = [0xD9_u8, 0xF7, 0x43, 0x4F, 0x83, 0x5D, 0x3E, 0x70, 0x1F, 0xCD, 0xA1, 0x4A, 0x49, 0xC1, 0x78, 0x83];
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut recovered = [0u8; 16];
    let len = taes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    println!("The length of plaintext = {}", len);
    assert_eq!(len, 0);
    let failure = taes.is_failed();
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


    // Failure case for encryption
    let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("IV =	{}", iv);
    let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                    - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                    + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);

    let message = "";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 4];
    let len = taes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    println!("The length of ciphertext = {}", len);
    assert_eq!(len, 0);
    let failure = taes.is_failed();
    assert_eq!(failure, true);
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "00 00 00 00 ");
    println!();

    // Failure case for decryption
    let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    println!("IV =	{}", iv);
    let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
                    - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
                    + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);

    let cipher = [0x17_u8, 0xC8, 0x15, 0x48];
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();

    let mut recovered = [0u8; 16];
    let len = taes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    println!("The length of plaintext = {}", len);
    assert_eq!(len, 0);
    let failure = taes.is_failed();
    assert_eq!(failure, true);
    println!("-------------------------------");
}
