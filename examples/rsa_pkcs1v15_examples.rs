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

    rsa_same_primes_test();
    rsa_composites_test();
    rsa_composite_prime_test();
}

fn rsa_encrypt()
{
    println!("rsa_encrypt");
    use std::fmt::Write;
    use cryptocol::asymmetric::PKCS1V15;
    
    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut rsa = RSA_1024::new_with_prepared_keys();
    println!("Private Key: {}", rsa.get_private_key());
    println!("Public Key: {}", rsa.get_public_key());
    println!("Product value: {}", rsa.get_modulus());
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
    use cryptocol::asymmetric::PKCS1V15;
    
    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);

    // Generates an RSA object
    fn generate() -> RSA_1024
    {
        let prime1 = U512::from_str_radix("950BE5031347033FAD37E4AA8FBA7B9687432E00C8D5E7829B0366B5E602FB308513C315D751E9F704127BFAD3995001765A47BD45C213E3CE22E5142C279F39", 16).unwrap();
        let prime2 = U512::from_str_radix("C820C641F488C32070384A6352B341734028B1911699D15920B9727EF9AAE80669D5AE563A440346662EC10C8C83FB0F5739B0167809B5889AF40C37506126BB", 16).unwrap();
        RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    }

    // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    // 2. Encrypts the certain AES key by RSA to get the encrypted certain AES.
    // 3. Sends the encrypted certain AES key and the ciphertext.
    fn send(public_key: U1024, modulus: U1024, message: &str) -> ([u8; 128], Vec<u8>)
    {
        use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
        let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
        println!("Public Key: {}", rsa.get_public_key());
        println!("Product value: {}", rsa.get_modulus());
        
        let mut key = [0u8; 16];
        for i in 0u8..16
            { key[i as usize] = i; }
        let mut aes = AES_128::new_with_key(&key);
        let mut cipher = Vec::new();
        aes.encrypt_str_into_vec(message, &mut cipher);
        let mut encrypted_key = [0u8; 128];
        rsa.encrypt(&key as *const u8, 16, &mut encrypted_key as *mut u8);
        (encrypted_key, cipher)
    }

    // 1. Receives the encrypted AES key and the ciphertext.
    // 2. Decrypts the encrypted AES key by RSA to get the original AES key.
    // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    fn recv(mut rsa: RSA_1024, encrypted_key: [u8; 128], cipher: &Vec<u8>) -> String
    {
        use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };

        print!("en_key = ");
        for i in 0..128
            { print!("{}", encrypted_key[i]); }
        println!();
        let mut key = [0; 16];
        let n = rsa.decrypt(encrypted_key.as_ptr(), key.as_mut_ptr());
        println!("n = {n}");
        print!("key = ");
        for i in 0..16
            { print!("{:X}", key[i]); }
        println!();
        let mut aes = AES_128::new_with_key(&key);
        let mut recovered = String::new();
        aes.decrypt_vec_into_string(&cipher, &mut recovered);
        println!("Recovered =\t{}", recovered);
        recovered
    }

    let rsa = generate();
    let (encrypted_key, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    let recovered = recv(rsa, encrypted_key, &cipher);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn rsa_decrypt_into_string()
{
    println!("rsa_decrypt");
    use std::fmt::Write;
    use cryptocol::asymmetric::PKCS1V15;
    
    // Example for RSA_1024
    use cryptocol::define_utypes_with;
    use cryptocol::asymmetric::RSA_1024;
    define_utypes_with!(u32);

    let prime1 = U512::from_str_radix("950BE5031347033FAD37E4AA8FBA7B9687432E00C8D5E7829B0366B5E602FB308513C315D751E9F704127BFAD3995001765A47BD45C213E3CE22E5142C279F39", 16).unwrap();
    let prime2 = U512::from_str_radix("C820C641F488C32070384A6352B341734028B1911699D15920B9727EF9AAE80669D5AE563A440346662EC10C8C83FB0F5739B0167809B5889AF40C37506126BB", 16).unwrap();
    let mut rsa = RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
    println!("Private Key: {}", rsa.get_private_key());
    println!("Public Key: {}", rsa.get_public_key());
    println!("Product value: {}", rsa.get_modulus());
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
    println!("-------------------------------");
}

fn rsa_same_primes_test()
{
    println!("rsa_same_primes_test");
    use std::fmt::Write;
    use cryptocol::number::BigUInt;
    use cryptocol::define_utypes_with;
    use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    define_utypes_with!(u32);

    // Example for two same prime numbers.
    let prime1 = BigUInt::<u32, 32>::from_str_radix("b1bbabfc84567b1a2cf004a81bcae9582cadc6c3de2b498778fec2ee7006c1b70a3363863626750243a94930d0538b7fd4f274b033ba021be777d3bea33ab289", 16).unwrap();
    let prime2 = prime1.clone();
    let mut rsa = RSA_1024::new_with_primes(prime1, prime2);
    println!("Private Key: {}", rsa.get_private_key());
    println!("Public Key: {}", rsa.get_public_key());
    println!("Product value: {}", rsa.get_modulus());
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 128];
    rsa.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { let _= write!(txt, "{:02X} ", c); }
    println!();

    let mut recovered = Vec::<u8>::new();
    rsa.decrypt_array_into_vec(&cipher, &mut recovered);
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
    // assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    // assert_eq!(converted, message);
    println!("-------------------------------");
}

fn rsa_composites_test()
{
    println!("rsa_composites_test");
    use std::fmt::Write;
    use cryptocol::number::BigUInt;
    use cryptocol::define_utypes_with;
    use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    define_utypes_with!(u32);

    // Example for two same prime numbers.
    let prime1 = BigUInt::<u32, 32>::from_str_radix("b1bbabfc84567b1a2cf004a81bcae9582cadc6c3de2b498778fec2ee7006c1b70a3363863626750243a94930d0538b7fd4f274b033ba021be777d3bea33ab283", 16).unwrap();
    let prime2 = BigUInt::<u32, 32>::from_str_radix("cd6e2bda2d076f711d812621a0cc0e26274e93f4b4b815f27b0663d063466b2e190ffc4caad0f6feb4710fede0a1d853f72dd170e7e94768d531bbefdf84bb70", 16).unwrap();
    let mut rsa = RSA_1024::new_with_primes(prime1, prime2);
    println!("Private Key: {}", rsa.get_private_key());
    println!("Public Key: {}", rsa.get_public_key());
    println!("Product value: {}", rsa.get_modulus());
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 128];
    rsa.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { let _= write!(txt, "{:02X} ", c); }
    println!();

    let mut recovered = Vec::<u8>::new();
    rsa.decrypt_array_into_vec(&cipher, &mut recovered);
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
    // assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    // assert_eq!(converted, message);
    println!("-------------------------------");
}

fn rsa_composite_prime_test()
{
    println!("rsa_composite_prime_test");
    use std::fmt::Write;
    use cryptocol::number::BigUInt;
    use cryptocol::define_utypes_with;
    use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    define_utypes_with!(u32);

    // Example for two same prime numbers.
    let prime1 = BigUInt::<u32, 32>::from_str_radix("b1bbabfc84567b1a2cf004a81bcae9582cadc6c3de2b498778fec2ee7006c1b70a3363863626750243a94930d0538b7fd4f274b033ba021be777d3bea33ab283", 16).unwrap();
    let prime2 = BigUInt::<u32, 32>::from_str_radix("cd6e2bda2d076f711d812621a0cc0e26274e93f4b4b815f27b0663d063466b2e190ffc4caad0f6feb4710fede0a1d853f72dd170e7e94768d531bbefdf84bb75", 16).unwrap();
    let mut rsa = RSA_1024::new_with_primes(prime1, prime2);
    println!("Private Key: {}", rsa.get_private_key());
    println!("Public Key: {}", rsa.get_public_key());
    println!("Product value: {}", rsa.get_modulus());
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 128];
    rsa.encrypt_str_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();
    let mut txt = String::new();
    for c in cipher.clone()
        { let _= write!(txt, "{:02X} ", c); }
    println!();

    let mut recovered = Vec::<u8>::new();
    rsa.decrypt_array_into_vec(&cipher, &mut recovered);
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
    // assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    // assert_eq!(converted, message);
    println!("-------------------------------");
}