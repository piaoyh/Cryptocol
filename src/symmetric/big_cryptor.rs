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


#[allow(non_camel_case_types)]
pub struct BigCryptor128
{
    block: LongerUnion,
    smallcryptor: Vec<Box<dyn SmallCryptor<u128, 16>>>,
}

impl BigCryptor128
{
    pub(super) const BLOCK_SIZE: usize = 16;
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
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.new)           
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.new_with_key)
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
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.new_with_key_u64)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.set_key)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.set_key)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.encrypt_u64)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.decrypt_u64)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.encrypt_array_u64)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.decrypt_array_u64)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.is_successful)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.is_failed)
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



/// A BigCryptor64 wrapper struct for cascading encryption/decryption
/// algorithms that has 64-bit block size
/// 
/// # Introduction
/// BigCryptor64 is mainly used to make TDES which folds DES by cascading DES
/// three times. However, it can be used to cascade any encryption/decryption
/// algorithms that has 64-bit block size.
///
/// # Quick Start
/// You have to import (use) the module `BigCryptor64` in order to cascade
/// encryption/decryption algorithms that has 64-bit block size such as DES
/// as shown in Example 1.
/// 
/// # Example 1
/// ```
/// use cryptocol::symmetric::BigCryptor64;
/// ```
/// 
/// You can instantiate the BigCryptor64 object with components (hereinafter,
/// referred to as small cryptor) that has `u64` keys as Example 2,
/// for example. In this case, you have to take endianness into account.
/// In little-endianness, 0x_1234567890ABCDEF_u64 is [0xEFu8, 0xCDu8, 0xABu8,
/// 0x90u8, 0x78u8, 0x56u8, 0x34u8, 0x12u8] while the same 
/// 0x_1234567890ABCDEF_u64 is [0x12u8, 0x34u8, 0x56u8, 0x78u8, 0x90u8, 0xABu8,
/// 0xCDu8, 0xEF_u64] in big-endianness.
/// The instantiated object should be mutable.
/// 
/// # Example 2
/// ```
/// use cryptocol::symmetric::{ BigCryptor64, DES };
/// let mut _tdes = BigCryptor64::new()
///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
///
/// ```
/// 
/// Also, you can instantiate the BigCryptor64 object with small cryptors
/// (component) that have `[u8; 8]` keys as shown in Example 3. In this case,
/// you don't have to take endianness into account. The instantiated object
/// should be mutable.
/// 
/// # Example 3
/// ```
/// use cryptocol::symmetric::{ BigCryptor64, DES };
/// let mut _tdes = BigCryptor64::new()
///                 + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
///                 - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
///                 + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
/// ```
/// 
/// You can instantiate the BigCryptor64 object without small cryptors
/// (components) that have key and set a `u64` key later as shown
/// in Example 4 or `[u8; 8]` keys later as shown in Example 5.
/// 
/// # Example 4
/// ```
/// use cryptocol::symmetric::{ BigCryptor64, DES };
/// let mut tdes = BigCryptor64::new();
/// let des1 = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
/// let des2 = DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64);
/// let des3 = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
/// tdes.push_small_cryptor(des1);
/// tdes.push_small_cryptor(des2);
/// tdes.push_small_cryptor(des3);
/// ```
/// 
/// # Example 5
/// ```
/// use cryptocol::symmetric::{ BigCryptor64, DES };
/// let mut tdes = BigCryptor64::new();
/// let des1 = DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
/// let des2 = DES::decryptor_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21]);
/// let des3 = DES::encryptor_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
/// tdes.push_small_cryptor(des1);
/// tdes.push_small_cryptor(des2);
/// tdes.push_small_cryptor(des3);
/// ```
/// 
/// Now, you can freely use any operation mode. This crate provide
/// ECB (Electronic CodeBook), CBC(Cipher Block Chaining), PCBC (Propagation
/// Cipher Block Chaining), CFB (Cipher FeedBack) OFB (Output FeedBack), and
/// CTR (CounTeR). You can choose the way of padding bits according to either
/// [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/) or
/// [ISO 7816-4](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
/// So, you can import (use) one of the following traits: ECB_PKCS7, ECB_ISO,
/// CBC_PKCS7, CBC_ISO, PCBC_PKCS7, PCBC_ISO, CFB, OFB, and CTR. The following
/// example 6 shows the case that you choose CBC operation mode and padding bits
/// according to PKCS #7.
/// 
/// # Example 6
/// ```
/// use std::fmt::Write as _;
/// use cryptocol::symmetric::{ BigCryptor64, DES, CBC_PKCS7 };
/// let mut tdes = BigCryptor64::new()
///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF])
///                             - DES::new_with_key([0xFE, 0xDC, 0xBA, 0x09, 0x87, 0x65, 0x43, 0x21])
///                             + DES::new_with_key([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
/// 
/// let iv = 0x_FEDCBA0987654321_u64;
/// println!("IV =	{:#018X}", iv);
/// let message = "In the beginning God created the heavens and the earth.";
/// println!("M =\t{}", message);
/// let mut cipher = Vec::<u8>::new();
/// tdes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
/// print!("C =\t");
/// for c in cipher.clone()
///     { print!("{:02X} ", c); }
/// println!();
/// let mut txt = String::new();
/// for c in cipher.clone()
///     { write!(txt, "{:02X} ", c); }
/// assert_eq!(txt, "86 2B D7 BF 00 2E CD 70 ED 0C E3 8D 75 18 CE 0F BD A7 AE AF E5 19 46 F8 15 7A 24 0E CB 20 91 C0 03 B9 56 C5 77 01 33 E8 8E 84 CA B9 F2 99 63 AC 3A 3D 1F EF CA CA CB 67 ");
///
/// let mut recovered = String::new();
/// tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
/// println!("B =\t{}", recovered);
/// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
/// assert_eq!(recovered, message);
/// ```
/// 
/// # Notice for Practical Use
/// Now, you can freely use any methods with any paddings
/// in any operation modes.
/// - This crate provides six operation modes:
///   ECB, CBC, PCBC, CFB, OFB, and CTR.
/// - This crate provides two padding ways: ISO 7816-4 and PKCS #7.
/// - The operation modes ECB, CBC and PCBC requires padding bytes.
/// - You can combine three operation modes and two padding ways.
/// - The operation modes
///   [`CFB`](./trait.CFB.html#trait.CFB),
///   [`OFB`](./trait.OFB.html#trait.OFB), and
///   [`CTR`](./trait.CTR.html#trait.CTR)
///   does not require padding bytes.
/// - The traits that implements combination of operation modes and padding
///   ways are provided such as
///   [`ECB_PKCS7`](./trait.ECB_PKCS7.html#trait.ECB_PKCS7),
///   [`ECB_ISO`](./trait.ECB_ISO.html#trait.ECB_ISO),
///   [`CBC_PKCS7`](./trait.CBC_PKCS7.html#trait.ECB_PKCS7),
///   [`CBC_ISO`](./trait.CBC_ISO.html#trait.CBC_ISO),
///   [`PCBC_PKCS7`](./trait.PCBC_PKCS7.html#trait.PCBC_PKCS7), and
///   [`PCBC_ISO`](./trait.PCBC_ISO.html#trait.PCBC_ISO).
/// - You can find detaild instructions and their helpful examples
///   if you go through those links.
///
/// In summary,
///
/// |      | padding PKCS7                                          | padding ISO                                      | no padding                        |
/// |------|--------------------------------------------------------|--------------------------------------------------|-----------------------------------|
/// | ECB  | [ECB_PKCS7](./trait.ECB_PKCS7.html#trait.ECB_PKCS7)    | [ECB_ISO](./trait.ECB_ISO.html#trait.ECB_ISO)    |                                   |
/// | CBC  | [CBC_PKCS7](./trait.CBC_PKCS7.html#trait.ECB_PKCS7)    | [CBC_ISO](./trait.CBC_ISO.html#trait.CBC_ISO)    |                                   |
/// | PCBC | [PCBC_PKCS7](./trait.PCBC_PKCS7.html#trait.PCBC_PKCS7) | [PCBC_ISO](./trait.PCBC_ISO.html#trait.PCBC_ISO) |                                   |
/// | CFB  |                                                        |                                                  | [CFB](./trait.CFB.html#trait.CFB) |
/// | OFB  |                                                        |                                                  | [OFB](./trait.OFB.html#trait.OFB) |
/// | CTR  |                                                        |                                                  | [CTR](./trait.CTR.html#trait.CTR) |
///
#[allow(non_camel_case_types)]
pub struct BigCryptor64
{
    block: LongUnion,
    smallcryptor: Vec<Box<dyn SmallCryptor<u64, 8>>>,
}

impl BigCryptor64
{
    pub(super) const BLOCK_SIZE: usize = 8;
    const SUCCESS: u64 = !0;
    const FAILURE: u64 = 0;

    // pub fn new() -> Self
    /// Constructs a new object BigCryptor64.
    /// 
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method does not set any small cryptor (component) by default.
    /// - You have to set as many small cryptors (components) as you want.
    /// - The small cryptors (components) should have the block size 64-bit.
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
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.new)           
    #[inline]
    pub fn new() -> Self
    {
        Self { block: LongUnion::zero(), smallcryptor: Vec::new() }
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.new_with_small_cryptor_array)
    pub fn new_with_small_cryptor_array<const N: usize>(smallcryptor: [Box<dyn SmallCryptor<u64, 8>>; N]) -> Self
    {
        let mut bigcryptor = Self::new();
        bigcryptor.push_small_cryptor_array(smallcryptor);
        bigcryptor
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
    /// let mut _tdes = BigCryptor64::new_with_small_cryptor_vec(cryptors);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.new_with_small_cryptor_vec)
    pub fn new_with_small_cryptor_vec(smallcryptor: Vec<Box<dyn SmallCryptor<u64, 8>>>) -> Self
    {
        let mut bigcryptor = Self::new();
        bigcryptor.push_small_cryptor_vec(smallcryptor);
        bigcryptor
    }

    // pub fn push_small_cryptor<S: SmallCryptor<u64, 8> + 'static>(&mut self, smallcryptor: S)
    /// Adds a small cryptor (component) to `Self`'s own small cryptor
    /// container.
    /// 
    /// # Arguments
    /// `smallcryptor` is a small cryptors (components).
    /// 
    /// # Features
    /// This method automatically wrap the small cryptor by Box.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::{ BigCryptor64, DES };
    /// 
    /// let mut tdes = BigCryptor64::new();
    /// let des1 = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let des2 = DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64);
    /// let des3 = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// tdes.push_small_cryptor(des1);
    /// tdes.push_small_cryptor(des2);
    /// tdes.push_small_cryptor(des3);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.push_small_cryptor)
    pub fn push_small_cryptor<S: SmallCryptor<u64, 8> + 'static>(&mut self, smallcryptor: S)
    {
        self.smallcryptor.push(Box::<S>::new(smallcryptor));
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
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.push_small_cryptor_array)
    pub fn push_small_cryptor_array<const N: usize>(&mut self, smallcryptors: [Box<dyn SmallCryptor<u64, 8>>; N])
    {
        for val in smallcryptors
            { self.smallcryptor.push(val); }
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
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.push_small_cryptor_vec)
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
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.encrypt_u64)
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
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.decrypt_u64)
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
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.encrypt_array_u64)
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
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.decrypt_array_u64)
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
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.is_successful)
    #[inline] pub fn is_successful(&self) -> bool
    {
        self.block.get() == Self::SUCCESS
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
    /// # For more examples,
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.is_failed)
    #[inline] pub fn is_failed(&self) -> bool
    {
        self.block.get() == Self::FAILURE
    }

    // pub(super) fn set_successful(&mut self)
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
    pub(super) fn set_successful(&mut self)
    {
        self.block.set(Self::SUCCESS);
    }

    // pub(super) fn set_failed(&mut self)
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
    pub(super) fn set_failed(&mut self)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.new)           
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.new_with_key)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.new_with_key_u64)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.set_key)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.set_key)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.encrypt_u64)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.decrypt_u64)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.encrypt_array_u64)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.decrypt_array_u64)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.is_successful)
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
    /// click [here](./documentation/big_cryptor64_basic/struct.BigCryptor64.html#method.is_failed)
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