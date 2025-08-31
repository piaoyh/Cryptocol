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
use crate::number::SmallUInt;
use crate::symmetric::pre_decrypt_into_vec_no_padding;



/// CTR (CounTeR) is one of the operation modes for encryption/decryption.
pub trait CTR<T> : Sized
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.encrypt)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ofb/struct.DES_Generic.html#method.encrypt)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// taes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.encrypt)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// tdes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.encrypt)
    fn encrypt(&mut self, nonce: T, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;

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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.encrypt_into_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.encrypt_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.encrypt_into_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.encrypt_into_vec)
    fn encrypt_into_vec<U>(&mut self, nonce: T, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone;

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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.encrypt_into_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_into_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// taes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.encrypt_into_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// tdes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.encrypt_into_array)
    fn encrypt_into_array<U, const N: usize>(&mut self, nonce: T, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone;

    // fn encrypt_str(&mut self, nonce: T, message: &str, cipher: *mut u8) -> u64
    /// Encrypts the data in a `str` object without any padding in CTR (CounTeR) mode.
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.encrypt_str)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_str)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// taes.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.encrypt_str)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// tdes.encrypt_str(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.encrypt_str)
    #[inline]
    fn encrypt_str(&mut self, nonce: T, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher)
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.encrypt_str_into_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_str_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.encrypt_str_into_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.encrypt_str_into_vec)
    #[inline]
    fn encrypt_str_into_vec<U>(&mut self, nonce: T, message: &str, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_str_into_array<U, const N: usize>(&mut self, nonce: T, message: &str, cipher: &mut [U; N]) -> u64
    /// Encrypts the data in a `str` object without any padding in CTR (CounTeR) mode, and stores the encrypted data in array `[U; N]`.
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.encrypt_str_into_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_str_into_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// let mut cipher = [0_u8; 55];
    /// taes.encrypt_str_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.encrypt_str_into_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// let mut cipher = [0_u8; 55];
    /// tdes.encrypt_str_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.encrypt_str_into_array)
    #[inline]
    fn encrypt_str_into_array<U, const N: usize>(&mut self, nonce: T, message: &str, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, cipher)
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.encrypt_string)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_string)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// let mut cipher = [0_u8; 55];
    /// taes.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.encrypt_string)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// let mut cipher = [0_u8; 55];
    /// tdes.encrypt_string(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.encrypt_string)
    #[inline]
    fn encrypt_string(&mut self, nonce: T, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher)
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_string_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.encrypt_string_into_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_string_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_string_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.encrypt_string_into_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_string_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.encrypt_string_into_vec)
    #[inline]
    fn encrypt_string_into_vec<U>(&mut self, nonce: T, message: &String, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string_into_array<U, const N: usize>(&mut self, nonce: T, message: &String, cipher: &mut [U; N]) -> u64
    /// Encrypts the data stored in a `String` object without any padding in CTR
    /// (CounTeR) mode, and stores the encrypted data in array `[U; N]`.
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_string_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.encrypt_string_into_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_string_into_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// let mut cipher = [0_u8; 55];
    /// taes.encrypt_string_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.encrypt_string_into_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// let mut cipher = [0_u8; 55];
    /// tdes.encrypt_string_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.encrypt_string_into_array)
    #[inline]
    fn encrypt_string_into_array<U, const N: usize>(&mut self, nonce: T, message: &String, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, cipher)
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.encrypt_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 55];
    /// taes.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.encrypt_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 55];
    /// tdes.encrypt_vec(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.encrypt_vec)
    #[inline]
    fn encrypt_vec<U>(&mut self, nonce: T, message: &Vec<U>, cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt(nonce, message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.encrypt_vec_into_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_vec_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.encrypt_vec_into_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_vec_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.encrypt_vec_into_vec)
    #[inline]
    fn encrypt_vec_into_vec<U, V>(&mut self, nonce: T, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(nonce, message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, nonce: T, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    /// Encrypts the data stored in a `Vec<U>` object without any padding in CTR
    /// (CounTeR) mode, and stores the encrypted data in array `[V; N]`.
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_vec_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.encrypt_vec_into_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_vec_into_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 55];
    /// taes.encrypt_vec_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.encrypt_vec_into_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 55];
    /// tdes.encrypt_vec_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.encrypt_vec_into_array)
    #[inline]
    fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, nonce: T, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(nonce, message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.encrypt_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 55];
    /// taes.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.encrypt_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 55];
    /// tdes.encrypt_array(nonce, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.encrypt_array)
    #[inline]
    fn encrypt_array<U, const N: usize>(&mut self, nonce: T, message: &[U; N], cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt(nonce, message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_array_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.encrypt_array_into_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_array_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_array_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.encrypt_array_into_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_array_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.encrypt_array_into_vec)
    #[inline]
    fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, nonce: T, message: &[U; N], cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(nonce, message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_array_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.encrypt_array_into_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_array_into_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 55];
    /// taes.encrypt_array_into_array(nonce, &message, &mut cipher);
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.encrypt_array_into_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 55];
    /// tdes.encrypt_array_into_array(nonce, &message, &mut cipher);
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.encrypt_array_into_array)
    #[inline]
    fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, nonce: T, message: &[U; N], cipher: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(nonce, message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(nonce.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.decrypt)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// taes.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.decrypt)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// tdes.decrypt(nonce, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.decrypt)
    #[inline]
    fn decrypt(&mut self, nonce: T, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        self.encrypt(nonce, cipher, length_in_bytes, message)
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(nonce.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_aes.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.decrypt_into_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// taes.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.decrypt_into_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// tdes.decrypt_into_vec(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.decrypt_into_vec)
    fn decrypt_into_vec<U>(&mut self, nonce: T, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        pre_decrypt_into_vec_no_padding!(message, length_in_bytes, U);
        let len = self.decrypt(nonce, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(len as usize);
        len
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(nonce.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// 
    /// let mut recovered = [0; 64];
    /// let len = a_aes.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.decrypt_into_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_into_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// 
    /// let mut recovered = [0u8; 56];
    /// let len = taes.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.decrypt_into_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// 
    /// let mut recovered = [0u8; 56];
    /// let len = tdes.decrypt_into_array(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.decrypt_into_array)
    fn decrypt_into_array<U, const N: usize>(&mut self, nonce: T, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone;

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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(nonce.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// println!();
    /// 
    /// let mut converted= String::new();
    /// a_aes.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.decrypt_into_string)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_into_string)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// 
    /// let mut recovered = String::new();
    /// taes.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.decrypt_into_string)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// 
    /// let mut recovered = String::new();
    /// tdes.decrypt_into_string(nonce, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.decrypt_into_string)
    #[inline]
    fn decrypt_into_string(&mut self, nonce: T, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_into_vec(nonce, cipher, length_in_bytes, unsafe { message.as_mut_vec() })
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.decrypt_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// taes.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.decrypt_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// tdes.decrypt_vec(nonce, &cipher, recovered.as_mut_ptr());
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
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.decrypt_vec)
    #[inline]
    fn decrypt_vec<U>(&mut self, nonce: T, cipher: &Vec<U>, message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_aes.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.decrypt_vec_into_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_vec_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// taes.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.decrypt_vec_into_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// tdes.decrypt_vec_into_vec(nonce, &cipher, &mut recovered);
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
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.decrypt_vec_into_vec)
    #[inline]
    fn decrypt_vec_into_vec<U, V>(&mut self, nonce: T, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_vec(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, nonce: T, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    /// Decrypts the data stored in a `Vec<U>` object without any padding in CTR
    /// (CounTeR) mode, and stores the decrypted data in array `[V; N]`.
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// 
    /// let mut recovered = [0; 64];
    /// let len = a_aes.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
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
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.decrypt_vec_into_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_vec_into_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// 
    /// let mut recovered = [0u8; 56];
    /// let len = taes.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
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
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.decrypt_vec_into_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// 
    /// let mut recovered = [0u8; 56];
    /// let len = tdes.decrypt_vec_into_array(nonce, &cipher, &mut recovered);
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
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.decrypt_vec_into_array)
    #[inline]
    fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, nonce: T, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_array(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// 
    /// let mut converted= String::new();
    /// a_aes.decrypt_vec_into_string(nonce, &cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.decrypt_vec_into_string)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_vec_into_string)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// taes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// 
    /// let mut recovered = String::new();
    /// taes.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.decrypt_vec_into_string)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// 
    /// let mut recovered = String::new();
    /// tdes.decrypt_vec_into_string(nonce, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.decrypt_vec_into_string)
    #[inline]
    fn decrypt_vec_into_string<U>(&mut self, nonce: T, cipher: &Vec<U>, message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt_into_string(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// println!();
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.decrypt_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 55];
    /// taes.encrypt_str_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// let len = taes.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.decrypt_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 55];
    /// tdes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// let len = tdes.decrypt_array(nonce, &cipher, recovered.as_mut_ptr());
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
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.decrypt_array)
    #[inline]
    fn decrypt_array<U, const N: usize>(&mut self, nonce: T, cipher: &[U; N], message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// println!();
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.decrypt_array_into_vec)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_array_into_vec)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 55];
    /// taes.encrypt_str_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// taes.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.decrypt_array_into_vec)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 55];
    /// tdes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// tdes.decrypt_array_into_vec(nonce, &cipher, &mut recovered);
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
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.decrypt_array_into_vec)
    #[inline]
    fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, nonce: T, cipher: &[U; N], message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_vec(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// println!();
    /// 
    /// let mut recovered = [0; 64];
    /// let len = a_aes.decrypt_array_into_array(nonce, &cipher, &mut recovered);
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
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.decrypt_array_into_array)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_array_into_array)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 55];
    /// taes.encrypt_str_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// 
    /// let mut recovered = [0u8; 56];
    /// let len = taes.decrypt_array_into_array(nonce, &cipher, &mut recovered);
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
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.decrypt_array_into_array)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 55];
    /// tdes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// 
    /// let mut recovered = [0u8; 56];
    /// let len = tdes.decrypt_array_into_array(nonce, &cipher, &mut recovered);
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
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.decrypt_array_into_array)
    #[inline]
    fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, nonce: T, cipher: &[U; N], message: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_array(nonce, cipher.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, message)
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
    /// # For Rijndael or AES, and its variants
    /// ## Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let nonce = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("Nonce =\t{:08X}{:08X}{:08X}{:08X}", nonce[0].to_be(), nonce[1].to_be(), nonce[2].to_be(), nonce[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(nonce.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 07 AF 4E C8 EF 5D 0A 74 97 3F 90 9E 05 9E 9E 32 FB 55 54 45 7C ED A2 2B F8 07 66 C0 7B CB 98 F3 BF 93 15 BA 26 1C 47 ");
    /// println!();
    /// 
    /// let mut converted= String::new();
    /// a_aes.decrypt_array_into_string(nonce, &cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/rijndael_ctr/struct.Rijndael_Generic.html#method.decrypt_array_into_string)
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_array_into_string)
    /// 
    /// # For BigCryptor128
    /// ## Example 1 for TAES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor128, AES_128, CTR };
    /// 
    /// let mut taes = AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128)
    ///                 + AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)
    ///                 + AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCBA0987654321_u128);
    /// let nonce = 0x_FEDCBA09876543211234567890ABCDEF_u128;
    /// println!("Nonce = {:#034X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 55];
    /// taes.encrypt_str_into_array(nonce, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "91 99 31 04 B7 AB 4B 93 4A 4B 15 04 8F 75 7C B7 F3 AD A2 CE 90 FA 59 BF 79 67 BF 10 DE 26 6E A1 77 B7 7A 8E 82 3E 98 00 4B F4 31 28 87 A3 7D DE 86 4E B4 0D 2D 7B 0A ");
    /// 
    /// let mut recovered = String::new();
    /// taes.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor128_ctr/struct.BigCryptor128.html#method.decrypt_array_into_string)
    /// 
    /// # For BigCryptor64
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, CTR };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let nonce = 0x_FEDCBA0987654321_u64;
    /// println!("Nonce = {:#018X}", nonce);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 55];
    /// tdes.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AE EA 58 8D 8D 8F E9 58 AE A5 40 76 D9 77 48 3F 72 CD 46 C1 50 97 0E C3 FD 7F C2 65 53 AA DF E8 17 86 76 4E A0 F9 2B 7E D9 79 DA 38 AC F9 EE 51 F3 3E 0A 6C 25 75 68 ");
    /// 
    /// let mut recovered = String::new();
    /// tdes.decrypt_array_into_string(nonce, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/big_cryptor64_ctr/struct.BigCryptor64.html#method.decrypt_array_into_string)
    #[inline]
    fn decrypt_array_into_string<U, const N: usize>(&mut self, nonce: T, cipher: &[U; N], message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt_into_string(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }
}
