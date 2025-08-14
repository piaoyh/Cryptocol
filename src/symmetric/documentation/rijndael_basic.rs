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

use crate::number::IntUnion;

/// rijndael.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to rijndael_basic.rs.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct Rijndael_Generic<const ROUND: usize = 10, const NB: usize = 4, const NK: usize = 4>
{
    // Dummy struct for documentation
    key:        [IntUnion; NK],
    block:      [[u8; NB]; 4],
    round_key:  Vec<[IntUnion; NB]>,
    enc:        fn (s: &mut Self, message: &[IntUnion; NB]) -> [IntUnion; NB],
    dec:        fn (s: &mut Self, cipher: &[IntUnion; NB]) -> [IntUnion; NB],
}

/// rijndael.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to rijndael_basic.rs. And, most of generic parameters
/// are omitted. It is not actual code but dummy code for compilation!!!
impl <const ROUND: usize, const NB: usize, const NK: usize>
Rijndael_Generic<ROUND, NB, NK>
{
    // pub fn new() -> Self
    /// Constructs a new object Rijndael_Generic..
    ///
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method sets the key to have all bits zeros.
    /// - The default key which has all bits zeros is not a weak key unlike DES.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::AES_128;
    /// let mut _a_aes = AES_128::new();
    /// let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    /// let ciphertext = _a_aes.encrypt_u128(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#032X}", plaintext);
    /// println!("Ciphertext:\t\t{:#032X}", ciphertext);
    /// assert_eq!(ciphertext, 0xE2C8CD3BFD4D72366A4806B100659867);
    ///
    /// let recovered_cipher_text = _a_aes.decrypt_u128(ciphertext);
    /// println!("Recovered-ciphertext:\t{:#032X}", recovered_cipher_text);
    /// assert_eq!(recovered_cipher_text, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(recovered_cipher_text, plaintext);
    /// ```
    ///
    /// # Compile-fail Example
    /// ```compile_fail
    /// use cryptocol::symmetric::AES_128;
    /// let _aes = AES_128::new();
    /// // It cannot be compiled!
    /// #[cfg(compile_fail)]    _aes.encrypt_u128(0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// ```
    pub fn new() -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn new_with_key<const K: usize>(key: &[u8; K]) -> Self
    /// Constructs a new object Rijndael_Generic.
    ///
    /// # Arguments
    /// - The argument `key` is the array of u8 that has `K` elements.
    ///
    /// # Features
    /// - This method sets the key to be the given argument `key`.
    /// - If `K` is less than `4 * NK`, the rest bits of
    ///   the key to be set after `K` bits will be set to be `zero`.
    /// - If `K` is greater than `4 * NK`, the rest bits after `4 * NK` bits
    ///   of the key given as the argument will be ignored.
    ///
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    /// let ciphertext = aes.encrypt_u128(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    ///
    /// let recovered_cipher_text = aes.decrypt_u128(ciphertext);
    /// println!("Recovered-ciphertext:\t{:#034X}", recovered_cipher_text);
    /// assert_eq!(recovered_cipher_text, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(recovered_cipher_text, plaintext);
    /// ```
    ///
    /// # Example 2 for the necessary key longer than the gien key
    /// ```
    /// use cryptocol::symmetric::AES_128;
    /// // The key [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF] is the same as
    /// // the key [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x00, 0x00,
    /// // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] for AES_128
    /// let mut aes1 = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    /// let mut aes2 = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    ///
    /// let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    /// let ciphertext1 = aes1.encrypt_u128(plaintext);
    /// let ciphertext2 = aes1.encrypt_u128(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#034X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#034X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0x07A3B9744B517ADB37FC90ADE4410D62_u128);
    /// assert_eq!(ciphertext2, 0x07A3B9744B517ADB37FC90ADE4410D62_u128);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let recovered_cipher_text1 = aes1.decrypt_u128(ciphertext1);
    /// let recovered_cipher_text2 = aes2.decrypt_u128(ciphertext2);
    /// println!("Recovered-ciphertext1:\t{:#034X}", recovered_cipher_text1);
    /// println!("Recovered-ciphertext2:\t{:#034X}", recovered_cipher_text2);
    /// assert_eq!(recovered_cipher_text1, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(recovered_cipher_text2, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(recovered_cipher_text1, plaintext);
    /// assert_eq!(recovered_cipher_text2, plaintext);
    /// ```
    ///
    /// # Example 3 for the necessary key shorter than the gien key
    /// ```
    /// use cryptocol::symmetric::AES_128;
    /// // The key [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34,
    /// // 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB,
    /// // 0xCD, 0xEF] is the same as the key [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB,
    /// // 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF] for AES_128
    /// let mut aes1 = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let mut aes2 = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    ///
    /// let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    /// let ciphertext1 = aes1.encrypt_u128(plaintext);
    /// let ciphertext2 = aes1.encrypt_u128(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext1:\t\t{:#034X}", ciphertext1);
    /// println!("Ciphertext2:\t\t{:#034X}", ciphertext2);
    /// assert_eq!(ciphertext1, 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    /// assert_eq!(ciphertext2, 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    /// assert_eq!(ciphertext1, ciphertext2);
    ///
    /// let recovered_cipher_text1 = aes1.decrypt_u128(ciphertext1);
    /// let recovered_cipher_text2 = aes2.decrypt_u128(ciphertext2);
    /// println!("Recovered-ciphertext1:\t{:#034X}", recovered_cipher_text1);
    /// println!("Recovered-ciphertext2:\t{:#034X}", recovered_cipher_text2);
    /// assert_eq!(recovered_cipher_text1, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(recovered_cipher_text2, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(recovered_cipher_text1, plaintext);
    /// assert_eq!(recovered_cipher_text2, plaintext);
    /// ```
    ///
    /// # Compile-fail Example
    /// ```compile_fail
    /// use cryptocol::symmetric::AES_128;
    /// let aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// // It cannot be compiled!
    /// #[cfg(compile_fail)]    aes.encrypt_u128(0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// ```
    pub fn new_with_key<const K: usize>(key: &[u8; K]) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn new_with_key_u128(key: u128) -> Self
    /// Constructs a new object Rijndael_Generic.
    ///
    /// # Arguments
    /// - The argument `key` is of `u128`.
    /// - It should be in the same endianness of machine. For example,
    ///   if the intended key is [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD,
    ///   0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF], the key in
    ///   `u128` for this argument is 0x_1234567890ABCDEF1234567890ABCDEF_u128
    ///   for big-endian machine, and the key in `u128` for this argument is
    ///   0x_EFCDAB9078563412EFCDAB9078563412_u128 for little-endian machine.
    ///
    /// # Features
    /// - This method sets the key to be the given argument `key`.
    /// - If `NK` is less than `4`, the rest bits at the address higher than
    ///   `4 * NK` of the key given as the argument will be ignored.
    /// - If `NK` is greater than `4`, the rest bits of the key to be set after
    ///   128 bits will be set to be `zero`.
    ///
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new_with_key_u128(0xEFCDAB9078563412EFCDAB9078563412);
    /// let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    /// let ciphertext = aes.encrypt_u128(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    ///
    /// let recovered_cipher_text = aes.decrypt_u128(ciphertext);
    /// println!("Recovered-ciphertext:\t{:#034X}\n", recovered_cipher_text);
    /// assert_eq!(recovered_cipher_text, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(recovered_cipher_text, plaintext);
    /// ```
    ///
    /// # Example 2 for the case for the necessary key longer than 128 bits
    /// ```
    /// use cryptocol::symmetric::AES_256;
    ///
    /// let mut aes = AES_256::new_with_key_u128(0xEFCDAB9078563412EFCDAB9078563412);
    /// let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    /// let ciphertext = aes.encrypt_u128(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x39293C7C8FF7F9B5F76038FDCE5E3470_u128);
    ///
    /// let recovered_cipher_text = aes.decrypt_u128(ciphertext);
    /// println!("Recovered-ciphertext:\t{:#034X}\n", recovered_cipher_text);
    /// assert_eq!(recovered_cipher_text, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(recovered_cipher_text, plaintext);
    /// ```
    ///
    /// # Example 3 for the case for the necessary key shorter than the given bits
    /// ```
    /// use cryptocol::symmetric::Rijndael_Generic;
    ///
    /// let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key_u128(0xEFCDAB9078563412EFCDAB9078563412);
    /// let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    /// let ciphertext = rijndael.encrypt_u128(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x9D6C20BD28996D5570E7E05DBF20110F_u128);
    ///
    /// let recovered_cipher_text = rijndael.decrypt_u128(ciphertext);
    /// println!("Recovered-ciphertext:\t{:#034X}\n", recovered_cipher_text);
    /// assert_eq!(recovered_cipher_text, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(recovered_cipher_text, plaintext);
    /// ```
    pub fn new_with_key_u128(key: u128) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encryptor_with_key<const K: usize>(key: &[u8; K]) -> Self
    /// Constructs a new object Rijndael_Generic as a positive encryptor (or
    /// an encryptor) for the component of BigCryptor128 and NAES.
    ///
    /// # Arguments
    /// - The argument `key` is the array of u8 that has `K` elements.
    ///
    /// # Features
    /// - You won't use this method unless you use BigCryptor128 and NAES
    ///   for such as Triple AES.
    /// - This method constructs the encryptor component of BigCryptor128
    ///   and NAES.
    /// - This method sets the key to be the given argument `key`.
    /// - If `K` is less than `4 * NK`, the rest bits of
    ///   the key to be set after `K` bits will be set to be `zero`.
    /// - If `K` is greater than `4 * NK`, the rest bits after `4 * NB` bits
    ///   of the key given as the argument will be ignored.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let keys: [Box<dyn SmallCryptor<u128, 16>>; 3]
    ///         = [ Box::new(AES_128::encryptor_with_key(&[0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])),
    ///             Box::new(AES_128::decryptor_with_key(&[0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])),
    ///             Box::new(AES_128::encryptor_with_key(&[0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])) ];
    /// let mut taes = BigCryptor128::new_with_small_cryptor_array(keys);
    /// let plaintext = 0x_1234567890ABCDEFFEDCA0987654321_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_E301940B5A5DE1600D78C375BF58F232_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_1234567890ABCDEFFEDCA0987654321_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let mut taes = BigCryptor128::new()
    ///                 + AES_128::encryptor_with_key(&[0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])
    ///                 - AES_128::encryptor_with_key(&[0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])
    ///                 + AES_128::encryptor_with_key(&[0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// let plaintext = 0x_1234567890ABCDEFFEDCA0987654321_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_E301940B5A5DE1600D78C375BF58F232_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_1234567890ABCDEFFEDCA0987654321_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    #[inline]
    pub fn encryptor_with_key<const K: usize>(key: &[u8; K]) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encryptor_with_key_u128(key: u128) -> Self
    /// Constructs a new object Rijndael_Generic as a positive encryptor (or
    /// an encryptor) for the component of BigCryptor128 and NAES.
    ///
    /// # Arguments
    /// - The argument `key` is of `u128`.
    /// - It should be in the same endianness of machine. For example,
    ///   if the intended key is [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD,
    ///   0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF], the key in
    ///   `u128` for this argument is 0x_1234567890ABCDEF1234567890ABCDEF_u128
    ///   for big-endian machine, and the key in `u128` for this argument is
    ///   0x_EFCDAB9078563412EFCDAB9078563412_u128 for little-endian machine.
    ///
    /// # Features
    /// - You won't use this method unless you use BigCryptor128 and NAES
    ///   for such as Triple AES.
    /// - This method constructs the encryptor component of BigCryptor128
    ///   and NAES.
    /// - This method sets the key to be the given argument `key`.
    /// - If `NK` is less than `4`, the rest bits at the address higher than
    ///   `4 * NK` of the key given as the argument will be ignored.
    /// - If `NK` is greater than `4`, the rest bits of the key to be set after
    ///   128 bits will be set to be `zero`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let mut taes = BigCryptor128::new_with_small_cryptor_array(
    ///             [Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)),
    ///             Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///             Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128))]
    /// );
    /// let plaintext = 0x_11223344556677889900AABBCCDDEEFF_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_DB56B1B7D320D7481BF40A1964E9C7C4_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let mut taes = BigCryptor128::new()
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)
    ///                 - AES_128::encryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128);
    /// let plaintext = 0x_11223344556677889900AABBCCDDEEFF_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_DB56B1B7D320D7481BF40A1964E9C7C4_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    #[inline]
    pub fn encryptor_with_key_u128(key: u128) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decryptor_with_key<const K: usize>(key: &[u8; K]) -> Self
    /// Constructs a new object Rijndael_Generic as a negative encryptor (or
    /// a decryptor) for the component of BigCryptor128 and NAES.
    ///
    /// # Arguments
    /// - The argument `key` is the array of u8 that has `K` elements.
    ///
    /// # Features
    /// - You won't use this method unless you use BigCryptor128 and NAES
    ///   for such as Triple AES.
    /// - This method constructs the encryptor component of BigCryptor128
    ///   and NAES.
    /// - This method sets the key to be the given argument `key`.
    /// - If `K` is less than `4 * NK`, the rest bits of
    ///   the key to be set after `K` bits will be set to be `zero`.
    /// - If `K` is greater than `4 * NK`, the rest bits after `4 * NB` bits
    ///   of the key given as the argument will be ignored.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let keys: [Box<dyn SmallCryptor<u128, 16>>; 3]
    ///         = [ Box::new(AES_128::encryptor_with_key(&[0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])),
    ///             Box::new(AES_128::decryptor_with_key(&[0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])),
    ///             Box::new(AES_128::encryptor_with_key(&[0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])) ];
    /// let mut taes = BigCryptor128::new_with_small_cryptor_array(keys);
    /// let plaintext = 0x_1234567890ABCDEFFEDCA0987654321_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_E301940B5A5DE1600D78C375BF58F232_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_1234567890ABCDEFFEDCA0987654321_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let mut taes = BigCryptor128::new()
    ///                 - AES_128::decryptor_with_key(&[0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])
    ///                 + AES_128::decryptor_with_key(&[0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])
    ///                 - AES_128::decryptor_with_key(&[0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE]);
    /// let plaintext = 0x_1234567890ABCDEFFEDCA0987654321_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_E301940B5A5DE1600D78C375BF58F232_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_1234567890ABCDEFFEDCA0987654321_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    pub fn decryptor_with_key<const K: usize>(key: &[u8; K]) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decryptor_with_key_u128(key: u128) -> Self
    /// Constructs a new object Rijndael_Generic as a negative encryptor (or
    /// a decryptor) for the component of BigCryptor128 and NAES.
    ///
    /// # Arguments
    /// - The argument `key` is of `u128`.
    /// - It should be in the same endianness of machine. For example,
    ///   if the intended key is [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD,
    ///   0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF], the key in
    ///   `u128` for this argument is 0x_1234567890ABCDEF1234567890ABCDEF_u128
    ///   for big-endian machine, and the key in `u128` for this argument is
    ///   0x_EFCDAB9078563412EFCDAB9078563412_u128 for little-endian machine.
    ///
    /// # Features
    /// - You won't use this method unless you use BigCryptor128 and NAES
    ///   for such as Triple AES.
    /// - This method constructs the encryptor component of BigCryptor128
    ///   and NAES.
    /// - This method sets the key to be the given argument `key`.
    /// - If `NK` is less than `4`, the rest bits at the address higher than
    ///   `4 * NK` of the key given as the argument will be ignored.
    /// - If `NK` is greater than `4`, the rest bits of the key to be set after
    ///   128 bits will be set to be `zero`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let mut taes = BigCryptor128::new_with_small_cryptor_array(
    ///             [Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)),
    ///             Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///             Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128))]
    /// );
    /// let plaintext = 0x_11223344556677889900AABBCCDDEEFF_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_DB56B1B7D320D7481BF40A1964E9C7C4_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let mut taes = BigCryptor128::new()
    ///                 - AES_128::decryptor_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 - AES_128::decryptor_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128);
    /// let plaintext = 0x_11223344556677889900AABBCCDDEEFF_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_DB56B1B7D320D7481BF40A1964E9C7C4_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    pub fn decryptor_with_key_u128(key: u128) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn get_key(&mut self) -> [u32; NK]
    /// Gets the key.
    ///
    /// # Output
    /// This method returns the key in the form of array of `u32`.
    ///
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut _aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let key = _aes.get_key();
    /// print!("K = ");
    /// for k in key
    ///     { print!("{:#010X} ", k); }
    /// println!();
    /// assert_eq!(key, [0x78563412, 0xEFCDAB90, 0x78563412, 0xEFCDAB90]);
    /// ```
    ///
    /// # Example 2 for AES_192
    /// ```
    /// use cryptocol::symmetric::AES_192;
    ///
    /// let mut _aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let key = _aes.get_key();
    /// print!("K = ");
    /// for k in key
    ///     { print!("{:#010X} ", k); }
    /// println!();
    /// assert_eq!(key, [0x78563412_u32, 0xEFCDAB90, 0x78563412_u32, 0xEFCDAB90, 0x78563412_u32, 0xEFCDAB90]);
    /// ```
    ///
    /// # Example 3 Rijndael_Generic::<10, 4, 2>
    /// ```
    /// use cryptocol::symmetric::Rijndael_Generic;
    ///
    /// let mut _rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let key = _rijndael.get_key();
    /// print!("K = ");
    /// for k in key
    ///     { print!("{:#010X} ", k); }
    /// println!();
    /// assert_eq!(key, [0x78563412_u32, 0xEFCDAB90]);
    /// ```
    pub fn get_key(&mut self) -> [u32; NK]
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn get_key_u128(&self) -> u128
    /// Gets the key.
    ///
    /// # Output
    /// This method returns the key in the form of `u128`.
    ///
    /// # Features
    /// - If `NK` is less than `4`, the rest bits at the address higher than
    ///   `4 * NK` of the returned key will be set to be `zero`.
    /// - If `NK` is greater than `4`, the rest bits of the key to be returned
    ///   after 128 bits will be ignored.
    ///
    /// # Example 1 for 128-bit key
    /// ```
    /// use cryptocol::symmetric::AES_128;
    /// let mut _aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let key = _aes.get_key_u128();
    /// println!("Key = {:#034X}", key);
    /// assert_eq!(key, 0xEFCDAB9078563412EFCDAB9078563412_u128);
    /// ```
    ///
    /// # Example 2 for the key longer than 128 bits
    /// ```
    /// use cryptocol::symmetric::AES_192;
    /// let mut _aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    /// let key = _aes.get_key_u128();
    /// println!("Key = {:#034X}", key);
    /// assert_eq!(key, 0x8877665544332211EFCDAB9078563412_u128);
    /// ```
    ///
    /// # Example 3 or the key shorter than 128 bits
    /// ```
    /// use cryptocol::symmetric::Rijndael_Generic;
    /// let mut _rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let key = _rijndael.get_key_u128();
    /// println!("Key = {:#034X}", key);
    /// assert_eq!(key, 0x0000000000000000EFCDAB9078563412_u128);
    /// ```
    pub fn get_key_u128(&self) -> u128
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_key<const K: usize>(&mut self, key: &[u8; K])
    /// Sets the key.
    ///
    /// # Arguments
    /// - The argument `key` is the array of `u8` that has `K` elements.
    ///
    /// # Features
    /// - This method sets the key to be the given argument `key`.
    /// - If `K` is less than `4 * NK`, the rest bits of
    ///   the key to be set after `K` bits will be set to be `zero`.
    /// - If `K` is greater than `4 * NK`, the rest bits after `4 * NK` bits
    ///   of the key given as the argument will be ignored.
    ///
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new();
    /// aes.set_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let key = aes.get_key();
    /// print!("K = ");
    /// for k in key
    ///     { print!("{:#010X} ", k); }
    /// println!();
    /// assert_eq!(key, [0x78563412, 0xEFCDAB90, 0x78563412, 0xEFCDAB90]);
    /// ```
    ///
    /// # Example 2 for the necessary key longer than the given key
    /// ```
    /// use cryptocol::symmetric::AES_192;
    ///
    /// let mut aes = AES_192::new();
    /// aes.set_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let key = aes.get_key();
    /// print!("K = ");
    /// for k in key
    ///     { print!("{:#010X} ", k); }
    /// println!();
    /// assert_eq!(key, [0x78563412_u32, 0xEFCDAB90, 0x78563412_u32, 0xEFCDAB90, 0x00000000, 0x00000000]);
    /// ```
    ///
    /// # Example 3 for the necessary key shorter than the given key
    ///
    /// ```
    /// use cryptocol::symmetric::Rijndael_Generic;
    ///
    /// let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key_u128(0xEFCDAB9078563412EFCDAB9078563412);
    /// rijndael.set_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let key = rijndael.get_key();
    /// print!("K = ");
    /// for k in key
    ///     { print!("{:#010X} ", k); }
    /// println!();
    /// assert_eq!(key, [0x78563412_u32, 0xEFCDAB90]);
    /// ```
    pub fn set_key<const K: usize>(&mut self, key: &[u8; K])
    {
        unimplemented!(); // Dummy code for documentation
    }

    //  pub fn set_key_u128(&mut self, key: u128)
    /// Constructs a new object Rijndael_Generic.
    ///
    /// # Arguments
    /// - The argument `key` is of `u128`.
    /// - It should be in the same endianness of machine. For example,
    ///   if the intended key is [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD,
    ///   0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF], the key in
    ///   `u128` for this argument is 0x_1234567890ABCDEF1234567890ABCDEF_u128
    ///   for big-endian machine, and the key in `u128` for this argument is
    ///   0x_EFCDAB9078563412EFCDAB9078563412_u128 for little-endian machine.
    ///
    /// # Features
    /// - This method sets the key to be the given argument `key`.
    /// - If `NK` is less than `4`, the rest bits at the address higher than
    ///   `4 * NK` of the key given as the argument will be ignored.
    /// - If `NK` is greater than `4`, the rest bits of the key to be set after
    ///   128 bits will be set to be `zero`.
    ///
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new();
    /// aes.set_key_u128(0xEFCDAB9078563412EFCDAB9078563412);
    /// let key = aes.get_key();
    /// print!("K = ");
    /// for k in key
    ///     { print!("{:#010X} ", k); }
    /// println!();
    /// assert_eq!(key, [0x78563412, 0xEFCDAB90, 0x78563412, 0xEFCDAB90]);
    /// ```
    ///
    /// # Example 2 for the necessary key longer than 128 bits
    /// ```
    /// use cryptocol::symmetric::AES_256;
    ///
    /// let mut aes = AES_256::new();
    /// aes.set_key_u128(0xEFCDAB9078563412EFCDAB9078563412);
    /// let key = aes.get_key();
    /// print!("K = ");
    /// for k in key
    ///     { print!("{:#010X} ", k); }
    /// println!();
    /// assert_eq!(key, [0x78563412_u32, 0xEFCDAB90, 0x78563412_u32, 0xEFCDAB90, 0x00000000, 0x00000000, 0x00000000, 0x00000000]);
    /// ```
    ///
    /// # Example 3 for the necessary key shorter than 128 bits
    /// ```
    /// use cryptocol::symmetric::Rijndael_Generic;
    ///
    /// let mut rijndael = Rijndael_Generic::<10, 4, 2>::new();
    /// rijndael.set_key_u128(0xEFCDAB9078563412EFCDAB9078563412);
    /// let key = rijndael.get_key();
    /// print!("K = ");
    /// for k in key
    ///     { print!("{:#010X} ", k); }
    /// println!();
    /// assert_eq!(key, [0x78563412_u32, 0xEFCDAB90]);
    /// ```
    pub fn set_key_u128(&mut self, key: u128)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn turn_inverse(&mut self)
    /// Flips its role in BigCryptor128.
    ///
    /// # Features
    /// - You won't use this method unless you use NDES for such as Triple DES.
    /// - Even if you are writing codes in the context of using NDES,
    ///   you will hardly use this method because it is high chance that you
    ///   will have constructed components with the methods,
    ///   encryptor_with_key(struct@DES_Generic#method.encryptor_with_key),
    ///   encryptor_with_key_u64(struct@DES_Generic#method.encryptor_with_key_u64),
    ///   decryptor_with_key(struct@DES_Generic#method.decryptor_with_key), and
    ///   decryptor_with_key_u64(struct@DES_Generic#method.decryptor_with_key_u64).
    /// - If it is constructed as encryptor for NDES,
    ///   it will be changed into decryptor.
    /// - If it is constructed as decryptor for NDES,
    ///   it will be changed into encryptor.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let mut keys: [Box<dyn SmallCryptor<u128, 16>>; 3]
    ///             = [ Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)),
    ///                 Box::new(AES_128::new_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///                 Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)) ];
    /// keys[1].turn_inverse();
    /// 
    /// let mut taes = BigCryptor128::new_with_small_cryptor_array(keys);
    /// let plaintext = 0x_11223344556677889900AABBCCDDEEFF_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_DB56B1B7D320D7481BF40A1964E9C7C4_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let des1 = AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128);
    /// let mut des2 = AES_128::new_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128);
    /// let des3 = AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128);
    /// des2.turn_inverse();
    /// 
    /// let mut tdes = BigCryptor128::new() + des1 + des2 + des3; 
    /// let plaintext = 0x_11223344556677889900AABBCCDDEEFF_u128;
    /// let ciphertext = tdes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_DB56B1B7D320D7481BF40A1964E9C7C4_u128);
    /// 
    /// let cipher_cipher_text = tdes.decrypt_u128(ciphertext);
    /// println!("Cipher-ciphertext:\t{:#034X}", cipher_cipher_text);
    /// assert_eq!(cipher_cipher_text, 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(cipher_cipher_text, plaintext);
    /// ```
    #[inline]
    pub fn turn_inverse(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn turn_encryptor(&mut self)
    /// Changes its role in BigCryptor128 or NAES to encryptor.
    ///
    /// # Features
    /// - You won't use this method unless you use BigCryptor128 or NAES
    ///   for such as Triple AES.
    /// - Even if you are writing codes in the context of using BigCryptor128
    ///   or NAES, you will hardly use this method because it is high chance
    ///   that you will have constructed components with the methods,
    ///   [encryptor_with_key](struct@Rijndael_Generic#method.encryptor_with_key),
    ///   [encryptor_with_key_u64](struct@Rijndael_Generic#method.encryptor_with_key_u64),
    ///   [decryptor_with_key](struct@Rijndael_Generic#method.decryptor_with_key), and
    ///   [decryptor_with_key_u64](struct@Rijndael_Generic#method.decryptor_with_key_u64).
    /// - If it is constructed as encryptor for BigCryptor128 or NAES,
    ///   it will not be changed at all.
    /// - If it is constructed as decryptor for BigCryptor128 or NAES,
    ///   it will be changed into encryptor.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let mut keys: [Box<dyn SmallCryptor<u128, 16>>; 3]
    ///         = [ Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)),
    ///             Box::new(AES_128::new_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///             Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)) ];
    /// keys[0].turn_encryptor();
    /// 
    /// let mut taes = BigCryptor128::new_with_small_cryptor_array(keys);
    /// let plaintext = 0x_11223344556677889900AABBCCDDEEFF_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_5C842CA9ECB742B2F3164BC33E0BDCB6_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let mut des1 = AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128);
    /// let des2 = AES_128::new_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128);
    /// let des3 = AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128);
    /// des1.turn_encryptor();
    /// 
    /// let mut taes = BigCryptor128::new() + des1 + des2 + des3; 
    /// let plaintext = 0x_11223344556677889900AABBCCDDEEFF_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_5C842CA9ECB742B2F3164BC33E0BDCB6_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    pub fn turn_encryptor(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn turn_decryptor(&mut self)
    /// Changes its role in BigCryptor128 or NAES to decryptor.
    ///
    /// # Features
    /// - You won't use this method unless you use BBigCryptor128 or NAES
    ///   for such as Triple DES.
    /// - Even if you are writing codes in the context of using BigCryptor128
    ///   or NAES, you will hardly use this method because it is high chance
    ///   that you will have constructed components with the methods,
    ///   [encryptor_with_key](struct@Rijndael_Generic#method.encryptor_with_key),
    ///   [encryptor_with_key_u128](struct@Rijndael_Generic#method.encryptor_with_key_u128),
    ///   [decryptor_with_key](struct@Rijndael_Generic#method.decryptor_with_key), and
    ///   [decryptor_with_key_u128](struct@Rijndael_Generic#method.decryptor_with_key_u128).
    /// - If it is constructed as encryptor for BigCryptor128 or NAES,
    ///   it will be changed into decryptor.
    /// - If it is constructed as decryptor for BigCryptor128 or NAES,
    ///   it will not be changed at all.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let mut keys: [Box<dyn SmallCryptor<u128, 16>>; 3]
    ///             = [ Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)),
    ///                 Box::new(AES_128::new_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///                 Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)) ];
    /// keys[1].turn_decryptor();
    /// 
    /// let mut taes = BigCryptor128::new_with_small_cryptor_array(keys);
    /// let plaintext = 0x_11223344556677889900AABBCCDDEEFF_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_DB56B1B7D320D7481BF40A1964E9C7C4_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let des1 = AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128);
    /// let mut des2 = AES_128::new_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128);
    /// let des3 = AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128);
    /// des2.turn_decryptor();
    /// 
    /// let mut taes = BigCryptor128::new() + des1 + des2 + des3; 
    /// let plaintext = 0x_11223344556677889900AABBCCDDEEFF_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_DB56B1B7D320D7481BF40A1964E9C7C4_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    pub fn turn_decryptor(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    //  pub fn encrypt_unit(&mut self, message: &[IntUnion; NB]) -> [IntUnion; NB]
    /// Encrypts data in the form of an array of IntUnion with `NB` elements.
    ///
    /// # Arguments
    /// `message` is of the type `&[IntUnion; NB]`
    /// and the plaintext to be encrypted.
    ///
    /// # Output
    /// This method returns the encrypted data in the array of `IntUnion`
    /// with `NB` elements.
    ///
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = [IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678), IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678)];
    /// let ciphertext = aes.encrypt_unit(&plaintext);
    ///
    /// println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}", plaintext[0].get(), plaintext[1].get(), plaintext[2].get(), plaintext[3].get());
    /// println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}", ciphertext[0].get(), ciphertext[1].get(), ciphertext[2].get(), ciphertext[3].get());
    /// assert_eq!(ciphertext[0].get(), 0x27584D87);
    /// assert_eq!(ciphertext[1].get(), 0x44E2BAE9);
    /// assert_eq!(ciphertext[2].get(), 0x4AECB5D6);
    /// assert_eq!(ciphertext[3].get(), 0x01CCF826);
    /// ```
    ///
    /// # Example 2 for AES_192
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_192;
    ///
    /// let mut aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = [IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678), IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678)];
    /// let ciphertext = aes.encrypt_unit(&plaintext);
    ///
    /// println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}", plaintext[0].get(), plaintext[1].get(), plaintext[2].get(), plaintext[3].get());
    /// println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}", ciphertext[0].get(), ciphertext[1].get(), ciphertext[2].get(), ciphertext[3].get());
    /// assert_eq!(ciphertext[0].get(), 0x77047F1E);
    /// assert_eq!(ciphertext[1].get(), 0x008E2C5B);
    /// assert_eq!(ciphertext[2].get(), 0x6E5EB091);
    /// assert_eq!(ciphertext[3].get(), 0x0DB5608E);
    /// ```
    ///
    /// # Example 3 for Rijndael_Generic::<10, 4, 2>
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::Rijndael_Generic;
    ///
    /// let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = [IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678), IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678)];
    /// let ciphertext = rijndael.encrypt_unit(&plaintext);
    ///
    /// println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}", plaintext[0].get(), plaintext[1].get(), plaintext[2].get(), plaintext[3].get());
    /// println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}", ciphertext[0].get(), ciphertext[1].get(), ciphertext[2].get(), ciphertext[3].get());
    /// assert_eq!(ciphertext[0].get(), 0xBF20110F);
    /// assert_eq!(ciphertext[1].get(), 0x70E7E05D);
    /// assert_eq!(ciphertext[2].get(), 0x28996D55);
    /// assert_eq!(ciphertext[3].get(), 0x9D6C20BD);
    /// ```
    pub fn encrypt_unit(&mut self, message: &[IntUnion; NB]) -> [IntUnion; NB]
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
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = 0x1234567890ABCDEF1234567890ABCDEF;
    /// let ciphertext = aes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    /// ```
    ///
    /// # Example 2 for AES_256
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_256;
    ///
    /// let mut aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = 0x1234567890ABCDEF1234567890ABCDEF;
    /// let ciphertext = aes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x0DB5608E6E5EB091008E2C5B77047F1E_u128);
    /// ```
    ///
    /// # Example 3 for Rijndael_Generic::<10, 4, 2>
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::Rijndael_Generic;
    ///
    /// let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = 0x1234567890ABCDEF1234567890ABCDEF;
    /// let ciphertext = rijndael.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x9D6C20BD28996D5570E7E05DBF20110F_u128);
    /// ```
    pub fn encrypt_u128(&mut self, message: u128) -> u128
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_unit(&mut self, cipher: &[IntUnion; NB]) -> [IntUnion; NB]
    /// Decrypts data in the form of an array of IntUnion with `NB` elements.
    ///
    /// # Arguments
    /// `cipher` is of the type `&[IntUnion; NB]`
    /// and the ciphertext to be decrypted.
    ///
    /// # Output
    /// This method returns the decrypted data in the array of `IntUnion`
    /// with `NB` elements.
    ///
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let ciphertext = [IntUnion::new_with(0x27584D87), IntUnion::new_with(0x44E2BAE9), IntUnion::new_with(0x4AECB5D6), IntUnion::new_with(0x01CCF826)];
    /// let plaintext = aes.decrypt_unit(&ciphertext);
    ///
    /// println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}", ciphertext[0].get(), ciphertext[1].get(), ciphertext[2].get(), ciphertext[3].get());
    /// println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}", plaintext[0].get(), plaintext[1].get(), plaintext[2].get(), plaintext[3].get());
    /// assert_eq!(plaintext[0].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[1].get(), 0x12345678);
    /// assert_eq!(plaintext[2].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[3].get(), 0x12345678);
    /// ```
    ///
    /// # Example 2 for AES_192
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_192;
    ///
    /// let mut aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let ciphertext = [IntUnion::new_with(0x77047F1E), IntUnion::new_with(0x008E2C5B), IntUnion::new_with(0x6E5EB091), IntUnion::new_with(0x0DB5608E)];
    /// let plaintext = aes.decrypt_unit(&ciphertext);
    ///
    /// println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}", ciphertext[0].get(), ciphertext[1].get(), ciphertext[2].get(), ciphertext[3].get());
    /// println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}", plaintext[0].get(), plaintext[1].get(), plaintext[2].get(), plaintext[3].get());
    /// assert_eq!(plaintext[0].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[1].get(), 0x12345678);
    /// assert_eq!(plaintext[2].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[3].get(), 0x12345678);
    /// ```
    ///
    /// # Example 3 for Rijndael_Generic::<10, 4, 2>
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::Rijndael_Generic;
    ///
    /// let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = [IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678), IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678)];
    /// let ciphertext = rijndael.encrypt_unit(&plaintext);
    ///
    /// println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}", plaintext[0].get(), plaintext[1].get(), plaintext[2].get(), plaintext[3].get());
    /// println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}", ciphertext[0].get(), ciphertext[1].get(), ciphertext[2].get(), ciphertext[3].get());
    /// assert_eq!(ciphertext[0].get(), 0xBF20110F);
    /// assert_eq!(ciphertext[1].get(), 0x70E7E05D);
    /// assert_eq!(ciphertext[2].get(), 0x28996D55);
    /// assert_eq!(ciphertext[3].get(), 0x9D6C20BD);
    /// ```
    pub fn decrypt_unit(&mut self, cipher: &[IntUnion; NB]) -> [IntUnion; NB]
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
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let ciphertext = 0x01CCF8264AECB5D644E2BAE927584D87_u128;
    /// let plaintext = aes.decrypt_u128(ciphertext);
    ///
    /// println!("Ciphertext:\t{:#034X}", ciphertext);
    /// println!("Plaintext:\t{:#034X}", plaintext);
    /// assert_eq!(plaintext, 0x1234567890ABCDEF1234567890ABCDEF);
    /// ```
    ///
    /// # Example 2 for AES_192
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_192;
    ///
    /// let mut aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let ciphertext = 0x0DB5608E6E5EB091008E2C5B77047F1E_u128;
    /// let plaintext = aes.decrypt_u128(ciphertext);
    ///
    /// println!("Ciphertext:\t{:#034X}", ciphertext);
    /// println!("Plaintext:\t{:#034X}", plaintext);
    /// assert_eq!(plaintext, 0x1234567890ABCDEF1234567890ABCDEF);
    /// ```
    ///
    /// # Example 3 for  Rijndael_Generic::<10, 4, 2>
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::Rijndael_Generic;
    ///
    /// let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let ciphertext = 0x9D6C20BD28996D5570E7E05DBF20110F_u128;
    /// let plaintext = rijndael.decrypt_u128(ciphertext);
    ///
    /// println!("Ciphertext:\t{:#034X}", ciphertext);
    /// println!("Plaintext:\t{:#034X}", plaintext);
    /// assert_eq!(plaintext, 0x1234567890ABCDEF1234567890ABCDEF);
    /// ```
    pub fn decrypt_u128(&mut self, cipher: u128) -> u128
    {
        unimplemented!(); // Dummy code for documentation
    }

    #[inline]
    pub(super) fn _encrypt(&mut self, message: &[IntUnion; NB]) -> [IntUnion; NB]
    {
        unimplemented!(); // Dummy code for documentation
    }

    #[inline]
    pub(super) fn _decrypt(&mut self, cipher: &[IntUnion; NB]) -> [IntUnion; NB]
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_array_unit<const N: usize>(&mut self, message: &[[IntUnion; NB]; N], cipher: &mut [[IntUnion; NB]; N])
    /// Encrypts an array of unit data, `[[IntUnion; NB]; N]`.
    ///
    /// # Arguments
    /// - `message` is of an array of `[IntUnion; NB]`-type and the plaintext
    ///   to be encrypted.
    /// - `cipher` is of an array of `[IntUnion; NB]`-type and the ciphertext
    ///   to be stored.
    ///
    /// # Features
    /// This method encrypts multiple of 64-bit data without padding anything
    /// in ECB (Electronic CodeBook) mode.
    ///
    /// # Counterpart methods
    /// - If you need to encrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in ECB operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::ECB_PKCS7`.
    ///   see [here](./traits_ecb_with_padding_pkcs7/trait.ECB_PKCS7.html)
    /// - If you need to encrypt data with padding bits according ISO
    ///   in ECB operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::ECB_ISO`.
    ///   see [here](./traits_ecb_with_padding_iso/trait.ECB_ISO.html)
    /// - If you need to encrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in CBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::CBC_PKCS7`.
    ///   see [here](./traits_cbc_with_padding_pkcs7/trait.CBC_PKCS7.html)
    /// - If you need to encrypt data with padding bits according ISO
    ///   in CBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::CBC_ISO`.
    ///   see [here](./traits_cbc_with_padding_iso/trait.CBC_ISO.html)
    /// - If you need to encrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in PCBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::PCBC_PKCS7`.
    ///   see [here](./traits_pcbc_with_padding_pkcs7/trait.PCBC_PKCS7.html)
    /// - If you need to encrypt data with padding bits according ISO
    ///   in PCBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::PCBC_ISO`.
    ///   see [here](./traits_pcbc_with_padding_iso/trait.PCBC_ISO.html)
    /// - If you need to encrypt data in CFB operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::CFB`.
    ///   see [here](./traits_cfb/trait.CFB.html)
    /// - If you need to encrypt data in OFB operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::OFB`.
    ///   see [here](./traits_ofb/trait.OFB.html)
    /// - If you need to encrypt data in CTR operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::CTR`.
    ///   see [here](./traits_ctr/trait.CTR.html)
    ///
    /// In summary,
    ///
    /// |      | padding PKCS7                                                        | padding ISO                                                    | no padding                         |
    /// |------|----------------------------------------------------------------------|----------------------------------------------------------------|------------------------------------|
    /// | ECB  | [ECB_PKCS7](./traits_ecb_with_padding_pkcs7/trait.ECB_PKCS7.html)    | [ECB_ISO](./traits_ecb_with_padding_iso/trait.ECB_ISO.html)    |                                    |
    /// | CBC  | [CBC_PKCS7](./traits_cbc_with_padding_pkcs7/trait.CBC_PKCS7.html)    | [CBC_ISO](./traits_cbc_with_padding_iso/trait.CBC_ISO.html)    |                                    |
    /// | PCBC | [PCBC_PKCS7](./traits_pcbc_with_padding_pkcs7/trait.PCBC_PKCS7.html) | [PCBC_ISO](./traits_pcbc_with_padding_iso/trait.PCBC_ISO.html) |                                    |
    /// | CFB  |                                                                      |                                                                | [CFB](./traits_cfb/trait.CFB.html) |
    /// | OFB  |                                                                      |                                                                | [OFB](./traits_ofb/trait.OFB.html) |
    /// | CTR  |                                                                      |                                                                | [CTR](./traits_ctr/trait.CTR.html) |
    ///
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = [[IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678), IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678)]; 3];
    /// let mut ciphertext = [[IntUnion::new(); 4]; 3];
    /// aes.encrypt_array_unit(&plaintext, &mut ciphertext);
    /// 
    /// println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
    ///         plaintext[0][0].get(), plaintext[0][1].get(), plaintext[0][2].get(), plaintext[0][3].get(),
    ///         plaintext[1][0].get(), plaintext[1][1].get(), plaintext[1][2].get(), plaintext[1][3].get(),
    ///         plaintext[2][0].get(), plaintext[2][1].get(), plaintext[2][2].get(), plaintext[2][3].get());
    /// println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
    ///         ciphertext[0][0].get(), ciphertext[0][1].get(), ciphertext[0][2].get(), ciphertext[0][3].get(),
    ///         ciphertext[1][0].get(), ciphertext[1][1].get(), ciphertext[1][2].get(), ciphertext[1][3].get(),
    ///         ciphertext[2][0].get(), ciphertext[2][1].get(), ciphertext[2][2].get(), ciphertext[2][3].get());
    /// assert_eq!(ciphertext[0][0].get(), 0x27584D87);
    /// assert_eq!(ciphertext[0][1].get(), 0x44E2BAE9);
    /// assert_eq!(ciphertext[0][2].get(), 0x4AECB5D6);
    /// assert_eq!(ciphertext[0][3].get(), 0x01CCF826);
    /// assert_eq!(ciphertext[1][0].get(), 0x27584D87);
    /// assert_eq!(ciphertext[1][1].get(), 0x44E2BAE9);
    /// assert_eq!(ciphertext[1][2].get(), 0x4AECB5D6);
    /// assert_eq!(ciphertext[1][3].get(), 0x01CCF826);
    /// assert_eq!(ciphertext[2][0].get(), 0x27584D87);
    /// assert_eq!(ciphertext[2][1].get(), 0x44E2BAE9);
    /// assert_eq!(ciphertext[2][2].get(), 0x4AECB5D6);
    /// assert_eq!(ciphertext[2][3].get(), 0x01CCF826);
    /// ```
    /// 
    /// # Example 2 for AES_192
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_192;
    /// let mut aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = [[IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678), IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678)]; 3];
    /// let mut ciphertext = [[IntUnion::new(); 4]; 3];
    ///  aes.encrypt_array_unit(&plaintext, &mut ciphertext);
    /// 
    /// println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
    ///         plaintext[0][0].get(), plaintext[0][1].get(), plaintext[0][2].get(), plaintext[0][3].get(),
    ///         plaintext[1][0].get(), plaintext[1][1].get(), plaintext[1][2].get(), plaintext[1][3].get(),
    ///         plaintext[2][0].get(), plaintext[2][1].get(), plaintext[2][2].get(), plaintext[2][3].get());
    /// println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
    ///         ciphertext[0][0].get(), ciphertext[0][1].get(), ciphertext[0][2].get(), ciphertext[0][3].get(),
    ///         ciphertext[1][0].get(), ciphertext[1][1].get(), ciphertext[1][2].get(), ciphertext[1][3].get(),
    ///         ciphertext[2][0].get(), ciphertext[2][1].get(), ciphertext[2][2].get(), ciphertext[2][3].get());
    /// assert_eq!(ciphertext[0][0].get(), 0x77047F1E);
    /// assert_eq!(ciphertext[0][1].get(), 0x008E2C5B);
    /// assert_eq!(ciphertext[0][2].get(), 0x6E5EB091);
    /// assert_eq!(ciphertext[0][3].get(), 0x0DB5608E);
    /// assert_eq!(ciphertext[1][0].get(), 0x77047F1E);
    /// assert_eq!(ciphertext[1][1].get(), 0x008E2C5B);
    /// assert_eq!(ciphertext[1][2].get(), 0x6E5EB091);
    /// assert_eq!(ciphertext[1][3].get(), 0x0DB5608E);
    /// assert_eq!(ciphertext[2][0].get(), 0x77047F1E);
    /// assert_eq!(ciphertext[2][1].get(), 0x008E2C5B);
    /// assert_eq!(ciphertext[2][2].get(), 0x6E5EB091);
    /// assert_eq!(ciphertext[2][3].get(), 0x0DB5608E);
    /// ```
    /// 
    /// # Example 3 for Rijndael_Generic::<10, 4, 2>
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::Rijndael_Generic;
    /// let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = [[IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678), IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678)]; 3];
    /// let mut ciphertext = [[IntUnion::new(); 4]; 3];
    /// rijndael.encrypt_array_unit(&plaintext, &mut ciphertext);
    /// 
    /// println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
    ///         plaintext[0][0].get(), plaintext[0][1].get(), plaintext[0][2].get(), plaintext[0][3].get(),
    ///         plaintext[1][0].get(), plaintext[1][1].get(), plaintext[1][2].get(), plaintext[1][3].get(),
    ///         plaintext[2][0].get(), plaintext[2][1].get(), plaintext[2][2].get(), plaintext[2][3].get());
    /// println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
    ///         ciphertext[0][0].get(), ciphertext[0][1].get(), ciphertext[0][2].get(), ciphertext[0][3].get(),
    ///         ciphertext[1][0].get(), ciphertext[1][1].get(), ciphertext[1][2].get(), ciphertext[1][3].get(),
    ///         ciphertext[2][0].get(), ciphertext[2][1].get(), ciphertext[2][2].get(), ciphertext[2][3].get());
    /// assert_eq!(ciphertext[0][0].get(), 0xBF20110F);
    /// assert_eq!(ciphertext[0][1].get(), 0x70E7E05D);
    /// assert_eq!(ciphertext[0][2].get(), 0x28996D55);
    /// assert_eq!(ciphertext[0][3].get(), 0x9D6C20BD);
    /// assert_eq!(ciphertext[1][0].get(), 0xBF20110F);
    /// assert_eq!(ciphertext[1][1].get(), 0x70E7E05D);
    /// assert_eq!(ciphertext[1][2].get(), 0x28996D55);
    /// assert_eq!(ciphertext[1][3].get(), 0x9D6C20BD);
    /// assert_eq!(ciphertext[2][0].get(), 0xBF20110F);
    /// assert_eq!(ciphertext[2][1].get(), 0x70E7E05D);
    /// assert_eq!(ciphertext[2][2].get(), 0x28996D55);
    /// assert_eq!(ciphertext[2][3].get(), 0x9D6C20BD);
    /// ```
    pub fn encrypt_array_unit<const N: usize>(&mut self, message: &[[IntUnion; NB]; N], cipher: &mut [[IntUnion; NB]; N])
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_array_u128<const N: usize>(&mut self, message: &[u128; N], cipher: &mut [u128; N])
    /// Encrypts an array of 128-bit data.
    ///
    /// # Arguments
    /// - `message` is of an array of `u128`-type and the plaintext
    ///   to be encrypted.
    /// - `cipher` is of an array of `u128`-type and the ciphertext
    ///   to be stored.
    ///
    /// # Features
    /// This method encrypts multiple of 128-bit data without padding anything
    /// in ECB (Electronic CodeBook) mode.
    ///
    /// # Counterpart methods
    /// - If you need to encrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in ECB operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::ECB_PKCS7`.
    ///   see [here](./traits_ecb_with_padding_pkcs7/trait.ECB_PKCS7.html)
    /// - If you need to encrypt data with padding bits according ISO
    ///   in ECB operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::ECB_ISO`.
    ///   see [here](./traits_ecb_with_padding_iso/trait.ECB_ISO.html)
    /// - If you need to encrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in CBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::CBC_PKCS7`.
    ///   see [here](./traits_cbc_with_padding_pkcs7/trait.CBC_PKCS7.html)
    /// - If you need to encrypt data with padding bits according ISO
    ///   in CBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::CBC_ISO`.
    ///   see [here](./traits_cbc_with_padding_iso/trait.CBC_ISO.html)
    /// - If you need to encrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in PCBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::PCBC_PKCS7`.
    ///   see [here](./traits_pcbc_with_padding_pkcs7/trait.PCBC_PKCS7.html)
    /// - If you need to encrypt data with padding bits according ISO
    ///   in PCBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::PCBC_ISO`.
    ///   see [here](./traits_pcbc_with_padding_iso/trait.PCBC_ISO.html)
    /// - If you need to encrypt data in CFB operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::CFB`.
    ///   see [here](./traits_cfb/trait.CFB.html)
    /// - If you need to encrypt data in OFB operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::OFB`.
    ///   see [here](./traits_ofb/trait.OFB.html)
    /// - If you need to encrypt data in CTR operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::CTR`.
    ///   see [here](./traits_ctr/trait.CTR.html)
    ///
    /// In summary,
    ///
    /// |      | padding PKCS7                                                        | padding ISO                                                    | no padding                         |
    /// |------|----------------------------------------------------------------------|----------------------------------------------------------------|------------------------------------|
    /// | ECB  | [ECB_PKCS7](./traits_ecb_with_padding_pkcs7/trait.ECB_PKCS7.html)    | [ECB_ISO](./traits_ecb_with_padding_iso/trait.ECB_ISO.html)    |                                    |
    /// | CBC  | [CBC_PKCS7](./traits_cbc_with_padding_pkcs7/trait.CBC_PKCS7.html)    | [CBC_ISO](./traits_cbc_with_padding_iso/trait.CBC_ISO.html)    |                                    |
    /// | PCBC | [PCBC_PKCS7](./traits_pcbc_with_padding_pkcs7/trait.PCBC_PKCS7.html) | [PCBC_ISO](./traits_pcbc_with_padding_iso/trait.PCBC_ISO.html) |                                    |
    /// | CFB  |                                                                      |                                                                | [CFB](./traits_cfb/trait.CFB.html) |
    /// | OFB  |                                                                      |                                                                | [OFB](./traits_ofb/trait.OFB.html) |
    /// | CTR  |                                                                      |                                                                | [CTR](./traits_ctr/trait.CTR.html) |
    ///
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_128;
    /// 
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = [0x1234567890ABCDEF1234567890ABCDEF_u128, 0x1234567890ABCDEF1234567890ABCDEF, 0x1234567890ABCDEF1234567890ABCDEF];
    /// let mut ciphertext = [0_u128; 3];
    /// aes.encrypt_array_u128(&plaintext, &mut ciphertext);
    /// 
    /// println!("Plaintext:\t{:#034X} {:#034X} {:#034X}", plaintext[0], plaintext[1], plaintext[2]);
    /// println!("Ciphertext:\t{:#034X} {:#034X} {:#034X}", ciphertext[0], ciphertext[1], ciphertext[2]);
    /// assert_eq!(ciphertext[0], 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    /// assert_eq!(ciphertext[1], 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    /// assert_eq!(ciphertext[2], 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    /// ```
    /// 
    /// # Example 2 for AES_192
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_192;
    /// 
    /// let mut aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = [0x1234567890ABCDEF1234567890ABCDEF_u128, 0x1234567890ABCDEF1234567890ABCDEF, 0x1234567890ABCDEF1234567890ABCDEF];
    /// let mut ciphertext = [0_u128; 3];
    /// aes.encrypt_array_u128(&plaintext, &mut ciphertext);
    /// 
    /// println!("Plaintext:\t{:#034X} {:#034X} {:#034X}", plaintext[0], plaintext[1], plaintext[2]);
    /// println!("Ciphertext:\t{:#034X} {:#034X} {:#034X}", ciphertext[0], ciphertext[1], ciphertext[2]);
    /// assert_eq!(ciphertext[0], 0x0DB5608E6E5EB091008E2C5B77047F1E_u128);
    /// assert_eq!(ciphertext[1], 0x0DB5608E6E5EB091008E2C5B77047F1E_u128);
    /// assert_eq!(ciphertext[2], 0x0DB5608E6E5EB091008E2C5B77047F1E_u128);
    /// ```
    /// 
    /// # Example 3 for Rijndael_Generic::<10, 4, 2>
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::Rijndael_Generic;
    /// 
    /// let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = [0x1234567890ABCDEF1234567890ABCDEF_u128, 0x1234567890ABCDEF1234567890ABCDEF, 0x1234567890ABCDEF1234567890ABCDEF];
    /// let mut ciphertext = [0_u128; 3];
    /// rijndael.encrypt_array_u128(&plaintext, &mut ciphertext);
    /// 
    /// println!("Plaintext:\t{:#034X} {:#034X} {:#034X}", plaintext[0], plaintext[1], plaintext[2]);
    /// println!("Ciphertext:\t{:#034X} {:#034X} {:#034X}", ciphertext[0], ciphertext[1], ciphertext[2]);
    /// assert_eq!(ciphertext[0], 0x9D6C20BD28996D5570E7E05DBF20110F_u128);
    /// assert_eq!(ciphertext[1], 0x9D6C20BD28996D5570E7E05DBF20110F_u128);
    /// assert_eq!(ciphertext[2], 0x9D6C20BD28996D5570E7E05DBF20110F_u128);
    /// ```
    pub fn encrypt_array_u128<const N: usize>(&mut self, message: &[u128; N], cipher: &mut [u128; N])
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_array_unit<const N: usize>(&mut self, cipher: &[[IntUnion; NB]; N], message: &mut [[IntUnion; NB]; N])
    /// Decrypts an array of unit data, `[[IntUnion; NB]; N]`.
    ///
    /// # Arguments
    /// - `cipher` is of an array of `[IntUnion; NB]`-type and the ciphertext
    ///   to be encrypted.
    /// - `message` is of an array of `[IntUnion; NB]`-type and the plaintext
    ///   to be stored.
    ///
    /// # Features
    /// This method decrypts multiple of 64-bit data without padding anything
    /// in ECB (Electronic CodeBook) mode.
    ///
    /// # Counterpart methods
    /// - If you need to decrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in ECB operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::ECB_PKCS7`.
    ///   see [here](./traits_ecb_with_padding_pkcs7/trait.ECB_PKCS7.html)
    /// - If you need to decrypt data with padding bits according ISO
    ///   in ECB operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::ECB_ISO`.
    ///   see [here](./traits_ecb_with_padding_iso/trait.ECB_ISO.html)
    /// - If you need to decrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in CBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::CBC_PKCS7`.
    ///   see [here](./traits_cbc_with_padding_pkcs7/trait.CBC_PKCS7.html)
    /// - If you need to decrypt data with padding bits according ISO
    ///   in CBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::CBC_ISO`.
    ///   see [here](./traits_cbc_with_padding_iso/trait.CBC_ISO.html)
    /// - If you need to decrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in PCBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::PCBC_PKCS7`.
    ///   see [here](./traits_pcbc_with_padding_pkcs7/trait.PCBC_PKCS7.html)
    /// - If you need to decrypt data with padding bits according ISO
    ///   in PCBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::PCBC_ISO`.
    ///   see [here](./traits_pcbc_with_padding_iso/trait.PCBC_ISO.html)
    /// - If you need to decrypt data in CFB operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::CFB`.
    ///   see [here](./traits_cfb/trait.CFB.html)
    /// - If you need to decrypt data in OFB operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::OFB`.
    ///   see [here](./traits_ofb/trait.OFB.html)
    /// - If you need to decrypt data in CTR operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::CTR`.
    ///   see [here](./traits_ctr/trait.CTR.html)
    ///
    /// In summary,
    ///
    /// |      | padding PKCS7                                                        | padding ISO                                                    | no padding                         |
    /// |------|----------------------------------------------------------------------|----------------------------------------------------------------|------------------------------------|
    /// | ECB  | [ECB_PKCS7](./traits_ecb_with_padding_pkcs7/trait.ECB_PKCS7.html)    | [ECB_ISO](./traits_ecb_with_padding_iso/trait.ECB_ISO.html)    |                                    |
    /// | CBC  | [CBC_PKCS7](./traits_cbc_with_padding_pkcs7/trait.CBC_PKCS7.html)    | [CBC_ISO](./traits_cbc_with_padding_iso/trait.CBC_ISO.html)    |                                    |
    /// | PCBC | [PCBC_PKCS7](./traits_pcbc_with_padding_pkcs7/trait.PCBC_PKCS7.html) | [PCBC_ISO](./traits_pcbc_with_padding_iso/trait.PCBC_ISO.html) |                                    |
    /// | CFB  |                                                                      |                                                                | [CFB](./traits_cfb/trait.CFB.html) |
    /// | OFB  |                                                                      |                                                                | [OFB](./traits_ofb/trait.OFB.html) |
    /// | CTR  |                                                                      |                                                                | [CTR](./traits_ctr/trait.CTR.html) |
    ///
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_128;
    /// 
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let ciphertext = [[IntUnion::new_with(0x27584D87), IntUnion::new_with(0x44E2BAE9), IntUnion::new_with(0x4AECB5D6), IntUnion::new_with(0x01CCF826)]; 3];
    /// let mut plaintext = [[IntUnion::new(); 4]; 3];
    /// aes.decrypt_array_unit(&ciphertext, &mut plaintext);
    /// 
    /// println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
    ///         ciphertext[0][0].get(), ciphertext[0][1].get(), ciphertext[0][2].get(), ciphertext[0][3].get(),
    ///         ciphertext[1][0].get(), ciphertext[1][1].get(), ciphertext[1][2].get(), ciphertext[1][3].get(),
    ///         ciphertext[2][0].get(), ciphertext[2][1].get(), ciphertext[2][2].get(), ciphertext[2][3].get());
    /// println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
    ///         plaintext[0][0].get(), plaintext[0][1].get(), plaintext[0][2].get(), plaintext[0][3].get(),
    ///         plaintext[1][0].get(), plaintext[1][1].get(), plaintext[1][2].get(), plaintext[1][3].get(),
    ///         plaintext[2][0].get(), plaintext[2][1].get(), plaintext[2][2].get(), plaintext[2][3].get());
    /// assert_eq!(plaintext[0][0].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[0][1].get(), 0x12345678);
    /// assert_eq!(plaintext[0][2].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[0][3].get(), 0x12345678);
    /// assert_eq!(plaintext[1][0].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[1][1].get(), 0x12345678);
    /// assert_eq!(plaintext[1][2].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[1][3].get(), 0x12345678);
    /// assert_eq!(plaintext[2][0].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[2][1].get(), 0x12345678);
    /// assert_eq!(plaintext[2][2].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[2][3].get(), 0x12345678);
    /// ```
    /// 
    /// # Example 2 for AES_192
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_192;
    /// 
    /// let mut aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let ciphertext = [[IntUnion::new_with(0x77047F1E), IntUnion::new_with(0x008E2C5B), IntUnion::new_with(0x6E5EB091), IntUnion::new_with(0x0DB5608E)]; 3];
    /// let mut plaintext = [[IntUnion::new(); 4]; 3];
    /// aes.decrypt_array_unit(&ciphertext, &mut plaintext);
    /// 
    /// println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
    ///         ciphertext[0][0].get(), ciphertext[0][1].get(), ciphertext[0][2].get(), ciphertext[0][3].get(),
    ///         ciphertext[1][0].get(), ciphertext[1][1].get(), ciphertext[1][2].get(), ciphertext[1][3].get(),
    ///         ciphertext[2][0].get(), ciphertext[2][1].get(), ciphertext[2][2].get(), ciphertext[2][3].get());
    /// println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
    ///         plaintext[0][0].get(), plaintext[0][1].get(), plaintext[0][2].get(), plaintext[0][3].get(),
    ///         plaintext[1][0].get(), plaintext[1][1].get(), plaintext[1][2].get(), plaintext[1][3].get(),
    ///         plaintext[2][0].get(), plaintext[2][1].get(), plaintext[2][2].get(), plaintext[2][3].get());
    /// assert_eq!(plaintext[0][0].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[0][1].get(), 0x12345678);
    /// assert_eq!(plaintext[0][2].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[0][3].get(), 0x12345678);
    /// assert_eq!(plaintext[1][0].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[1][1].get(), 0x12345678);
    /// assert_eq!(plaintext[1][2].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[1][3].get(), 0x12345678);
    /// assert_eq!(plaintext[2][0].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[2][1].get(), 0x12345678);
    /// assert_eq!(plaintext[2][2].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[2][3].get(), 0x12345678);
    /// ```
    /// 
    /// # Example 3 for Rijndael_Generic::<10, 4, 2>
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::Rijndael_Generic;
    /// 
    /// let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let ciphertext = [[IntUnion::new_with(0xBF20110F), IntUnion::new_with(0x70E7E05D), IntUnion::new_with(0x28996D55), IntUnion::new_with(0x9D6C20BD)]; 3];
    /// let mut plaintext = [[IntUnion::new(); 4]; 3];
    /// rijndael.decrypt_array_unit(&ciphertext, &mut plaintext);
    /// 
    /// println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
    ///         ciphertext[0][0].get(), ciphertext[0][1].get(), ciphertext[0][2].get(), ciphertext[0][3].get(),
    ///         ciphertext[1][0].get(), ciphertext[1][1].get(), ciphertext[1][2].get(), ciphertext[1][3].get(),
    ///         ciphertext[2][0].get(), ciphertext[2][1].get(), ciphertext[2][2].get(), ciphertext[2][3].get());
    /// println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
    ///         plaintext[0][0].get(), plaintext[0][1].get(), plaintext[0][2].get(), plaintext[0][3].get(),
    ///         plaintext[1][0].get(), plaintext[1][1].get(), plaintext[1][2].get(), plaintext[1][3].get(),
    ///         plaintext[2][0].get(), plaintext[2][1].get(), plaintext[2][2].get(), plaintext[2][3].get());
    /// assert_eq!(plaintext[0][0].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[0][1].get(), 0x12345678);
    /// assert_eq!(plaintext[0][2].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[0][3].get(), 0x12345678);
    /// assert_eq!(plaintext[1][0].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[1][1].get(), 0x12345678);
    /// assert_eq!(plaintext[1][2].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[1][3].get(), 0x12345678);
    /// assert_eq!(plaintext[2][0].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[2][1].get(), 0x12345678);
    /// assert_eq!(plaintext[2][2].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[2][3].get(), 0x12345678);
    /// ```
    pub fn decrypt_array_unit<const N: usize>(&mut self, cipher: &[[IntUnion; NB]; N], message: &mut [[IntUnion; NB]; N])
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_array_u128<const N: usize>(&mut self, cipher: &[u128; N], message: &mut [u128; N])
    /// Decrypts an array of 128-bit data.
    ///
    /// # Arguments
    /// - `cipher` is of an array of `u128`-type and the ciphertext to be
    ///   decrypted.
    /// - `message` is of an array of `u128`-type and the plaintext to be stored.
    ///
    /// # Features
    /// This method decrypts multiple of 64-bit data without padding anything
    /// in ECB (Electronic CodeBook) mode.
    ///
    /// # Counterpart methods
    /// - If you need to decrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in ECB operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::ECB_PKCS7`.
    ///   see [here](./traits_ecb_with_padding_pkcs7/trait.ECB_PKCS7.html)
    /// - If you need to decrypt data with padding bits according ISO
    ///   in ECB operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::ECB_ISO`.
    ///   see [here](./traits_ecb_with_padding_iso/trait.ECB_ISO.html)
    /// - If you need to decrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in CBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::CBC_PKCS7`.
    ///   see [here](./traits_cbc_with_padding_pkcs7/trait.CBC_PKCS7.html)
    /// - If you need to decrypt data with padding bits according ISO
    ///   in CBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::CBC_ISO`.
    ///   see [here](./traits_cbc_with_padding_iso/trait.CBC_ISO.html)
    /// - If you need to decrypt data with padding bits according
    ///   [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/)
    ///   in PCBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::PCBC_PKCS7`.
    ///   see [here](./traits_pcbc_with_padding_pkcs7/trait.PCBC_PKCS7.html)
    /// - If you need to decrypt data with padding bits according ISO
    ///   in PCBC operation mode, you may need to import (use)
    ///   `cryptocol::symmetric::PCBC_ISO`.
    ///   see [here](./traits_pcbc_with_padding_iso/trait.PCBC_ISO.html)
    /// - If you need to decrypt data in CFB operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::CFB`.
    ///   see [here](./traits_cfb/trait.CFB.html)
    /// - If you need to decrypt data in OFB operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::OFB`.
    ///   see [here](./traits_ofb/trait.OFB.html)
    /// - If you need to decrypt data in CTR operation mode,
    ///   you may need to import (use) `cryptocol::symmetric::CTR`.
    ///   see [here](./traits_ctr/trait.CTR.html)
    ///
    /// In summary,
    ///
    /// |      | padding PKCS7                                                        | padding ISO                                                    | no padding                         |
    /// |------|----------------------------------------------------------------------|----------------------------------------------------------------|------------------------------------|
    /// | ECB  | [ECB_PKCS7](./traits_ecb_with_padding_pkcs7/trait.ECB_PKCS7.html)    | [ECB_ISO](./traits_ecb_with_padding_iso/trait.ECB_ISO.html)    |                                    |
    /// | CBC  | [CBC_PKCS7](./traits_cbc_with_padding_pkcs7/trait.CBC_PKCS7.html)    | [CBC_ISO](./traits_cbc_with_padding_iso/trait.CBC_ISO.html)    |                                    |
    /// | PCBC | [PCBC_PKCS7](./traits_pcbc_with_padding_pkcs7/trait.PCBC_PKCS7.html) | [PCBC_ISO](./traits_pcbc_with_padding_iso/trait.PCBC_ISO.html) |                                    |
    /// | CFB  |                                                                      |                                                                | [CFB](./traits_cfb/trait.CFB.html) |
    /// | OFB  |                                                                      |                                                                | [OFB](./traits_ofb/trait.OFB.html) |
    /// | CTR  |                                                                      |                                                                | [CTR](./traits_ctr/trait.CTR.html) |
    ///
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_128;
    /// 
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let ciphertext = [0x01CCF8264AECB5D644E2BAE927584D87_u128, 0x01CCF8264AECB5D644E2BAE927584D87_u128, 0x01CCF8264AECB5D644E2BAE927584D87_u128];
    /// let mut plaintext = [0_u128; 3];
    /// aes.decrypt_array_u128(&ciphertext, &mut plaintext);
    /// 
    /// println!("Ciphertext:\t{:#034X} {:#034X} {:#034X}", ciphertext[0], ciphertext[1], ciphertext[2]);
    /// println!("Plaintext:\t{:#034X} {:#034X} {:#034X}", plaintext[0], plaintext[1], plaintext[2]);
    /// assert_eq!(plaintext[0], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(plaintext[1], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(plaintext[2], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// ```
    /// 
    /// # Example 2 for AES_192
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_192;
    /// 
    /// let mut aes = AES_192::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let ciphertext = [0x0DB5608E6E5EB091008E2C5B77047F1E_u128, 0x0DB5608E6E5EB091008E2C5B77047F1E_u128, 0x0DB5608E6E5EB091008E2C5B77047F1E_u128];
    /// let mut plaintext = [0_u128; 3];
    /// aes.decrypt_array_u128(&ciphertext, &mut plaintext);
    /// 
    /// println!("Ciphertext:\t{:#034X} {:#034X} {:#034X}", ciphertext[0], ciphertext[1], ciphertext[2]);
    /// println!("Plaintext:\t{:#034X} {:#034X} {:#034X}", plaintext[0], plaintext[1], plaintext[2]);
    /// assert_eq!(plaintext[0], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(plaintext[1], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(plaintext[2], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// ```
    /// 
    /// # Example 3 for Rijndael_Generic::<10, 4, 2>
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::Rijndael_Generic;
    /// 
    /// let mut rijndael = Rijndael_Generic::<10, 4, 2>::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let ciphertext = [0x9D6C20BD28996D5570E7E05DBF20110F_u128, 0x9D6C20BD28996D5570E7E05DBF20110F_u128, 0x9D6C20BD28996D5570E7E05DBF20110F_u128];
    /// let mut plaintext = [0_u128; 3];
    /// rijndael.decrypt_array_u128(&ciphertext, &mut plaintext);
    /// 
    /// println!("Ciphertext:\t{:#034X} {:#034X} {:#034X}", ciphertext[0], ciphertext[1], ciphertext[2]);
    /// println!("Plaintext:\t{:#034X} {:#034X} {:#034X}", plaintext[0], plaintext[1], plaintext[2]);
    /// assert_eq!(plaintext[0], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(plaintext[1], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(plaintext[2], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// ```
    pub fn decrypt_array_u128<const N: usize>(&mut self, cipher: &[u128; N], message: &mut [u128; N])
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
    /// # Example 1 for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
    /// println!("K =\t{:#034X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// let len = a_aes.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 16);
    /// let success = a_aes.is_successful();
    /// assert_eq!(success, true);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "26 F2 F8 B7 B7 FD 46 9A 97 97 F3 24 E7 51 99 47 ");
    /// ```
    ///
    /// # Example 2 for the original message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
    /// println!("K =\t{:#034X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let cipher = [0x26_u8, 0xF2, 0xF8, 0xB7, 0xB7, 0xFD, 0x46, 0x9A, 0x97, 0x97, 0xF3, 0x24, 0xE7, 0x51, 0x99, 0x47];
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut recovered = [0u8; 16];
    /// let len = a_aes.decrypt_array_into_array(&cipher, &mut recovered);
    /// println!("The length of plaintext = {}", len);
    /// assert_eq!(len, 0);
    /// let success = a_aes.is_successful();
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
    /// # Example 3 for Failed case for encryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CFB };
    /// 
    /// let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
    /// println!("K =\t{:#034X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = [0xEFCDAB90_u32, 0x78563412, 0xEFCDAB90, 0x78563412];
    /// println!("IV = {} {} {} {}", iv[0], iv[1], iv[2], iv[3]);
    /// let mut cipher = [0_u8; 40];
    /// let len = a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 0);
    /// let success = a_aes.is_successful();
    /// assert_eq!(success, false);
    /// ```
    /// # Example 4 for Failed case for decryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CFB };
    /// 
    /// let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
    /// println!("K =\t{:#034X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let cipher = [0x2Eu8, 0x1E, 0xE1, 0x51, 0xFD, 0xB3, 0xB0, 0x4B, 0x79, 0x3A, 0xA1, 0x78, 0xEC, 0xCD, 0x02, 0x72, 0x6A, 0xC4, 0x41, 0x7C, 0x25, 0xA4, 0x2C, 0x07, 0xFC, 0x77, 0x25, 0x49, 0x12, 0x55, 0x0F, 0x8A, 0xED, 0x44, 0xC3, 0xE4, 0xDC, 0x91, 0x69, 0x0F, 0x40, 0x72, 0x7F, 0xF2, 0xD9, 0xB7, 0x54, 0x9F, 0x36, 0x91, 0xC5, 0x85, 0x4F, 0x9B, 0x30];
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut recovered = [0u8; 40];
    /// let len = a_aes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    /// println!("The length of plaintext = {}", len);
    /// assert_eq!(len, 0);
    /// let success = a_aes.is_successful();
    /// assert_eq!(success, false);
    /// ```
    pub fn is_successful(&self) -> bool
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
    /// # Example 1 for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
    /// println!("K =\t{:#034X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// let len = a_aes.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 16);
    /// let failure = a_aes.is_failed();
    /// assert_eq!(failure, false);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "26 F2 F8 B7 B7 FD 46 9A 97 97 F3 24 E7 51 99 47 ");
    /// ```
    ///
    /// # Example 2 for the original message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
    /// println!("K =\t{:#034X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let cipher = [0x26_u8, 0xF2, 0xF8, 0xB7, 0xB7, 0xFD, 0x46, 0x9A, 0x97, 0x97, 0xF3, 0x24, 0xE7, 0x51, 0x99, 0x47];
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut recovered = [0u8; 16];
    /// let len = a_aes.decrypt_array_into_array(&cipher, &mut recovered);
    /// println!("The length of plaintext = {}", len);
    /// assert_eq!(len, 0);
    /// let failure = a_aes.is_failed();
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
    /// # Example 3 for Failed case for encryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CFB };
    /// 
    /// let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
    /// println!("K =\t{:#034X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = [0xEFCDAB90_u32, 0x78563412, 0xEFCDAB90, 0x78563412];
    /// println!("IV = {} {} {} {}", iv[0], iv[1], iv[2], iv[3]);
    /// let mut cipher = [0_u8; 40];
    /// let len = a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 0);
    /// let failure = a_aes.is_failed();
    /// assert_eq!(failure, true);
    /// ```
    /// # Example 4 for Failed case for decryption
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CFB };
    /// 
    /// let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
    /// println!("K =\t{:#034X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let cipher = [0x2Eu8, 0x1E, 0xE1, 0x51, 0xFD, 0xB3, 0xB0, 0x4B, 0x79, 0x3A, 0xA1, 0x78, 0xEC, 0xCD, 0x02, 0x72, 0x6A, 0xC4, 0x41, 0x7C, 0x25, 0xA4, 0x2C, 0x07, 0xFC, 0x77, 0x25, 0x49, 0x12, 0x55, 0x0F, 0x8A, 0xED, 0x44, 0xC3, 0xE4, 0xDC, 0x91, 0x69, 0x0F, 0x40, 0x72, 0x7F, 0xF2, 0xD9, 0xB7, 0x54, 0x9F, 0x36, 0x91, 0xC5, 0x85, 0x4F, 0x9B, 0x30];
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut recovered = [0u8; 40];
    /// let len = a_aes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    /// println!("The length of plaintext = {}", len);
    /// assert_eq!(len, 0);
    /// let failure = a_aes.is_failed();
    /// assert_eq!(failure, true);
    /// ```
    pub fn is_failed(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn get_desirable_round() -> usize
    /// Returns the desirable number of rounds
    /// according to the Rijndael documents
    /// 
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::symmetric::AES_128;
    /// let rounds = AES_128::get_desirable_round();
    /// println!("The desirable number of rounds of AES_128 is {}", rounds);
    /// assert_eq!(rounds, 10);
    /// ```
    /// 
    /// # Example 2 for AES_192
    /// ```
    /// use cryptocol::symmetric::AES_192;
    /// let rounds = AES_192::get_desirable_round();
    /// println!("The desirable number of rounds of AES_192 is {}", rounds);
    /// assert_eq!(rounds, 12);
    /// ```
    /// 
    /// # Example 3 for AES_256
    /// ```
    /// use cryptocol::symmetric::AES_256;
    /// let rounds = AES_256::get_desirable_round();
    /// println!("The desirable number of rounds of AES_256 is {}", rounds);
    /// assert_eq!(rounds, 14);
    /// ```
    /// 
    /// # Example 4 for Rijndael_256_256
    /// ```
    /// use cryptocol::symmetric::Rijndael_256_256;
    /// let rounds = Rijndael_256_256::get_desirable_round();
    /// println!("The desirable number of rounds of Rijndael_256_256 is {}", rounds);
    /// assert_eq!(rounds, 14);
    /// ```
    /// 
    /// # Example 5 for Rijndael_64_64
    /// ```
    /// use cryptocol::symmetric::Rijndael_64_64;
    /// let rounds = Rijndael_64_64::get_desirable_round();
    /// println!("The desirable number of rounds of Rijndael_64_64 is {}", rounds);
    /// assert_eq!(rounds, 8);
    /// ```
    /// 
    /// # Example 6 for Rijndael_Generic
    /// ```
    /// use cryptocol::symmetric::Rijndael_Generic;
    /// let rounds = Rijndael_Generic::<0, 16, 16>::get_desirable_round();
    /// println!("The desirable number of rounds of Rijndael_Generic<0, 16, 16> is {}", rounds);
    /// assert_eq!(rounds, 22);
    /// ```
    pub fn get_desirable_round() -> usize
    {
        unimplemented!(); // Dummy code for documentation
    }
}