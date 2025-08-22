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
use crate::number::LongUnion;
use crate::symmetric::SmallCryptor;

/// big_cryptor.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to big_cryptor64_basic.rs. And, most of generic parameters
/// are omitted. It is not actual code but dummy code for compilation!!!
#[allow(non_camel_case_types)]
pub struct BigCryptor64
{
    // Dummy struct for documentation
    block: LongUnion,
    smallcryptor: Vec<Box<dyn SmallCryptor<u64, 8>>>
}

/// big_cryptor.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to big_cryptor64_basic.rs. And, most of generic parameters
/// are omitted. It is not actual code but dummy code for compilation!!!
impl BigCryptor64
{
    // pub fn new() -> Self
    /// Constructs a new object BigCryptor64.
    /// 
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method does not set any component by default.
    /// - You have to set as many components as you want.
    /// - The components should have the block size 64-bit.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    /// let mut tdes = BigCryptor64::new();
    /// tdes.push_small_cryptor(DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]));
    /// tdes.push_small_cryptor(DES::decryptor_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]));
    /// tdes.push_small_cryptor(DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]));
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// let mut _tdes = BigCryptor64::new()
    ///               + DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///               + DES::decryptor_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///               + DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// ```
    /// 
    /// # Compile-fail Example
    /// ```compile_fail
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    /// let mut tdes = BigCryptor64::new();
    /// // It cannot be compiled!
    /// tdes.push_small_cryptor(DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]));
    /// ```
    pub fn new() -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor<u64, 8>>; N]) -> Self
    /// Constructs a new object BigCryptor64 with some small cryptors
    /// (components).
    /// 
    /// # Arguments
    /// `smallcryptor` is an array of small cryptors (components)
    ///  wrapped by `Box`.
    /// 
    /// # Features
    /// This method sets the small cryptor to be the given argument `smallcryptor`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, SmallCryptor, DES };
    /// let cryptors: [Box<dyn SmallCryptor<u64, 8>>; 3] = [ Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)),
    ///                                         Box::new(DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)),
    ///                                         Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    /// let mut _tdes = BigCryptor64::new_with_small_cryptor_array(cryptors);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.new_with_key)
    pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor<u64, 8>>; N]) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor<u64, 8>>>) -> Self
    /// Constructs a new object BigCryptor64 with some small cryptors
    /// (components).
    /// 
    /// # Arguments
    /// `smallcryptor` is a Vec object of small cryptors (components)
    ///  wrapped by `Box`.
    /// 
    /// # Features
    /// This method sets the small cryptor to be the given argument `smallcryptor`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, SmallCryptor, DES };
    /// 
    /// let cryptors: Vec<Box<dyn SmallCryptor<u64, 8>>> = vec![ Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)),
    ///                                         Box::new(DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)),
    ///                                         Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    /// let mut _tdes = BigCryptor64::new_with_small_cryptor_array(cryptors);
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
    /// use cryptocol::symmetric::{ BigCryptor64, SmallCryptor, DES };
    /// 
    /// let mut tdes = BigCryptor64::new();
    /// let cryptors: [Box<dyn SmallCryptor<u64, 8>>; 3] = [ Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)),
    ///                                         Box::new(DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)),
    ///                                         Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    /// tdes.push_small_cryptor_array(cryptors);
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
    /// `smallcryptors` is a Vec object of small cryptors (components).
    /// 
    /// # Features
    /// Each element of the Vec object of the small cryptors
    /// should be wrapped by Box.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, SmallCryptor, DES };
    /// 
    /// let mut tdes = BigCryptor64::new();
    /// let cryptors: Vec<Box<dyn SmallCryptor<u64, 8>>> = vec![ Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)),
    ///                                         Box::new(DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)),
    ///                                         Box::new(DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)) ];
    /// tdes.push_small_cryptor_vec(cryptors);
    /// ```
    pub fn push_small_cryptor_vec(&mut self, smallcryptor: Vec<Box<dyn SmallCryptor<u64, 8>>>)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_u64(&mut self, message: u64) -> u64
    /// Encrypts a 64-bit data.
    /// 
    /// # Arguments
    /// `message` is of `u64`-type and the plaintext to be encrypted.
    /// 
    /// # Output
    /// This method returns the encrypted data of `u64`-type from `message`.
    /// 
    /// # Example 1 for Normal case 
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    /// 
    /// let mut tdes = BigCryptor64::new()
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                             - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let message = 0x1234567890ABCDEF_u64;
    /// println!("M = {:#018X}", message);
    /// let cipher = tdes.encrypt_u64(message);
    /// println!("C = {:#018X}", cipher);
    /// assert_eq!(cipher, 0x_CA61814E7AE964BA_u64);
    /// ```
    pub fn encrypt_u64(&mut self, message: u64) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_u64(&mut self, cipher: u64) -> u64
    /// Decrypts a 64-bit data.
    /// 
    /// # Arguments
    /// `cioher` is of `u64`-type and the ciphertext to be decrypted.
    /// 
    /// # Output
    /// This method returns the decrypted data of `u64`-type from `cipher`.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    /// 
    /// let mut tdes = BigCryptor64::new()
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                             - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let message = 0x1234567890ABCDEF_u64;
    /// println!("M = {:#018X}", message);
    /// let cipher = tdes.encrypt_u64(message);
    /// println!("C = {:#018X}", cipher);
    /// assert_eq!(cipher, 0x_CA61814E7AE964BA_u64);
    /// 
    /// let recovered = tdes.decrypt_u64(cipher);
    /// println!("B = {:#018X}", recovered);
    /// assert_eq!(recovered, 0x1234567890ABCDEF_u64);
    /// assert_eq!(recovered, message);
    /// ```
    pub fn decrypt_u64(&mut self, cipher: u64) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_array_u64<const M: usize>(&mut self, message: &[u64; M], cipher: &mut [u64; M])
    /// Encrypts an array of 64-bit data.
    /// 
    /// # Arguments
    /// - `message` is of an array of `u64`-type and the plaintext to be
    ///   encrypted.
    /// - `cipher` is of an array of `u64`-type and the ciphertext to be
    ///   stored.
    /// 
    /// # Example 1 for Normal case 
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    /// 
    /// let mut tdes = BigCryptor64::new()
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                             - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let message = [0x1234567890ABCDEF_u64, 0x1122334455667788, 0x9900AABBCCDDEEFF];
    /// print!("M = ");
    /// for msg in message.clone()
    ///     { print!("{:#018X} ", msg); }
    /// println!();
    /// 
    /// let mut cipher = [0_u64; 3];
    /// tdes.encrypt_array_u64(&message, &mut cipher);
    /// print!("C = ");
    /// for c in cipher.clone()
    ///     { print!("{:#018X} ", c); }
    /// println!();
    /// assert_eq!(cipher[0], 0x_CA61814E7AE964BA_u64);
    /// assert_eq!(cipher[1], 0x_073450DF82262B1B_u64);
    /// assert_eq!(cipher[2], 0x_51712805A458A102_u64);
    /// ```
    pub fn encrypt_array_u64<const M: usize>(&mut self, message: &[u64; M], cipher: &mut [u64; M])
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_array_u64<const M: usize>(&mut self, cipher: &[u64; M], message: &mut [u64; M])
    /// Decrypts an array of 64-bit data.
    /// 
    /// # Arguments
    /// - `cipher` is of an array of `u64`-type and the ciphertext to be
    ///   decrypted.
    /// - `message` is of an array of `u64`-type and the plaintext to be
    ///   stored.
    /// 
    /// # Example 1 for Normal case 
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    /// 
    /// let mut tdes = BigCryptor64::new()
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                             - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let message = [0x1234567890ABCDEF_u64, 0x1122334455667788, 0x9900AABBCCDDEEFF];
    /// print!("M = ");
    /// for msg in message.clone()
    ///     { print!("{:#018X} ", msg); }
    /// println!();
    /// 
    /// let mut cipher = [0_u64; 3];
    /// tdes.encrypt_array_u64(&message, &mut cipher);
    /// print!("C = ");
    /// for c in cipher.clone()
    ///     { print!("{:#018X} ", c); }
    /// println!();
    /// assert_eq!(cipher[0], 0x_CA61814E7AE964BA_u64);
    /// assert_eq!(cipher[1], 0x_073450DF82262B1B_u64);
    /// assert_eq!(cipher[2], 0x_51712805A458A102_u64);
    /// 
    /// let mut recovered = [0_u64; 3];
    /// tdes.decrypt_array_u64(&cipher, &mut recovered);
    /// print!("B = ");
    /// for r in recovered.clone()
    ///     { print!("{:#018X} ", r); }
    /// println!();
    /// assert_eq!(recovered[0], 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered[1], 0x_1122334455667788_u64);
    /// assert_eq!(recovered[2], 0x_9900AABBCCDDEEFF_u64);
    /// assert_eq!(recovered[0], message[0]);
    /// assert_eq!(recovered[1], message[1]);
    /// assert_eq!(recovered[2], message[2]);
    /// ```
    pub fn decrypt_array_u64<const M: usize>(&mut self, cipher: &[u64; M], message: &mut [u64; M])
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
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut tdes = BigCryptor64::new()
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                             - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// let len = tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 8);
    /// let success = tdes.is_successful();
    /// assert_eq!(success, true);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "17 C8 15 48 EE 85 42 43 ");
    /// ```
    /// 
    /// # Example 2 for Successful case for decryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut tdes = BigCryptor64::new()
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                             - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// 
    /// let cipher = [0x17_u8, 0xC8, 0x15, 0x48, 0xEE, 0x85, 0x42, 0x43];
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut recovered = [0u8; 8];
    /// let len = tdes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    /// println!("The length of plaintext = {}", len);
    /// assert_eq!(len, 0);
    /// let success = tdes.is_successful();
    /// assert_eq!(success, true);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "00 00 00 00 00 00 00 00 ");
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
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut tdes = BigCryptor64::new()
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                             - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 4];
    /// let len = tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 0);
    /// let success = tdes.is_successful();
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
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut tdes = BigCryptor64::new()
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                             - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// 
    /// let cipher = [0x17_u8, 0xC8, 0x15, 0x48];
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut recovered = [0u8; 8];
    /// let len = tdes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    /// println!("The length of plaintext = {}", len);
    /// assert_eq!(len, 0);
    /// let success = tdes.is_successful();
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
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut tdes = BigCryptor64::new()
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                             - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// let len = tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 8);
    /// let failure = tdes.is_failed();
    /// assert_eq!(failure, false);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "17 C8 15 48 EE 85 42 43 ");
    /// ```
    /// 
    /// # Example 2 for Successful case for decryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut tdes = BigCryptor64::new()
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                             - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// 
    /// let cipher = [0x17_u8, 0xC8, 0x15, 0x48, 0xEE, 0x85, 0x42, 0x43];
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut recovered = [0u8; 8];
    /// let len = tdes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    /// println!("The length of plaintext = {}", len);
    /// assert_eq!(len, 0);
    /// let failure = tdes.is_failed();
    /// assert_eq!(failure, false);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "00 00 00 00 00 00 00 00 ");
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
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut tdes = BigCryptor64::new()
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                             - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 4];
    /// let len = tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 0);
    /// let failure = tdes.is_failed();
    /// assert_eq!(failure, true);
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "00 00 00 00 ");
    /// ```
    /// 
    /// # Example 3 for Failure case for decryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };
    /// 
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut tdes = BigCryptor64::new()
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
    ///                             - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
    ///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
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
