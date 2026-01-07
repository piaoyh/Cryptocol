// Copyright 2025, 2026 PARK Youngho.
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
use crate::number::TraitsBigUInt;

/// trait_ctr.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to des_ctr.rs. And, most of generic parameters
/// are omitted. It is not actual code but dummy code for compilation!!!
#[allow(non_camel_case_types)]
pub struct DES_Generic
{
    // Dummy struct for documentation
}

/// trait_ctr.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to des_ctr.rs. And, most of generic parameters
/// are omitted. It is not actual code but dummy code for compilation!!!
impl DES_Generic
{
    // fn encrypt(&mut self, nonce: T, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;
    /// Encrypts the data without any padding in CTR (CounTeR) mode.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
    /// - `message` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the plaintext `message` in bytes.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext.
    /// - If this method failed in encryption or `length_in_bytes` is `0`,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as `encrypt_*_into_*()`.
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null string.
    ///   So, nothing will be encrypted, and stored in the memory area
    ///   that starts from `cipher`.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be the same
    ///   as `length_in_bytes` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    ///
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = [0_u8; 55];
    /// let mut cipher2 = [0_u8; 55];
    /// c_des.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher1.as_mut_ptr());
    /// d_des.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher2.as_mut_ptr());
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 0];
    /// a_des.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 7];
    /// a_des.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 12];
    /// a_des.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    /// ```
    pub fn encrypt(&mut self, nonce: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_into_vec<U>(&mut self, nonce: T, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data without any padding in CTR (CounTeR) mode,
    /// and stores the encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
    /// - `message` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the plaintext `message` in bytes.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext.
    /// - If this method failed in encryption or `length_in_bytes` is `0`,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_*_into_vec().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is a null string.
    ///   So, nothing will be encrypted, and stored in the `Vec<U>` object which
    ///   is referred to as `cipher`. 
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    /// ```
    pub fn encrypt_into_vec<U>(&mut self, nonce: u64, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_into_array<U, const N: usize>(&mut self, nonce: T, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    /// Encrypts the data without any padding in CTR (CounTeR) mode,
    /// and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
    /// - `message` is an immutable pointer to `u8` which is `*const u8`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the plaintext `message` in bytes.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext.
    /// - If this method failed in encryption or `length_in_bytes` is `0`,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_*_into_array().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null data.
    ///   So, nothing will be encrypted,
    ///   and stored in the array `[U; N]` object `cipher`.
    /// - If `U::size_in_bytes()` * `N` is less than `length_in_bytes`,
    ///   this method does not perform encryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `N` is equal to `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and returns the size of the ciphertext in bytes.
    /// - If `U::size_in_bytes()` * `N` is greater than `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and then fills the rest of the elements of the array
    ///   `cipher` with zeros, and returns the size of the ciphertext in bytes.
    /// - The size of the area for ciphertext should be prepared to be the same
    ///   as `length_in_bytes` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = [0_u8; 55];
    /// let mut cipher2 = [0_u8; 55];
    /// c_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 0];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 7];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 12];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    /// ```
    pub fn encrypt_into_array<U, const N: usize>(&mut self, nonce: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_str(&mut self, nonce: T, message: &str, cipher: *mut u8) -> u64
    /// Encrypts the data in a `str` object without any padding in CFB (Cipher
    /// FeedBack) mode.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
    /// - `message` is an immutable reference to `str` object which is `&str`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext.
    /// - If this method failed in encryption or `message.len()` is `0`,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_str_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `message` is a null string "", nothing will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `message.len()` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = [0_u8; 55];
    /// let mut cipher2 = [0_u8; 55];
    /// c_des.encrypt_str(nonce, &message, cipher1.as_mut_ptr());
    /// d_des.encrypt_str(nonce, &message, cipher2.as_mut_ptr());
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 0];
    /// a_des.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 7];
    /// a_des.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 12];
    /// a_des.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    /// ```
    pub fn encrypt_str(&mut self, nonce: u64, message: &str, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_str_into_vec<U>(&mut self, nonce: T, message: &str, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data in `str` without any padding in CTR (CounTeR)
    /// mode, and stores the encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
    /// - `message` is an immutable reference to `str` object which is `&str`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext.
    /// - If this method failed in encryption or `message.len()` is `0`,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is a null string "", nothing will be encrypted,
    ///   and stored in the `Vec<U>` object which is referred to as `cipher`.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_str_into_vec(nonce, &message, &mut cipher1);
    /// d_des.encrypt_str_into_vec(nonce, &message, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    /// ```
    pub fn encrypt_str_into_vec<U>(&mut self, nonce: u64, message: &str, cipher: &mut Vec<U>) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_str_into_array<U, const N: usize>(&mut self, nonce: T, message: &str, cipher: &mut [U; N]) -> u64
    /// Encrypts the data in a `str` object without any padding in CFB (Cipher
    /// FeedBack) mode, and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
    /// - `message` is an immutable reference to `str` object which is `&str`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext.
    /// - If this method failed in encryption or `message.len()` is `0`,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is a null string "", nothing will be encrypted,
    ///   and stored in the array `[U; N]` object `cipher`.
    /// - If `U::size_in_bytes()` * `N` is less than `message.len()`,
    ///   this method does not perform encryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `N` is equal to `message.len()`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and returns the size of the ciphertext in bytes.
    /// - If `U::size_in_bytes()` * `N` is greater than `message.len()`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and then fills the rest of the elements of the array
    ///   `cipher` with zeros, and returns the size of the ciphertext in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `message.len()` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_str_into_array(nonce, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_str_into_array(nonce, &message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = [0_u8; 55];
    /// let mut cipher2 = [0_u8; 55];
    /// c_des.encrypt_str_into_array(nonce, &message, &mut cipher1);
    /// d_des.encrypt_str_into_array(nonce, &message, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 0];
    /// a_des.encrypt_str_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 7];
    /// a_des.encrypt_str_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_str_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 12];
    /// a_des.encrypt_str_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_str_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    /// ```
    pub fn encrypt_str_into_array<U, const N: usize>(&mut self, nonce: u64, message: &str, cipher: &mut [U; N]) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_string(&mut self, nonce: T, message: &String, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `String` object without any padding
    /// in CTR (CounTeR) mode.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
    /// - `message` is an immutable reference to `String` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext.
    /// - If this method failed in encryption or `message.len()` is `0`,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_string_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `message` is a `String` object that has a null string "", nothing
    ///   will be encrypted, and stored in the memory area that
    ///   starts from `cipher`.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `message.len()` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = [0_u8; 55];
    /// let mut cipher2 = [0_u8; 55];
    /// c_des.encrypt_string(nonce, &message, cipher1.as_mut_ptr());
    /// d_des.encrypt_string(nonce, &message, cipher2.as_mut_ptr());
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 0];
    /// a_des.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 7];
    /// a_des.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 12];
    /// a_des.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    /// ```
    pub fn encrypt_string(&mut self, nonce: u64, message: &String, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_string_into_vec<U>(&mut self, nonce: T, message: &String, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data stored in a `String` object without any padding in
    /// CTR (CounTeR) mode, and stores the encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
    /// - `message` is an immutable reference to `String` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext.
    /// - If this method failed in encryption or `message.len()` is `0`,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is a `String` object that has a null string "", nothing
    ///   will be encrypted, and stored in the `Vec<U>` object
    ///   which is referred to as `cipher`.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_string_into_vec(nonce, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_string_into_vec(nonce, &message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_string_into_vec(nonce, &message, &mut cipher1);
    /// d_des.encrypt_string_into_vec(nonce, &message, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_string_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_string_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_string_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_string_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_string_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    /// ```
    pub fn encrypt_string_into_vec<U>(&mut self, nonce: u64, message: &String, cipher: &mut Vec<U>) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_string_into_array<U, const N: usize>(&mut self, nonce: T, message: &String, cipher: &mut [U; N]) -> u64
    /// Encrypts the data stored in a `String` object without any padding in CFB
    /// (Cipher FeedBack) mode, and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
    /// - `message` is an immutable reference to `String` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext.
    /// - If this method failed in encryption or `message.len()` is `0`,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is a `String` object that has a null string "", nothing
    ///   will be encrypted, and stored in the array `[U; N]` object `cipher`.
    /// - If `U::size_in_bytes()` * `N` is less than `message.len()`,
    ///   this method does not perform encryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `N` is equal to `message.len()`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and returns the size of the ciphertext in bytes.
    /// - If `U::size_in_bytes()` * `N` is greater than `message.len()`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and then fills the rest of the elements of the array
    ///   `cipher` with zeros, and returns the size of the ciphertext in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `message.len()` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_string_into_array(nonce, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_string_into_array(nonce, &message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = [0_u8; 55];
    /// let mut cipher2 = [0_u8; 55];
    /// c_des.encrypt_string_into_array(nonce, &message, &mut cipher1);
    /// d_des.encrypt_string_into_array(nonce, &message, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 0];
    /// a_des.encrypt_string_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 7];
    /// a_des.encrypt_string_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_string_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 12];
    /// a_des.encrypt_string_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.".to_string();
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_string_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    /// ```
    pub fn encrypt_string_into_array<U, const N: usize>(&mut self, nonce: u64, message: &String, cipher: &mut [U; N]) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_vec<U>(&mut self, nonce: T, message: &Vec<U>, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `Vec<U>` object without any padding
    /// in CTR (CounTeR) mode.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
    /// - `message` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext.
    /// - If this method failed in encryption or `message.len()` is `0`,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `size_of::<U>()` * `message.len()` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - If `message` is an empty `Vec<U>` object `Vec::<U>::new()`, nothing
    ///   will be encrypted, and stored in the memory area that
    ///   starts from `cipher`.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = [0_u8; 55];
    /// let mut cipher2 = [0_u8; 55];
    /// c_des.encrypt_vec(nonce, &message, cipher1.as_mut_ptr());
    /// d_des.encrypt_vec(nonce, &message, cipher2.as_mut_ptr());
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 0];
    /// a_des.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 7];
    /// a_des.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 12];
    /// a_des.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    /// ```
    pub fn encrypt_vec<U>(&mut self, nonce: u64, message: &Vec<U>, cipher: *mut u8) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_vec_into_vec<U, V>(&mut self, nonce: T, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in a `Vec<U>` object without any padding in
    /// CTR (CounTeR) mode, and stores the encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
    /// - `message` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext.
    /// - If this method failed in encryption or `message.len()` is `0`,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is an empty `Vec<U>` object `Vec::<U>::new()`, nothing
    ///   will be encrypted, and stored in the `Vec<U>` object
    ///   which is referred to as `cipher`.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    ///
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_vec_into_vec(nonce, &message, &mut cipher1);
    /// d_des.encrypt_vec_into_vec(nonce, &message, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    /// ```
    pub fn encrypt_vec_into_vec<U, V>(&mut self, nonce: u64, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, nonce: T, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    /// Encrypts the data stored in a `Vec<U>` object without any padding in CFB
    /// (Cipher FeedBack) mode, and stores the encrypted data in array `[V; N]`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
    /// - `message` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext.
    /// - If this method failed in encryption or `message.len()` is `0`,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is an empty `Vec<U>` object `Vec::<U>::new()`, nothing
    ///   will be encrypted, and stored in the array `[U; N]` object `cipher`.
    /// - If `size_of::<V>()` * `N` is less than
    ///   `size_of::<U>() * message.len()`,
    ///   this method does not perform encryption but returns `zero`.
    /// - If `size_of::<V>()` * `N` is equal to
    ///   `size_of::<U>() * message.len()`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and returns the size of the ciphertext in bytes.
    /// - If `size_of::<V>()` * `N` is greater than
    ///   `size_of::<U>() * message.len()`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and then fills the rest of the elements of the array
    ///   `cipher` with zeros, and returns the size of the ciphertext in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `size_of::<U>()` * `message.len()` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_vec_into_array(nonce, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_vec_into_array(nonce, &message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = [0_u8; 55];
    /// let mut cipher2 = [0_u8; 55];
    /// c_des.encrypt_vec_into_array(nonce, &message, &mut cipher1);
    /// d_des.encrypt_vec_into_array(nonce, &message, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 0];
    /// a_des.encrypt_vec_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 7];
    /// a_des.encrypt_vec_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_vec_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 12];
    /// a_des.encrypt_vec_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_vec_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    /// ```
    pub fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, nonce: u64, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_array<U, const N: usize>(&mut self, nonce: T, message: &[U; N], cipher: *mut u8) -> u64
    /// Encrypts the data stored in an array `[U; N]` object without any
    /// padding in CTR (CounTeR) mode.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
    /// - `message` is an immutable reference to an array `[U; N]` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext.
    /// - If this method failed in encryption or `message.len()` is `0`,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `size_of::<U>()` * `N` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - If `message` is an empty array `[U; 0]` object, nothing will be
    ///   encrypted, and stored in the memory area that starts from `cipher`.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = [0_u8; 55];
    /// let mut cipher2 = [0_u8; 55];
    /// c_des.encrypt_array(nonce, &message, cipher1.as_mut_ptr());
    /// d_des.encrypt_array(nonce, &message, cipher2.as_mut_ptr());
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 0];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 0];
    /// a_des.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "7 bytes";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 7];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 7];
    /// a_des.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "I am OK.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 8];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "PARK Youngho";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 12];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 12];
    /// a_des.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "고맙습니다.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 16];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    /// ```
    pub fn encrypt_array<U, const N: usize>(&mut self, nonce: u64, message: &[U; N], cipher: *mut u8) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, nonce: T, message: &[U; N], cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in an array `[U; N]` object without any padding
    /// in CTR (CounTeR) mode, and stores the encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
    /// - `message` is an immutable reference to an array `[U; N]` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext.
    /// - If this method failed in encryption or `message.len()` is `0`,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is an empty array `[U; 0]` object, nothing
    ///   will be encrypted, and stored in the `Vec<U>` object `cipher`.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_array_into_vec(nonce, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_array_into_vec(nonce, &message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    ///
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_array_into_vec(nonce, &message, &mut cipher1);
    /// d_des.encrypt_array_into_vec(nonce, &message, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 0];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_array_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "7 bytes";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 7];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_array_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "I am OK.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 8];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_array_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "PARK Youngho";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 12];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_array_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "고맙습니다.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 16];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_array_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    /// ```
    pub fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, nonce: u64, message: &[U; N], cipher: &mut Vec<V>) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, nonce: T, message: &[U; N], cipher: &mut [V; M]) -> u64
    /// Encrypts the data stored in an array `[U; N]` object without any padding
    /// in CTR (CounTeR) mode, and stores the encrypted data
    /// in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
    /// - `message` is an immutable reference to an array `[U; N]` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[V; M]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext.
    /// - If this method failed in encryption or `message.len()` is `0`,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is an empty array `[U; 0]` object, nothing
    ///   will be encrypted, and stored in the array `[V; M]` object `cipher`.
    /// - If `V::size_in_bytes()` * `M` is less than `U::size_in_bytes()` * `N`,
    ///   this method does not perform encryption and returns `zero`.
    /// - If `V::size_in_bytes()` * `M` is equal to `U::size_in_bytes()` * `N`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns the size of the ciphertext in bytes.
    /// - If `V::size_in_bytes()` * `M` is greater than
    ///   `U::size_in_bytes()` * `N`, this method performs encryption, fills the
    ///   array `cipher` with the encrypted ciphertext, and then fills the rest
    ///   of the elements of the array `cipher` with zeros, and returns the size
    ///   of the ciphertext in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   `size_of::<U>()` * `message.len()` at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_array_into_array(nonce, &message, &mut cipher);
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_array_into_array(nonce, &message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = [0_u8; 55];
    /// let mut cipher2 = [0_u8; 55];
    /// c_des.encrypt_array_into_array(nonce, &message, &mut cipher1);
    /// d_des.encrypt_array_into_array(nonce, &message, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 0];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 0];
    /// a_des.encrypt_array_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "7 bytes";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 7];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 7];
    /// a_des.encrypt_array_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "I am OK.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 8];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_array_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "PARK Youngho";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 12];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 12];
    /// a_des.encrypt_array_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "고맙습니다.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 16];
    /// message.copy_from_slice(mes.as_bytes());
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_array_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    /// ```
    pub fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, nonce: u64, message: &[U; N], cipher: &mut [V; M]) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt(&mut self, nonce: T, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64;
    /// Decrypts the data without any padding in CTR (CounTeR) mode.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
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
    /// - If `length_in_bytes` is greater than `0` (which means that the
    ///   original plaintext is surely not empty data) and it returns `zero`,
    ///   you can interpret it that this method surely failed in decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as `decrypt_*_into_*()`.
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The size of the memory area which starts at `message` is assumed to
    ///   be enough to store the plaintext. So, it is responsible for you to
    ///   prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `length_in_bytes`.
    /// - If the size of the area for plaintext is prepared more than
    ///   `length_in_bytes`, the rest of the area will be filled with `0`s.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = vec![0; 55];
    /// a_des.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    ///
    /// let mut recovered = vec![0; 55];
    /// a_des.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    ///
    /// let mut recovered1 = vec![0; 55];
    /// let mut recovered2 = vec![0; 55];
    /// c_des.decrypt(nonce, cipher1.as_ptr(), cipher1.len() as u64, recovered1.as_mut_ptr());
    /// d_des.decrypt(nonce, cipher2.as_ptr(), cipher2.len() as u64, recovered2.as_mut_ptr());
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
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    ///
    /// let mut recovered = vec![0; 8];
    /// let len = a_des.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// converted.truncate(len as usize);
    /// 
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// 
    /// let mut recovered = vec![0; 8];
    /// let len = a_des.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "37 20 62 79 74 65 73 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// converted.truncate(len as usize);
    ///
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "7 bytes");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// 
    /// let mut recovered = vec![0; 16];
    /// let len = a_des.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 20 61 6D 20 4F 4B 2E 00 00 00 00 00 00 00 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// converted.truncate(len as usize);
    /// 
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "I am OK.");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    ///
    /// let mut recovered = vec![0; 16];
    /// let len = a_des.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "50 41 52 4B 20 59 6F 75 6E 67 68 6F 00 00 00 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// converted.truncate(len as usize);
    /// 
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "PARK Youngho");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    ///
    /// let mut recovered = vec![0; 24];
    /// let len = a_des.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "EA B3 A0 EB A7 99 EC 8A B5 EB 8B 88 EB 8B A4 2E 00 00 00 00 00 00 00 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// converted.truncate(len as usize);
    /// 
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "고맙습니다.");
    /// assert_eq!(converted, message);
    /// ```
    pub fn decrypt(&mut self, nonce: u64, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_into_vec<U>(&mut self, nonce: T, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    /// Decrypts the data without any padding in CTR (CounTeR) mode,
    /// and stores the decrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
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
    /// - If `length_in_bytes` is greater than `0` (which means that the
    ///   original plaintext is surely not empty data) and it returns `zero`,
    ///   you can interpret it that this method surely failed in decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_vec().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    ///
    /// let mut recovered1 = Vec::<u8>::new();
    /// let mut recovered2 = Vec::<u8>::new();
    /// c_des.decrypt_into_vec(nonce, cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    /// d_des.decrypt_into_vec(nonce, cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
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
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    pub fn decrypt_into_vec<U>(&mut self, nonce: u64, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_into_array<U, const N: usize>(&mut self, nonce: T, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    /// Decrypts the data without any padding in CTR (CounTeR) mode,
    /// and stores the decrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
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
    /// - If `length_in_bytes` is greater than `0` (which means that the
    ///   original plaintext is surely not empty data) and it returns `zero`,
    ///   you can interpret it that this method surely failed in decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_array().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `U::size_in_bytes()` * `N` is less than `length_in_bytes`,
    ///   this method does not perform decryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `N` is equal to or greater than
    ///   `length_in_bytes`, this method performs decryption, fills the
    ///   array `message` with the decrypted data, and then fills the rest of
    ///   the elements of the array `message` with zeros, and returns the size
    ///   of the plaintext.
    /// - It is responsible for you to prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `length_in_bytes`.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = [0u8; 56];
    /// let len = a_des.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    ///
    /// let mut recovered = [0u8; 56];
    /// let len = a_des.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    ///
    /// let mut recovered1 = [0u8; 56];
    /// let mut recovered2 = [0u8; 56];
    /// let len1 = c_des.decrypt_into_array(nonce, cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    /// let len2 = d_des.decrypt_into_array(nonce, cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
    /// print!("B1a (0 rounds) =\t");
    /// for b in recovered1.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 ");
    /// print!("B2a (0 rounds) =\t");
    /// for b in recovered2.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 ");
    ///
    /// let mut converted1 = String::new();
    /// let mut converted2 = String::new();
    /// unsafe { converted1.as_mut_vec() }.write(&recovered1);
    /// unsafe { converted2.as_mut_vec() }.write(&recovered2);
    /// unsafe { converted1.as_mut_vec() }.truncate(len1 as usize);
    /// unsafe { converted2.as_mut_vec() }.truncate(len2 as usize);
    /// println!("B1b (0 rounds) =\t{}", converted1);
    /// println!("B2b (0 rounds) =\t{}", converted2);
    /// assert_eq!(converted1, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted2, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted1, message);
    /// assert_eq!(converted2, message);
    /// assert_eq!(converted1, converted2);
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    ///
    /// let mut recovered = [0u8; 8];
    /// let len = a_des.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    ///
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
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    ///
    /// let mut recovered = [0u8; 8];
    /// let len = a_des.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    ///
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "37 20 62 79 74 65 73 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "7 bytes");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    ///
    /// let mut recovered = [0u8; 16];
    /// let len = a_des.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    ///
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 20 61 6D 20 4F 4B 2E 00 00 00 00 00 00 00 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "I am OK.");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    ///
    /// let mut recovered = [0u8; 16];
    /// let len = a_des.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    ///
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "50 41 52 4B 20 59 6F 75 6E 67 68 6F 00 00 00 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "PARK Youngho");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    ///
    /// let mut recovered = [0u8; 24];
    /// let len = a_des.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    ///
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "EA B3 A0 EB A7 99 EC 8A B5 EB 8B 88 EB 8B A4 2E 00 00 00 00 00 00 00 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "고맙습니다.");
    /// assert_eq!(converted, message);
    /// ```
    pub fn decrypt_into_array<U, const N: usize>(&mut self, nonce: u64, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_into_string(&mut self, nonce: T, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    /// Decrypts the data without any padding in CTR (CounTeR) mode,
    /// and stores the decrypted data in a `String`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
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
    /// - If `length_in_bytes` is greater than `0` (which means that the
    ///   original plaintext is surely not empty data) and it returns `zero`,
    ///   you can interpret it that this method surely failed in decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_string().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B (16 rounds) =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B (128 rounds) =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    ///
    /// let mut recovered1 = String::new();
    /// let mut recovered2 = String::new();
    /// c_des.decrypt_into_string(nonce, cipher1.as_ptr(), cipher1.len() as u64, &mut recovered1);
    /// d_des.decrypt_into_string(nonce, cipher2.as_ptr(), cipher2.len() as u64, &mut recovered2);
    /// println!("B1 (0 rounds) =\t{}", recovered1);
    /// println!("B2 (0 rounds) =\t{}", recovered2);
    /// assert_eq!(recovered1, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered2, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered1, message);
    /// assert_eq!(recovered2, message);
    /// assert_eq!(recovered1, recovered2);
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "7 bytes");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "I am OK.");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "PARK Youngho");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "고맙습니다.");
    /// assert_eq!(recovered, message);
    /// ```
    pub fn decrypt_into_string(&mut self, nonce: u64, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_vec<U>(&mut self, nonce: T, cipher: &Vec<U>, message: *mut u8) -> u64
    /// Decrypts the data stored in a `Vec<U>` object without any padding
    /// in CTR (CounTeR) mode.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
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
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `0`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The size of the memory area which starts at `message` is assumed to
    ///   be enough to store the plaintext. So, it is responsible for you to
    ///   prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `size_of::<U>()` * `cipher.len()`.
    /// - If the size of the area for plaintext is prepared more than
    ///   `size_of::<U>()` * `cipher.len()`, the rest of the area will be
    ///   filled with `0`s.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = vec![0; 55];
    /// a_des.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    ///
    /// let mut recovered = vec![0; 55];
    /// a_des.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    ///
    /// let mut recovered1 = vec![0; 55];
    /// let mut recovered2 = vec![0; 55];
    /// c_des.decrypt_vec(nonce, &cipher1, recovered1.as_mut_ptr());
    /// d_des.decrypt_vec(nonce, &cipher2, recovered2.as_mut_ptr());
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
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    ///
    /// let mut recovered = vec![0; 8];
    /// let len = a_des.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// converted.truncate(len as usize);
    /// 
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// 
    /// let mut recovered = vec![0; 8];
    /// let len = a_des.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "37 20 62 79 74 65 73 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// converted.truncate(len as usize);
    ///
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "7 bytes");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// 
    /// let mut recovered = vec![0; 16];
    /// let len = a_des.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 20 61 6D 20 4F 4B 2E 00 00 00 00 00 00 00 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// converted.truncate(len as usize);
    /// 
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "I am OK.");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    ///
    /// let mut recovered = vec![0; 16];
    /// let len = a_des.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "50 41 52 4B 20 59 6F 75 6E 67 68 6F 00 00 00 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// converted.truncate(len as usize);
    /// 
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "PARK Youngho");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    ///
    /// let mut recovered = vec![0; 24];
    /// let len = a_des.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "EA B3 A0 EB A7 99 EC 8A B5 EB 8B 88 EB 8B A4 2E 00 00 00 00 00 00 00 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.append(&mut recovered);
    /// converted.truncate(len as usize);
    /// 
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "고맙습니다.");
    /// assert_eq!(converted, message);
    /// ```
    pub fn decrypt_vec<U>(&mut self, nonce: u64, cipher: &Vec<U>, message: *mut u8) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_vec_into_vec<U, V>(&mut self, nonce: T, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in a `Vec<U>` object without any padding in
    /// CTR (CounTeR) mode, and stores the decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
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
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `0`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    ///
    /// let mut recovered1 = Vec::<u8>::new();
    /// let mut recovered2 = Vec::<u8>::new();
    /// c_des.decrypt_vec_into_vec(nonce, &cipher1, &mut recovered1);
    /// d_des.decrypt_vec_into_vec(nonce, &cipher2, &mut recovered2);
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
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    pub fn decrypt_vec_into_vec<U, V>(&mut self, nonce: u64, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, nonce: T, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    /// Decrypts the data stored in a `Vec<U>` object without any padding in CFB
    /// (Cipher FeedBack) mode, and stores the decrypted data in array `[V; N]`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
    /// - `cipher` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the ciphertext to be decrypted is stored.
    /// - `message` is a mutable reference to an array `[V; N]` object, and
    ///   is the place where the decrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method failed in decryption, it always returns `zero`.
    /// - Even if this method succeeded in decryption, it returns `zero` when
    ///   the original plaintext is zero-length empty data. Then, you will have
    ///   to check whether or not it failed by using the method
    ///   `is_successful()` or `is_failed()`.
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `0`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - If `size_of::<V>()` * `N` is less than 
    ///   `size_of::<U>()` * `cipher.len()`,
    ///   this method does not perform decryption but returns `zero`.
    /// - If `size_of::<V>()` * `N` is equal to or greater than
    ///   `size_of::<U>()` * `cipher.len()`, this method performs
    ///   decryption, fills the array `message` with the decrypted data, and
    ///   then fills the rest of the elements of the array `message` with zeros,
    ///   and returns the size of the plaintext.
    /// - It is responsible for you to prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `size_of::<U>()` * `cipher.len()`.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = [0u8; 56];
    /// let len = a_des.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    ///
    /// let mut recovered = [0u8; 56];
    /// let len = a_des.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    ///
    /// let mut recovered1 = [0u8; 56];
    /// let mut recovered2 = [0u8; 56];
    /// let len1 = c_des.decrypt_vec_into_array(nonce, &cipher1, &mut recovered1);
    /// let len2 = d_des.decrypt_vec_into_array(nonce, &cipher2, &mut recovered2);
    /// print!("B1a (0 rounds) =\t");
    /// for b in recovered1.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 ");
    /// print!("B2a (0 rounds) =\t");
    /// for b in recovered2.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 ");
    ///
    /// let mut converted1 = String::new();
    /// let mut converted2 = String::new();
    /// unsafe { converted1.as_mut_vec() }.write(&recovered1);
    /// unsafe { converted2.as_mut_vec() }.write(&recovered2);
    /// unsafe { converted1.as_mut_vec() }.truncate(len1 as usize);
    /// unsafe { converted2.as_mut_vec() }.truncate(len2 as usize);
    /// println!("B1b (0 rounds) =\t{}", converted1);
    /// println!("B2b (0 rounds) =\t{}", converted2);
    /// assert_eq!(converted1, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted2, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted1, message);
    /// assert_eq!(converted2, message);
    /// assert_eq!(converted1, converted2);
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    ///
    /// let mut recovered = [0u8; 8];
    /// let len = a_des.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
    ///
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
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    ///
    /// let mut recovered = [0u8; 8];
    /// let len = a_des.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
    ///
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "37 20 62 79 74 65 73 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "7 bytes");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    ///
    /// let mut recovered = [0u8; 16];
    /// let len = a_des.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
    ///
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 20 61 6D 20 4F 4B 2E 00 00 00 00 00 00 00 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "I am OK.");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    ///
    /// let mut recovered = [0u8; 16];
    /// let len = a_des.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
    ///
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "50 41 52 4B 20 59 6F 75 6E 67 68 6F 00 00 00 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "PARK Youngho");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    ///
    /// let mut recovered = [0u8; 24];
    /// let len = a_des.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
    ///
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "EA B3 A0 EB A7 99 EC 8A B5 EB 8B 88 EB 8B A4 2E 00 00 00 00 00 00 00 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "고맙습니다.");
    /// assert_eq!(converted, message);
    /// ```
    pub fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, nonce: u64, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_vec_into_string(&mut self, nonce: T, cipher: &str, message: &mut String) -> u64
    /// Decrypts the data in `str` without any padding in CTR (CounTeR)
    /// mode, and stores the decrypted data in `String`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
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
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `0`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    /// println!("B (16 rounds) =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    /// println!("B (128 rounds) =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    ///
    /// let mut recovered1 = String::new();
    /// let mut recovered2 = String::new();
    /// c_des.decrypt_vec_into_string(nonce, &cipher1, &mut recovered1);
    /// d_des.decrypt_vec_into_string(nonce, &cipher2, &mut recovered2);
    /// println!("B1 (0 rounds) =\t{}", recovered1);
    /// println!("B2 (0 rounds) =\t{}", recovered2);
    /// assert_eq!(recovered1, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered2, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered1, message);
    /// assert_eq!(recovered2, message);
    /// assert_eq!(recovered1, recovered2);
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "7 bytes");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "I am OK.");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "PARK Youngho");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "고맙습니다.");
    /// assert_eq!(recovered, message);
    /// ```
    pub fn decrypt_vec_into_string<U>(&mut self, nonce: u64, cipher: &Vec<U>, message: &mut String) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_array<U, const N: usize>(&mut self, nonce: T, cipher: &[U; N], message: *mut u8) -> u64
    /// Decrypts the data stored in an array `[U; N]` object without any padding
    /// in CTR (CounTeR) mode.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
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
    /// - If `size_of::<U>()` * `N` is greater than `0`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_array_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The size of the memory area which starts at `message` is assumed to
    ///   be enough to store the plaintext. So, it is responsible for you to
    ///   prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `size_of::<U>()` * `N`.
    /// - If the size of the area for plaintext is prepared more than
    ///   `size_of::<U>()` * `N`, the rest of the area will be filled with `0`s.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = vec![0; 55];
    /// let len = a_des.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    ///
    /// let mut recovered = vec![0; 55];
    /// let len = a_des.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
    /// recovered.truncate(len as usize);
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = [0_u8; 55];
    /// let mut cipher2 = [0_u8; 55];
    /// c_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    ///
    /// let mut recovered1 = vec![0; 55];
    /// let mut recovered2 = vec![0; 55];
    /// let len1 = c_des.decrypt_array(nonce, &cipher1, recovered1.as_mut_ptr());
    /// let len2 = d_des.decrypt_array(nonce, &cipher2, recovered2.as_mut_ptr());
    /// recovered1.truncate(len1 as usize);
    /// recovered2.truncate(len2 as usize);
    ///
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
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 0];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    ///
    /// let mut recovered = vec![0; 8];
    /// let len = a_des.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
    /// recovered.truncate(len as usize);
    ///
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
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 7];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// 
    /// let mut recovered = vec![0; 8];
    /// let len = a_des.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
    /// recovered.truncate(len as usize);
    ///
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
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// 
    /// let mut recovered = vec![0; 16];
    /// let len = a_des.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
    /// recovered.truncate(len as usize);
    ///
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
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 12];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    ///
    /// let mut recovered = vec![0; 16];
    /// let len = a_des.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
    /// recovered.truncate(len as usize);
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
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    ///
    /// let mut recovered = vec![0; 24];
    /// let len = a_des.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
    /// recovered.truncate(len as usize);
    ///
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
    pub fn decrypt_array<U, const N: usize>(&mut self, nonce: u64, cipher: &[U; N], message: *mut u8) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, nonce: T, cipher: &[U; N], message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in an array `[U; N]` object without any padding
    /// in CTR (CounTeR) mode, and stores the decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
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
    /// - If `size_of::<U>()` * `N` is greater than `0` (which means that the
    ///   original plaintext is surely not empty data) and it returns `zero`,
    ///   you can interpret it that this method surely failed in decryption.
    /// 
    /// # Features
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = [0_u8; 55];
    /// let mut cipher2 = [0_u8; 55];
    /// c_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    ///
    /// let mut recovered1 = Vec::<u8>::new();
    /// let mut recovered2 = Vec::<u8>::new();
    /// c_des.decrypt_array_into_vec(nonce, &cipher1, &mut recovered1);
    /// d_des.decrypt_array_into_vec(nonce, &cipher2, &mut recovered2);
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
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 0];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 7];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 12];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    pub fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, nonce: u64, cipher: &[U; N], message: &mut Vec<V>) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, nonce: T, cipher: &[U; N], message: &mut [V; M]) -> u64
    /// Decrypts the data stored in an array `[U; N]` object without any padding
    /// in CTR (CounTeR) mode, and stores the decrypted data
    /// in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
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
    /// - If `size_of::<U>()` * `N` is greater than `0`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - If `size_of::<V>()` * `M` is less than `size_of::<U>()` * `N`,
    ///   this method does not perform decryption but returns `zero`.
    /// - If `size_of::<V>()` * `M` is equal to or greater than
    ///   `size_of::<U>()` * `N`, this method performs decryption,
    ///   fills the array `message` with the decrypted data, and then
    ///   fills the rest of the elements of the array `message` with zeros, and
    ///   returns the size of the plaintext.
    /// - It is responsible for you to prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `size_of::<U>()` * `N`.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = [0u8; 56];
    /// let len = a_des.decrypt_array_into_array(nonce, &cipher, &mut recovered);
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    ///
    /// let mut recovered = [0u8; 56];
    /// let len = a_des.decrypt_array_into_array(nonce, &cipher, &mut recovered);
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = [0_u8; 55];
    /// let mut cipher2 = [0_u8; 55];
    /// c_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    ///
    /// let mut recovered1 = [0u8; 56];
    /// let mut recovered2 = [0u8; 56];
    /// let len1 = c_des.decrypt_array_into_array(nonce, &cipher1, &mut recovered1);
    /// let len2 = d_des.decrypt_array_into_array(nonce, &cipher2, &mut recovered2);
    /// print!("B1a (0 rounds) =\t");
    /// for b in recovered1.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 ");
    /// print!("B2a (0 rounds) =\t");
    /// for b in recovered2.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E 00 ");
    ///
    /// let mut converted1 = String::new();
    /// let mut converted2 = String::new();
    /// unsafe { converted1.as_mut_vec() }.write(&recovered1);
    /// unsafe { converted2.as_mut_vec() }.write(&recovered2);
    /// unsafe { converted1.as_mut_vec() }.truncate(len1 as usize);
    /// unsafe { converted2.as_mut_vec() }.truncate(len2 as usize);
    /// println!("B1b (0 rounds) =\t{}", converted1);
    /// println!("B2b (0 rounds) =\t{}", converted2);
    /// assert_eq!(converted1, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted2, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted1, message);
    /// assert_eq!(converted2, message);
    /// assert_eq!(converted1, converted2);
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 0];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    ///
    /// let mut recovered = [0u8; 8];
    /// let len = a_des.decrypt_array_into_array(nonce, &cipher, &mut recovered);
    ///
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
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 7];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    ///
    /// let mut recovered = [0u8; 8];
    /// let len = a_des.decrypt_array_into_array(nonce, &cipher, &mut recovered);
    ///
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "37 20 62 79 74 65 73 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "7 bytes");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    ///
    /// let mut recovered = [0u8; 16];
    /// let len = a_des.decrypt_array_into_array(nonce, &cipher, &mut recovered);
    ///
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "49 20 61 6D 20 4F 4B 2E 00 00 00 00 00 00 00 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "I am OK.");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 12];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    ///
    /// let mut recovered = [0u8; 16];
    /// let len = a_des.decrypt_array_into_array(nonce, &cipher, &mut recovered);
    ///
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "50 41 52 4B 20 59 6F 75 6E 67 68 6F 00 00 00 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "PARK Youngho");
    /// assert_eq!(converted, message);
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    ///
    /// let mut recovered = [0u8; 24];
    /// let len = a_des.decrypt_array_into_array(nonce, &cipher, &mut recovered);
    ///
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in recovered.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "EA B3 A0 EB A7 99 EC 8A B5 EB 8B 88 EB 8B A4 2E 00 00 00 00 00 00 00 00 ");
    ///
    /// let mut converted = String::new();
    /// unsafe { converted.as_mut_vec() }.write(&recovered);
    /// unsafe { converted.as_mut_vec() }.truncate(len as usize);
    /// println!("Bb =\t{}", converted);
    /// assert_eq!(converted, "고맙습니다.");
    /// assert_eq!(converted, message);
    /// ```
    pub fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, nonce: u64, cipher: &[U; N], message: &mut [V; M]) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_array_into_string<U, const N: usize>(&mut self, nonce: T, cipher: &[U; N], message: &mut String) -> u64
    /// Decrypts the data stored in an array `[U; N]` object without any padding
    /// in CTR (CounTeR) mode, and stores the decrypted data in `String`.
    /// 
    /// # Arguments
    /// - `nonce` is an initialization vector for CTR mode.
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
    /// - If `size_of::<U>()` * `N` is greater than `0`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    /// println!("B (16 rounds) =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    /// println!("B (128 rounds) =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K1 =\t{:#016X}", key1);
    /// println!("K2 =\t{:#016X}", key2);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher1 = [0_u8; 55];
    /// let mut cipher2 = [0_u8; 55];
    /// c_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    ///
    /// let mut recovered1 = String::new();
    /// let mut recovered2 = String::new();
    /// c_des.decrypt_array_into_string(nonce, &cipher1, &mut recovered1);
    /// d_des.decrypt_array_into_string(nonce, &cipher2, &mut recovered2);
    /// println!("B1 (0 rounds) =\t{}", recovered1);
    /// println!("B2 (0 rounds) =\t{}", recovered2);
    /// assert_eq!(recovered1, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered2, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered1, message);
    /// assert_eq!(recovered2, message);
    /// assert_eq!(recovered1, recovered2);
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 0];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 7];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "7 bytes");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "I am OK.");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 12];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "PARK Youngho");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce =	{}", nonce);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "고맙습니다.");
    /// assert_eq!(recovered, message);
    /// ```
    pub fn decrypt_array_into_string<U, const N: usize>(&mut self, nonce: u64, cipher: &[U; N], message: &mut String) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }
}