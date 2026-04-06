// Copyright 2025, 2026 PARK Youngho.
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

use cryptocol::random::Slapdash_DES;




pub fn main()
{
    rsa_set_prng();
    rsa_encrypt_string();
    rsa_encrypt_string_into_vec();
    rsa_encrypt_string_into_array();
    rsa_decrypt();
    rsa_decrypt_into_vec();
    rsa_decrypt_into_array();
    rsa_decrypt_into_string();
    rsa_decrypt_vec();
    rsa_decrypt_vec_into_vec();
    rsa_decrypt_vec_into_array();
    rsa_decrypt_vec_into_string();
    rsa_decrypt_array();
    rsa_decrypt_array_into_vec();
    rsa_decrypt_array_into_array();
    rsa_decrypt_array_into_string();

    rsa_same_primes_test();
    rsa_composites_test();
    rsa_composite_prime_test();
    prime_test();
}

fn rsa_set_prng()
{
    println!("rsa_set_prng");
    use cryptocol::random::Random;
    use cryptocol::asymmetric::{ RSA_1024, PKCS1V15 };
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let prng = Slapdash_DES::new();
    let mut rsa = RSA_1024::new();
    rsa.set_prng(prng);
    println!("-------------------------------");
}

fn rsa_encrypt_string()
{
    println!("rsa_encrypt");
    use cryptocol::asymmetric::PKCS1V15;
    
    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let prime1 = U512::from_str_radix("C69D50BE9165760F2218B04D870B1C797E8CF9071548465735E20DD240B50AAF28EFE285E826E989836D3674BDAAD4C18BC0F6C697EA5879CEC372A867073D3B", 16).unwrap();
    let prime2 = U512::from_str_radix("921907C5F8773D1922DE00B302C0757DA8136DE1945D02F6386D361CA312FF4A005933C263B96BD68A11DA07D94DD4EDA6412956CF8C51F355945097EF88D4C1", 16).unwrap();
    let mut rsa = RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
    println!("Private Key: {}", rsa.get_private_key());
    println!("Public Key: {}", rsa.get_public_key());
    println!("Product value: {}", rsa.get_modulus());
    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 128];
    rsa.encrypt_string(&message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();

    let mut recovered = String::new();
    rsa.decrypt_into_string(cipher.as_ptr(), &mut recovered);
    println!("Recovered =\t {}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn rsa_encrypt_string_into_vec()
{
    println!("rsa_encrypt_string_into_vec");
    use cryptocol::asymmetric::PKCS1V15;
    
    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let prime1 = U512::from_str_radix("C22E75B2ECE32673042DA138C285720DEE2EAEF521D08E1E3FD3BC50F7D5B8CCB1C2B7FD7A8B9B8A233139B27CC94D7444E84E5F08C92E02DE01FC426E8B0EDF", 16).unwrap();
    let prime2 = U512::from_str_radix("EA0752E2BB207AFEF136323991B8BE152FDEB579C07A73999D0D6F20F3836301F2F880007D28356DACCE3E032EB8F64450EB97AAB4D912AFC90919C7E205A45B", 16).unwrap();
    let mut rsa = RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
    println!("Private Key: {}", rsa.get_private_key());
    println!("Public Key: {}", rsa.get_public_key());
    println!("Product value: {}", rsa.get_modulus());
    let message = String::from("In the beginning God created the heavens and the earth.");
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    rsa.encrypt_string_into_vec(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();

    let mut recovered = String::new();
    rsa.decrypt_vec_into_string(&cipher, &mut recovered);
    println!("Recovered =\t {}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn rsa_encrypt_string_into_array()
{
    println!("rsa_encrypt_string_into_array");
    use cryptocol::asymmetric::PKCS1V15;
    
    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let prime1 = U512::from_str_radix("EB40B5394E5BDA1BA07FDCAAE0A9B72A855B5C60144A45E0411B43CA7E1DD818E099A9B758DBBF71F5D91C62CD5C9B3A2083BAF67A34F5A3BBB2AADA4FFBCF05", 16).unwrap();
    let prime2 = U512::from_str_radix("BB4A66D84474FCC903BF67EF90F455D1C5B1C93B5A9A80D819486C533871940DEB2AB38A9E43733B99219E479CC9C86F8951600ABD95CD305ABEC4325DDC062B", 16).unwrap();
    let mut rsa = RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
    println!("Private Key: {}", rsa.get_private_key());
    println!("Public Key: {}", rsa.get_public_key());
    println!("Product value: {}", rsa.get_modulus());
    let message = "In the beginning God created the heavens and the earth.".to_string();
    println!("M =\t{}", message);
    let mut cipher = [0u8; 128];
    rsa.encrypt_string_into_array(&message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();

    let mut recovered = String::new();
    rsa.decrypt_array_into_string(&cipher, &mut recovered);
    println!("Recovered =\t {}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
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
        let prime1 = U512::from_str_radix("D70BD5CA4EA8EC514D116988AC3A9C8CF5545B511A2B414F3D6D293B788A6FE264D90D426F681E672235DF0EA0D56BE95C9F992D9ABEF6CD5F370EAF86B0E871", 16).unwrap();
        let prime2 = U512::from_str_radix("D888E920751043B12A95466E0FCB07CE16519BDF0F953547FC37E5E09810E1B7D5D76375A6BD0D9F75C216E616D40ED5C490E7A386FD24594CC1705C14C9E4D7", 16).unwrap();
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
    fn recv(mut rsa: RSA_1024, encrypted_key: [u8; 128], cipher: Vec<u8>) -> String
    {
        use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };

        print!("Encrypted key = ");
        for i in 0..128
            { print!("{}", encrypted_key[i]); }
        println!();
        let mut key = [0; 16];
        rsa.decrypt(encrypted_key.as_ptr(), key.as_mut_ptr());
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
    let recovered = recv(rsa, encrypted_key, cipher);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn rsa_decrypt_into_vec()
{
    println!("rsa_decrypt_into_vec");
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
        let prime1 = U512::from_str_radix("F00C8AEBB56F5F37FA3A26689DA44E6AF45AC2E6561B0C30E5D6CC0C4ADEFEC86D272A6358D82379EFF88FEED21177E62E06BF68557C2CA0ECD387744A1A8D2D", 16).unwrap();
        let prime2 = U512::from_str_radix("F834AFC1753260793D0AFC137354D4A8696605701D4CA995AF6088AB21036A8789174D3D14A8C64EDBB93ED599F701911B8D245530B06A2A68DF6904ABB1AA01", 16).unwrap();
        RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    }

    // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    // 2. Encrypts the certain AES key by RSA to get the encrypted certain AES.
    // 3. Sends the encrypted certain AES key and the ciphertext.
    fn send(public_key: U1024, modulus: U1024, message: &str) -> ([u8; 128], Vec<u8>)
    {
        use cryptocol::symmetric::{ AES_128, ECB_ISO };
        let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
        println!("Public Key: {}", rsa.get_public_key());
        println!("Product value: {}", rsa.get_modulus());
        
        let mut key = Vec::<u8>::new();
        for i in 0u8..16
            { key.push(i); }
        let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
        let mut cipher = Vec::new();
        aes.encrypt_str_into_vec(message, &mut cipher);
        let mut encrypted_key = [0u8; 128];
        rsa.encrypt_vec(&key, &mut encrypted_key as *mut u8);
        (encrypted_key, cipher)
    }

    // 1. Receives the encrypted AES key and the ciphertext.
    // 2. Decrypts the encrypted AES key by RSA to get the original AES key.
    // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    fn recv(mut rsa: RSA_1024, encrypted_key: [u8; 128], cipher: Vec<u8>) -> String
    {
        use cryptocol::symmetric::{ AES_128, ECB_ISO };

        print!("Encrypted key = ");
        for i in 0..128
            { print!("{}", encrypted_key[i]); }
        println!();
        let mut key = Vec::<u8>::new();
        rsa.decrypt_into_vec(encrypted_key.as_ptr(), &mut key);
        print!("key = ");
        for k in key.clone()
            { print!("{:X}", k); }
        println!();
        let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
        let mut recovered = String::new();
        aes.decrypt_vec_into_string(&cipher, &mut recovered);
        println!("Recovered =\t{}", recovered);
        recovered
    }

    let rsa = generate();
    let (encrypted_key, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    let recovered = recv(rsa, encrypted_key, cipher);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn rsa_decrypt_into_array()
{
    println!("rsa_decrypt_into_array");
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
        let prime1 = U512::from_str_radix("AC3108622D6FB87127289FDF4A996EE10A6F32E4134979275EBB8F358300DE758B91334F6D0FF697773E18927332FFC31537EB746C00EEA0D0BE1C87BF4A78AD", 16).unwrap();
        let prime2 = U512::from_str_radix("FA5FD6A3ACC44C6FA488CBF657F5256582E9C1B539F8BA3730E51DDF0156CDB34C1F9A6DFE2DCD443368EAB8804092BE5C4CBEF3DD28AEEC58D7D8A55C5C2CEB", 16).unwrap();
        RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    }

    // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    // 2. Encrypts the certain AES key and IV by RSA to get the encrypted certain AES key and encrypted IV.
    // 3. Sends the encrypted certain AES key, IV and the ciphertext.
    fn send(public_key: U1024, modulus: U1024, message: &str) -> ([u8; 128], [u8; 128], Vec<u8>)
    {
        use cryptocol::symmetric::{ AES_128, CBC_PKCS7 };
        use cryptocol::number::SharedArrays;
        let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
        println!("Public Key: {}", rsa.get_public_key());
        println!("Product value: {}", rsa.get_modulus());
        
        let mut key = [0u8; 16];
        for i in 0u8..16
            { key[i as usize] = i; }
        let mut iv = [0u8; 16];
        iv[0] = 1;
        iv[1] = 2;
        for i in 2..16
            { iv[i] = iv[i-1].wrapping_add(iv[i-2]); }
        let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
        let mut aes = AES_128::new_with_key(&key);
        let mut cipher = Vec::new();
        aes.encrypt_str_into_vec(iv.clone(), message, &mut cipher);
        let mut encrypted_key = [0u8; 128];
        rsa.encrypt_array(&key, &mut encrypted_key as *mut u8);
        let mut encrypted_iv= [0u8; 128];
        rsa.encrypt_array(&iv, &mut encrypted_iv as *mut u8);

        (encrypted_key, encrypted_iv, cipher)
    }

    // 1. Receives the encrypted AES key, IV and the ciphertext.
    // 2. Decrypts the encrypted AES key and encrypted IV by RSA to get the original AES key and IV.
    // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    fn recv(mut rsa: RSA_1024, encrypted_key: [u8; 128], encrypted_iv: [u8; 128], cipher: Vec<u8>) -> String
    {
        use cryptocol::symmetric::{ AES_128, CBC_PKCS7 };
        use cryptocol::number::SharedArrays;

        print!("Encrypted key = ");
        for i in 0..128
            { print!("{}", encrypted_key[i]); }
        println!();
        let mut key = [0u8; 16];
        rsa.decrypt_into_array(encrypted_key.as_ptr(), &mut key);
        print!("key = ");
        for i in 0_usize..16
            { print!("{:X}", key[i]); }
        println!();
        let mut iv = [0u8; 16];
        rsa.decrypt_into_array(encrypted_iv.as_ptr(), &mut iv);

        print!("iv = ");
        for i in 0_usize..16
            { print!("{:X}", iv[i]); }
        println!();
        let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
        let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
        let mut recovered = String::new();
        aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
        println!("Recovered =\t{}", recovered);
        recovered
    }

    let rsa = generate();
    let (encrypted_key, encrypted_iv, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    let recovered = recv(rsa, encrypted_key, encrypted_iv, cipher);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");

}

fn rsa_decrypt_into_string()
{
    println!("rsa_decrypt_into_string");
    use cryptocol::asymmetric::PKCS1V15;
    
    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let prime1 = U512::from_str_radix("B7E494B40672CA779511317A9AD1E3707FEC6B62CD4E8E453702FC20B7B09CE9FC6C7B398E42EBC2F9B2F01CC90247259DABDA191E36A7681268503DC70219A9", 16).unwrap();
    let prime2 = U512::from_str_radix("EDD1578DEA65D27459F67FF8A7546455C5327DB7A2AEBA51BC4F06843DF93E8CECCF95E92830065FF52833330085285C49B7C2A9AAB3A9F961120612F3D5B0AD", 16).unwrap();
    let mut rsa = RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
    println!("Private Key: {}", rsa.get_private_key());
    println!("Public Key: {}", rsa.get_public_key());
    println!("Product value: {}", rsa.get_modulus());
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0_u8; 128];
    rsa.encrypt_str(message, cipher.as_mut_ptr());
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();

    let mut recovered = String::new();
    rsa.decrypt_into_string(cipher.as_ptr(), &mut recovered);
    println!("Recovered =\t {}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn rsa_decrypt_vec()
{
    println!("rsa_decrypt_vec");
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
        let prime1 = U512::from_str_radix("F00C8AEBB56F5F37FA3A26689DA44E6AF45AC2E6561B0C30E5D6CC0C4ADEFEC86D272A6358D82379EFF88FEED21177E62E06BF68557C2CA0ECD387744A1A8D2D", 16).unwrap();
        let prime2 = U512::from_str_radix("F834AFC1753260793D0AFC137354D4A8696605701D4CA995AF6088AB21036A8789174D3D14A8C64EDBB93ED599F701911B8D245530B06A2A68DF6904ABB1AA01", 16).unwrap();
        RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    }

    // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    // 2. Encrypts the certain AES key and IV by RSA to get the encrypted certain AES key and encrypted IV.
    // 3. Sends the encrypted certain AES key, IV and the ciphertext.
    fn send(public_key: U1024, modulus: U1024, message: &str) -> (Vec<u8>, Vec<u8>, Vec<u8>)
    {
        use cryptocol::symmetric::{ AES_128, CBC_ISO };
        use cryptocol::number::SharedArrays;

        let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
        println!("Public Key: {}", rsa.get_public_key());
        println!("Product value: {}", rsa.get_modulus());
        
        let mut key = Vec::<u8>::new();
        for i in 0u8..16
            { key.push(i); }
        let mut iv = [0u8; 16];
        iv[0] = 13;
        iv[1] = 17;
        for i in 2..16
            { iv[i] = iv[i-1].wrapping_mul(iv[i-2]); }
        let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
        let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
        let mut cipher = Vec::new();
        aes.encrypt_str_into_vec(iv.clone(), message, &mut cipher);
        let mut encrypted_key = Vec::<u8>::new();
        rsa.encrypt_into_vec(key.as_ptr(), key.len() as u64, &mut encrypted_key);
        let mut encrypted_iv= Vec::<u8>::new();
        rsa.encrypt_into_vec(iv.as_ptr() as *const u8, 16, &mut encrypted_iv);
        (encrypted_key, encrypted_iv, cipher)
    }

    // 1. Receives the encrypted AES key, IV and the ciphertext.
    // 2. Decrypts the encrypted AES key and encrypted IV by RSA to get the original AES key and IV.
    // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    fn recv(mut rsa: RSA_1024, encrypted_key: Vec<u8>, encrypted_iv: Vec<u8>, cipher: Vec<u8>) -> String
    {
        use cryptocol::symmetric::{ AES_128, CBC_ISO };
        use cryptocol::number::SharedArrays;

        print!("Encrypted_key = ");
        for i in 0..128
            { print!("{}", encrypted_key[i]); }
        println!();
        let mut key = [0u8; 16];
        rsa.decrypt_vec(&encrypted_key, key.as_mut_ptr());
        print!("key = ");
        for k in key.clone()
            { print!("{:X}", k); }
        println!();
        let mut iv = [0u8; 16];
        rsa.decrypt_vec(&encrypted_iv, iv.as_mut_ptr());
        print!("iv = ");
        for i in 0_usize..16
            { print!("{:X}", iv[i]); }
        println!();
        let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
        let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
        let mut recovered = String::new();
        aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
        println!("Recovered =\t{}", recovered);
        recovered
    }

    let rsa = generate();
    let (encrypted_key, encrypted_iv, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    let recovered = recv(rsa, encrypted_key, encrypted_iv, cipher);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn rsa_decrypt_vec_into_vec()
{
    println!("rsa_decrypt_vec_into_vec");
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
        let prime1 = U512::from_str_radix("EABED4A764F510FC1E5A70DAFCBD67245D61F2B27AA381BA406A8387B805A6AFD144D856A9B7ADE7FDCAED2324C66CB21D565160CEEBF572497C27B6BD1FBCD7", 16).unwrap();
        let prime2 = U512::from_str_radix("8EF1B1D1A88754D1E360D317516E5AE73C617694DB9B36CA6E029B6714537ED09D282AB6601A3A9D6C137503FD7097F7627B20E14CED7CE676734C554F3DB0B9", 16).unwrap();
        RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    }

    // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    // 2. Encrypts the certain AES key and IV by RSA to get the encrypted certain AES key and encrypted IV.
    // 3. Sends the encrypted certain AES key, IV and the ciphertext.
    fn send(public_key: U1024, modulus: U1024, message: &str) -> (Vec<u8>, Vec<u8>, Vec<u8>)
    {
        use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
        use cryptocol::number::SharedArrays;

        let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
        println!("Public Key: {}", rsa.get_public_key());
        println!("Product value: {}", rsa.get_modulus());
        
        let mut key = Vec::<u8>::new();
        for i in 0u8..16
            { key.push(i); }
        let mut iv = [0u8; 16];
        iv[0] = 13;
        iv[1] = 17;
        for i in 2..16
            { iv[i] = iv[i-1].wrapping_mul(iv[i-2]); }
        let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
        let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
        let mut cipher = Vec::new();
        aes.encrypt_str_into_vec(iv.clone(), message, &mut cipher);
        let mut encrypted_key = Vec::<u8>::new();
        rsa.encrypt_vec_into_vec(&key, &mut encrypted_key);
        let mut encrypted_iv= Vec::<u8>::new();
        rsa.encrypt_vec_into_vec(&iv.to_vec(), &mut encrypted_iv);
        (encrypted_key, encrypted_iv, cipher)
    }

    // 1. Receives the encrypted AES key, IV and the ciphertext.
    // 2. Decrypts the encrypted AES key and encrypted IV by RSA to get the original AES key and IV.
    // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    fn recv(mut rsa: RSA_1024, encrypted_key: Vec<u8>, encrypted_iv: Vec<u8>, cipher: Vec<u8>) -> String
    {
        use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
        use cryptocol::number::SharedArrays;

        print!("Encrypted_key = ");
        for i in 0..128
            { print!("{}", encrypted_key[i]); }
        println!();
        let mut key = vec![0u8; 16];
        rsa.decrypt_vec_into_vec(&encrypted_key, &mut key);
        print!("key = ");
        for k in key.clone()
            { print!("{:X}", k); }
        println!();
        let mut iv = vec![0u8; 16];
        rsa.decrypt_vec_into_vec(&encrypted_iv, &mut iv);
        let iv: [u8; 16] = iv.try_into().expect("Not fit to the size of array");
        print!("iv = ");
        for i in 0_usize..16
            { print!("{:X}", iv[i]); }
        println!();
        let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
        let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
        let mut recovered = String::new();
        aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
        println!("Recovered =\t{}", recovered);
        recovered
    }

    let rsa = generate();
    let (encrypted_key, encrypted_iv, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    let recovered = recv(rsa, encrypted_key, encrypted_iv, cipher);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn rsa_decrypt_vec_into_array()
{
    println!("rsa_decrypt_vec_into_array");
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
        let prime1 = U512::from_str_radix("A285ADDEA1E045833842C582CF6CFC1780693C6656946E86ABB4D651E9E55D895291CA1B7CFDCFA27BE47095D55EB3FE72A27F40DF5C17E11AA8241092D47F35", 16).unwrap();
        let prime2 = U512::from_str_radix("CC218034E0F69E535885629ED450BFA01DE2407B5EA0ABB51B49587E6A973869D005032C0A5DBA2D174502A0014F8BB767F09AFD4E5495D704E090DA3AED4C89", 16).unwrap();
        RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    }

    // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    // 2. Encrypts the certain AES key and IV by RSA to get the encrypted certain AES key and encrypted IV.
    // 3. Sends the encrypted certain AES key, IV and the ciphertext.
    fn send(public_key: U1024, modulus: U1024, message: &str) -> (Vec<u8>, Vec<u8>, Vec<u8>)
    {
        use cryptocol::symmetric::{ AES_128, PCBC_ISO };
        use cryptocol::number::SharedArrays;

        let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
        println!("Public Key: {}", rsa.get_public_key());
        println!("Product value: {}", rsa.get_modulus());
        
        let mut key = [0u8; 16];
        for i in 0u8..16
            { key[i as usize] = i; }
        let mut iv = [0u8; 16];
        iv[0] = 13;
        iv[1] = 17;
        for i in 2..16
            { iv[i] = iv[i-1].wrapping_mul(iv[i-2]); }
        let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
        let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
        let mut cipher = Vec::new();
        aes.encrypt_str_into_vec(iv.clone(), message, &mut cipher);
        let mut encrypted_key = Vec::<u8>::new();
        rsa.encrypt_array_into_vec(&key, &mut encrypted_key);
        let mut encrypted_iv= Vec::<u8>::new();
        rsa.encrypt_array_into_vec(&iv, &mut encrypted_iv);
        (encrypted_key, encrypted_iv, cipher)
    }

    // 1. Receives the encrypted AES key, IV and the ciphertext.
    // 2. Decrypts the encrypted AES key and encrypted IV by RSA to get the original AES key and IV.
    // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    fn recv(mut rsa: RSA_1024, encrypted_key: Vec<u8>, encrypted_iv: Vec<u8>, cipher: Vec<u8>) -> String
    {
        use cryptocol::symmetric::{ AES_128, PCBC_ISO };
        use cryptocol::number::SharedArrays;

        print!("Encrypted_key = ");
        for i in 0..128
            { print!("{}", encrypted_key[i]); }
        println!();
        let mut key = [0u8; 16];
        rsa.decrypt_vec_into_array(&encrypted_key, &mut key);
        print!("key = ");
        for k in key.clone()
            { print!("{:X}", k); }
        println!();
        let mut iv = [0u8; 16];
        rsa.decrypt_vec_into_array(&encrypted_iv, &mut iv);
        print!("iv = ");
        for i in 0_usize..16
            { print!("{:X}", iv[i]); }
        println!();
        let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
        let mut aes = AES_128::new_with_key(&key);
        let mut recovered = String::new();
        aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
        println!("Recovered =\t{}", recovered);
        recovered
    }

    let rsa = generate();
    let (encrypted_key, encrypted_iv, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    let recovered = recv(rsa, encrypted_key, encrypted_iv, cipher);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn rsa_decrypt_vec_into_string()
{
    println!("rsa_decrypt_vec_into_string");
    use cryptocol::asymmetric::PKCS1V15;
    
    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let prime1 = U512::from_str_radix("86CD3DACD10854D23176009D6D3C351D04541E22E0DC48F242FB4D92F08497214002FB4BFEAD9FBD0270C71901B5BBE4C968D0712E741E919134A281B5004AAB", 16).unwrap();
    let prime2 = U512::from_str_radix("A2FA368C35E68E3D0A584AD3A671516FE942BC816A6A784346910A48CE81C6C5EF354EC458E91FDEE7D0C1FC3AD66E8011B60CFFFEB1919526EDFC60018E07C5", 16).unwrap();
    let mut rsa = RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
    println!("Private Key: {}", rsa.get_private_key());
    println!("Public Key: {}", rsa.get_public_key());
    println!("Product value: {}", rsa.get_modulus());
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = Vec::<u8>::new();
    rsa.encrypt_str_into_vec(message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();

    let mut recovered = String::new();
    rsa.decrypt_vec_into_string(&cipher, &mut recovered);
    println!("Recovered =\t {}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn rsa_decrypt_array()
{
    println!("rsa_decrypt_array");
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
        let prime1 = U512::from_str_radix("9B3D6591A504C40FA6C4BC5465639914FB94A514BA86C399B586BF57822C49A08001DBAACDE3D0D08F5457B12ABB3C1DDCBADF2A91A0F701C14942569AA4BE6B", 16).unwrap();
        let prime2 = U512::from_str_radix("ADC1B51955B59316288B0C23C45DA60D7E808740EF7ABD8D0D21782C7600E0A582D7A81F77959BF52DD4A6BA26ECC1BD8D105273C60DF73738C2D6FCB6E4A99B", 16).unwrap();
        RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    }

    // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    // 2. Encrypts the certain AES key and IV by RSA to get the encrypted certain AES key and encrypted IV.
    // 3. Sends the encrypted certain AES key, IV and the ciphertext.
    fn send(public_key: U1024, modulus: U1024, message: &str) -> ([u8; 128], [u8; 128], Vec<u8>)
    {
        use cryptocol::symmetric::{ AES_128, CFB };
        use cryptocol::number::SharedArrays;

        let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
        println!("Public Key: {}", rsa.get_public_key());
        println!("Product value: {}", rsa.get_modulus());
        
        let mut key = Vec::<u8>::new();
        for i in 0u8..16
            { key.push(i); }
        let mut iv = [0u8; 16];
        iv[0] = 13;
        iv[1] = 17;
        for i in 2..16
            { iv[i] = iv[i-1].wrapping_mul(iv[i-2]); }
        let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
        let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
        let mut cipher = Vec::new();
        aes.encrypt_str_into_vec(iv.clone(), message, &mut cipher);
        let mut encrypted_key = [0u8; 128];
        rsa.encrypt_into_array(key.as_ptr(), key.len() as u64, &mut encrypted_key);
        let mut encrypted_iv= [0u8; 128];
        rsa.encrypt_into_array(iv.as_ptr() as *const u8, 16, &mut encrypted_iv);
        (encrypted_key, encrypted_iv, cipher)
    }

    // 1. Receives the encrypted AES key, IV and the ciphertext.
    // 2. Decrypts the encrypted AES key and encrypted IV by RSA to get the original AES key and IV.
    // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    fn recv(mut rsa: RSA_1024, encrypted_key: [u8; 128], encrypted_iv: [u8; 128], cipher: Vec<u8>) -> String
    {
        use cryptocol::symmetric::{ AES_128, CFB };
        use cryptocol::number::SharedArrays;

        print!("Encrypted_key = ");
        for i in 0..128
            { print!("{}", encrypted_key[i]); }
        println!();
        let mut key = [0u8; 16];
        rsa.decrypt_array(&encrypted_key, key.as_mut_ptr());
        print!("key = ");
        for k in key.clone()
            { print!("{:X}", k); }
        println!();
        let mut iv = [0u8; 16];
        rsa.decrypt_array(&encrypted_iv, iv.as_mut_ptr());
        print!("iv = ");
        for i in 0_usize..16
            { print!("{:X}", iv[i]); }
        println!();
        let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
        let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
        let mut recovered = String::new();
        aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
        println!("Recovered =\t{}", recovered);
        recovered
    }

    let rsa = generate();
    let (encrypted_key, encrypted_iv, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    let recovered = recv(rsa, encrypted_key, encrypted_iv, cipher);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn rsa_decrypt_array_into_vec()
{
    println!("rsa_decrypt_array_into_vec");
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
        let prime1 = U512::from_str_radix("C17B561347CF845495B16CBDA4994EE2FE843C58AC40B4747411CFEE7A592D8273B5A6A120283D69D8C31FC5088D5B9D7A209B8C4B8048DFBDF10EFE16D3E71D", 16).unwrap();
        let prime2 = U512::from_str_radix("D18734B895C1B6217024B56820FC66D6D7EDC2F5680B1AED9590A856C636BC5F419BD71CFCB91F67CFDFC5ECE1E9494A02FECF054F4F43D8E069C4D5E71B6077", 16).unwrap();
        RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    }

    // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    // 2. Encrypts the certain AES key and IV by RSA to get the encrypted certain AES key and encrypted IV.
    // 3. Sends the encrypted certain AES key, IV and the ciphertext.
    fn send(public_key: U1024, modulus: U1024, message: &str) -> ([u8; 128], [u8; 128], Vec<u8>)
    {
        use cryptocol::symmetric::{ AES_128, OFB };
        use cryptocol::number::SharedArrays;

        let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
        println!("Public Key: {}", rsa.get_public_key());
        println!("Product value: {}", rsa.get_modulus());
        
        let mut key = [0u8; 16];
        for i in 0u8..16
            { key[i as usize] = i; }
        let mut iv = [0u8; 16];
        iv[0] = 13;
        iv[1] = 17;
        for i in 2..16
            { iv[i] = iv[i-1].wrapping_mul(iv[i-2]); }
        let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
        let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
        let mut cipher = Vec::new();
        aes.encrypt_str_into_vec(iv.clone(), message, &mut cipher);
        let mut encrypted_key = [0u8; 128];
        rsa.encrypt_array_into_array(&key, &mut encrypted_key);
        let mut encrypted_iv= [0u8; 128];
        rsa.encrypt_array_into_array(&iv, &mut encrypted_iv);
        (encrypted_key, encrypted_iv, cipher)
    }

    // 1. Receives the encrypted AES key, IV and the ciphertext.
    // 2. Decrypts the encrypted AES key and encrypted IV by RSA to get the original AES key and IV.
    // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    fn recv(mut rsa: RSA_1024, encrypted_key: [u8; 128], encrypted_iv: [u8; 128], cipher: Vec<u8>) -> String
    {
        use cryptocol::symmetric::{ AES_128, OFB };
        use cryptocol::number::SharedArrays;

        print!("Encrypted_key = ");
        for i in 0..128
            { print!("{}", encrypted_key[i]); }
        println!();
        let mut key = vec![0u8; 16];
        rsa.decrypt_array_into_vec(&encrypted_key, &mut key);
        print!("key = ");
        for k in key.clone()
            { print!("{:X}", k); }
        println!();
        let mut iv = vec![0u8; 16];
        rsa.decrypt_array_into_vec(&encrypted_iv, &mut iv);
        print!("iv = ");
        for i in 0_usize..16
            { print!("{:X}", iv[i]); }
        println!();
        let mut iv_ = [0u8; 16];
        for i in 0_usize..16
            { iv_[i] = iv[i]; }
        let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv_).des };
        let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
        let mut recovered = String::new();
        aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
        println!("Recovered =\t{}", recovered);
        recovered
    }

    let rsa = generate();
    let (encrypted_key, encrypted_iv, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    let recovered = recv(rsa, encrypted_key, encrypted_iv, cipher);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn rsa_decrypt_array_into_array()
{
    println!("rsa_decrypt_array_into_array");
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
        let prime1 = U512::from_str_radix("E2080D05BC860FE280F457BB314F9AB284420B6E6B341596E17AB1F69679AFF0D08175DD9392743708F07884BE7CB7962D65BFF3C9FA2B3F13320F48BB1796EF", 16).unwrap();
        let prime2 = U512::from_str_radix("EC8230D29D07370222F3A98487A8F7B5438A07399BE492C2BA2E83E33BB52E7F0FD2404024B130A464FA10DDA5C1FB9C095286E21DBF79AC162C14571DA7DAC5", 16).unwrap();
        RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint())
    }

    // 1. Encrypts a message by AES with a certain AES key to get the ciphertext.
    // 2. Encrypts the certain AES key and IV by RSA to get the encrypted certain AES key and encrypted IV.
    // 3. Sends the encrypted certain AES key, IV and the ciphertext.
    fn send(public_key: U1024, modulus: U1024, message: &str) -> ([u8; 128], [u8; 128], Vec<u8>)
    {
        use cryptocol::symmetric::{ AES_128, PCBC_ISO };
        use cryptocol::number::SharedArrays;

        let mut rsa = RSA_1024::new_with_keys(public_key.clone(), public_key, modulus);
        println!("Public Key: {}", rsa.get_public_key());
        println!("Product value: {}", rsa.get_modulus());
        
        let mut key = Vec::<u8>::new();
        for i in 0u8..16
            { key.push(i); }
        let mut iv = [0u8; 16];
        iv[0] = 13;
        iv[1] = 17;
        for i in 2..16
            { iv[i] = iv[i-1].wrapping_mul(iv[i-2]); }
        let iv_ = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
        let mut aes = AES_128::new_with_key(& unsafe { *(key.as_ptr() as *const [u8; 16]) });
        let mut cipher = Vec::new();
        aes.encrypt_str_into_vec(iv_.clone(), message, &mut cipher);
        let mut encrypted_key = [0u8; 128];
        rsa.encrypt_vec_into_array(&key, &mut encrypted_key);
        let iv = iv.to_vec();
        let mut encrypted_iv= [0u8; 128];
        rsa.encrypt_vec_into_array(&iv, &mut encrypted_iv);
        (encrypted_key, encrypted_iv, cipher)
    }

    // 1. Receives the encrypted AES key, IV and the ciphertext.
    // 2. Decrypts the encrypted AES key and encrypted IV by RSA to get the original AES key and IV.
    // 3. Decrypts the ciphertext by the AES with the decrypted original AES key.
    fn recv(mut rsa: RSA_1024, encrypted_key: [u8; 128], encrypted_iv: [u8; 128], cipher: Vec<u8>) -> String
    {
        use cryptocol::symmetric::{ AES_128, PCBC_ISO };
        use cryptocol::number::SharedArrays;

        print!("Encrypted_key = ");
        for i in 0..128
            { print!("{}", encrypted_key[i]); }
        println!();
        let mut key = [0u8; 16];
        rsa.decrypt_array_into_array(&encrypted_key, &mut key);
        print!("key = ");
        for k in key.clone()
            { print!("{:X}", k); }
        println!();
        let mut iv = [0u8; 16];
        rsa.decrypt_array_into_array(&encrypted_iv, &mut iv);
        print!("iv = ");
        for i in 0_usize..16
            { print!("{:X}", iv[i]); }
        println!();
        let iv = unsafe { SharedArrays::<u32, 4, u8, 16>::from_src(&iv).des };
        let mut aes = AES_128::new_with_key(&key);
        let mut recovered = String::new();
        aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
        println!("Recovered =\t{}", recovered);
        recovered
    }

    let rsa = generate();
    let (encrypted_key, encrypted_iv, cipher) = send(rsa.get_public_key(), rsa.get_modulus(), &message);
    let recovered = recv(rsa, encrypted_key, encrypted_iv, cipher);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
    println!("-------------------------------");
}

fn rsa_decrypt_array_into_string()
{
    println!("rsa_decrypt_array_into_string");
    use cryptocol::asymmetric::PKCS1V15;
    
    // Example for RSA_1024
    use cryptocol::asymmetric::RSA_1024;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let prime1 = U512::from_str_radix("DA97ECE2D8B5B50AB6263044B74054EE6E14DE616A6102DD53C6047DD08C38C234289851DE79B50F963CDC5D7A0C1DFA20CD9CDE3D61C4871A1F55E40CBB0E9F", 16).unwrap();
    let prime2 = U512::from_str_radix("A5F59876FE0FE52E2017BF43E181739DB88C76DDA8F834D25B6B9A3213464A0AC46B742D971677F187C7ED45091127E62FA13ADFBC2A96F3368652EC9D963D4B", 16).unwrap();
    let mut rsa = RSA_1024::new_with_primes(prime1.into_biguint(), prime2.into_biguint());
    println!("Private Key: {}", rsa.get_private_key());
    println!("Public Key: {}", rsa.get_public_key());
    println!("Product value: {}", rsa.get_modulus());
    let message = "In the beginning God created the heavens and the earth.";
    println!("M =\t{}", message);
    let mut cipher = [0u8; 128];
    rsa.encrypt_str_into_array(message, &mut cipher);
    print!("C =\t");
    for c in cipher.clone()
        { print!("{:02X} ", c); }
    println!();

    let mut recovered = String::new();
    rsa.decrypt_array_into_string(&cipher, &mut recovered);
    println!("Recovered =\t {}", recovered);
    assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    assert_eq!(recovered, message);
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

fn prime_test()
{
    use cryptocol::random::Random;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut r = Random::new();
    let (p, q): (U1024, U1024) = r.random_prime_with_half_length_using_rsa_biguint(7);
    println!("{}\n{}", p, q);
}
