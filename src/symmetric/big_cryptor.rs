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
use crate::number::{ LongUnion, LongerUnion, SmallUInt };
use crate::symmetric::{ DES, SmallCryptor };

/*
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
*/


#[allow(non_camel_case_types)]
pub struct BigCryptor128
{
    block: LongerUnion,
    smallcryptor: Vec<Box<dyn SmallCryptor<u128, 16>>>,
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

    // pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor<u128, 16>>; N]) -> Self
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
    pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor<u128, 16>>; N]) -> Self
    {
        let mut bigcryptor = Self::new();
        bigcryptor.push_small_cryptor_array(smallcryptor);
        bigcryptor
    }

    // pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor<u128, 16>>>) -> Self
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
    pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor<u128, 16>>>) -> Self
    {
        let mut bigcryptor = Self::new();
        bigcryptor.push_small_cryptor_vec(smallcryptor);
        bigcryptor
    }

    pub fn push_small_cryptor<S: SmallCryptor<u128, 16> + 'static>(&mut self, smallcryptor: S)
    {
        self.smallcryptor.push(Box::<S>::new(smallcryptor));
    }

    // pub fn push_small_cryptor_array<const N: usize>(&mut self, smallcryptor: [Box<dyn SmallCryptor<u128, 16>>; N])
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
    pub fn push_small_cryptor_array<const N: usize>(&mut self, smallcryptor: [Box<dyn SmallCryptor<u128, 16>>; N])
    {
        for val in smallcryptor
            { self.smallcryptor.push(val); }
    }

    // pub fn push_small_cryptor_vec(&mut self, smallcryptor: Vec<Box<dyn SmallCryptor<u128, 16>>>)
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
    pub fn push_small_cryptor_vec(&mut self, smallcryptor: Vec<Box<dyn SmallCryptor<u128, 16>>>)
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
/// use cryptocol::symmetric::{ BigCryptor64, SmallCryptor<u64, 8> };
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
/// ```text
/// // to do
/// ```
/// 
/// You can instantiate the DES object without key and set a `u64` key later as
/// shown in Example 4 or a `[u8; 8]` key later as shown in Example 5.
/// 
/// # Example 4
/// ```text
/// // to do
/// ```
/// 
/// # Example 5
/// ```text
/// // to do
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
/// ```text
/// // to do
/// ```
/// 
/// # Example 7
/// ```text
/// // to do
/// ```
#[allow(non_camel_case_types)]
pub struct BigCryptor64
{
    block: LongUnion,
    smallcryptor: Vec<Box<dyn SmallCryptor<u64, 8>>>,
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

    // pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor<u64, 8>>; N]) -> Self
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
    pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor<u64, 8>>; N]) -> Self
    {
        let mut bigcryptor = Self::new();
        bigcryptor.push_small_cryptor_array(smallcryptor);
        bigcryptor
    }

    // pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor<u64, 8>>>) -> Self
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
    pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor<u64, 8>>>) -> Self
    {
        let mut bigcryptor = Self::new();
        bigcryptor.push_small_cryptor_vec(smallcryptor);
        bigcryptor
    }

    pub fn push_small_cryptor<S: SmallCryptor<u64, 8> + 'static>(&mut self, smallcryptor: S)
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
    pub fn push_small_cryptor_array<const N: usize>(&mut self, smallcryptor: [Box<dyn SmallCryptor<u64, 8>>; N])
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
    pub fn push_small_cryptor_vec(&mut self, smallcryptor: Vec<Box<dyn SmallCryptor<u64, 8>>>)
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



/*
use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
                BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, RangeBounds };

pub struct BigCryptor<T, Union, const N: usize>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    Union: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=Union> + AddAssign + Sub<Output=Union> + SubAssign
        + Mul<Output=Union> + MulAssign + Div<Output=Union> + DivAssign
        + Rem<Output=Union> + RemAssign
        + Shl<Output=Union> + ShlAssign + Shr<Output=Union> + ShrAssign
        + BitAnd<Output=Union> + BitAndAssign + BitOr<Output=Union> + BitOrAssign
        + BitXor<Output=Union> + BitXorAssign + Not<Output=Union>
        + PartialEq + PartialOrd
{
    block: Union,
    smallcryptor: Vec<Box<dyn SmallCryptor<T, N>>>,
}

impl<T, Union, const N: usize> BigCryptor<T, Union, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    Union: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=Union> + AddAssign + Sub<Output=Union> + SubAssign
        + Mul<Output=Union> + MulAssign + Div<Output=Union> + DivAssign
        + Rem<Output=Union> + RemAssign
        + Shl<Output=Union> + ShlAssign + Shr<Output=Union> + ShrAssign
        + BitAnd<Output=Union> + BitAndAssign + BitOr<Output=Union> + BitOrAssign
        + BitXor<Output=Union> + BitXorAssign + Not<Output=Union>
        + PartialEq + PartialOrd
{
    const SUCCESS: T = T::MAX;
    const FAILURE: T = T::ZERO;

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
        Self { block: Union::zero(), smallcryptor: Vec::new() }
    }

    // pub fn new_with_small_cryptor_array<const M: usize>(smallcryptor: [Box<dyn SmallCryptor<T, N>>; M]) -> Self
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
    pub fn new_with_small_cryptor_array<const M: usize>(smallcryptor: [Box<dyn SmallCryptor<T, N>>; M]) -> Self
    {
        let mut bigcryptor = Self::new();
        bigcryptor.push_small_cryptor_array(smallcryptor);
        bigcryptor
    }

    // pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor<T, N>>>) -> Self
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
    pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor<T, N>>>) -> Self
    {
        let mut bigcryptor = Self::new();
        bigcryptor.push_small_cryptor_vec(smallcryptor);
        bigcryptor
    }

    pub fn push_small_cryptor<S: SmallCryptor<T, N> + 'static>(&mut self, smallcryptor: S)
    {
        self.smallcryptor.push(Box::<S>::new(smallcryptor));
    }

    // pub fn push_small_cryptor_array<const M: usize>(&mut self, smallcryptor: [Box<dyn SmallCryptor<T, N>>; M])
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
    pub fn push_small_cryptor_array<const M: usize>(&mut self, smallcryptor: [Box<dyn SmallCryptor<T, N>>; M])
    {
        for val in smallcryptor
            { self.smallcryptor.push(val); }
    }

    // pub fn push_small_cryptor_vec(&mut self, smallcryptor: Vec<Box<dyn SmallCryptor<T, N>>>)
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
    pub fn push_small_cryptor_vec(&mut self, smallcryptor: Vec<Box<dyn SmallCryptor<T, N>>>)
    {
        self.smallcryptor = smallcryptor;
    }

    // pub fn encrypt_unit(&mut self, message: T) -> T
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
    pub fn encrypt_unit(&mut self, message: T) -> T
    {
        self.set_block(message);
        self.encrypt_block();
        self.block.get()
    }

    // pub fn decrypt_unit(&mut self, cipher: T) -> T
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
    pub fn decrypt_unit(&mut self, cipher: T) -> T
    {
        self.set_block(cipher);
        self.decrypt_block();
        self.block.get()
    }

    // pub fn encrypt_array_unit<const M: usize>(&mut self, message: &[T; M], cipher: &mut [T; M])
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
    pub fn encrypt_array_unit<const M: usize>(&mut self, message: &[T; M], cipher: &mut [T; M])
    {
        for i in 0..N
            { cipher[i] = self.encrypt_unit(message[i]); }
    }

    // pub fn decrypt_array_unit<const M: usize>(&mut self, cipher: &[T; M], message: &mut [T; M])
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
    pub fn decrypt_array_unit<const M: usize>(&mut self, cipher: &[T; M], message: &mut [T; M])
    {
        for i in 0..N
            { message[i] = self.decrypt_unit(cipher[i]); }
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
*/