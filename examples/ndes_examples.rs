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


pub fn main()
{
    ndes_quick_start_main();
    ndes_basic_operation_main();
}

fn ndes_quick_start_main()
{
    ndes_quick_start_instantiation_with_keys();
}

fn ndes_quick_start_instantiation_with_keys()
{
    println!("des_quick_start_instantiation_with_keys()");
    use cryptocol::symmetric::{ NDES, SmallDES };
    println!("-------------------------------");
}


fn ndes_basic_operation_main()
{
    ndes_new_with_keys();
    ndes_new_with_keys_u64();
}

fn ndes_new_with_keys()
{
    println!("ndes_new_with_keys()");
    use cryptocol::symmetric::{ NDES, DES };

    let keys = [DES::encryptor_with_key_u64(0x1234567890ABCDEF_u64),
                DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64),
                DES::encryptor_with_key_u64(0x1234567890ABCDEF_u64)];
    let mut tdes = NDES::new_with_small_des_array(keys);
    let plaintext = 0x1234567890ABCDEF_u64;
    let ciphertext = tdes.encrypt_u64(plaintext);

    println!("Plaintext:\t\t{:#016X}", plaintext);
    println!("Ciphertext:\t\t{:#016X}", ciphertext);
    assert_eq!(ciphertext, 0x272A2AC7B4E66748_u64);

    let cipher_cipher_text = tdes.decrypt_u64(ciphertext);
    println!("Cipher-ciphertext:\t{:#016X}", cipher_cipher_text);
    assert_eq!(cipher_cipher_text, 0x1234567890ABCDEF_u64);
    assert_eq!(cipher_cipher_text, plaintext);  // So, you can't use the default key!!!
    println!("-------------------------------");
}

fn ndes_new_with_keys_u64()
{
    println!("ndes_new_with_keys_u64()");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::symmetric::{ NDES, DES };

    let keys = [DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64),
                DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64),
                DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)];
    let mut tdes = NDES::new_with_small_des_array(keys);

    let message = "I am OK.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 16];
    tdes.encrypt_with_padding_pkcs7(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "88 89 99 44 30 72 CA 2F 22 4F 7C E0 55 FA 28 C3 ");
    println!();

    let mut recovered = vec![0; 16];
    let len = tdes.decrypt_with_padding_pkcs7(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "49 20 61 6D 20 4F 4B 2E 00 00 00 00 00 00 00 00 ");

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    converted.truncate(len as usize);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "I am OK.");
    assert_eq!(converted, message);
    println!();

    println!("-------------------------------");
}
