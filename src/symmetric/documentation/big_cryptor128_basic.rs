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
#![allow(unused_variables)]
// #![warn(rustdoc::missing_doc_code_examples)]


use std::vec::Vec;
use crate::symmetric::SmallCryptor;

/// big_cryptor.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to big_cryptor128_basic.rs. And, most of generic parameters
/// are omitted. It is not actual code but dummy code for compilation!!!
#[allow(non_camel_case_types)]
pub struct BigCryptor128
{
    // Dummy struct for documentation
}

/// big_cryptor.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to big_cryptor64_basic.rs. And, most of generic parameters
/// are omitted. It is not actual code but dummy code for compilation!!!
impl BigCryptor128
{
    // pub fn new() -> Self
    /// Constructs a new object BigCryptor128.
    /// 
    /// # Output
    /// A new object BigCryptor128 that has no small cryptors by default.
    /// 
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method does not set any small cryptor (component) by default.
    /// - You have to set as many small cryptors (components) as you want.
    /// - The small cryptors (components) should have the block size 128-bit.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128 };
    /// let mut taes = BigCryptor128::new();
    /// taes.push_small_cryptor(AES_128::encryptor_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]));
    /// taes.push_small_cryptor(AES_128::decryptor_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]));
    /// taes.push_small_cryptor(AES_128::encryptor_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]));
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128 };
    /// let mut _taes = AES_128::encryptor_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 + AES_128::decryptor_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::encryptor_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// ```
    pub fn new() -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor<u128, 16>>; N]) -> Self
    /// Constructs a new object BigCryptor128.
    /// 
    /// # Arguments
    /// - `smallcryptor` is the array of small cryptors (components),
    ///   each small cryptors of which is wrapped by `Box`.
    /// 
    /// # Output
    /// A new object BigCryptor128 that has small cryptors given as arguments.
    /// 
    /// # Features
    /// This method sets the small cryptor to be the given argument `smallcryptor`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, SmallCryptor, AES_128 };
    /// 
    /// let cryptors: [Box<dyn SmallCryptor<u128, 16>>; 3] = [ Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)),
    ///                                         Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///                                         Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)) ];
    /// let mut _taes = BigCryptor128::new_with_small_cryptor_array(cryptors);
    /// ```
    pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor<u64, 8>>; N]) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor<u128, 16>>>) -> Self
    /// Constructs a new object BigCryptor128.
    /// 
    /// # Arguments
    /// - `smallcryptor` is the `Vec` object of small cryptors (components),
    ///   each small cryptors of which is wrapped by `Box`.
    /// 
    /// # Output
    /// A new object BigCryptor128 that has small cryptors given as arguments.
    /// 
    /// # Features
    /// This method sets the key to be the given argument `key`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, SmallCryptor, DES };
    /// 
    /// let cryptors: Vec<Box<dyn SmallCryptor<u64, 8>>> = vec![ Box::new(DES::encryptor_with_key_u128(0x_1234567890ABCDEF_u128)),
    ///                                         Box::new(DES::decryptor_with_key_u128(0x_FEDCBA0987654321_u128)),
    ///                                         Box::new(DES::encryptor_with_key_u128(0x_1234567890ABCDEF_u128)) ];
    /// let mut _tdes = BigCryptor128::new_with_small_cryptor_array(cryptors);
    /// ```
    pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor<u64, 8>>>) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn push_small_cryptor_array<const N: usize>(&mut self, smallcryptors: [Box<dyn SmallCryptor<u64, 8>>; N])
    /// Adds small cryptors (components) to `Self`'s own small cryptor
    /// container.
    /// 
    /// # Arguments
    /// `smallcryptors` is an array of small cryptors (components).
    /// 
    /// # Features
    /// Each element of the array the small cryptors should be wrapped by Box.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, SmallCryptor, AES_128 };
    /// let mut taes = BigCryptor128::new();
    /// let cryptors: [Box<dyn SmallCryptor<u128, 16>>; 3] = [ Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)),
    ///                                         Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///                                         Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)) ];
    /// taes.push_small_cryptor_array(cryptors);
    /// ```
    pub fn push_small_cryptor_array<const N: usize>(&mut self, smallcryptors: [Box<dyn SmallCryptor<u64, 8>>; N])
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn push_small_cryptor_vec<const N: usize>(&mut self, smallcryptor: [SmallCryptor; N])
    /// Adds small cryptors (components) to `Self`'s own small cryptor
    /// container.
    /// 
    /// # Arguments
    /// `smallcryptors` is a `Vec` object of small cryptors (components).
    /// 
    /// # Features
    /// Each element of the Vec object of the small cryptors
    /// should be wrapped by Box.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, SmallCryptor, AES_128 };
    /// let mut taes = BigCryptor128::new();
    /// let cryptors: Vec<Box<dyn SmallCryptor<u128, 16>>> = vec![ Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)),
    ///                                         Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///                                         Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)) ];
    /// taes.push_small_cryptor_vec(cryptors);
    /// ```
    pub fn push_small_cryptor_vec(&mut self, smallcryptor: Vec<Box<dyn SmallCryptor<u64, 8>>>)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn turn_inverse(&mut self)
    /// Flips its role in BigCryptor128.
    ///
    /// # Features
    /// - If it is constructed as encryptor for embracing BigCryptor128,
    ///   it will be changed into decryptor.
    /// - If it is constructed as decryptor for embracing BigCryptor128,
    ///   it will be changed into encryptor.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, AES_192, SmallCryptor };
    /// let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// 
    /// let aes = AES_128::new_with_key(&[0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// let rijndael = AES_192::new_with_key(&[0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// taes.turn_inverse();
    /// let mut bigcryptor = aes + rijndael + taes;
    /// 
    /// let plaintext = 0x_1234567890ABCDEFFEDCBA0987654321_u128;
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// let ciphertext = bigcryptor.encrypt_u128(plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_B881F06147B26243D0742CAA82602E97_u128);
    /// 
    /// let recovered_text = bigcryptor.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    pub fn turn_inverse(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn turn_encryptor(&mut self)
    /// Changes its role in BigCryptor128 to encryptor.
    ///
    /// # Features
    /// - If it is constructed as encryptor for embracing BigCryptor128,
    ///   it will not be changed at all.
    /// - If it is constructed as decryptor for embracing BigCryptor128,
    ///   it will be changed into encryptor.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, AES_192, SmallCryptor };
    /// let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// 
    /// let aes = AES_128::new_with_key(&[0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// let rijndael = AES_192::new_with_key(&[0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// taes.turn_encryptor();
    /// let mut bigcryptor = aes + rijndael + taes;
    /// 
    /// let plaintext = 0x_1234567890ABCDEFFEDCBA0987654321_u128;
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// let ciphertext = bigcryptor.encrypt_u128(plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_1E561632CF3EDD44E8955A26ABA0AF7E_u128);
    /// 
    /// let recovered_text = bigcryptor.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    pub fn turn_encryptor(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn turn_encryptor(&mut self)
    /// Changes its role in BigCryptor128 to encryptor.
    ///
    /// # Features
    /// - If it is constructed as encryptor for embracing BigCryptor128,
    ///   it will not be changed at all.
    /// - If it is constructed as decryptor for embracing BigCryptor128,
    ///   it will be changed into encryptor.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, AES_192, SmallCryptor };
    /// let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// 
    /// let aes = AES_128::new_with_key(&[0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// let rijndael = AES_192::new_with_key(&[0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// taes.turn_decryptor();
    /// let mut bigcryptor = aes + rijndael + taes;
    /// 
    /// let plaintext = 0x_1234567890ABCDEFFEDCBA0987654321_u128;
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// let ciphertext = bigcryptor.encrypt_u128(plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_B881F06147B26243D0742CAA82602E97_u128);
    /// 
    /// let recovered_text = bigcryptor.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    pub fn turn_decryptor(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_u128(&mut self, message: u128) -> u128
    /// Encrypts a 128-bit data.
    /// 
    /// # Arguments
    /// `message` is of `u128`-type and the plaintext to be encrypted.
    /// 
    /// # Output
    /// This method returns the encrypted data of `u128`-type from `message`.
    /// 
    /// # Example 1 for Normal case 
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128 };
    /// 
    /// let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///              - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///              + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// let message = 0x_1234567890ABCDEFFEDCBA0987654321_u128;
    /// println!("M = {:#034X}", message);
    /// let cipher = taes.encrypt_u128(message);
    /// println!("C = {:#034X}", cipher);
    /// assert_eq!(cipher, 0x_965C637ECAC29A9B0BE3F62C9593C04C_u128);
    /// ```
    pub fn encrypt_u128(&mut self, message: u64) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_u128(&mut self, cipher: u128) -> u128
    /// Decrypts a 128-bit data.
    /// 
    /// # Arguments
    /// `cioher` is of `u128`-type and the ciphertext to be decrypted.
    /// 
    /// # Output
    /// This method returns the decrypted data of `u128`-type from `cipher`.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128 };
    /// 
    /// let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///              - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///              + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// let message = 0x_1234567890ABCDEFFEDCBA0987654321_u128;
    /// println!("M = {:#034X}", message);
    /// let cipher = taes.encrypt_u128(message);
    /// println!("C = {:#034X}", cipher);
    /// assert_eq!(cipher, 0x_965C637ECAC29A9B0BE3F62C9593C04C_u128);
    /// 
    /// let recovered = taes.decrypt_u128(cipher);
    /// println!("B = {:#034X}", recovered);
    /// assert_eq!(recovered, 0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// assert_eq!(recovered, message);
    /// ```
    pub fn decrypt_u128(&mut self, cipher: u64) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_array_u128<const N: usize>(&mut self, message: &[u128; N], cipher: &mut [u128; N])
    /// Encrypts an array of 128-bit data.
    /// 
    /// # Arguments
    /// - `message` is of an array of `u128`-type and the plaintext to be
    ///   encrypted.
    /// - `cipher` is of an array of `u128`-type and the ciphertext to be
    ///   stored.
    /// 
    /// # Example 1 for Normal case 
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128 };
    /// 
    /// let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///              - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///              + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// let message = [0x_1234567890ABCDEFFEDCBA0987654321_u128, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211_u128];
    /// print!("M = ");
    /// for msg in message.clone()
    ///     { print!("{:#034X} ", msg); }
    /// println!();
    /// 
    /// let mut cipher = [0_u128; 3];
    /// taes.encrypt_array_u128(&message, &mut cipher);
    /// print!("C = ");
    /// for c in cipher.clone()
    ///     { print!("{:#034X} ", c); }
    /// println!();
    /// assert_eq!(cipher[0], 0x_965C637ECAC29A9B0BE3F62C9593C04C_u128);
    /// assert_eq!(cipher[1], 0x_A397AABE9537829FABA0596B5D3EA8B9_u128);
    /// assert_eq!(cipher[2], 0x_85457798D08431CCB8A4A58517A5D452_u128);
    /// ```
    pub fn encrypt_array_u128<const M: usize>(&mut self, message: &[u64; M], cipher: &mut [u64; M])
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_array_u128<const N: usize>(&mut self, cipher: &[u128; N], message: &mut [u128; N])
    /// Decrypts an array of 128-bit data.
    /// 
    /// # Arguments
    /// - `cipher` is of an array of `u128`-type and the ciphertext to be
    ///   decrypted.
    /// - `message` is of an array of `u128`-type and the plaintext to be
    ///   stored.
    /// 
    /// # Example 1 for Normal case 
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128 };
    /// 
    /// let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// let message = [0x_1234567890ABCDEFFEDCBA0987654321_u128, 0x11223344556677889900AABBCCDDEEFF, 0xFFEEDDCCBBAA00998877665544332211];
    /// print!("M = ");
    /// for msg in message.clone()
    ///     { print!("{:#034X} ", msg); }
    /// println!();
    /// 
    /// let mut cipher = [0_u128; 3];
    /// taes.encrypt_array_u128(&message, &mut cipher);
    /// print!("C = ");
    /// for c in cipher.clone()
    ///     { print!("{:#034X} ", c); }
    /// println!();
    /// assert_eq!(cipher[0], 0x_965C637ECAC29A9B0BE3F62C9593C04C_u128);
    /// assert_eq!(cipher[1], 0x_A397AABE9537829FABA0596B5D3EA8B9_u128);
    /// assert_eq!(cipher[2], 0x_85457798D08431CCB8A4A58517A5D452_u128);
    /// 
    /// let mut recovered = [0_u128; 3];
    /// taes.decrypt_array_u128(&cipher, &mut recovered);
    /// print!("B = ");
    /// for r in recovered.clone()
    ///     { print!("{:#034X} ", r); }
    /// println!();
    /// assert_eq!(recovered[0], 0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// assert_eq!(recovered[1], 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(recovered[2], 0x_FFEEDDCCBBAA00998877665544332211_u128);
    /// assert_eq!(recovered[0], message[0]);
    /// assert_eq!(recovered[1], message[1]);
    /// assert_eq!(recovered[2], message[2]);
    /// ```
    pub fn decrypt_array_u128<const M: usize>(&mut self, cipher: &[u64; M], message: &mut [u64; M])
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_succeful(&self) -> bool
    /// Checks whether the previous encryption or decryption was successful.
    /// 
    /// # Output
    /// If the previous encryption or decryption was successful, this method
    /// returns true. Otherwise, it returns false.
    /// 
    /// # Features
    /// - Usually, you don't have to use this method because the encryption
    ///   methods returns the length of ciphertext and the decryption methods
    ///   returns the length of plaintext but they returns `0` when they failed.
    /// - If the ciphertext is 16 bytes for decryption with the padding either
    ///   pkcs7 or iso, the return value `0` of the decryption methods is not
    ///   discriminatory. You don't know whether the previous decryption was
    ///   failed or the original plaintext was just null string or "". In this
    ///   case you can check its success with this method.
    /// 
    /// # Example 1 for Normal case for Successful case for encryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{}", iv);
    /// let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// let len = taes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 16);
    /// let success = taes.is_successful();
    /// assert_eq!(success, true);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "D9 F7 43 4F 83 5D 3E 70 1F CD A1 4A 49 C1 78 83 ");
    /// ```
    /// 
    /// # Example 2 for Successful case for decryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{}", iv);
    /// let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// 
    /// let cipher = [0xD9_u8, 0xF7, 0x43, 0x4F, 0x83, 0x5D, 0x3E, 0x70, 0x1F, 0xCD, 0xA1, 0x4A, 0x49, 0xC1, 0x78, 0x83];
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut recovered = [0u8; 16];
    /// let len = taes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    /// println!("The length of plaintext = {}", len);
    /// assert_eq!(len, 0);
    /// let success = taes.is_successful();
    /// assert_eq!(success, true);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 3 for Failure case for encryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{}", iv);
    /// let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 4];
    /// let len = taes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 0);
    /// let success = taes.is_successful();
    /// assert_eq!(success, false);
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "00 00 00 00 ");
    /// ```
    /// 
    /// # Example 4 for Failure case for decryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{}", iv);
    /// let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// 
    /// let cipher = [0x17_u8, 0xC8, 0x15, 0x48];
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut recovered = [0u8; 16];
    /// let len = taes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    /// println!("The length of plaintext = {}", len);
    /// assert_eq!(len, 0);
    /// let success = taes.is_successful();
    /// assert_eq!(success, false);
    /// ```
    #[inline] pub fn is_successful(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_failed(&self) -> bool
    /// Checks whether the previous encryption or decryption was failed.
    /// 
    /// # Output
    /// If the previous encryption or decryption was failed, this method
    /// returns true. Otherwise, it returns false.
    /// 
    /// # Features
    /// - Usually, you don't have to use this method because the encryption
    ///   methods returns the length of ciphertext and the decryption methods
    ///   returns the length of plaintext but they returns `0` when they failed.
    /// - If the ciphertext is 8 bytes for decryption with the padding either
    ///   pkcs7 or iso, the return value `0` of the decryption methods is not
    ///   discriminatory. You don't know whether the previous decryption was
    ///   failed or the original plaintext was just null string or "". In this
    ///   case you can check its success with this method.
    /// 
    /// # Example 1 for Successful case for encryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{}", iv);
    /// let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// let len = taes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 16);
    /// let failure = taes.is_failed();
    /// assert_eq!(failure, false);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "D9 F7 43 4F 83 5D 3E 70 1F CD A1 4A 49 C1 78 83 ");
    /// ```
    /// 
    /// # Example 2 for Successful case for decryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{}", iv);
    /// let mut taes = BigCryptor128::new()
    ///                             + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                             - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                             + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// 
    /// let cipher = [0xD9_u8, 0xF7, 0x43, 0x4F, 0x83, 0x5D, 0x3E, 0x70, 0x1F, 0xCD, 0xA1, 0x4A, 0x49, 0xC1, 0x78, 0x83];
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut recovered = [0u8; 16];
    /// let len = taes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    /// println!("The length of plaintext = {}", len);
    /// assert_eq!(len, 0);
    /// let failure = taes.is_failed();
    /// assert_eq!(failure, false);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 3 for Failure case for encryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{}", iv);
    /// let mut taes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 - AES_128::new_with_key(&[0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 + AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 4];
    /// let len = taes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 0);
    /// let failure = taes.is_failed();
    /// assert_eq!(failure, true);
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "00 00 00 00 ");
    /// ```
    /// 
    /// # Example 4 for Failure case for decryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA0987654321_u128;
    /// println!("IV =	{}", iv);
    /// let mut tdes = DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                 - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                 + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// 
    /// let cipher = [0x17_u8, 0xC8, 0x15, 0x48];
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// 
    /// let mut recovered = [0u8; 8];
    /// let len = tdes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    /// println!("The length of plaintext = {}", len);
    /// assert_eq!(len, 0);
    /// let failure = tdes.is_failed();
    /// assert_eq!(failure, true);
    /// ```
    #[inline] pub fn is_failed(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }
}
