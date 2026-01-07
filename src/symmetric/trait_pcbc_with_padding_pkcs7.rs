// Copyright 2025, 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


// #![allow(missing_docs)]
// #![allow(unused_must_use)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![warn(rustdoc::missing_doc_code_examples)]


use std::vec::Vec;
use crate::number::TraitsBigUInt;
use crate::symmetric::pre_decrypt_into_vec;



/// PCBC (Propagation Cipher-Block Chaining) is one of the operation modes
///  forencryption/decryption. And PKCS #7 is the one of the padding ways.
/// 
/// # Caution
/// For Rijndael or AES, if NB > 64, you are not supposed to use the padding
/// defined in PKCS #7 because its behavior is not defined.
#[allow(non_camel_case_types)]
pub trait PCBC_PKCS7<T> : Sized
{
    // fn encrypt(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;
    /// Encrypts the data with the padding defined according to PKCS #7
    /// in PCBC (Propagation Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `message` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the plaintext `message` in bytes.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `size_of::<T>()`,
    ///   and cannot be other than a multiple of `size_of::<T>()`.
    /// - If this method failed in encryption,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as `encrypt_*_into_*()`.
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null string.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`length_in_bytes` + `1`).next_multiple_of(`size_of::<T>()`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.encrypt)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.encrypt)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// taes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.encrypt)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.encrypt)
    fn encrypt(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;

    // fn encrypt_into_vec<U>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data with the padding defined according to PKCS #7
    /// in PCBC (Propagation Cipher-Block Chaining) mode, and stores the encrypted data
    /// in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `message` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the plaintext `message` in bytes.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `size_of::<T>()`,
    ///   and cannot be other than a multiple of `size_of::<T>()`.
    /// - If this method failed in encryption,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_*_into_vec().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is a null string.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the `Vec<U>` object which is referred to as `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.encrypt_into_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.encrypt_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.encrypt_into_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
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
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.encrypt_into_vec)
    fn encrypt_into_vec<U>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    where U: TraitsBigUInt<U>;

    // fn encrypt_into_array<U, const N: usize>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    /// Encrypts the data with the padding defined according to PKCS #7
    /// in PCBC (Propagation Cipher-Block Chaining) mode, and stores the encrypted data
    /// in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `message` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the plaintext `message` in bytes.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `size_of::<T>()`,
    ///   and cannot be other than a multiple of `size_of::<T>()`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_*_into_array().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null data.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the array `[U; N]` object `cipher`.
    /// - If `U::size_in_bytes()` * `N` is less than `length_in_bytes`'s next
    ///   multiple of `size_of::<T>()`, this method does not perform
    ///   encryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `N` is equal to `length_in_bytes`'s next
    ///   multiple of `size_of::<T>()`, this method performs encryption,
    ///   fills the array `cipher` with the encrypted data, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - If `U::size_in_bytes()` * `N` is greater than `length_in_bytes`'s next
    ///   multiple of `size_of::<T>()`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`length_in_bytes` + `1`).next_multiple_of(`size_of::<T>()`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.encrypt_into_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.encrypt_into_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// taes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.encrypt_into_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.encrypt_into_array)
    fn encrypt_into_array<U, const N: usize>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: TraitsBigUInt<U>;

    // fn encrypt_str(&mut self, iv: T, message: &str, cipher: *mut u8) -> u64
    /// Encrypts the data in a `str` object with the padding defined
    /// according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `message` is an immutable reference to `str` object which is `&str`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `size_of::<T>()`,
    ///   and cannot be other than a multiple of `size_of::<T>()`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_str_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`message.len()` + `1`).next_multiple_of(`size_of::<T>()`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.encrypt_str)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.encrypt_str)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// taes.encrypt_str(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.encrypt_str)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_str(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.encrypt_str)
    #[inline]
    fn encrypt_str(&mut self, iv: T, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_str_into_vec<U>(&mut self, iv: T, message: &str, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data in `str` with the padding defined according to
    /// PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode, and stores the
    /// encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `message` is an immutable reference to `str` object which is `&str`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `size_of::<T>()`,
    ///   and cannot be other than a multiple of `size_of::<T>()`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the `Vec<U>` object which is referred to as `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.encrypt_str_into_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.encrypt_str_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.encrypt_str_into_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.encrypt_str_into_vec)
    #[inline]
    fn encrypt_str_into_vec<U>(&mut self, iv: T, message: &str, cipher: &mut Vec<U>) -> u64
    where U: TraitsBigUInt<U>
    {
        self.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_str_into_array<U, const N: usize>(&mut self, iv: T, message: &str, cipher: &mut [U; N]) -> u64
    /// Encrypts the data in a `str` object with the padding defined
    /// according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode,
    /// and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `message` is an immutable reference to `str` object which is `&str`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `size_of::<T>()`,
    ///   and cannot be other than a multiple of `size_of::<T>()`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the array `[U; N]` object `cipher`.
    /// - If `U::size_in_bytes()` * `N` is less than `message.len()`'s next
    ///   multiple of `size_of::<T>()`, this method does not perform
    ///   encryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `N` is equal to `message.len()`'s next
    ///   multiple of `size_of::<T>()`, this method performs encryption,
    ///   fills the array `cipher` with the encrypted data, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - If `U::size_in_bytes()` * `N` is greater than `message.len()`'s next
    ///   multiple of `size_of::<T>()`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`message.len()` + `1`).next_multiple_of(`size_of::<T>()`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.encrypt_str_into_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.encrypt_str_into_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// let mut cipher = [0_u8; 64];
    /// taes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.encrypt_str_into_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.encrypt_str_into_array)
    #[inline]
    fn encrypt_str_into_array<U, const N: usize>(&mut self, iv: T, message: &str, cipher: &mut [U; N]) -> u64
    where U: TraitsBigUInt<U>
    {
        self.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string(&mut self, iv: T, message: &String, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `String` object with the padding
    /// according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `message` is an immutable reference to `String` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `size_of::<T>()`,
    ///   and cannot be other than a multiple of `size_of::<T>()`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_string_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `message` is a `String` object that has a null string "", only
    ///   padding bytes will be encrypted, and stored in the memory area that
    ///   starts from `cipher`.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`message.len()` + `1`).next_multiple_of(`size_of::<T>()`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_string(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.encrypt_string)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =\t{}", iv);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 C5 9E EF 01 A9 4A FD 5B ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.encrypt_string)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// let mut cipher = [0_u8; 64];
    /// taes.encrypt_string(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.encrypt_string)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_string(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.encrypt_string)
    #[inline]
    fn encrypt_string(&mut self, iv: T, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string_into_vec<U>(&mut self, iv: T, message: &String, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data stored in a `String` object with the padding
    /// according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode,
    /// and stores the encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `message` is an immutable reference to `String` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `size_of::<T>()`,
    ///   and cannot be other than a multiple of `size_of::<T>()`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is a `String` object that has a null string "", only
    ///   padding bytes will be encrypted, and stored in the `Vec<U>` object
    ///   which is referred to as `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.encrypt_string_into_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_string_into_vec(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.encrypt_string_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_string_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.encrypt_string_into_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_string_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.encrypt_string_into_vec)
    #[inline]
    fn encrypt_string_into_vec<U>(&mut self, iv: T, message: &String, cipher: &mut Vec<U>) -> u64
    where U: TraitsBigUInt<U>
    {
        self.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string_into_array<U, const N: usize>(&mut self, iv: T, message: &String, cipher: &mut [U; N]) -> u64
    /// Encrypts the data stored in a `String` object with the padding
    /// according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode,
    /// and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `message` is an immutable reference to `String` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `size_of::<T>()`,
    ///   and cannot be other than a multiple of `size_of::<T>()`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is a `String` object that has a null string "", only
    ///   padding bytes will be encrypted, and stored in the array `[U; N]`
    ///   object `cipher`.
    /// - If `size_of::<U>()` * `N` is less than `message.len()`'s next
    ///   multiple of `size_of::<T>()`, this method does not perform
    ///   encryption but returns `zero`.
    /// - If `size_of::<U>()` * `N` is equal to `message.len()`'s next
    ///   multiple of `size_of::<T>()`, this method performs encryption,
    ///   fills the array `cipher` with the encrypted data, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - If `size_of::<U>()` * `N` is greater than `message.len()`'s next
    ///   multiple of `size_of::<T>()`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`message.len()` + `1`).next_multiple_of(`size_of::<T>()`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.encrypt_string_into_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_string_into_array(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.encrypt_string_into_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// let mut cipher = [0_u8; 64];
    /// taes.encrypt_string_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.encrypt_string_into_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_string_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.encrypt_string_into_array)
    #[inline]
    fn encrypt_string_into_array<U, const N: usize>(&mut self, iv: T, message: &String, cipher: &mut [U; N]) -> u64
    where U: TraitsBigUInt<U>
    {
        self.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_vec<U>(&mut self, iv: T, message: &Vec<U>, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `message` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `size_of::<T>()`,
    ///   and cannot be other than a multiple of `size_of::<T>()`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`size_of::<U>()` * `message.len()` + `1`).next_multiple_of(`size_of::<T>()`)
    ///   at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - If `message` is an empty `Vec<U>` object `Vec::<U>::new()`, only
    ///   padding bytes will be encrypted, and stored in the memory area that
    ///   starts from `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.encrypt_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.encrypt_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// taes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.encrypt_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.encrypt_vec)
    #[inline]
    fn encrypt_vec<U>(&mut self, iv: T, message: &Vec<U>, cipher: *mut u8) -> u64
    where U: TraitsBigUInt<U>
    {
        self.encrypt(iv, message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_vec_into_vec<U, V>(&mut self, iv: T, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode, and
    /// stores the encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `message` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `size_of::<T>()`,
    ///   and cannot be other than a multiple of `size_of::<T>()`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is an empty `Vec<U>` object `Vec::<U>::new()`, only
    ///   padding bytes will be encrypted, and stored in the `Vec<U>` object
    ///   which is referred to as `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_vec_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.encrypt_vec_into_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =\t{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_vec_into_vec(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 C5 9E EF 01 A9 4A FD 5B ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.encrypt_vec_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_vec_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.encrypt_vec_into_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_vec_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.encrypt_vec_into_vec)
    #[inline]
    fn encrypt_vec_into_vec<U, V>(&mut self, iv: T, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        self.encrypt_into_vec(iv, message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: T, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode, and
    /// stores the encrypted data in array `[V; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `message` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `size_of::<T>()`,
    ///   and cannot be other than a multiple of `size_of::<T>()`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is an empty `Vec<U>` object `Vec::<U>::new()`, only
    ///   padding bytes will be encrypted, and stored in the array `[U; N]`
    ///   object `cipher`.
    /// - If `size_of::<V>()` * `N` is less than 
    ///   `size_of::<U>() * message.len()`'s next multiple of
    ///   `size_of::<T>()`, this method does not perform
    ///   encryption but returns `zero`.
    /// - If `size_of::<V>()` * `N` is equal to
    ///   `size_of::<U>() * message.len()`'s next multiple of
    ///   `size_of::<T>()`, this method performs encryption,
    ///   fills the array `cipher` with the encrypted data, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - If `size_of::<V>()` * `N` is greater than
    ///   `size_of::<U>() * message.len()`'s next multiple of `size_of::<T>()`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and then fills the rest of the elements of the array
    ///   `cipher` with zeros, and returns the size of the ciphertext including
    ///   padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`size_of::<U>()` * `message.len()` + `1`).next_multiple_of(`size_of::<T>()`)
    ///   at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_vec_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.encrypt_vec_into_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.encrypt_vec_into_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// taes.encrypt_vec_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.encrypt_vec_into_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_vec_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.encrypt_vec_into_array)
    #[inline]
    fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: T, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        self.encrypt_into_array(iv, message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array<U, const N: usize>(&mut self, iv: T, message: &[U; N], cipher: *mut u8) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `message` is an immutable reference to an array `[U; N]` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `size_of::<T>()`,
    ///   and cannot be other than a multiple of `size_of::<T>()`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`size_of::<U>()` * `N` + `1`).next_multiple_of(`size_of::<T>()`)
    ///   at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - If `message` is an empty array `[U; 0]` object, only padding bytes
    ///   will be encrypted, and stored in the memory area that starts from `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.encrypt_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.encrypt_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let mut cipher = [0_u8; 64];
    /// taes.encrypt_array(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.encrypt_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_array(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.encrypt_array)
    #[inline]
    fn encrypt_array<U, const N: usize>(&mut self, iv: T, message: &[U; N], cipher: *mut u8) -> u64
    where U: TraitsBigUInt<U>
    {
        self.encrypt(iv, message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: T, message: &[U; N], cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode, and stores the
    /// encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `message` is an immutable reference to an array `[U; N]` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `size_of::<T>()`,
    ///   and cannot be other than a multiple of `size_of::<T>()`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is an empty array `[U; 0]` object, only padding bytes
    ///   will be encrypted, and stored in the `Vec<U>` object `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_array_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.encrypt_array_into_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_array_into_vec(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.encrypt_array_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_array_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.encrypt_array_into_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_array_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.encrypt_array_into_vec)
    #[inline]
    fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: T, message: &[U; N], cipher: &mut Vec<V>) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        self.encrypt_into_vec(iv, message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: T, message: &[U; N], cipher: &mut [V; M]) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode, and stores the
    /// encrypted data in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `message` is an immutable reference to an array `[U; N]` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `size_of::<T>()`,
    ///   and cannot be other than a multiple of `size_of::<T>()`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is an empty array `[U; 0]` object, only padding bytes
    ///   will be encrypted, and stored in the array `[V; M]` object `cipher`.
    /// - If `V::size_in_bytes()` * `M` is less than 
    ///   `U::size_in_bytes()` * `N`'s next multiple of `size_of::<T>()`,
    ///   this method does not perform encryption and returns `zero`.
    /// - If `V::size_in_bytes()` * `M` is equal to
    ///   `U::size_in_bytes()` * `N`'s next multiple of `size_of::<T>()`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns the size of the ciphertext including
    ///   padding bits in bytes.
    /// - If `V::size_in_bytes()` * `M` is greater than
    ///   `U::size_in_bytes()` * `N`'s next multiple of `size_of::<T>()`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and then fills the rest of the elements of the
    ///   array `cipher` with zeros, and returns the size of the ciphertext
    ///   including padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`size_of::<U>()` * `message.len()` + `1`).next_multiple_of(`size_of::<T>()`)
    ///   at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_array_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.encrypt_array_into_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_array_into_array(iv, &message, &mut cipher);
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.encrypt_array_into_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let mut cipher = [0_u8; 64];
    /// taes.encrypt_array_into_array(iv, &message, &mut cipher);
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.encrypt_array_into_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_array_into_array(iv, &message, &mut cipher);
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.encrypt_array_into_array)
    #[inline]
    fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: T, message: &[U; N], cipher: &mut [V; M]) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        self.encrypt_into_array(iv, message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn decrypt(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64;
    /// Decrypts the data with the padding defined according to PKCS #7
    /// in PCBC (Propagation Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `cipher` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the ciphertext to be decrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the ciphertext `cipher` in bytes.
    /// - `message` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// - If `length_in_bytes` is greater than `size_of::<T>()` (which means
    ///   that the original plaintext is surely not empty data) and it returns
    ///   `zero`, you can interpret it that this method surely failed in
    ///   decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as `decrypt_*_into_*()`.
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be other than any multiple of `size_of::<T>()`.
    /// - The size of the memory area which starts at `message` is assumed to
    ///   be enough to store the plaintext. So, it is responsible for you to
    ///   prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `length_in_bytes` - `1`.
    /// - If the size of the area for plaintext is prepared more than
    ///   `length_in_bytes` - `1`, the rest of the area will be filled with
    ///   `0`s.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.decrypt)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    ///
    /// let mut recovered = vec![0; 55];
    /// a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.decrypt)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// taes.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.decrypt)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// tdes.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.decrypt)
    fn decrypt(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64;

    // fn decrypt_into_vec<U>(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    /// Decrypts the data with the padding defined according to PKCS #7
    /// in PCBC (Propagation Cipher-Block Chaining) mode, and stores the decrypted data
    /// in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `cipher` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the ciphertext to be decrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the ciphertext `cipher` in bytes.
    /// - `message` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// - If `length_in_bytes` is greater than `size_of::<T>()` (which means
    ///   that the original plaintext is surely not empty data) and it returns
    ///   `zero`, you can interpret it that this method surely failed in
    ///   decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_vec().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be other than any multiple of `size_of::<T>()`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_aes.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.decrypt_into_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.decrypt_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// taes.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.decrypt_into_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// tdes.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.decrypt_into_vec)
    fn decrypt_into_vec<U>(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    where U: TraitsBigUInt<U>
    {
        pre_decrypt_into_vec!(message, length_in_bytes, U);
        let len = self.decrypt(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(len as usize);
        len
    }

    // fn decrypt_into_array<U, const N: usize>(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    /// Decrypts the data with the padding defined according to PKCS #7
    /// in PCBC (Propagation Cipher-Block Chaining) mode, and stores the decrypted data
    /// in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `cipher` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the ciphertext to be decrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the ciphertext `cipher` in bytes.
    /// - `message` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// - If `length_in_bytes` is greater than `size_of::<T>()` (which means
    ///   that the original plaintext is surely not empty data) and it returns
    ///   `zero`, you can interpret it that this method surely failed in
    ///   decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_array().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be other than any multiple of `size_of::<T>()`.
    /// - If `U::size_in_bytes()` * `N` is less than `length_in_bytes` - `1`,
    ///   this method does not perform decryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `N` is equal to or greater than
    ///   `length_in_bytes` - `1`, this method performs decryption, fills the
    ///   array `message` with the decrypted data, and then fills the rest of
    ///   the elements of the array `message` with zeros, and returns the size
    ///   of the plaintext.
    /// - It is responsible for you to prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `length_in_bytes` - `1`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// 
    /// let mut recovered = [0; 64];
    /// let len = a_aes.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 00 00 00 00 00 00 00 00 ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// 
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.decrypt_into_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    ///
    /// let mut recovered = [0u8; 56];
    /// let len = a_des.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba (16 rounds) =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb (16 rounds) =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.decrypt_into_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// 
    /// let mut recovered = [0u8; 64];
    /// let len = taes.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 00 00 00 00 00 00 00 00 ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.decrypt_into_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// 
    /// let mut recovered = [0u8; 56];
    /// let len = tdes.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.decrypt_into_array)
    fn decrypt_into_array<U, const N: usize>(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    where U: TraitsBigUInt<U>;

    // fn decrypt_into_string(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    /// Decrypts the data with the padding defined according to PKCS #7
    /// in PCBC (Propagation Cipher-Block Chaining) mode, and stores the decrypted data
    /// in a `String`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `cipher` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the ciphertext to be decrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the ciphertext `cipher` in bytes.
    /// - `message` is a mutable reference to a `String` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// - If `length_in_bytes` is greater than `size_of::<T>()` (which means
    ///   that the original plaintext is surely not empty data) and it returns
    ///   `zero`, you can interpret it that this method surely failed in
    ///   decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_string().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be other than any multiple of `size_of::<T>()`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// 
    /// let mut converted = String::new();
    /// a_aes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.decrypt_into_string)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B (16 rounds) =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.decrypt_into_string)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// 
    /// let mut recovered = String::new();
    /// taes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.decrypt_into_string)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// 
    /// let mut recovered = String::new();
    /// tdes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.decrypt_into_string)
    #[inline]
    fn decrypt_into_string(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_into_vec(iv, cipher, length_in_bytes, unsafe { message.as_mut_vec() })
    }

    // fn decrypt_vec<U>(&mut self, iv: T, cipher: &Vec<U>, message: *mut u8) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `cipher` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the plaintext to be decrypted is stored.
    /// - `message` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `size_of::<T>()`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `size_of::<U>()` * `cipher.len()` cannot be other than any multiple
    ///   of `size_of::<T>()`.
    /// - The size of the memory area which starts at `message` is assumed to
    ///   be enough to store the plaintext. So, it is responsible for you to
    ///   prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `size_of::<U>()` * `cipher.len()` - `1`.
    /// - If the size of the area for plaintext is prepared more than
    ///   `size_of::<U>()` * `cipher.len()` - `1`, the rest of the area will be
    ///   filled with `0`s.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.decrypt_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    ///
    /// let mut recovered = vec![0; 55];
    /// a_des.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.decrypt_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// taes.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.decrypt_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// tdes.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.decrypt_vec)
    #[inline]
    fn decrypt_vec<U>(&mut self, iv: T, cipher: &Vec<U>, message: *mut u8) -> u64
    where U: TraitsBigUInt<U>
    {
        self.decrypt(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_vec_into_vec<U, V>(&mut self, iv: T, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode, and
    /// stores the decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `cipher` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the ciphertext to be decrypted is stored.
    /// - `message` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `size_of::<T>()`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_aes.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.decrypt_vec_into_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.decrypt_vec_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// taes.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.decrypt_vec_into_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// tdes.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.decrypt_vec_into_vec)
    #[inline]
    fn decrypt_vec_into_vec<U, V>(&mut self, iv: T, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        self.decrypt_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: T, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode, and
    /// stores the decrypted data in array `[V; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `cipher` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the ciphertext to be decrypted is stored.
    /// - `message` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `size_of::<T>()`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - If `size_of::<V>()` * `N` is less than 
    ///   `size_of::<U>()` * `cipher.len()` - `1`, this method does not perform
    ///   decryption but returns `zero`.
    /// - If `size_of::<V>()` * `N` is equal to or greater than
    ///   `size_of::<U>()` * `cipher.len()` - `1`, this method performs
    ///   decryption, fills the array `message` with the decrypted data, and then
    ///   fills the rest of the elements of the array `message` with zeros, and
    ///   returns the size of the plaintext.
    /// - It is responsible for you to prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `size_of::<U>()` * `cipher.len()` - `1`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// 
    /// let mut recovered = [0; 64];
    /// let len = a_aes.decrypt_vec_into_array(iv, &cipher, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 00 00 00 00 00 00 00 00 ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.decrypt_vec_into_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    ///
    /// let mut recovered = [0u8; 56];
    /// let len = a_des.decrypt_vec_into_array(iv, &cipher, &mut recovered);
    /// print!("Ba (16 rounds) =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb (16 rounds) =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.decrypt_vec_into_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// 
    /// let mut recovered = [0u8; 64];
    /// let len = taes.decrypt_vec_into_array(iv, &cipher, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 00 00 00 00 00 00 00 00 ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.decrypt_vec_into_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// 
    /// let mut recovered = [0u8; 56];
    /// let len = tdes.decrypt_vec_into_array(iv, &cipher, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.decrypt_vec_into_array)
    #[inline]
    fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: T, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        self.decrypt_into_array(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_vec_into_string(&mut self, iv: T, cipher: &str, message: &mut String) -> u64
    /// Decrypts the data in `str` with the padding defined according to
    /// PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode, and stores the
    /// decrypted data in `String`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `cipher` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the ciphertext to be decrypted is stored.
    /// - `message` is a mutable reference to a `String` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `size_of::<T>()`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// 
    /// let mut converted= String::new();
    /// a_aes.decrypt_vec_into_string(iv, &cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.decrypt_vec_into_string)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    /// println!("B (16 rounds) =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.decrypt_vec_into_string)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// 
    /// let mut recovered = String::new();
    /// taes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.decrypt_vec_into_string)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// 
    /// let mut recovered = String::new();
    /// tdes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.decrypt_vec_into_string)
    #[inline]
    fn decrypt_vec_into_string<U>(&mut self, iv: T, cipher: &Vec<U>, message: &mut String) -> u64
    where U: TraitsBigUInt<U>
    {
        self.decrypt_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_array<U, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: *mut u8) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `cipher` is an immutable reference to an array `[U; N]` object, and
    ///   is the place where the plaintext to be decrypted is stored.
    /// - `message` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// - If `size_of::<U>()` * `N` is greater than `size_of::<T>()`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_array_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `size_of::<U>()` * `N` cannot be other than any multiple
    ///   of `size_of::<T>()`.
    /// - The size of the memory area which starts at `message` is assumed to
    ///   be enough to store the plaintext. So, it is responsible for you to
    ///   prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `size_of::<U>()` * `N` - `1`.
    /// - If the size of the area for plaintext is prepared more than
    ///   `size_of::<U>()` * `N` - `1`, the rest of the area will be
    ///   filled with `0`s.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.decrypt_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    ///
    /// let mut recovered = vec![0; 55];
    /// let len = a_des.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
    /// recovered.truncate(len as usize);
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
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.decrypt_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 64];
    /// taes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// let len = taes.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
    /// recovered.truncate(len as usize);
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.decrypt_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 56];
    /// tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// let len = tdes.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
    /// recovered.truncate(len as usize);
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.decrypt_array)
    #[inline]
    fn decrypt_array<U, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: *mut u8) -> u64
    where U: TraitsBigUInt<U>
    {
        self.decrypt(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode,
    /// and stores the decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `cipher` is an immutable reference to an array `[U; N]` object, and
    ///   is the place where the plaintext to be decrypted is stored.
    /// - `message` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// - If `size_of::<U>()` * `N` is greater than `size_of::<T>()`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt_array_into_vec(iv, &cipher, &mut recovered);
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.decrypt_array_into_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.decrypt_array_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 64];
    /// taes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// taes.decrypt_array_into_vec(iv, &cipher, &mut recovered);
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.decrypt_array_into_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 56];
    /// tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// tdes.decrypt_array_into_vec(iv, &cipher, &mut recovered);
    /// print!("Ba =\t");
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
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.decrypt_array_into_vec)
    #[inline]
    fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut Vec<V>) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        self.decrypt_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut [V; M]) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode,
    /// and stores the decrypted data in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `cipher` is an immutable reference to an array `[U; N]` object, and
    ///   is the place where the plaintext to be decrypted is stored.
    /// - `message` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// - If `size_of::<U>()` * `N` is greater than `size_of::<T>()`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - If `size_of::<V>()` * `M` is less than 
    ///   `size_of::<U>()` * `N` - `1`, this method does not perform
    ///   decryption but returns `zero`.
    /// - If `size_of::<V>()` * `M` is equal to or greater than
    ///   `size_of::<U>()` * `N` - `1`, this method performs decryption,
    ///   fills the array `message` with the decrypted data, and then
    ///   fills the rest of the elements of the array `message` with zeros, and
    ///   returns the size of the plaintext.
    /// - It is responsible for you to prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `size_of::<U>()` * `N` - `1`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// 
    /// let mut recovered = [0; 64];
    /// let len = a_aes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 00 00 00 00 00 00 00 00 ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.decrypt_array_into_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    ///
    /// let mut recovered = [0u8; 56];
    /// let len = a_des.decrypt_array_into_array(iv, &cipher, &mut recovered);
    /// print!("Ba (16 rounds) =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb (16 rounds) =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.decrypt_array_into_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 64];
    /// taes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// 
    /// let mut recovered = [0u8; 64];
    /// let len = taes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 00 00 00 00 00 00 00 00 ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.decrypt_array_into_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 56];
    /// tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// 
    /// let mut recovered = [0u8; 56];
    /// let len = tdes.decrypt_array_into_array(iv, &cipher, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 ");
    /// 
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.decrypt_array_into_array)
    #[inline]
    fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut [V; M]) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        self.decrypt_into_array(iv, cipher.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_array_into_string<U, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut String) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #7 in PCBC (Propagation Cipher-Block Chaining) mode,
    /// and stores the decrypted data in `String`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for PCBC mode.
    /// - `cipher` is an immutable reference to an array `[U; N]` object, and
    ///   is the place where the plaintext to be decrypted is stored.
    /// - `message` is a mutable reference to a `String` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// - If `size_of::<U>()` * `N` is greater than `size_of::<T>()`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// 
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 EC 7B 5E 0B F6 7C AB 84 BB DD 0F 27 F4 63 B5 E3 C2 1D 11 06 C2 BF B0 32 24 81 DB FD A4 CE 56 F6 0B 49 DD 7A 22 C9 C8 91 34 F6 0A 3A E7 C1 59 7A ");
    /// 
    /// let mut converted = String::new();
    /// a_aes.decrypt_array_into_string(iv, &cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_pcbc_pkcs7/struct.Rijndal_Generic.html#method.decrypt_array_into_string)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, PCBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D DB B4 41 7D 93 0B BD CD 0E B8 80 D4 EC 13 FC 57 D6 7E FF 69 1C 76 8A CD A1 A6 77 7C 6E 86 28 21 DD DB 59 0C 72 39 9B 95 01 BB EE 98 FC B6 40 01 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_array_into_string(iv, &cipher, &mut recovered);
    /// println!("B (16 rounds) =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_pcbc_pkcs7/struct.DES_Generic.html#method.decrypt_array_into_string)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, PCBC_PKCS7 };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let iv = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("IV =	{:#034X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 64];
    /// taes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "87 B7 B0 67 C9 41 CF C6 CF B8 69 0D 39 D3 9B EC E2 3A 60 04 1D 95 41 45 BA B6 1F 41 67 F0 72 03 83 D8 7B 9D 64 CC BD A6 BF C2 DB AA E9 38 97 B7 A9 7B 95 FD 5D 9F 65 ED 00 8F 80 79 C7 C0 41 5C ");
    /// 
    /// let mut recovered = String::new();
    /// taes.decrypt_array_into_string(iv, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_pcbc_pkcs7/struct.BigCryptor128.html#method.decrypt_array_into_string)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, PCBC_PKCS7 };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{:#018X}", iv);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 56];
    /// tdes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "01 A5 7E BC ED 83 28 FB DD AC 7F 93 37 1B A6 28 89 A6 19 D3 9E E6 A0 29 B6 EA 99 D9 D1 F1 F0 E4 9D 54 E9 55 2B FD 40 DA B7 B3 3D 5F 86 2D 30 BF B2 28 DD A3 42 A4 70 06 ");
    /// 
    /// let mut recovered = String::new();
    /// tdes.decrypt_array_into_string(iv, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_pcbc_pkcs7/struct.BigCryptor64.html#method.decrypt_array_into_string)
    #[inline]
    fn decrypt_array_into_string<U, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut String) -> u64
    where U: TraitsBigUInt<U>
    {
        self.decrypt_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }
}
