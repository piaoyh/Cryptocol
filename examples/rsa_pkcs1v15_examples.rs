// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]
// #[allow(non_camel_case_types)]
// #[allow(non_snake_case)]


pub fn main()
{
    rsa_encrypt();
    rsa_decrypt();
}

fn rsa_encrypt()
{
    println!("rsa_encrypt");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::asymmetric::PKCS1V15;
    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;
    let mut rsa = RSA_1024::new();
    rsa.find_keys();
    println!("Private Key: {}", rsa.get_private_key());
    println!("Public Key: {}", rsa.get_public_key());
    println!("Product value: {}", rsa.get_number());
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 128];
    rsa.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    println!();

    println!("-------------------------------");
}

fn rsa_decrypt()
{
    println!("rsa_decrypt");
    use std::io::Write;
    use std::fmt::Write as _;
    use cryptocol::asymmetric::PKCS1V15;
    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;
    let mut rsa = RSA_1024::new();
    rsa.find_keys();
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 128];
    rsa.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { write!(txt, "{:02X} ", c); }
    assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    println!();

    let mut recovered = vec![0; 117];
    rsa.decrypt(cipher.as_ptr(), recovered.as_mut_ptr());
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
    println!("-------------------------------");
}