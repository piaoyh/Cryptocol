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


use std::ptr::copy_nonoverlapping;
use std::vec::Vec;

use crate::number::{ LongUnion, LongerUnion, SmallUInt };
use crate::symmetric::{ DES, SmallCryptor64, SmallCryptor128 };


macro_rules! pre_encrypt_into_vec {
    ($to:expr, $length_in_bytes:expr, $type:ty) => {
        let mut len = if <$type>::size_in_bytes() == 16 {16_usize} else {8};
        len = ($length_in_bytes + 1).next_multiple_of(len as u64) as usize / <$type>::size_in_bytes() as usize;
        $to.truncate(len - 1);
        $to.resize(len, <$type>::zero());
    };
    // pre_encrypt_into_vec!(cipher, length_in_bytes, T);
    //
    // let mut len = if T::size_in_bytes() == 16 {16_usize} else {8};
    // len = (length_in_bytes + 1).next_multiple_of(len as u64) as usize / T::size_in_bytes();
    // cipher.truncate(len - 1);
    // cipher.resize(len, T::zero());
}

macro_rules! pre_decrypt_into_vec {
    ($to:expr, $length_in_bytes:expr, $type:ty) => {
        let len = $length_in_bytes as usize / <$type>::size_in_bytes() as usize;
        $to.truncate(len - 1);
        $to.resize(len, <$type>::zero());
    };
}

macro_rules! pre_encrypt_into_array {
    ($to:expr, $length_in_bytes:expr, $type:ty) => {
        let mut len = if <$type>::size_in_bytes() == 16 {16_usize} else {8};
        len = ($length_in_bytes + 1).next_multiple_of(len as u64) as usize / <$type>::size_in_bytes() as usize;
        for i in len - 1..$to.len()
            { $to[i] = <$type>::zero(); }
    };
    // pre_encrypt_into_array!(cipher, length_in_bytes, T);
    //
    // let mut len = if T::size_in_bytes() == 16 {16_usize} else {8};
    // len = (length_in_bytes + 1).next_multiple_of(len as u64) as usize / T::size_in_bytes();
    // for i in len..M
    //     { cipher[i] = T::zero(); }
}

macro_rules! pre_decrypt_into_array {
    ($to:expr, $length_in_bytes:expr, $type:ty) => {
        let len = $length_in_bytes as usize / <$type>::size_in_bytes() as usize;
        for i in len - 1..$to.len()
            { $to[i] = <$type>::zero(); }
    };
}


#[allow(non_camel_case_types)]
pub struct BigCryptor128
{
    block: LongerUnion,
    smallcryptor: Vec<Box<dyn SmallCryptor128>>,
}

impl BigCryptor128
{
    const SUCCESS: u128 = !0;
    const FAILURE: u128 = 0;

    // pub fn new() -> Self
    /// Constructs a new object BigCryptor.
    /// 
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method sets the key to be [0, 0, 0, 0, 0, 0, 0, 0].
    /// - Do not use this default key [0, 0, 0, 0, 0, 0, 0, 0]
    ///   because it is known as one of the weak keys.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::DES;
    /// 
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.new)           
    #[inline]
    pub fn new() -> Self
    {
        Self { block: LongerUnion::zero(), smallcryptor: Vec::new() }
    }

    // pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor128>; N]) -> Self
    /// Constructs a new object BigCryptor.
    /// 
    /// # Arguments
    /// 
    /// # Features
    /// This method sets the small cryptor to be the given argument `smallcryptor`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.new_with_key)
    pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor128>; N]) -> Self
    {
        let mut bigcryptor = Self::new();
        bigcryptor.push_small_cryptor_array(smallcryptor);
        bigcryptor
    }

    // pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor128>>) -> Self
    /// Constructs a new object DES_Generic.
    /// 
    /// # Arguments
    /// - The argument `key` is of `u64`.
    /// - It should be in the same endianness of machine. For example,
    ///   if a key is [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF],
    ///   the key in `u64` is 0x_1234567890ABCDEF_u64 for big-endian machine,
    ///   and the key in `u64` is 0x_EFCDAB9078563412_u64 for little-endian
    ///   machine.
    /// - Remember that inverted parity bits do not affect the 56-bit real key.
    ///   So, 0x_0000_0000_0000_0000_u4, 0x_0101_0101_0101_0101_u64,
    ///   0x_0000_0000_0000_0001_u64, 0x_0000_0000_0000_0100_u64,
    ///   0x_0100_0010_0000_0001_u64, etc. are all the same keys. 
    ///   Each key has 255 different equivalent keys in DES. 
    /// 
    /// # Features
    /// This method sets the key to be the given argument `key`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    /// 
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.new_with_key_u64)
    pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor128>>) -> Self
    {
        let mut bigcryptor = Self::new();
        bigcryptor.push_small_cryptor_vec(smallcryptor);
        bigcryptor
    }

    pub fn push_small_cryptor<S: SmallCryptor128 + 'static>(&mut self, smallcryptor: S)
    {
        self.smallcryptor.push(Box::<S>::new(smallcryptor));
    }

    // pub fn push_small_cryptor_array<const N: usize>(&mut self, smallcryptor: [Box<dyn SmallCryptor128>; N])
    /// Sets the smallcryptor.
    /// 
    /// # Arguments
    /// - The argument `key` is the array of u8 that has 8 elements.
    /// - Remember that inverted parity bits do not affect the 56-bit real key.
    ///   So, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    ///   [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01],
    ///   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
    ///   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00],
    ///   [0x01, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x01], etc.
    ///   are all the same keys. Each key has 255 different equivalent keys
    ///   in DES. 
    /// 
    /// # Features
    /// This method sets the key to be the given argument `key`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    /// 
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.set_key)
    pub fn push_small_cryptor_array<const N: usize>(&mut self, smallcryptor: [Box<dyn SmallCryptor128>; N])
    {
        for val in smallcryptor
            { self.smallcryptor.push(val); }
    }

    // pub fn push_small_cryptor_vec(&mut self, smallcryptor: Vec<Box<dyn SmallCryptor128>>)
    /// Sets the smallcryptor.
    /// 
    /// # Arguments
    /// - The argument `key` is the array of u8 that has 8 elements.
    /// - Remember that inverted parity bits do not affect the 56-bit real key.
    ///   So, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    ///   [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01],
    ///   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
    ///   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00],
    ///   [0x01, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x01], etc.
    ///   are all the same keys. Each key has 255 different equivalent keys
    ///   in DES. 
    /// 
    /// # Features
    /// This method sets the key to be the given argument `key`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    /// 
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.set_key)
    pub fn push_small_cryptor_vec(&mut self, smallcryptor: Vec<Box<dyn SmallCryptor128>>)
    {
        self.smallcryptor = smallcryptor;
    }

    // pub fn encrypt_u128(&mut self, message: u128) -> u128
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
    /// use cryptocol::symmetric::DES;
    /// 
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.encrypt_u64)
    pub fn encrypt_u128(&mut self, message: u128) -> u128
    {
        self.set_block(message);
        self.encrypt_block();
        self.block.get()
    }

    // pub fn decrypt_u128(&mut self, cipher: u128) -> u128
    /// Decrypts a 128-bit data.
    /// 
    /// # Arguments
    /// `cioher` is of `u64`-type and the ciphertext to be decrypted.
    /// 
    /// # Output
    /// This method returns the decrypted data of `u64`-type from `cipher`.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// 
    /// let message = 0x_1234567890ABCDEF_u64;
    /// println!("M_u64 =\t{:#016X}", message);
    /// 
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let cipher = a_des.encrypt_u64(message);
    /// println!("C_u64 (16 rounds) =\t{:#016X}", cipher);
    /// assert_eq!(cipher, 0x_1BC4896735BBE206_u64);
    /// 
    /// let recovered = a_des.decrypt_u64(cipher);
    /// println!("B_u64 (16 rounds) =\t{:#016X}", recovered);
    /// assert_eq!(recovered, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.decrypt_u64)
    pub fn decrypt_u128(&mut self, cipher: u128) -> u128
    {
        self.set_block(cipher);
        self.decrypt_block();
        self.block.get()
    }

    // pub fn encrypt_array_u128<const N: usize>(&mut self, message: &[u128; N], cipher: &mut [u128; N])
    /// Encrypts an array of 128-bit data.
    /// 
    /// # Arguments
    /// - `message` is of an array of `u64`-type and the plaintext to be
    ///   encrypted.
    /// - `cipher` is of an array of `u64`-type and the ciphertext to be
    ///   stored.
    /// 
    /// # Example 1 for Normal case 
    /// ```
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// 
    /// let message = [0x_1234567890ABCDEF_u64, 0xEFCDAB9078563412, 0xFEDCBA0987654321 ];
    /// print!("M =\t");
    /// for m in message
    ///     { print!("{:#016X} ", m); }
    /// println!();
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let mut cipher = [0; 3];
    /// a_des.encrypt_array_u64(&message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher
    ///     { print!("{:#016X} ", c); }
    /// println!();
    /// assert_eq!(cipher[0], 0x_1BC4896735BBE206_u64);
    /// assert_eq!(cipher[1], 0x_1D8A61E5E62226A4_u64);
    /// assert_eq!(cipher[2], 0x_2990D69525C17067_u64);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.encrypt_array_u64)
    pub fn encrypt_array_u128<const N: usize>(&mut self, message: &[u128; N], cipher: &mut [u128; N])
    {
        for i in 0..N
            { cipher[i] = self.encrypt_u128(message[i]); }
    }

    // pub fn decrypt_array_u128<const N: usize>(&mut self, cipher: &[u128; N], message: &mut [u64; N])
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
    /// use cryptocol::symmetric::AES;
    /// 
    /// let key = 0x_1234567890ABCDEFFEDCBA0987654321_u128;
    /// println!("K =\t{:#016X}", key);
    /// 
    /// let message = [0x_1234567890ABCDEF_u64, 0xEFCDAB9078563412, 0xFEDCBA0987654321 ];
    /// print!("M =\t");
    /// for m in message
    ///     { print!("{:#016X} ", m); }
    /// println!();
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let mut cipher = [0; 3];
    /// a_des.encrypt_array_u64(&message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher
    ///     { print!("{:#016X} ", c); }
    /// println!();
    /// assert_eq!(cipher[0], 0x_1BC4896735BBE206_u64);
    /// assert_eq!(cipher[1], 0x_1D8A61E5E62226A4_u64);
    /// assert_eq!(cipher[2], 0x_2990D69525C17067_u64);
    /// 
    /// let mut recovered = [0; 3];
    /// a_des.decrypt_array_u64(&cipher, &mut recovered);
    /// print!("B (16 rounds) =\t");
    /// for r in recovered
    ///     { print!("{:#016X} ", r); }
    /// println!();
    /// assert_eq!(recovered[0], 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered[1], 0x_EFCDAB9078563412_u64);
    /// assert_eq!(recovered[2], 0x_FEDCBA0987654321_u64);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.decrypt_array_u64)
    pub fn decrypt_array_u128<const N: usize>(&mut self, cipher: &[u128; N], message: &mut [u128; N])
    {
        for i in 0..N
            { message[i] = self.decrypt_u128(cipher[i]); }
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
    /// # Example 1 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::AES;
    /// 
    /// let key = 0x_1234567890ABCDEFFEDCBA0987654321_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES::new_with_key_u128(key);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// let len = a_aes.encrypt_with_padding_pkcs7_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 8);
    /// let success = a_aes.is_successful();
    /// assert_eq!(success, true);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.is_successful)
    #[inline] pub fn is_successful(&self) -> bool { self.block.get() == Self::SUCCESS }

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
    /// # Example 1 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::AES;
    /// 
    /// let key = 0x_1234567890ABCDEFFEDCBA0987654321_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES::new_with_key_u128(key);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// let len = a_aes.encrypt_with_padding_pkcs7_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 8);
    /// let failure = a_aes.is_failed();
    /// assert_eq!(failure, false);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.is_failed)
    #[inline] pub fn is_failed(&self) -> bool   { self.block.get() == Self::FAILURE }

    // pub fn set_successful(&mut self)
    /// Sets the flag to mean that the previous encryption or decryption
    /// was successful.
    /// 
    /// # Features
    /// You won't use this method unless you write codes for implementation
    /// of a trait for BigCryptor.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::symmetric::AES;
    /// let mut a_aes = AES::new_with_key_u64(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// assert_eq!(a_des.is_successful(), false);
    /// 
    /// a_aes.set_successful();
    /// assert_eq!(a_aes.is_successful(), true);
    /// ```
    #[inline]
    pub fn set_successful(&mut self)
    {
        self.block.set(Self::SUCCESS);
    }

    // pub fn set_failed(&mut self)
    /// Sets the flag to mean that the previous encryption or decryption
    /// was failed.
    /// 
    /// # Features
    /// You won't use this method unless you write codes for implementation
    /// of a trait for BigCryptor.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::symmetric::AES;
    /// let mut a_aes = AES::new_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// a_aes.encrypt_u128(0x1234567890ABCDEF_u64);
    /// assert_eq!(a_aes.is_failed(), false);
    /// 
    /// a_aes.set_failed();
    /// assert_eq!(a_aes.is_failed(), true);
    /// ```
    #[inline]
    pub fn set_failed(&mut self)
    {
        self.block.set(Self::FAILURE);
    }

    fn encrypt_block(&mut self)
    {
        let mut block = self.smallcryptor[0].encrypt_unit(self.block.get());
        for i in 1..self.smallcryptor.len()
            { block = self.smallcryptor[i].encrypt_unit(block); }
        self.block.set(block);
    }

    fn decrypt_block(&mut self)
    {
        let len = self.smallcryptor.len();
        let mut block = self.smallcryptor[len-1].decrypt_unit(self.block.get());
        for i in 2..len+1
            { block = self.smallcryptor[len-i].decrypt_unit(block); }
        self.block.set(block);
    }

    #[inline] fn get_block(&self) -> u128            { self.block.get() }
    #[inline] fn set_block(&mut self, block: u128)   { self.block.set(block); }
}



/// An BigCryptor64 symmetric-key algorithm for the encryption of digital data
/// 
/// # Note
/// **This descryption about BigCryptor is according to big endianness.**
/// MSB (Most Significant Bit) is the first bit and LSB (Least Significant Bit)
/// is the 64th bit in this descryption.
/// 
/// # Introduction
/// BigCryptor64 is the acronym of N (any number) Data Encryption Standard. It is the
/// symmetric key encryption/decryption algorithm composed of multiple DES. It
/// was originally developed after DES was broken.
///
/// 
/// # Vulnerability
/// - Its key length is only 56-bit. It is considered to be too short against
///   modern computing power. Actually, in July, 1998, the DES key was broken by
///   brute-force attack within 56 hours with a machine DES cracker (Deep Crack)
///   made by EEF (Electronic Frontier Foundation). And, in January, 1999, Deep
///   Crack and distributed.net broke a DES key together within 22 hours and
///   15 minutes.
/// - Weak keys: 0x0000000000000000, 0x0101010101010101, 0xFFFFFFFFFFFFFFFF,
///   0xFEFEFEFEFEFEFEFE, 0xE0E0E0E0F1F1F1F1, 0xE1E1E1E1F0F0F0F0,
///   0x1F1F1F1F0E0E0E0E, 0x1E1E1E1E0F0F0F0F in big-endianness.
///   Actually, if the parity bits in keys are ignored,
///   the keys 0x0000000000000000 and 0x0101010101010101 are the same key.
///   In fact, not only 0x0101010101010101 is the same key as
///   0x0000000000000000. 0x0100000000000000 is also the same key. All the 256
///   keys that have only different parity bits and all other bits same are the
///   same key as 0x0000000000000000, too. Though only representative keys will
///   be mentioned in this description, please keep in mind that all the 256
///   keys that have only different parity bits and all other bits same are the
///   same key.
///   And, the keys 0xFFFFFFFFFFFFFFFF and 0xFEFEFEFEFEFEFEFE are also the same key.
///   And, The keys 0xE0E0E0E0F1F1F1F1 and 0xE1E1E1E1F0F0F0F0 are also the same key.
///   And, the keys 0x1F1F1F1F0E0E0E0E and 0x1E1E1E1E0F0F0F0F are the same key, too.
///   For instance, if you encrypt your data with the key 0x0000000000000000 and
///   encrypt the output ciphertext again with the same key 0x0000000000000000,
///   you will get the original plaintext! So, the ciphertext is only
///   secure-looking.
/// - Semi-week keys: The pairs 0x011F011F010E010E and 0x1F011F010E010E01,
///   0x01E001E001F101F1 and 0xE001E001F101F101,
///   0x01FE01FE01FE01FE and 0xFE01FE01FE01FE01,
///   0x1FE01FE00EF10EF1 and 0xE01FE01FF10EF10E,
///   0x1FFE1FFE0EFE0EFE and 0xFE1FFE1FFE0EFE0E, and
///   0xE0FEE0FEF1FEF1FE and 0xFEE0FEE0FEF1FEF1 in big-endianness are considered
///   to be week.
///   For example, if you encrypt your data with the key 0x011F011F010E010E and
///   encrypt the output ciphertext again with its counterpart key
///   0xE001E001F101F101, you will get the original plaintext!
///   So, the ciphertext is only secure-looking.
/// 
/// # Use of DES and its variants
/// This algorithm is implemented generic way. Most of the constants are
/// implemented as generic constants. So, without changing any code, you can
/// change the algorithm by changing the generic parameter. You can design and
/// implement your own algorithm based on DES which uses Feistel structure. 
/// 
/// # Generic Parameters
/// - ROUND: You can determine how many times the Fiestel network will be
///   repeated. Its maximum value is 128 and its minimum value is 0.
///   Original DES has 16 rounds for its Feistel structure but you can increase
///   the number of rounds up to 128 rounds, and decrease it down to 0.
/// - SHIFT: According to the number of rounds, you have to determine `SHIFT`
///   which is used for generating round keys. You can determine how many bits
///   the half keys will be rotated left for the corresponding round in the key
///   generator. If `SHIFT` is '0b10011001', the half keys will be rotated left
///   by one bit for the first round, and will be rotated left by two bits for
///   the second round, and will be rotated left by two bits for  the third round,
///   and so on. The LSB (Least Significant Bit) is for the first round and the
///   MSB (Most Significant Bit) is for the 128th round. `0` means that the half
///   keys will be rotated left by two bits and `1` means that the half keys
///   will be rotated left by one bit. Up to only `ROUND` bits from the LSB of
///   `SHIFT` will be meaningful. For example, if `ROUND` is 5 and `SHIFT` is
///   '0b10011001', only '0b11001' out of '0b10011001' is meaningful.
///   If `ROUND` is 16 and `SHIFT` is '0b10011001', `SHIFT` will be understood
///   to be '0b0000000010011001'.
/// - PC101 ~ PC248: Permutation compression. In key generator, the 64-bit key
///   which includes parity bits is compressed into 56-bit key by dropping all
///   the parity bits (8th, 16th, 24th, 32th, 40th, 48th, 56th, and 64th bits)
///   and permutating (or transposing or shuffling) all bits. They are 1-based.
///   For this operation, PC101 ~ PC156 are used. You can change the permutation
///   compression algorithm by changing these parameters. Note that PC101 ~
///   PC248 should not include 8, 16, 24, 32, 40, 48, 56, and 64 because they
///   are all parity bits. If you include any of those numbers, the whole DES
///   encryption/decryption algorithm is broken or unstable or collapses. Also,
///   in key generator, 56-bit key is compressed into 48-bit key by dropping all
///   the bits (9th, 18th, 22nd, 25th, 35th, 43rd, and 54th bits) and
///   permutating (or transposing or shuffling) all bits. For this operation,
///   PC201 ~ PC248 are used. You can change the permutation compression
///   algorithm by changing these parameters. In this case, you can drop other
///   bits intead of dropping 9th, 18th, 22nd, 25th, 35th, 43rd, and 54th bits.
///   Dropping other bits does not kill the whole DES encryption/decryption
///   algorithm.
/// - IP01 ~ IP64: Inital permutation constants. They are 1-based. For example,
///   `IP01 = 58` means that the 58th bit of data is moved to the first bit of
///   the data which is MSB at initial permutation. You can change inital
///   permutation wire by changing these constants. However, when you change
///   these constants, you have to remember that you should included all the
///   bits. You cannot drop any bit. Your dropping any bit will surely kill the
///   whole DES encryption/decryption algorithm. Final permutation constants is
///   automatically calculated from Inital permutation constants. FP01 ~ FP64
///   is inverse version of IP01 ~ IP64. So, FP01 ~ FP64 will be automagically
///   determined. You don't have to determine them.
/// - S000 ~ S763: S-Box constants. Its index such as 000, 212, etc. is
///   0-based. S0XX means S-Box 0, S1XX means S-Box 1, and so on. S000 is the
///   first element of S-Box 0.
///   According to [the document](https://page.math.tu-berlin.de/~kant/teaching/hess/krypto-ws2006/des.htm),
///   the input six bits determines the output of S-Box. The first and the last
///   bit of the six bits represent in base 2 a number in the decimal range 0 to
///   3 (or binary 00 to 11) which is row number. The rest middle four bits
///   represent in base 2 a number in the decimal range 0 to 15 (binary 0000 to
///   1111) which is column number. It is considered that the DES designers
///   explained the S-box structure _unnecessarily too complicated_. The
///   above-described S-box indexing way looks two dimensional, but actually is
///   one dimensional. So, in this crate, S-boxes are implemented to be
///   two-dimensional array which is an array of S-boxes. Each S-box is an array
///   of 64 four-bit numbers. The input six-bit number is used as the index of
///   the one-dimensional array of these 64 four-bit numbers. So, the S-box
///   tables have been rearranged to be the one-dimensional array. You can cange
///   S-Box by changing these constants. However, you have know that *the change
///   of these constants may hurt the security a lot*. And the principle of
///   S-box has been unknown so far.
/// - EP01 ~ EP48: Expansion permutation constants. They are 1-based. For
///   example, `EP01 = 28` means that the 28th bit of data is moved to the first
///   bit of the data which is MSB at expansion permutation. They expand the
///   32-bit right-half data from the Feistel structure of the corresponding
///   round into 48 bits in order to perform XOR (exclusive OR) with the
///   corresponding 48-bit round key. When you change these constants, you have
///   to remember that you should included all the bits. You cannot drop any
///   bit. Your dropping any bit will surely kill the whole DES
///   encryption/decryption algorithm.
/// - TP01 ~ TP32: Translation permutation constans.  They are 1-based. For
///   example, `TP01 = 16` means that the 16th bit of data is moved to the
///   first bit of the data which is MSB at translation permutation. You can
///   change translation permutation wire by changing these constants. However,
///   when you change these constants, you have to remember that you should
///   included all the bits. You cannot drop any bit. Your dropping any bit will
///   surely kill the whole DES encryption/decryption algorithm.
/// 
/// # Reference
/// [Read more](https://en.wikipedia.org/wiki/Data_Encryption_Standard)
/// about DES in brief.
/// Watch [this video](https://www.youtube.com/watch?v=kPBJIhpcZgE)
/// and [this video](https://www.youtube.com/watch?v=l-7YW06BFNs) in series
/// for more (or deeper or full) understanding of DES.
/// 
/// # Quick Start
/// You have to import (use) the module `des` in order to use official DES
/// as shown in Example 1.
/// 
/// # Example 1
/// ```
/// use cryptocol::symmetric::{ BigCryptor64, SmallCryptor64 };
/// ```
/// 
/// You can instantiate the DES object with `u64` key as Example 2.
/// In this case, you have to take endianness into account.
/// In little-endianness, 0x_1234567890ABCDEF_u64 is [0xEFu8, 0xCDu8, 0xABu8,
/// 0x90u8, 0x78u8, 0x56u8, 0x34u8, 0x12u8] while the same 
/// 0x_1234567890ABCDEF_u64 is [0x12u8, 0x34u8, 0x56u8, 0x78u8, 0x90u8, 0xABu8,
/// 0xCDu8, 0xEF_u64] in big-endianness.
/// The instantiated object should be mutable.
/// 
/// # Example 2
/// ```
/// use cryptocol::symmetric::DES;
/// let key = 0x_1234567890ABCDEF_u64;
/// let mut _a_des = DES::new_with_key_u64(key);
/// ```
/// 
/// Also, you can instantiate the DES object with `[u8; 8]` key as shown in
/// Example 3. In this case, you don't have to take endianness into account.
/// The instantiated object should be mutable.
/// 
/// # Example 3
/// ```
/// ```
/// 
/// You can instantiate the DES object without key and set a `u64` key later as
/// shown in Example 4 or a `[u8; 8]` key later as shown in Example 5.
/// 
/// # Example 4
/// ```
/// ```
/// 
/// # Example 5
/// ```
/// ```
/// 
/// Now, you can freely use any methods
/// encrypt_[vec|array|str|string]_with_padding_[pkcs7|iso]_[ecb|cbc|pcbc]_into_vec()
/// and encrypt_[vec|array|str|string]_[cfb|ofb|ctr]_into_vec() to encrypt,
/// decrypt_[vec|array]_with_padding_[pkcs7|iso]_into_[vec|string]() and
/// decrypt_[vec|array]_[cfb|ofb|ctr]_into_[vec|string]() to decrypt. However,
/// you are encouraged to avoid using the methods such as
/// encrypt_with_padding_[pkcs7|iso](), encrypt_[cfb|ofb|ctr](),
/// decrypt_with_padding_[pkcs7|iso](), and decrypt_[cfb|ofb|ctr]()
/// that receive pointer arguments and the data length
/// unless you develope in hybrid programming context especially with C/C++. 
/// Instead, you are highly encouraged to use the methods 
/// encrypt_[vec|array|str|string]_with_padding_[pkcs7|iso]_[ecb|cbc|pcbc]_into_vec(),
/// encrypt_[vec|array|str|string]_[cfb|ofb|ctr]_into_vec()
/// decrypt_[vec|array]_with_padding_[pkcs7|iso]_into_[vec|string](), and
/// decrypt_[vec|array]_[cfb|ofb|ctr]_into_[vec|string]() because you don't
/// have to consider the length of data so that you will meak less mistakes.
/// 
/// # Example 6
/// ```
/// ```
/// 
/// # Example 7
/// ```
/// ```
#[allow(non_camel_case_types)]
pub struct BigCryptor64
{
    block: LongUnion,
    smallcryptor: Vec<Box<dyn SmallCryptor64>>,
}

impl BigCryptor64
{
    const SUCCESS: u64 = !0;
    const FAILURE: u64 = 0;

    // pub fn new() -> Self
    /// Constructs a new object BigCryptor.
    /// 
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method sets the key to be [0, 0, 0, 0, 0, 0, 0, 0].
    /// - Do not use this default key [0, 0, 0, 0, 0, 0, 0, 0]
    ///   because it is known as one of the weak keys.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::DES;
    /// 
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.new)           
    #[inline]
    pub fn new() -> Self
    {
        Self { block: LongUnion::zero(), smallcryptor: Vec::new() }
    }

    // pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor64>; N]) -> Self
    /// Constructs a new object BigCryptor.
    /// 
    /// # Arguments
    /// 
    /// # Features
    /// This method sets the small cryptor to be the given argument `smallcryptor`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.new_with_key)
    pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor64>; N]) -> Self
    {
        let mut bigcryptor = Self::new();
        bigcryptor.push_small_cryptor_array(smallcryptor);
        bigcryptor
    }

    // pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor64>>) -> Self
    /// Constructs a new object DES_Generic.
    /// 
    /// # Arguments
    /// - The argument `key` is of `u64`.
    /// - It should be in the same endianness of machine. For example,
    ///   if a key is [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF],
    ///   the key in `u64` is 0x_1234567890ABCDEF_u64 for big-endian machine,
    ///   and the key in `u64` is 0x_EFCDAB9078563412_u64 for little-endian
    ///   machine.
    /// - Remember that inverted parity bits do not affect the 56-bit real key.
    ///   So, 0x_0000_0000_0000_0000_u4, 0x_0101_0101_0101_0101_u64,
    ///   0x_0000_0000_0000_0001_u64, 0x_0000_0000_0000_0100_u64,
    ///   0x_0100_0010_0000_0001_u64, etc. are all the same keys. 
    ///   Each key has 255 different equivalent keys in DES. 
    /// 
    /// # Features
    /// This method sets the key to be the given argument `key`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    /// 
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.new_with_key_u64)
    pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor64>>) -> Self
    {
        let mut bigcryptor = Self::new();
        bigcryptor.push_small_cryptor_vec(smallcryptor);
        bigcryptor
    }

    pub fn push_small_cryptor<S: SmallCryptor64 + 'static>(&mut self, smallcryptor: S)
    {
        self.smallcryptor.push(Box::<S>::new(smallcryptor));
    }

    // pub fn push_small_cryptor_array<const N: usize>(&mut self, smallcryptor: [Box<dyn SmallCryptor<u64, 8>>; N])
    /// Sets the smallcryptor.
    /// 
    /// # Arguments
    /// - The argument `key` is the array of u8 that has 8 elements.
    /// - Remember that inverted parity bits do not affect the 56-bit real key.
    ///   So, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    ///   [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01],
    ///   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
    ///   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00],
    ///   [0x01, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x01], etc.
    ///   are all the same keys. Each key has 255 different equivalent keys
    ///   in DES. 
    /// 
    /// # Features
    /// This method sets the key to be the given argument `key`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    /// 
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.set_key)
    pub fn push_small_cryptor_array<const N: usize>(&mut self, smallcryptor: [Box<dyn SmallCryptor64>; N])
    {
        for val in smallcryptor
            { self.smallcryptor.push(val); }
    }

    // pub fn push_small_cryptor_vec<const N: usize>(&mut self, smallcryptor: [SmallCryptor; N])
    /// Sets the smallcryptor.
    /// 
    /// # Arguments
    /// - The argument `key` is the array of u8 that has 8 elements.
    /// - Remember that inverted parity bits do not affect the 56-bit real key.
    ///   So, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    ///   [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01],
    ///   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
    ///   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00],
    ///   [0x01, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x01], etc.
    ///   are all the same keys. Each key has 255 different equivalent keys
    ///   in DES. 
    /// 
    /// # Features
    /// This method sets the key to be the given argument `key`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::DES;
    /// 
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.set_key)
    pub fn push_small_cryptor_vec(&mut self, smallcryptor: Vec<Box<dyn SmallCryptor64>>)
    {
        self.smallcryptor = smallcryptor;
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
    /// use cryptocol::symmetric::DES;
    /// 
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.encrypt_u64)
    pub fn encrypt_u64(&mut self, message: u64) -> u64
    {
        self.set_block(message);
        self.encrypt_block();
        self.block.get()
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
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// 
    /// let message = 0x_1234567890ABCDEF_u64;
    /// println!("M_u64 =\t{:#016X}", message);
    /// 
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let cipher = a_des.encrypt_u64(message);
    /// println!("C_u64 (16 rounds) =\t{:#016X}", cipher);
    /// assert_eq!(cipher, 0x_1BC4896735BBE206_u64);
    /// 
    /// let recovered = a_des.decrypt_u64(cipher);
    /// println!("B_u64 (16 rounds) =\t{:#016X}", recovered);
    /// assert_eq!(recovered, 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.decrypt_u64)
    pub fn decrypt_u64(&mut self, cipher: u64) -> u64
    {
        self.set_block(cipher);
        self.decrypt_block();
        self.block.get()
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
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// 
    /// let message = [0x_1234567890ABCDEF_u64, 0xEFCDAB9078563412, 0xFEDCBA0987654321 ];
    /// print!("M =\t");
    /// for m in message
    ///     { print!("{:#016X} ", m); }
    /// println!();
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let mut cipher = [0; 3];
    /// a_des.encrypt_array_u64(&message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher
    ///     { print!("{:#016X} ", c); }
    /// println!();
    /// assert_eq!(cipher[0], 0x_1BC4896735BBE206_u64);
    /// assert_eq!(cipher[1], 0x_1D8A61E5E62226A4_u64);
    /// assert_eq!(cipher[2], 0x_2990D69525C17067_u64);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.encrypt_array_u64)
    pub fn encrypt_array_u64<const M: usize>(&mut self, message: &[u64; M], cipher: &mut [u64; M])
    {
        for i in 0..M
            { cipher[i] = self.encrypt_u64(message[i]); }
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
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// 
    /// let message = [0x_1234567890ABCDEF_u64, 0xEFCDAB9078563412, 0xFEDCBA0987654321 ];
    /// print!("M =\t");
    /// for m in message
    ///     { print!("{:#016X} ", m); }
    /// println!();
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let mut cipher = [0; 3];
    /// a_des.encrypt_array_u64(&message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher
    ///     { print!("{:#016X} ", c); }
    /// println!();
    /// assert_eq!(cipher[0], 0x_1BC4896735BBE206_u64);
    /// assert_eq!(cipher[1], 0x_1D8A61E5E62226A4_u64);
    /// assert_eq!(cipher[2], 0x_2990D69525C17067_u64);
    /// 
    /// let mut recovered = [0; 3];
    /// a_des.decrypt_array_u64(&cipher, &mut recovered);
    /// print!("B (16 rounds) =\t");
    /// for r in recovered
    ///     { print!("{:#016X} ", r); }
    /// println!();
    /// assert_eq!(recovered[0], 0x_1234567890ABCDEF_u64);
    /// assert_eq!(recovered[1], 0x_EFCDAB9078563412_u64);
    /// assert_eq!(recovered[2], 0x_FEDCBA0987654321_u64);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.decrypt_array_u64)
    pub fn decrypt_array_u64<const M: usize>(&mut self, cipher: &[u64; M], message: &mut [u64; M])
    {
        for i in 0..M
            { message[i] = self.decrypt_u64(cipher[i]); }
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
    /// # Example 1 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// let len = a_des.encrypt_with_padding_pkcs7_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 8);
    /// let success = a_des.is_successful();
    /// assert_eq!(success, true);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.is_successful)
    #[inline] pub fn is_successful(&self) -> bool { self.block.get() == Self::SUCCESS }

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
    /// # Example 1 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// let len = a_des.encrypt_with_padding_pkcs7_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 8);
    /// let failure = a_des.is_failed();
    /// assert_eq!(failure, false);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/des_basic/struct.DES_Generic.html#method.is_failed)
    #[inline] pub fn is_failed(&self) -> bool   { self.block.get() == Self::FAILURE }

    // pub fn set_successful(&mut self)
    /// Sets the flag to mean that the previous encryption or decryption
    /// was successful.
    /// 
    /// # Features
    /// You won't use this method unless you write codes for implementation
    /// of a trait for BigCryptor.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::symmetric::DES;
    /// let mut a_des = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    /// assert_eq!(a_des.is_successful(), false);
    /// 
    /// a_des.set_successful();
    /// assert_eq!(a_des.is_successful(), true);
    /// ```
    #[inline]
    pub fn set_successful(&mut self)
    {
        self.block.set(Self::SUCCESS);
    }

    // pub fn set_failed(&mut self)
    /// Sets the flag to mean that the previous encryption or decryption
    /// was failed.
    /// 
    /// # Features
    /// You won't use this method unless you write codes for implementation
    /// of a trait for BigCryptor.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::symmetric::DES;
    /// let mut a_des = DES::new_with_key_u64(0x_1234567890ABCDEF_u64);
    /// a_des.encrypt_u64(0x1234567890ABCDEF_u64);
    /// assert_eq!(a_des.is_failed(), false);
    /// 
    /// a_des.set_failed();
    /// assert_eq!(a_des.is_failed(), true);
    /// ```
    #[inline]
    pub fn set_failed(&mut self)
    {
        self.block.set(Self::FAILURE);
    }

    fn encrypt_block(&mut self)
    {
        let mut block = self.smallcryptor[0].encrypt_unit(self.block.get());
        for i in 1..self.smallcryptor.len()
            { block = self.smallcryptor[i].encrypt_unit(block); }
        self.block.set(block);
    }

    fn decrypt_block(&mut self)
    {
        let len = self.smallcryptor.len();
        let mut block = self.smallcryptor[len-1].decrypt_unit(self.block.get());
        for i in 2..len+1
            { block = self.smallcryptor[len-i].decrypt_unit(block); }
        self.block.set(block);
    }

    #[inline] fn get_block(&self) -> u64            { self.block.get() }
    #[inline] fn set_block(&mut self, block: u64)   { self.block.set(block); }
    

    // pub fn encrypt_with_padding_pkcs7(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    /// Encrypts the data with the padding defined in PKCS #7.
    /// 
    /// # Arguments
    /// - `message` is a pointer to u8 which is `*const u8`,
    ///   and the plaintext to be encrypted.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and the length of the plaintext `message` in bytes.
    /// - `cipher` is a pointer to u8 which is `*mut u8`,
    ///   and the ciphertext to be stored.
    /// - The size of the memory area which starts at `cipher` and the
    ///   ciphertext will be stored at is assumed to be enough.
    ///   The size of the area should be prepared to be
    ///   (`length_in_bytes` + 1).next_multiple_of(8) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Output
    /// This method returns the size of ciphertext including padding bits
    /// in bytes.
    /// 
    /// # Features
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - The padding bits composed of the bytes that indicate the length of
    ///   the plaintext. For more information about the padding bits according
    ///   to PKCS#7, Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - This method performs pure encryption without any operation mode.
    ///   It is equivalent to ECB (Electronic Code Book) mode.
    /// 
    /// # Example 1
    /// ```
    /// 
    /// ```
    pub fn encrypt_with_padding_pkcs7(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut encoded: u64;
        for _ in 0..length_in_bytes >> 3    // length_in_bytes >> 3 == length_in_bytes / 8
        {
            let block = unsafe { *(message.add(progress as usize) as *const u64 ) };
            encoded = self.encrypt_u64(block);
            unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
            progress += 8;
        }

        let mut block = 0_u64;
        let mut block_union = LongUnion::new_with(0x_08_08_08_08__08_08_08_08);
        if progress != length_in_bytes
        {
            let tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
            let padding = 8 - tail as u8;
            block_union.set(block);
            for i in tail..8
                { block_union.set_ubyte_(i, padding); }
        }
        encoded = self.encrypt_u64(block_union.get());
        unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
        self.block.set(Self::SUCCESS);
        progress + 8
    }

    // pub fn encrypt_with_padding_pkcs7_into_vec<T>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    /// Encrypts the data with the padding defined in PKCS #7.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and pushed into the vector `cipher`.
    /// 
    /// # Example 1
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// 
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "6F 10 01 6D 99 BF 41 F8 BC 00 A8 1D 81 B7 4B 20 6F B5 30 0A 14 03 A9 8E 69 E7 A6 33 42 AF 97 59 ED 9D E0 95 35 DC DF 0D 99 58 FA 92 13 50 4D 50 D3 4E 76 9C C5 BB 9E CB ");
    /// ```
    /// 
    /// # Example 2 for 128 rounds
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES_Expanded;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// 
    /// let mut b_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// b_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "DD C6 D8 D1 B0 66 D9 AC F7 F3 B4 FD D6 6C ED 78 20 FB A6 8D 35 38 EA 65 B0 65 23 05 FF D4 53 B1 D1 E0 C5 52 36 1E AC E2 19 EF 94 B8 98 04 A9 69 CC 6A BC 81 7D 6B 29 C0 ");
    /// ```
    /// 
    /// # Example 3 for 128 rounds
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES_Expanded;
    /// 
    /// // Expanded case for 0 rounds which means that key is meaningless
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// println!("K =\t{:#016X}", key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// 
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");
    /// ```
    /// 
    /// # Example 4 for the message of 0 bytes
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let message = "";
    /// println!("M =\t{}", message);
    /// 
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");
    /// ```
    /// 
    /// # Example 5 for the message shorter than 8 bytes
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// 
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F6 F0 41 DD 55 55 3B 35 ");
    /// ```
    /// 
    /// # Example 6 for the message of 8 bytes
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// 
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "27 F5 93 EE 76 DC 64 87 41 7F 89 79 08 CD A1 4C ");
    /// ```
    /// 
    /// # Example 7 for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// 
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "8E 52 20 47 78 78 51 B7 00 69 10 77 91 B7 52 36 ");
    /// ```
    /// 
    /// # Example 8 for the message of 16 bytes
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// 
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");
    /// ```
    pub fn encrypt_with_padding_pkcs7_into_vec<T>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_encrypt_into_vec!(cipher, length_in_bytes, T);
        self.encrypt_with_padding_pkcs7(message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    // pub fn encrypt_with_padding_pkcs7_into_array<T, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    /// Encrypts the data with the padding defined in PKCS #7.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and stored into the array `cipher`.
    /// - If `N` is less than the next multiple of 8 from `length_in_bytes`,
    ///   this method does not perform encryption and returns `false`.
    /// - If `N` is equal to the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns `true`.
    /// - If `N` is greater than the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and then fills the rest of elements of
    ///   the array `cipher`, and returns `true`.
    /// 
    pub fn encrypt_with_padding_pkcs7_into_array<T, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if (length_in_bytes as u128 + 1).next_multiple_of(8) > T::size_in_bytes() as u128 * N as u128
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_encrypt_into_array!(cipher, length_in_bytes, T);
        self.encrypt_with_padding_pkcs7(message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn encrypt_str_with_padding_pkcs7(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_pkcs7(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_pkcs7_into_vec<T>(&mut self, message: &str, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_pkcs7_into_array<T, const N: usize>(&mut self, message: &str, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_into_array(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_pkcs7(&mut self, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_pkcs7( message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_pkcs7_into_vec<T>(&mut self, message: &String, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_pkcs7_into_array<T, const N: usize>(&mut self, message: &String, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_into_array(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_pkcs7<T>(&mut self, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7( message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_pkcs7_into_vec<T, U>(&mut self, message: &Vec<T>, cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_into_vec(message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_pkcs7_into_array<T, U, const N: usize>(&mut self, message: &Vec<T>, cipher: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_into_array(message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_pkcs7<T, const N: usize>(&mut self, message: &[T; N], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7(message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_pkcs7_into_vec<T, U, const N: usize>(&mut self, message: &[T; N], cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_into_vec(message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_pkcs7_into_array<T, U, const N: usize, const M: usize>(&mut self, message: &[T; N], cipher: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_into_array(message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }


    // pub fn decrypt_with_padding_pkcs7(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    /// Decrypts the data with the padding defined in PKCS #7 and stores in Vec.
    /// 
    /// # Arguments
    /// - `cipher` is the ciphertext to be decrypted, and is a pointer to u8
    ///   which is `*const u8`.
    /// - `length_in_bytes` is the length of the ciphertext `cipher` in bytes,
    ///   and of `u64`-type,
    /// - `message` is the container where the plaintext will be stored, and
    ///   of `*mut u8` type.
    /// 
    /// # Output
    /// This method returns the size of the decrypted plaintext in bytes.
    /// 
    /// # Features
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be less than `8`.
    /// - If `message` has some values before decryption, it will be removed.
    /// - The padding bits is composed of the bytes that indicate the length of
    ///   the plaintext. For more information about the padding bits according
    ///   to PKCS#7, Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - This method performs pure decryption without any operation mode.
    ///   It is equivalent to ECB (Electronic Code Book) mode.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    /// ```
    pub fn decrypt_with_padding_pkcs7(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        if length_in_bytes < 8
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        let mut progress = 0_u64;
        let mut decoded: u64;
        let mut block: u64;
        if length_in_bytes > 8
        {
            for _ in 0..(length_in_bytes >> 3) - 1 // length_in_bytes >> 3 == length_in_bytes / 8
            {
                block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
                decoded = self.decrypt_u64(block);
                unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), 8); }
                progress += 8;
            }
        }
        block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
        decoded = self.decrypt_u64(block);
        let decoded_union = LongUnion::new_with(decoded);
        let padding_bytes = decoded_union.get_ubyte_(7);
        let message_bytes = 8 - padding_bytes as usize;
        for i in message_bytes..8
        {
            if decoded_union.get_ubyte_(i) != padding_bytes
            {
                self.block.set(Self::FAILURE);
                return 0;
            }
        }
        unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), message_bytes); }
        self.block.set(Self::SUCCESS);
        progress + message_bytes as u64
    }

    // pub fn decrypt_with_padding_pkcs7_into_vec<T>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    /// Decrypts the data with the padding defined in PKCS #7 and stores in Vec.
    /// 
    /// # Arguments
    /// - `cipher` is the ciphertext to be decrypted, and is a pointer to u8
    ///   which is `*const u8`.
    /// - `length_in_bytes` is the length of the ciphertext `cipher` in bytes,
    ///   and of `u64`-type,
    /// - `message` is the container where the plaintext will be stored, and
    ///   of `&mut Vec<T>` type.
    /// 
    /// # Output
    /// This method returns the size of the decrypted plaintext in bytes.
    /// 
    /// # Features
    /// - `length_in_bytes` cannot be less than `8`.
    /// - If `message` has some values before decryption, it will be removed.
    /// - The padding bits is composed of the bytes that indicate the length of
    ///   the plaintext. For more information about the padding bits according
    ///   to PKCS#7, Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - This method performs pure decryption without any operation mode.
    ///   It is equivalent to ECB (Electronic Code Book) mode.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// 
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "6F 10 01 6D 99 BF 41 F8 BC 00 A8 1D 81 B7 4B 20 6F B5 30 0A 14 03 A9 8E 69 E7 A6 33 42 AF 97 59 ED 9D E0 95 35 DC DF 0D 99 58 FA 92 13 50 4D 50 D3 4E 76 9C C5 BB 9E CB ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba (16 rounds) =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// 
    /// println!("Bb (16 rounds) =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 2 for 128 rounds
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES_Expanded;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// 
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "DD C6 D8 D1 B0 66 D9 AC F7 F3 B4 FD D6 6C ED 78 20 FB A6 8D 35 38 EA 65 B0 65 23 05 FF D4 53 B1 D1 E0 C5 52 36 1E AC E2 19 EF 94 B8 98 04 A9 69 CC 6A BC 81 7D 6B 29 C0 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba (128 rounds) =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// 
    /// println!("Bb (128 rounds) =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 3 for 128 rounds
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES_Expanded;
    /// 
    /// // Expanded case for 0 rounds which means that key is meaningless
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// println!("K =\t{:#016X}", key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// 
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "86 9D 10 B8 94 9A 10 91 9A 9B 96 9D 9D 96 9D 9B 10 8B 9F 98 10 93 B1 9A 92 B8 9A 98 10 B8 94 9A 10 94 9A 92 B9 9A 9D B3 10 92 9D 98 10 B8 94 9A 10 9A 92 B1 B8 94 1D 02 ");
    /// 
    /// let mut recovered1 = Vec::<u8>::new();
    /// let mut recovered2 = Vec::<u8>::new();
    /// c_des.decrypt_with_padding_pkcs7_into_vec(cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    /// d_des.decrypt_with_padding_pkcs7_into_vec(cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
    /// print!("B1a (0 rounds) =\t");
    /// for b in recovered1.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");
    /// print!("B2a (0 rounds) =\t");
    /// for b in recovered2.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");
    /// 
    /// let mut converted1 = String::new();
    /// let mut converted2 = String::new();
    /// unsafe { converted1.as_mut_vec() }.append(&mut recovered1);
    /// unsafe { converted2.as_mut_vec() }.append(&mut recovered2);
    /// 
    /// println!("B1b (0 rounds) =\t{}", converted1);
    /// assert_eq!(converted1, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted1, message);
    /// println!("B2b (0 rounds) =\t{}", converted2);
    /// assert_eq!(converted2, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted2, message);
    /// assert_eq!(converted1, converted1);
    /// ```
    /// 
    /// # Example 4 for the message of 0 bytes
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let message = "";
    /// println!("M =\t{}", message);
    /// 
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "41 7F 89 79 08 CD A1 4C ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// 
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 5 for the message shorter than 8 bytes
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// 
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F6 F0 41 DD 55 55 3B 35 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "37 20 62 79 74 65 73 ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// 
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "7 bytes");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 6 for the message of 8 bytes
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// 
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "27 F5 93 EE 76 DC 64 87 41 7F 89 79 08 CD A1 4C ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 20 61 6D 20 4F 4B 2E ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// 
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "I am OK.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 7 for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// 
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "8E 52 20 47 78 78 51 B7 00 69 10 77 91 B7 52 36 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "50 41 52 4B 20 59 6F 75 6E 67 68 6F ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// 
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "PARK Youngho");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 8 for the message of 16 bytes
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::symmetric::DES;
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// 
    /// let mut a_des = DES::new_with_key_u64(key);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_with_padding_pkcs7_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "EA B3 A0 EB A7 99 EC 8A B5 EB 8B 88 EB 8B A4 2E ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// 
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "고맙습니다.");
    /// assert_eq!(converted, message);
    /// ```
    pub fn decrypt_with_padding_pkcs7_into_vec<T>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_decrypt_into_vec!(message, length_in_bytes, T);
        let res = self.decrypt_with_padding_pkcs7(cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(res as usize);
        res
    }

    // pub fn decrypt_with_padding_pkcs7_into_array<T, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    /// `message` has to have at least the same size as that of `cipher`. 
    /// 
    /// # Example 1
    /// ```
    /// 
    /// ```
    pub fn decrypt_with_padding_pkcs7_into_array<T, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if length_in_bytes as u128 > T::size_in_bytes() as u128 * N as u128 + 1
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_decrypt_into_array!(message, length_in_bytes, T);
        self.decrypt_with_padding_pkcs7(cipher, length_in_bytes, message.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn decrypt_with_padding_pkcs7_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_with_padding_pkcs7_into_vec(cipher, length_in_bytes, unsafe { message.as_mut_vec() })
    }

    #[inline]
    pub fn decrypt_vec_with_padding_pkcs7<T>(&mut self, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7(cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_pkcs7_into_vec<T, U>(&mut self, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_pkcs7_into_array<T, U, const N: usize>(&mut self, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_into_array(cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_pkcs7_into_string<T>(&mut self, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_into_string(cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_pkcs7<T, const N: usize>(&mut self, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7(cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_pkcs7_into_vec<T, U, const N: usize>(&mut self, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_into_vec(cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_pkcs7_into_array<T, U, const N: usize, const M: usize>(&mut self, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_into_array(cipher.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_pkcs7_into_string<T, const N: usize>(&mut self, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_into_string(cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }



    pub fn encrypt_with_padding_iso(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut encoded: u64;
        for _ in 0..length_in_bytes >> 3    // length_in_bytes >> 3 == length_in_bytes / 8
        {
            let block = unsafe { *(message.add(progress as usize) as *const u64 ) };
            encoded = self.encrypt_u64(block);
            unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
            progress += 8;
        }

        let mut block = 0_u64;
        let mut block_union = LongUnion::new_with(0b_1000_0000);
        if progress != length_in_bytes
        {
            let tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
            block_union.set(block);
            block_union.set_ubyte_(tail, 0b_1000_0000);
        }
        encoded = self.encrypt_u64(block_union.get());
        unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
        self.block.set(Self::SUCCESS);
        progress + 8
    }

    pub fn encrypt_with_padding_iso_into_vec<T>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_encrypt_into_vec!(cipher, length_in_bytes, T);
        self.encrypt_with_padding_iso(message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    pub fn encrypt_with_padding_iso_into_array<T, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if (length_in_bytes as u128 + 1).next_multiple_of(8) > T::size_in_bytes() as u128 * N as u128
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_encrypt_into_array!(cipher, length_in_bytes, T);
        self.encrypt_with_padding_iso(message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn encrypt_str_with_padding_iso(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_iso(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_iso_into_vec<T>(&mut self, message: &str, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_into_vec(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_iso_into_array<T, const N: usize>(&mut self, message: &str, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_into_array(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_iso(&mut self, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_iso(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_iso_into_vec<T>(&mut self, message: &String, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_into_vec(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_iso_into_array<T, const N: usize>(&mut self, message: &String, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_into_array(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_iso<T, const N: usize>(&mut self, message: &[T; N], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso( message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_iso_into_vec<T, U, const N: usize>(&mut self, message: &[T; N], cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_into_vec(message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_iso_into_array<T, U, const N: usize, const M: usize>(&mut self, message: &[T; N], cipher: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_into_array(message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_iso<T>(&mut self, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso( message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_iso_into_vec<T, U>(&mut self, message: &Vec<T>, cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_into_vec(message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_iso_into_array<T, U, const N: usize>(&mut self, message: &Vec<T>, cipher: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_into_array(message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }



    pub fn decrypt_with_padding_iso(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut decoded: u64;
        let mut block: u64;
        if length_in_bytes > 8
        {
            for i in 0..(length_in_bytes as usize >> 3) - 1
            {
                block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
                decoded = self.decrypt_u64(block);
                unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), 8); }
                progress += 8;
            }
        }

        block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
        decoded = self.decrypt_u64(block);
        let decoded_union = LongUnion::new_with(decoded);
        for i in 0..8_usize
        {
            if decoded_union.get_ubyte_(7-i) == 0
                { continue; }
            if decoded_union.get_ubyte_(7-i) == 0b_1000_0000_u8
            {
                let message_bytes = 7-i;
                unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), message_bytes); }
                self.block.set(Self::SUCCESS);
                return progress + message_bytes as u64;
            }
            else
            {
                self.block.set(Self::FAILURE);
                return 0;
            }
        }
        self.block.set(Self::FAILURE);
        return 0;
    }

    pub fn decrypt_with_padding_iso_into_vec<T>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_decrypt_into_vec!(message, length_in_bytes, T);
        let len = self.decrypt_with_padding_iso(cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(len as usize);
        len
    }

    pub fn decrypt_with_padding_iso_into_array<T, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if length_in_bytes as u128 > T::size_in_bytes() as u128 * N as u128 + 1
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_decrypt_into_array!(message, length_in_bytes, T);
        self.decrypt_with_padding_iso(cipher, length_in_bytes, message.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn decrypt_with_padding_iso_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_with_padding_iso_into_vec(cipher, length_in_bytes, unsafe { message.as_mut_vec() })
    }

    #[inline]
    pub fn decrypt_vec_with_padding_iso<T>(&mut self, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso(cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_iso_into_vec<T, U>(&mut self, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_into_vec(cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_iso_into_array<T, U, const N: usize>(&mut self, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_into_array(cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_iso_into_string<T>(&mut self, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_into_string(cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_iso<T, const N: usize>(&mut self, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso(cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_iso_into_vec<T, U, const N: usize>(&mut self, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_into_vec(cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_iso_into_array<T, U, const N: usize, const M: usize>(&mut self, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_into_array(cipher.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_iso_into_string<T, const N: usize>(&mut self, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_into_string(cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }



    #[inline]
    pub fn encrypt_with_padding_pkcs7_ecb(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_pkcs7(message, length_in_bytes, cipher)
    }

    #[inline]
    pub fn encrypt_with_padding_pkcs7_ecb_into_vec<T>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_into_vec(message, length_in_bytes, cipher)
    }

    #[inline]
    pub fn encrypt_with_padding_pkcs7_ecb_into_array<T, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_into_array(message, length_in_bytes, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_pkcs7_ecb(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_str_with_padding_pkcs7(message, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_pkcs7_ecb_into_vec<T>(&mut self, message: &str, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_str_with_padding_pkcs7_into_vec(message, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_pkcs7_ecb_into_array<T, const N: usize>(&mut self, message: &str, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_str_with_padding_pkcs7_into_array(message, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_pkcs7_ecb(&mut self, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_string_with_padding_pkcs7(message, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_pkcs7_ecb_into_vec<T>(&mut self, message: &String, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_string_with_padding_pkcs7_into_vec(message, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_pkcs7_ecb_into_array<T, const N: usize>(&mut self, message: &String, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_string_with_padding_pkcs7_into_array(message, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_pkcs7_ecb<T>(&mut self, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_vec_with_padding_pkcs7(message, cipher)
    }
    
    #[inline]
    pub fn encrypt_vec_with_padding_pkcs7_ecb_into_vec<T, U>(&mut self, message: &Vec<T>, cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_vec_with_padding_pkcs7_into_vec(message, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_pkcs7_ecb_into_array<T, U, const N: usize>(&mut self, message: &Vec<T>, cipher: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_vec_with_padding_pkcs7_into_array(message, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_pkcs7_ecb<T, const N: usize>(&mut self, message: &[T; N], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_array_with_padding_pkcs7( message, cipher)
    }
    
    #[inline]
    pub fn encrypt_array_with_padding_pkcs7_ecb_into_vec<T, U, const N: usize>(&mut self, message: &[T; N], cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_array_with_padding_pkcs7_into_vec(message, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_pkcs7_ecb_into_array<T, U, const N: usize, const M: usize>(&mut self, message: &[T; N], cipher: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_array_with_padding_pkcs7_into_array(message, cipher)
    }



    #[inline]
    pub fn decrypt_with_padding_pkcs7_ecb(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        self.decrypt_with_padding_pkcs7(cipher, length_in_bytes, message)
    }

    #[inline]
    pub fn decrypt_with_padding_pkcs7_ecb_into_vec<T>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_into_vec(cipher, length_in_bytes, message)
    }

    #[inline]
    pub fn decrypt_with_padding_pkcs7_ecb_into_array<T, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_into_array(cipher, length_in_bytes, message)
    }

    #[inline]
    pub fn decrypt_with_padding_pkcs7_ecb_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_with_padding_pkcs7_into_string(cipher, length_in_bytes, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_pkcs7_ecb<T>(&mut self, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_vec_with_padding_pkcs7(cipher, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_pkcs7_ecb_into_vec<T, U>(&mut self, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_vec_with_padding_pkcs7_into_vec(cipher, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_pkcs7_ecb_into_array<T, U, const N: usize>(&mut self, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_vec_with_padding_pkcs7_into_array(cipher, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_pkcs7_ecb_into_string<T>(&mut self, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_vec_with_padding_pkcs7_into_string(cipher, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_pkcs7_ecb<T, const N: usize>(&mut self, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_array_with_padding_pkcs7(cipher, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_pkcs7_ecb_into_vec<T, U, const N: usize>(&mut self, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_array_with_padding_pkcs7_into_vec(cipher, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_pkcs7_ecb_into_array<T, U, const N: usize, const M: usize>(&mut self, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_array_with_padding_pkcs7_into_array(cipher, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_pkcs7_ecb_into_string<T, const N: usize>(&mut self, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_array_with_padding_pkcs7_into_string(cipher, message)
    }



    #[inline]
    pub fn encrypt_with_padding_iso_ecb(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_iso(message, length_in_bytes, cipher)
    }

    #[inline]
    pub fn encrypt_with_padding_iso_ecb_into_vec<T>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_into_vec(message, length_in_bytes, cipher)
    }

    #[inline]
    pub fn encrypt_with_padding_iso_ecb_into_array<T, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_into_array(message, length_in_bytes, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_iso_ecb(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_str_with_padding_iso(message, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_iso_ecb_into_vec<T>(&mut self, message: &str, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_str_with_padding_iso_into_vec(message, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_iso_ecb_into_array<T, const N: usize>(&mut self, message: &str, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_str_with_padding_iso_into_array(message, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_iso_ecb(&mut self, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_string_with_padding_iso(message, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_iso_ecb_into_vec<T>(&mut self, message: &String, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_string_with_padding_iso_into_vec(message, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_iso_ecb_into_array<T, const N: usize>(&mut self, message: &String, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_string_with_padding_iso_into_array(message, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_iso_ecb<T>(&mut self, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_vec_with_padding_iso(message, cipher)
    }
    
    #[inline]
    pub fn encrypt_vec_with_padding_iso_ecb_into_vec<T, U>(&mut self, message: &Vec<T>, cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_vec_with_padding_iso_into_vec(message, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_iso_ecb_into_array<T, U, const N: usize>(&mut self, message: &Vec<T>, cipher: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_vec_with_padding_iso_into_array(message, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_iso_ecb<T, const N: usize>(&mut self, message: &[T; N], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_array_with_padding_iso( message, cipher)
    }
    
    #[inline]
    pub fn encrypt_array_with_padding_iso_ecb_into_vec<T, U, const N: usize>(&mut self, message: &[T; N], cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_array_with_padding_iso_into_vec(message, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_iso_ecb_into_array<T, U, const N: usize, const M: usize>(&mut self, message: &[T; N], cipher: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_array_with_padding_iso_into_array(message, cipher)
    }

    #[inline]
    pub fn decrypt_with_padding_iso_ecb(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        self.decrypt_with_padding_iso(cipher, length_in_bytes, message)
    }

    #[inline]
    pub fn decrypt_with_padding_iso_ecb_into_vec<T>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_into_vec(cipher, length_in_bytes, message)
    }

    #[inline]
    pub fn decrypt_with_padding_iso_ecb_into_array<T, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_into_array(cipher, length_in_bytes, message)
    }

    #[inline]
    pub fn decrypt_with_padding_iso_ecb_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_with_padding_iso_into_string(cipher, length_in_bytes, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_iso_ecb<T>(&mut self, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_vec_with_padding_iso(cipher, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_iso_ecb_into_vec<T, U>(&mut self, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_vec_with_padding_iso_into_vec(cipher, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_iso_ecb_into_array<T, U, const N: usize>(&mut self, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_vec_with_padding_iso_into_array(cipher, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_iso_ecb_into_string<T>(&mut self, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_vec_with_padding_iso_into_string(cipher, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_iso_ecb<T, const N: usize>(&mut self, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_array_with_padding_iso(cipher, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_iso_ecb_into_vec<T, U, const N: usize>(&mut self, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_array_with_padding_iso_into_vec(cipher, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_iso_ecb_into_array<T, U, const N: usize, const M: usize>(&mut self, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_array_with_padding_iso_into_array(cipher, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_iso_ecb_into_string<T, const N: usize>(&mut self, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_array_with_padding_iso_into_string(cipher, message)
    }



    pub fn encrypt_with_padding_pkcs7_cbc(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut encoded = iv;
        for _ in 0..length_in_bytes >> 3    // length_in_bytes >> 3 == length_in_bytes / 8
        {
            let block = unsafe { *(message.add(progress as usize) as *const u64 ) };
            encoded = self.encrypt_u64(block ^ encoded);
            unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
            progress += 8;
        }
        let mut block = 0_u64;
        let mut block_union = LongUnion::new_with(0x_08_08_08_08__08_08_08_08);
        if progress != length_in_bytes
        {
            let tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
            let padding = 8 - tail as u8;
            block_union.set(block);
            for i in tail..8
                { block_union.set_ubyte_(i, padding); }
        }
        encoded = self.encrypt_u64(block_union.get() ^ encoded);
        unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
        self.block.set(Self::SUCCESS);
        progress + 8
    }

    /// Encrypts the data with the padding defined in PKCS #7.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and pushed into the vector `cipher`.
    pub fn encrypt_with_padding_pkcs7_cbc_into_vec<T>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_encrypt_into_vec!(cipher, length_in_bytes, T);
        self.encrypt_with_padding_pkcs7_cbc(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    // pub fn encrypt_with_padding_pkcs7_cbc_into_array<T, const N: usize>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    /// Encrypts the data with the padding defined in PKCS #7.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and stored into the array `cipher`.
    /// - If `N` is less than the next multiple of 8 from `length_in_bytes`,
    ///   this method does not perform encryption and returns `false`.
    /// - If `N` is equal to the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns `true`.
    /// - If `N` is greater than the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and then fills the rest of elements of
    ///   the array `cipher`, and returns `true`.
    /// 
    pub fn encrypt_with_padding_pkcs7_cbc_into_array<T, const N: usize>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if (length_in_bytes as u128 + 1).next_multiple_of(8) > T::size_in_bytes() as u128 * N as u128
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_encrypt_into_array!(cipher, length_in_bytes, T);
        self.encrypt_with_padding_pkcs7_cbc(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn encrypt_str_with_padding_pkcs7_cbc(&mut self, iv: u64, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_pkcs7_cbc(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_pkcs7_cbc_into_vec<T>(&mut self, iv: u64, message: &str, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_cbc_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_pkcs7_cbc_into_array<T, const N: usize>(&mut self, iv: u64, message: &str, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_cbc_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_pkcs7_cbc(&mut self, iv: u64, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_pkcs7_cbc(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_pkcs7_cbc_into_vec<T>(&mut self, iv: u64, message: &String, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_cbc_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_pkcs7_cbc_into_array<T, const N: usize>(&mut self, iv: u64, message: &String, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_cbc_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_pkcs7_cbc<T>(&mut self, iv: u64, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_cbc(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_pkcs7_cbc_into_vec<T, U>(&mut self, iv: u64, message: &Vec<T>, cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_cbc_into_vec(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_pkcs7_cbc_into_array<T, U, const N: usize>(&mut self, iv: u64, message: &Vec<T>, cipher: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_cbc_into_array(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_pkcs7_cbc<T, const N: usize>(&mut self, iv: u64, message: &[T; N], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_cbc(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_pkcs7_cbc_into_vec<T, U, const N: usize>(&mut self, iv: u64, message: &[T; N], cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_cbc_into_vec(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_pkcs7_cbc_into_array<T, U, const N: usize, const M: usize>(&mut self, iv: u64, message: &[T; N], cipher: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_cbc_into_array(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }



    pub fn decrypt_with_padding_pkcs7_cbc(&mut self, mut iv: u64, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut decoded: u64;
        let mut block: u64;
        if length_in_bytes > 8
        {
            for _ in 0..(length_in_bytes >> 3) - 1  // length_in_bytes >> 3 == length_in_bytes / 8
            {
                block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
                decoded = iv ^ self.decrypt_u64(block);
                iv = block;
                unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), 8); }
                progress += 8;
            }
        }

        block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
        decoded = iv ^ self.decrypt_u64(block);
        let decoded_union = LongUnion::new_with(decoded);
        let padding_bytes = decoded_union.get_ubyte_(7);
        if padding_bytes > 8
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        let message_bytes = 8 - padding_bytes as usize;
        for i in (message_bytes)..8
        {
            if decoded_union.get_ubyte_(i) != padding_bytes
            {
                self.block.set(Self::FAILURE);
                return 0;
            }
        }
        unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), message_bytes); }
        self.block.set(Self::SUCCESS);
        progress + message_bytes as u64
    }

    pub fn decrypt_with_padding_pkcs7_cbc_into_vec<T>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_decrypt_into_vec!(message, length_in_bytes, T);
        let len = self.decrypt_with_padding_pkcs7_cbc(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(len as usize);
        len
    }

    pub fn decrypt_with_padding_pkcs7_cbc_into_array<T, const N: usize>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if length_in_bytes as u128 > T::size_in_bytes() as u128 * N as u128 + 1
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_decrypt_into_array!(message, length_in_bytes, T);
        self.decrypt_with_padding_pkcs7_cbc(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn decrypt_with_padding_pkcs7_cbc_into_string(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_with_padding_pkcs7_cbc_into_vec(iv, cipher, length_in_bytes, unsafe { message.as_mut_vec() })
    }

    #[inline]
    pub fn decrypt_vec_with_padding_pkcs7_cbc<T>(&mut self, iv: u64, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_cbc(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_pkcs7_cbc_into_vec<T, U>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_cbc_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_pkcs7_cbc_into_array<T, U, const N: usize>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_cbc_into_array(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_pkcs7_cbc_into_string<T>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_cbc_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_pkcs7_cbc<T, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_cbc(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_pkcs7_cbc_into_vec<T, U, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_cbc_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_pkcs7_cbc_into_array<T, U, const N: usize, const M: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_cbc_into_array(iv, cipher.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_pkcs7_cbc_into_string<T, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_cbc_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }



    pub fn encrypt_with_padding_iso_cbc(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut encoded = iv;
        for _ in 0..length_in_bytes >> 3    // length_in_bytes >> 3 == length_in_bytes / 8
        {
            let block = unsafe { *(message.add(progress as usize) as *const u64 ) };
            encoded = self.encrypt_u64(block ^ encoded);
            unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
            progress += 8;
        }
        let mut block = 0_u64;
        let mut block_union = LongUnion::new_with(0b_1000_0000);
        if progress != length_in_bytes
        {
            let tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
            block_union.set(block);
            block_union.set_ubyte_(tail, 0b_1000_0000);
        }
        encoded = self.encrypt_u64(block_union.get() ^ encoded);
        unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
        self.block.set(Self::SUCCESS);
        progress + 8
    }

    pub fn encrypt_with_padding_iso_cbc_into_vec<T>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_encrypt_into_vec!(cipher, length_in_bytes, T);
        self.encrypt_with_padding_iso_cbc(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    pub fn encrypt_with_padding_iso_cbc_into_array<T, const N: usize>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if (length_in_bytes as u128 + 1).next_multiple_of(8) > T::size_in_bytes() as u128 * N as u128
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_encrypt_into_array!(cipher, length_in_bytes, T);
        self.encrypt_with_padding_iso_cbc(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn encrypt_str_with_padding_iso_cbc(&mut self, iv: u64, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_iso_cbc(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_iso_cbc_into_vec<T>(&mut self, iv: u64, message: &str, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_cbc_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_iso_cbc_into_array<T, const N: usize>(&mut self, iv: u64, message: &str, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_cbc_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_iso_cbc(&mut self, iv: u64, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_iso_cbc(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_iso_cbc_into_vec<T>(&mut self, iv: u64, message: &String, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_cbc_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_iso_cbc_into_array<T, const N: usize>(&mut self, iv: u64, message: &String, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_cbc_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_iso_cbc<T, const N: usize>(&mut self, iv: u64, message: &[T; N], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_cbc(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_iso_cbc_into_vec<T, U, const N: usize>(&mut self, iv: u64, message: &[T; N], cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_cbc_into_vec(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_iso_cbc_into_array<T, U, const N: usize, const M: usize>(&mut self, iv: u64, message: &[T; N], cipher: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_cbc_into_array(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_iso_cbc<T>(&mut self, iv: u64, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_cbc(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_iso_cbc_into_vec<T, U>(&mut self, iv: u64, message: &Vec<T>, cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_cbc_into_vec(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_iso_cbc_into_array<T, U, const N: usize>(&mut self, iv: u64, message: &Vec<T>, cipher: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_cbc_into_array(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }



    pub fn decrypt_with_padding_iso_cbc(&mut self, mut iv: u64, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut decoded: u64;
        let mut block: u64;
        for _ in 0..(length_in_bytes >> 3) - 1  // length_in_bytes >> 3 == length_in_bytes / 8
        {
            block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
            decoded = iv ^ self.decrypt_u64(block);
            iv = block;
            unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), 8); }
            progress += 8;
        }

        block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
        decoded = iv ^ self.decrypt_u64(block);
        let decoded_union = LongUnion::new_with(decoded);
        for i in 0..8_usize
        {
            if decoded_union.get_ubyte_(7-i) == 0
                { continue; }
            if decoded_union.get_ubyte_(7-i) == 0b_1000_0000_u8
            {
                let message_bytes = 7-i;
                unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), message_bytes); }
                self.block.set(Self::SUCCESS);
                return progress + message_bytes as u64;
            }
            else
            {
                self.block.set(Self::FAILURE);
                return 0;
            }
        }
        self.block.set(Self::FAILURE);
        return 0;
    }

    pub fn decrypt_with_padding_iso_cbc_into_vec<T>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_decrypt_into_vec!(message, length_in_bytes, T);
        let len = self.decrypt_with_padding_iso_cbc(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(len as usize);
        len
    }

    pub fn decrypt_with_padding_iso_cbc_into_array<T, const N: usize>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if length_in_bytes as u128 > T::size_in_bytes() as u128 * N as u128 + 1
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_decrypt_into_array!(message, length_in_bytes, T);
        self.decrypt_with_padding_iso_cbc(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn decrypt_with_padding_iso_cbc_into_string(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_with_padding_iso_cbc_into_vec(iv, cipher, length_in_bytes, unsafe { message.as_mut_vec() })
    }

    #[inline]
    pub fn decrypt_vec_with_padding_iso_cbc<T>(&mut self, iv: u64, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_cbc(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_iso_cbc_into_vec<T, U>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_cbc_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_iso_cbc_into_array<T, U, const N: usize>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_cbc_into_array(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_iso_cbc_into_string<T>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_cbc_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_iso_cbc<T, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_cbc(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_iso_cbc_into_vec<T, U, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_cbc_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_iso_cbc_into_array<T, U, const N: usize, const M: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_cbc_into_array(iv, cipher.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_iso_cbc_into_string<T, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_cbc_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }



    pub fn encrypt_with_padding_pkcs7_pcbc(&mut self, mut iv: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        for _ in 0..length_in_bytes >> 3    // length_in_bytes >> 3 == length_in_bytes / 8
        {
            let block = unsafe { *(message.add(progress as usize) as *const u64 ) };
            let encoded = self.encrypt_u64(block ^ iv);
            unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
            iv = block ^ encoded;
            progress += 8;
        }

        let mut block = 0_u64;
        let mut block_union = LongUnion::new_with(0x_08_08_08_08__08_08_08_08);
        if progress != length_in_bytes
        {
            let tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
            let padding = 8 - tail as u8;
            block_union.set(block);
            for i in tail..8
                { block_union.set_ubyte_(i, padding); }
        }
        let encoded = self.encrypt_u64(block_union.get() ^ iv);
        unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
        self.block.set(Self::SUCCESS);
        progress + 8
    }

    /// Encrypts the data with the padding defined in PKCS #7.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and pushed into the vector `cipher`.
    pub fn encrypt_with_padding_pkcs7_pcbc_into_vec<T>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_encrypt_into_vec!(cipher, length_in_bytes, T);
        self.encrypt_with_padding_pkcs7_pcbc(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    /// Encrypts the data with the padding defined in PKCS #7.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and stored into the array `cipher`.
    /// - If `N` is less than the next multiple of 8 from `length_in_bytes`,
    ///   this method does not perform encryption and returns `false`.
    /// - If `N` is equal to the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns `true`.
    /// - If `N` is greater than the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and then fills the rest of elements of
    ///   the array `cipher`, and returns `true`.
    /// 
    pub fn encrypt_with_padding_pkcs7_pcbc_into_array<T, const N: usize>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if (length_in_bytes as u128 + 1).next_multiple_of(8) > T::size_in_bytes() as u128 * N as u128
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_encrypt_into_array!(cipher, length_in_bytes, T);
        self.encrypt_with_padding_pkcs7_pcbc(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn encrypt_str_with_padding_pkcs7_pcbc(&mut self, iv: u64, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_pkcs7_pcbc(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_pkcs7_pcbc_into_vec<T>(&mut self, iv: u64, message: &str, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_pcbc_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_pkcs7_pcbc_into_array<T, const N: usize>(&mut self, iv: u64, message: &str, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_pcbc_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_pkcs7_pcbc(&mut self, iv: u64, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_pkcs7_pcbc(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_pkcs7_pcbc_into_vec<T>(&mut self, iv: u64, message: &String, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_pcbc_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_pkcs7_pcbc_into_array<T, const N: usize>(&mut self, iv: u64, message: &String, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_pcbc_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_pkcs7_pcbc<T>(&mut self, iv: u64, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_pcbc(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_pkcs7_pcbc_into_vec<T, U>(&mut self, iv: u64, message: &Vec<T>, cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_pcbc_into_vec(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_pkcs7_pcbc_into_array<T, U, const N: usize>(&mut self, iv: u64, message: &Vec<T>, cipher: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_pcbc_into_array(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_pkcs7_pcbc<T, const N: usize>(&mut self, iv: u64, message: &[T; N], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_pcbc(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_pkcs7_pcbc_into_vec<T, U, const N: usize>(&mut self, iv: u64, message: &[T; N], cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_pcbc_into_vec(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_pkcs7_pcbc_into_array<T, U, const N: usize, const M: usize>(&mut self, iv: u64, message: &[T; N], cipher: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_pkcs7_pcbc_into_array(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }




    pub fn decrypt_with_padding_pkcs7_pcbc(&mut self, mut iv: u64, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        if length_in_bytes > 8
        {
            for _ in 0..(length_in_bytes >> 3) - 1
            {
                let block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
                let decoded = iv ^ self.decrypt_u64(block);
                iv = block ^ decoded;
                unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), 8); }
                progress += 8;
            }
        }

        let block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
        let decoded = iv ^ self.decrypt_u64(block);
        let decoded_union = LongUnion::new_with(decoded);
        let padding_bytes = decoded_union.get_ubyte_(7);
        let message_bytes = 8 - padding_bytes as usize;
        for i in (message_bytes)..8
        {
            if decoded_union.get_ubyte_(i) != padding_bytes
            {
                self.block.set(Self::FAILURE);
                return 0;
            }
        }
        unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), message_bytes); }
        self.block.set(Self::SUCCESS);
        progress + message_bytes as u64
    }

    pub fn decrypt_with_padding_pkcs7_pcbc_into_vec<T>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_decrypt_into_vec!(message, length_in_bytes, T);
        let len = self.decrypt_with_padding_pkcs7_pcbc(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(len as usize);
        len
    }

    pub fn decrypt_with_padding_pkcs7_pcbc_into_array<T, const N: usize>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if length_in_bytes as u128 > T::size_in_bytes() as u128 * N as u128 + 1
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_decrypt_into_array!(message, length_in_bytes, T);
        self.decrypt_with_padding_pkcs7_pcbc(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn decrypt_with_padding_pkcs7_pcbc_into_string(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_with_padding_pkcs7_pcbc_into_vec(iv, cipher, length_in_bytes, unsafe { message.as_mut_vec() })
    }

    #[inline]
    pub fn decrypt_vec_with_padding_pkcs7_pcbc<T>(&mut self, iv: u64, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_pcbc(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_pkcs7_pcbc_into_vec<T, U>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_pcbc_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_pkcs7_pcbc_into_array<T, U, const N: usize>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_pcbc_into_array(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_pkcs7_pcbc_into_string<T>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_pcbc_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_pkcs7_pcbc<T, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_pcbc(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_pkcs7_pcbc_into_vec<T, U, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_pcbc_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_pkcs7_pcbc_into_array<T, U, const N: usize, const M: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_pcbc_into_array(iv, cipher.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_pkcs7_pcbc_into_string<T, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_pkcs7_pcbc_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }



    pub fn encrypt_with_padding_iso_pcbc(&mut self, mut iv: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        for _ in 0..length_in_bytes >> 3    // length_in_bytes >> 3 == length_in_bytes / 8
        {
            let block = unsafe { *(message.add(progress as usize) as *const u64 ) };
            let encoded = self.encrypt_u64(block ^ iv);
            unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
            iv = block ^ encoded;
            progress += 8;
        }

        let mut block = 0_u64;
        let mut block_union = LongUnion::new_with(0b_1000_0000);
        if progress != length_in_bytes
        {
            let tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
            block_union.set(block);
            block_union.set_ubyte_(tail, 0b_1000_0000);
        }
        let encoded = self.encrypt_u64(block_union.get() ^ iv);
        unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
        self.block.set(Self::SUCCESS);
        progress + 8
    }

    pub fn encrypt_with_padding_iso_pcbc_into_vec<T>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_encrypt_into_vec!(cipher, length_in_bytes, T);
        self.encrypt_with_padding_iso_pcbc(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    pub fn encrypt_with_padding_iso_pcbc_into_array<T, const N: usize>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if (length_in_bytes as u128 + 1).next_multiple_of(8) > T::size_in_bytes() as u128 * N as u128
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_encrypt_into_array!(cipher, length_in_bytes, T);
        self.encrypt_with_padding_iso_pcbc(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn encrypt_str_with_padding_iso_pcbc(&mut self, iv: u64, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_iso_pcbc(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_iso_pcbc_into_vec<T>(&mut self, iv: u64, message: &str, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_pcbc_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_with_padding_iso_pcbc_into_array<T, const N: usize>(&mut self, iv: u64, message: &str, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_pcbc_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_iso_pcbc(&mut self, iv: u64, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_with_padding_iso_pcbc(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_iso_pcbc_into_vec<T>(&mut self, iv: u64, message: &String, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_pcbc_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_with_padding_iso_pcbc_into_array<T, const N: usize>(&mut self, iv: u64, message: &String, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_pcbc_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_iso_pcbc<T, const N: usize>(&mut self, iv: u64, message: &[T; N], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_pcbc(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_iso_pcbc_into_vec<T, U, const N: usize>(&mut self, iv: u64, message: &[T; N], cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_pcbc_into_vec(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_with_padding_iso_pcbc_into_array<T, U, const N: usize, const M: usize>(&mut self, iv: u64, message: &[T; N], cipher: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_pcbc_into_array(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_iso_pcbc<T>(&mut self, iv: u64, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_pcbc(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_iso_pcbc_into_vec<T, U>(&mut self, iv: u64, message: &Vec<T>, cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_pcbc_into_vec(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_with_padding_iso_pcbc_into_array<T, U, const N: usize>(&mut self, iv: u64, message: &Vec<T>, cipher: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_with_padding_iso_pcbc_into_array(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }



    pub fn decrypt_with_padding_iso_pcbc(&mut self, mut iv: u64, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        if length_in_bytes > 8
        {
            for i in 0..(length_in_bytes >> 3) - 1
            {
                let block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
                let decoded = iv ^ self.decrypt_u64(block);
                iv = block ^ decoded;
                unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), 8); }
                progress += 8;
            }
        }

        let block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
        let decoded = iv ^ self.decrypt_u64(block);
        let decoded_union = LongUnion::new_with(decoded);
        for i in 0..8_usize
        {
            if decoded_union.get_ubyte_(7-i) == 0
                { continue; }
            if decoded_union.get_ubyte_(7-i) == 0b_1000_0000_u8
            {
                let message_bytes = 7-i;
                unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), message_bytes); }
                self.block.set(Self::SUCCESS);
                return progress + message_bytes as u64;
            }
            else
            {
                self.block.set(Self::FAILURE);
                return 0;
            }
        }
        self.block.set(Self::FAILURE);
        return 0;
    }

    pub fn decrypt_with_padding_iso_pcbc_into_vec<T>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_decrypt_into_vec!(message, length_in_bytes, T);
        let len = self.decrypt_with_padding_iso_pcbc(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(len as usize);
        len
    }

    pub fn decrypt_with_padding_iso_pcbc_into_array<T, const N: usize>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if length_in_bytes as u128 > T::size_in_bytes() as u128 * N as u128 + 1
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_decrypt_into_array!(message, length_in_bytes, T);
        self.decrypt_with_padding_iso_pcbc(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn decrypt_with_padding_iso_pcbc_into_string(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_with_padding_iso_pcbc_into_vec(iv, cipher, length_in_bytes, unsafe { message.as_mut_vec() })
    }

    #[inline]
    pub fn decrypt_vec_with_padding_iso_pcbc<T>(&mut self, iv: u64, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_pcbc(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_iso_pcbc_into_vec<T, U>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_pcbc_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_iso_pcbc_into_array<T, U, const N: usize>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_pcbc_into_array(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_with_padding_iso_pcbc_into_string<T>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_pcbc_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_iso_pcbc<T, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_pcbc(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_iso_pcbc_into_vec<T, U, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_pcbc_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_iso_pcbc_into_array<T, U, const N: usize, const M: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_pcbc_into_array(iv, cipher.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_with_padding_iso_pcbc_into_string<T, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_with_padding_iso_pcbc_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }



    pub fn encrypt_cfb(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut encoded = iv;
        for _ in 0..length_in_bytes >> 3    // length_in_bytes >> 3 == length_in_bytes / 8
        {
            let block = unsafe { *(message.add(progress as usize) as *const u64 ) };
            encoded = block ^ self.encrypt_u64(encoded);
            unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
            progress += 8;
        }

        if progress == length_in_bytes
        {
            self.block.set(Self::SUCCESS);
            progress
        }
        else
        {
            let mut block = 0_u64;
            let tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
            encoded = block ^ self.encrypt_u64(encoded);
            unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), tail); }
            self.block.set(Self::SUCCESS);
            progress + tail as u64
        }
    }

    // pub fn encrypt_cfb_into_vec<T>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    /// Encrypts the data without padding.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and pushed into the vector `cipher`.
    pub fn encrypt_cfb_into_vec<T>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_encrypt_into_vec!(cipher, length_in_bytes, T);
        let len = self.encrypt_cfb(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8);
        cipher.truncate(len as usize);
        len
    }

    // pub fn encrypt_cfb_into_array<T, const N: usize>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    /// Encrypts the data with the padding defined in PKCS #7.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and stored into the array `cipher`.
    /// - If `N` is less than the next multiple of 8 from `length_in_bytes`,
    ///   this method does not perform encryption and returns `false`.
    /// - If `N` is equal to the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns `true`.
    /// - If `N` is greater than the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and then fills the rest of elements of
    ///   the array `cipher`, and returns `true`.
    /// 
    pub fn encrypt_cfb_into_array<T, const N: usize>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if length_in_bytes as u128 > T::size_in_bytes() as u128 * N as u128
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_encrypt_into_array!(cipher, length_in_bytes, T);
        self.encrypt_cfb(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn encrypt_str_cfb(&mut self, iv: u64, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_cfb(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_cfb_into_vec<T>(&mut self, iv: u64, message: &str, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_cfb_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_cfb_into_array<T, const N: usize>(&mut self, iv: u64, message: &str, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_cfb_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_cfb(&mut self, iv: u64, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_cfb(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_cfb_into_vec<T>(&mut self, iv: u64, message: &String, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_cfb_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_cfb_into_array<T, const N: usize>(&mut self, iv: u64, message: &String, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_cfb_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_cfb<T>(&mut self, iv: u64, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_cfb(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_cfb_into_vec<T, U>(&mut self, iv: u64, message: &Vec<T>, cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_cfb_into_vec(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_cfb_into_array<T, U, const N: usize>(&mut self, iv: u64, message: &Vec<T>, cipher: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_cfb_into_array(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_cfb<T, const N: usize>(&mut self, iv: u64, message: &[T; N], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_cfb(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_cfb_into_vec<T, U, const N: usize>(&mut self, iv: u64, message: &[T; N], cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_cfb_into_vec(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_cfb_into_array<T, U, const N: usize, const M: usize>(&mut self, iv: u64, message: &[T; N], cipher: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_cfb_into_array(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }



    pub fn decrypt_cfb(&mut self, mut iv: u64, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        for _ in 0..length_in_bytes >> 3    // length_in_bytes >> 3 == length_in_bytes / 8
        {
            let block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
            let decoded = block ^ self.encrypt_u64(iv);
            iv = block;
            unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), 8); }
            progress += 8;
        }

        if progress == length_in_bytes
        {
            progress
        }
        else
        {
            let mut block = 0_u64;
            let tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { cipher.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
            let decoded = block ^ self.encrypt_u64(iv);
            unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), tail); }
            progress + tail as u64
        }
    }

    pub fn decrypt_cfb_into_vec<T>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_decrypt_into_vec!(message, length_in_bytes, T);
        let len = self.decrypt_cfb(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(len as usize);
        len
    }

    pub fn decrypt_cfb_into_array<T, const N: usize>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if length_in_bytes as u128 > T::size_in_bytes() as u128 * N as u128
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_decrypt_into_array!(message, length_in_bytes, T);
        self.decrypt_cfb(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn decrypt_cfb_into_string(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_cfb_into_vec(iv, cipher, length_in_bytes, unsafe { message.as_mut_vec() })
    }

    #[inline]
    pub fn decrypt_vec_cfb<T>(&mut self, iv: u64, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_cfb(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_cfb_into_vec<T, U>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_cfb_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_cfb_into_array<T, U, const N: usize>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_cfb_into_array(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_cfb_into_string<T>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_cfb_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_cfb<T, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_cfb(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_cfb_into_vec<T, U, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_cfb_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_cfb_into_array<T, U, const N: usize, const M: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_cfb_into_array(iv, cipher.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_cfb_into_string<T, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_cfb_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }



    fn crypt_ofb(&mut self, mut iv: u64, from: *const u8, length_in_bytes: u64, to: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        for _ in 0..length_in_bytes >> 3    // length_in_bytes >> 3 == length_in_bytes / 8
        {
            let block = unsafe { *(from.add(progress as usize) as *const u64 ) };
            iv = self.encrypt_u64(iv);
            let coded = block ^ iv;
            unsafe { copy_nonoverlapping(&coded as *const u64 as *const u8, to.add(progress as usize), 8); }
            progress += 8;
        }

        if progress == length_in_bytes
        {
            self.block.set(Self::SUCCESS);
            progress
        }
        else
        {
            let mut block = 0_u64;
            let tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { from.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
            let coded = block ^ self.encrypt_u64(iv);
            unsafe { copy_nonoverlapping(&coded as *const u64 as *const u8, to.add(progress as usize), tail); }
            self.block.set(Self::SUCCESS);
            progress + tail as u64
        }
    }

    pub fn encrypt_ofb(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        self.crypt_ofb(iv, message, length_in_bytes, cipher)
    }

    // pub fn encrypt_ofb_into_vec<T>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    /// Encrypts the data without padding.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and pushed into the vector `cipher`.
    pub fn encrypt_ofb_into_vec<T>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_encrypt_into_vec!(cipher, length_in_bytes, T);
        let len = self.encrypt_ofb(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8);
        cipher.truncate(len as usize);
        len
    }

    // pub fn encrypt_ofb_into_array<T, const N: usize>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    /// Encrypts the data with the padding defined in PKCS #7.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and stored into the array `cipher`.
    /// - If `N` is less than the next multiple of 8 from `length_in_bytes`,
    ///   this method does not perform encryption and returns `false`.
    /// - If `N` is equal to the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns `true`.
    /// - If `N` is greater than the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and then fills the rest of elements of
    ///   the array `cipher`, and returns `true`.
    /// 
    pub fn encrypt_ofb_into_array<T, const N: usize>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if length_in_bytes as u128 > T::size_in_bytes() as u128 * N as u128
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_encrypt_into_array!(cipher, length_in_bytes, T);
        self.encrypt_ofb(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn encrypt_str_ofb(&mut self, iv: u64, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_ofb(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_ofb_into_vec<T>(&mut self, iv: u64, message: &str, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_ofb_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_ofb_into_array<T, const N: usize>(&mut self, iv: u64, message: &str, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_ofb_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_ofb(&mut self, iv: u64, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_ofb(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_ofb_into_vec<T>(&mut self, iv: u64, message: &String, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_ofb_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_ofb_into_array<T, const N: usize>(&mut self, iv: u64, message: &String, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_ofb_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_ofb<T>(&mut self, iv: u64, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_ofb(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_ofb_into_vec<T, U>(&mut self, iv: u64, message: &Vec<T>, cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_ofb_into_vec(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_ofb_into_array<T, U, const N: usize>(&mut self, iv: u64, message: &Vec<T>, cipher: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_ofb_into_array(iv, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_ofb<T, const N: usize>(&mut self, iv: u64, message: &[T; N], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_ofb(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_ofb_into_vec<T, U, const N: usize>(&mut self, iv: u64, message: &[T; N], cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_ofb_into_vec(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_ofb_into_array<T, U, const N: usize, const M: usize>(&mut self, iv: u64, message: &[T; N], cipher: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_ofb_into_array(iv, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }



    #[inline]
    pub fn decrypt_ofb(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        self.crypt_ofb(iv, cipher, length_in_bytes, message)
    }

    pub fn decrypt_ofb_into_vec<T>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_decrypt_into_vec!(message, length_in_bytes, T);
        let len = self.decrypt_ofb(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(len as usize);
        len
    }

    pub fn decrypt_ofb_into_array<T, const N: usize>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if length_in_bytes as u128 > T::size_in_bytes() as u128 * N as u128
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_decrypt_into_array!(message, length_in_bytes, T);
        self.decrypt_ofb(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn decrypt_ofb_into_string(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_ofb_into_vec(iv, cipher, length_in_bytes, unsafe { message.as_mut_vec() })
    }

    #[inline]
    pub fn decrypt_vec_ofb<T>(&mut self, iv: u64, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_ofb(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_ofb_into_vec<T, U>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_ofb_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_ofb_into_array<T, U, const N: usize>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_ofb_into_array(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_ofb_into_string<T>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_ofb_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_ofb<T, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_ofb(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_ofb_into_vec<T, U, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_ofb_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_ofb_into_array<T, U, const N: usize, const M: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_ofb_into_array(iv, cipher.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_ofb_into_string<T, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_ofb_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    
    fn crypt_ctr(&mut self, mut nonce: u64, from: *const u8, length_in_bytes: u64, to: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        nonce = nonce.wrapping_add(1);
        for _ in 0..length_in_bytes >> 3    // length_in_bytes >> 3 == length_in_bytes / 8
        {
            let block = unsafe { *(from.add(progress as usize) as *const u64 ) };
            let coded = block ^ self.encrypt_u64(nonce);
            nonce = nonce.wrapping_add(1);
            unsafe { copy_nonoverlapping(&coded as *const u64 as *const u8, to.add(progress as usize), 8); }
            progress += 8;
        }

        let mut tail = 8_usize;
        let mut block: u64;
        if progress + 8 == length_in_bytes
        {
            block = unsafe { *(from.add(progress as usize - 8) as *const u64 ) };
        }
        else
        {
            block = 0_u64;
            tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { from.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
        }
        let coded = block ^ self.encrypt_u64(nonce);
        unsafe { copy_nonoverlapping(&coded as *const u64 as *const u8, to.add(progress as usize), tail); }
        self.block.set(Self::SUCCESS);
        progress + tail as u64
    }

    #[inline]
    pub fn encrypt_ctr(&mut self, nonce: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        self.crypt_ctr(nonce, message, length_in_bytes, cipher)
    }

    // pub fn encrypt_ctr_into_vec<T>(&mut self, nonce: u64, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    /// Encrypts the data without padding.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and pushed into the vector `cipher`.
    pub fn encrypt_ctr_into_vec<T>(&mut self, nonce: u64, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_encrypt_into_vec!(cipher, length_in_bytes, T);
        let len = self.encrypt_ctr(nonce, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8);
        cipher.truncate(len as usize);
        len
    }

    // pub fn encrypt_ctr_into_array<T, const N: usize>(&mut self, nonce: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    /// Encrypts the data with the padding defined in PKCS #7.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and stored into the array `cipher`.
    /// - If `N` is less than the next multiple of 8 from `length_in_bytes`,
    ///   this method does not perform encryption and returns `false`.
    /// - If `N` is equal to the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns `true`.
    /// - If `N` is greater than the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and then fills the rest of elements of
    ///   the array `cipher`, and returns `true`.
    /// 
    pub fn encrypt_ctr_into_array<T, const N: usize>(&mut self, nonce: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if length_in_bytes as u128 > T::size_in_bytes() as u128 * N as u128
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_encrypt_into_array!(cipher, length_in_bytes, T);
        self.encrypt_ctr(nonce, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn encrypt_str_ctr(&mut self, nonce: u64, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt_ctr(nonce, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_ctr_into_vec<T>(&mut self, nonce: u64, message: &str, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_ctr_into_vec(nonce, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_str_ctr_into_array<T, const N: usize>(&mut self, nonce: u64, message: &str, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_ctr_into_array(nonce, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_ctr(&mut self, nonce: u64, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt_ctr(nonce, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_ctr_into_vec<T>(&mut self, nonce: u64, message: &String, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_ctr_into_vec(nonce, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_string_ctr_into_array<T, const N: usize>(&mut self, nonce: u64, message: &String, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_ctr_into_array(nonce, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_ctr<T>(&mut self, nonce: u64, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_ctr(nonce, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_ctr_into_vec<T, U>(&mut self, nonce: u64, message: &Vec<T>, cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_ctr_into_vec(nonce, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_vec_ctr_into_array<T, U, const N: usize>(&mut self, nonce: u64, message: &Vec<T>, cipher: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_ctr_into_array(nonce, message.as_ptr() as *const u8, (message.len() as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_ctr<T, const N: usize>(&mut self, nonce: u64, message: &[T; N], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.encrypt_ctr(nonce, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_ctr_into_vec<T, U, const N: usize>(&mut self, nonce: u64, message: &[T; N], cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_ctr_into_vec(nonce, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    pub fn encrypt_array_ctr_into_array<T, U, const N: usize, const M: usize>(&mut self, nonce: u64, message: &[T; N], cipher: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.encrypt_ctr_into_array(nonce, message.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, cipher)
    }



    #[inline]
    pub fn decrypt_ctr(&mut self, nonce: u64, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        self.crypt_ctr(nonce, cipher, length_in_bytes, message)
    }

    pub fn decrypt_ctr_into_vec<T>(&mut self, nonce: u64, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        pre_decrypt_into_vec!(message, length_in_bytes, T);
        let len = self.decrypt_ctr(nonce, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(len as usize);
        len
    }

    pub fn decrypt_ctr_into_array<T, const N: usize>(&mut self, nonce: u64, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        if length_in_bytes as u128 > T::size_in_bytes() as u128 * N as u128
        {
            self.block.set(Self::FAILURE);
            return 0;
        }
        pre_decrypt_into_array!(message, length_in_bytes, T);
        self.decrypt_ctr(nonce, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8)
    }

    #[inline]
    pub fn decrypt_ctr_into_string(&mut self, nonce: u64, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_ctr_into_vec(nonce, cipher, length_in_bytes, unsafe { message.as_mut_vec() })
    }

    #[inline]
    pub fn decrypt_vec_ctr<T>(&mut self, nonce: u64, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_ctr(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_ctr_into_vec<T, U>(&mut self, nonce: u64, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_ctr_into_vec(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_ctr_into_array<T, U, const N: usize>(&mut self, nonce: u64, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_ctr_into_array(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_vec_ctr_into_string<T>(&mut self, nonce: u64, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_ctr_into_string(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_ctr<T, const N: usize>(&mut self, nonce: u64, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_ctr(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_ctr_into_vec<T, U, const N: usize>(&mut self, nonce: u64, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_ctr_into_vec(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_ctr_into_array<T, U, const N: usize, const M: usize>(&mut self, nonce: u64, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        self.decrypt_ctr_into_array(nonce, cipher.as_ptr() as *const u8, (N as u32 * T::size_in_bytes()) as u64, message)
    }

    #[inline]
    pub fn decrypt_array_ctr_into_string<T, const N: usize>(&mut self, nonce: u64, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        self.decrypt_ctr_into_string(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * T::size_in_bytes()) as u64, message)
    }
}


pub struct TDES {}

impl TDES
{
    pub fn new_with_2_keys_u64(key1: u64, key2: u64) -> BigCryptor64
    {
        BigCryptor64::new() + DES::encryptor_with_key_u64(key1)
                            + DES::decryptor_with_key_u64(key2)
                            + DES::encryptor_with_key_u64(key1)
    }

    pub fn new_with_3_keys_u64(key1: u64, key2: u64, key3: u64) -> BigCryptor64
    {
        BigCryptor64::new() + DES::encryptor_with_key_u64(key1)
                            + DES::decryptor_with_key_u64(key2)
                            + DES::encryptor_with_key_u64(key3)
    }
    
    pub fn new_with_2_keys(key1: [u8; 8], key2: [u8; 8]) -> BigCryptor64
    {
        BigCryptor64::new() + DES::encryptor_with_key(key1)
                            + DES::decryptor_with_key(key2)
                            + DES::encryptor_with_key(key1)
    }

    pub fn new_with_3_keys(key1: [u8; 8], key2: [u8; 8], key3: [u8; 8]) -> BigCryptor64
    {
        BigCryptor64::new() + DES::encryptor_with_key(key1)
                            + DES::decryptor_with_key(key2)
                            + DES::encryptor_with_key(key3)
    }

    pub fn new_with_keys_u128(key: u128) -> BigCryptor64
    {
        let key: LongerUnion = LongerUnion::new_with(key);
        BigCryptor64::new() + DES::encryptor_with_key_u64(key.get_ulong_(0))
                            + DES::decryptor_with_key_u64(key.get_ulong_(1))
                            + DES::encryptor_with_key_u64(key.get_ulong_(0))
    }
}


pub struct DDES {}

impl DDES
{
    pub fn new_with_2_keys_u64(key1: u64, key2: u64) -> BigCryptor64
    {
        BigCryptor64::new() + DES::encryptor_with_key_u64(key1)
                            + DES::decryptor_with_key_u64(key2)
    }
    
    pub fn new_with_2_keys(key1: [u8; 8], key2: [u8; 8]) -> BigCryptor64
    {
        BigCryptor64::new() + DES::encryptor_with_key(key1)
                            + DES::decryptor_with_key(key2)
    }

    pub fn new_with_keys_u128(key: u128) -> BigCryptor64
    {
        let key: LongerUnion = LongerUnion::new_with(key);
        BigCryptor64::new() + DES::encryptor_with_key_u64(key.get_ulong_(0))
                            + DES::decryptor_with_key_u64(key.get_ulong_(1))
    }
}


