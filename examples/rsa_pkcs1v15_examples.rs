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

use cryptocol::number::BigUInt;


pub fn main()
{
    rsa_encrypt();
    rsa_decrypt();
}

fn rsa_encrypt()
{
    println!("rsa_encrypt");
    use std::fmt::Write;
    use cryptocol::asymmetric::PKCS1V15;
    
    // Example for RSA_1024
    use cryptocol::number::BigUInt;
    use cryptocol::define_utypes_with;
    use cryptocol::asymmetric::RSA_1024;

    define_utypes_with!(u32);
    let prime1 = BigUInt::<u32, 32>::from_str_radix("b1bbabfc84567b1a2cf004a81bcae9582cadc6c3de2b498778fec2ee7006c1b70a3363863626750243a94930d0538b7fd4f274b033ba021be777d3bea33ab289", 16).unwrap();
    let prime2 = BigUInt::<u32, 32>::from_str_radix("cd6e2bda2d076f711d812621a0cc0e26274e93f4b4b815f27b0663d063466b2e190ffc4caad0f6feb4710fede0a1d853f72dd170e7e94768d531bbefdf84bb75", 16).unwrap();
    let mut rsa = RSA_1024::new_with_primes(prime1, prime2);
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
        { let _= write!(txt, "{:02X} ", c); }
    println!();

    println!("-------------------------------");
}

fn rsa_decrypt()
{
    println!("rsa_decrypt");
    use std::fmt::Write;
    use cryptocol::asymmetric::PKCS1V15;
    // Example for RSA_1024
    use cryptocol::number::BigUInt;
    use cryptocol::define_utypes_with;
    use cryptocol::asymmetric::RSA_1024;

    define_utypes_with!(u32);
    let prime1 = BigUInt::<u32, 32>::from_str_radix("b1bbabfc84567b1a2cf004a81bcae9582cadc6c3de2b498778fec2ee7006c1b70a3363863626750243a94930d0538b7fd4f274b033ba021be777d3bea33ab289", 16).unwrap();
    let prime2 = BigUInt::<u32, 32>::from_str_radix("cd6e2bda2d076f711d812621a0cc0e26274e93f4b4b815f27b0663d063466b2e190ffc4caad0f6feb4710fede0a1d853f72dd170e7e94768d531bbefdf84bb75", 16).unwrap();
    let mut rsa = RSA_1024::new_with_primes(prime1, prime2);
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
        { let _= write!(txt, "{:02X} ", c); }
    println!();

    let mut recovered = vec![0; 55];
    rsa.decrypt(cipher.as_ptr(), recovered.as_mut_ptr());
    print!("Ba =\t");
    for b in recovered.clone()
        { print!("{:02X} ", b); }
    println!();
    let mut txt = String::new();
    for c in recovered.clone()
        { let _ = write!(txt, "{:02X} ", c); }

    let mut converted = String::new();
    unsafe { converted.as_mut_vec() }.append(&mut recovered);
    
    println!("Bb =\t{}", converted);
    assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    assert_eq!(converted, message);
    println!();
    println!("-------------------------------");
}