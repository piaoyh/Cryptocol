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


/// trait_ecb_with_padding_pkcs7.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to rijndael_ecb_pkcs7.rs. And, most of
/// generic parameters are omitted. It is not actual code but dummy code for
/// compilation!!!
#[allow(non_camel_case_types)]
pub struct Rijndael_Generic<const NB: usize = 4, const NK: usize = 4>
{
    // Dummy struct for documentation
}

/// trait_ecb_with_padding_pkcs7.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to rijndael_ecb_pkcs7.rs. And, most of
/// generic parameters are omitted. It is not actual code but dummy code for
/// compilation!!!
impl <const NB: usize, const NK: usize> Rijndael_Generic<NB, NK>
{
    // pub fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;
    /// Encrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic CodeBook) mode.
    /// 
    /// # Arguments
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
    /// - The output will be at least `4` * `NB`,
    ///   and cannot be other than a multiple of `4` * `NB`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_*_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null string.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`length_in_bytes` + `1`).next_multiple_of(`4` * `NB`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// ```
    pub fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_into_vec<U>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic CodeBook) mode, and stores the encrypted data
    /// in `Vec<U>`.
    /// 
    /// # Arguments
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
    /// - The output will be at least `4` * `NB`,
    ///   and cannot be other than a multiple of `4` * `NB`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_*_into_*().
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// ```
    pub fn encrypt_into_vec<U>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_into_array<U, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    /// Encrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic CodeBook) mode, and stores the encrypted data
    /// in array `[U; N]`.
    /// 
    /// # Arguments
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
    /// - The output will be at least `4` * `NB`,
    ///   and cannot be other than a multiple of `4` * `NB`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_*_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null data.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the array `[U; N]` object `cipher`.
    /// - If `U::size_in_bytes()` * `N` is less than `length_in_bytes`'s next
    ///   multiple of `4` * `NB`, this method does not perform
    ///   encryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `N` is equal to `length_in_bytes`'s next
    ///   multiple of `4` * `NB`, this method performs encryption,
    ///   fills the array `cipher` with the encrypted data, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - If `U::size_in_bytes()` * `N` is greater than `length_in_bytes`'s next
    ///   multiple of `4` * `NB`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`length_in_bytes` + `1`).next_multiple_of(`4` * `NB`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// ```
    pub fn encrypt_into_array<U, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_str(&mut self, message: &str, cipher: *mut u8) -> u64
    /// Encrypts the data in a `str` object with the padding defined
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `str` object which is `&str`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `4` * `NB`,
    ///   and cannot be other than a multiple of `4` * `NB`.
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
    ///   (`message.len()` + `1`).next_multiple_of(`4` * `NB`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// ```
    pub fn encrypt_str(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_str_into_vec<U>(&mut self, message: &str, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data in `str` with the padding defined according to
    /// PKCS #7 in ECB (Electronic CodeBook) mode, and stores the
    /// encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `str` object which is `&str`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `8`,
    ///   and cannot be other than a multiple of `8`.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// ```
    pub fn encrypt_str_into_vec<U>(&mut self, message: &str, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_str_into_array<U, const N: usize>(&mut self, message: &str, cipher: &mut [U; N]) -> u64
    /// Encrypts the data in a `str` object with the padding defined
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode,
    /// and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `str` object which is `&str`,
    ///   and is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `4` * `NB`,
    ///   and cannot be other than a multiple of `4` * `NB`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the array `[U; N]` object `cipher`.
    /// - If `U::size_in_bytes()` * `N` is less than `message.len()`'s next
    ///   multiple of `4` * `NB`, this method does not perform
    ///   encryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `N` is equal to `message.len()`'s next
    ///   multiple of `4` * `NB`, this method performs encryption,
    ///   fills the array `cipher` with the encrypted data, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - If `U::size_in_bytes()` * `N` is greater than `message.len()`'s next
    ///   multiple of `4` * `NB`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`message.len()` + `1`).next_multiple_of(`4` * `NB`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// ```
    pub fn encrypt_str_into_array<U, const N: usize>(&mut self, message: &str, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_string(&mut self, message: &String, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `String` object with the padding
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `String` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `4` * `NB`,
    ///   and cannot be other than a multiple of `4` * `NB`.
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
    ///   (`message.len()` + `1`).next_multiple_of(`4` * `NB`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_string(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_string(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_string(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_string(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_string(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// ```
    pub fn encrypt_string(&mut self, message: &String, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_string_into_vec<U>(&mut self, message: &String, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data stored in a `String` object with the padding
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode,
    /// and stores the encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `String` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `4` * `NB`,
    ///   and cannot be other than a multiple of `4` * `NB`.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_string_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_encrypt_string_into_vecvec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_string_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// ```
    /// 
    /// # Example 4 for ijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_string_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_string_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// ```
    pub fn encrypt_string_into_vec<U>(&mut self, message: &String, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_string_into_array<U, const N: usize>(&mut self, message: &String, cipher: &mut [U; N]) -> u64
    /// Encrypts the data stored in a `String` object with the padding
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode,
    /// and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `String` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `4` * `NB`,
    ///   and cannot be other than a multiple of `4` * `NB`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is a `String` object that has a null string "", only
    ///   padding bytes will be encrypted, and stored in the array `[U; N]`
    ///   object `cipher`.
    /// - If `size_of::<U>()` * `N` is less than `message.len()`'s next
    ///   multiple of `4` * `NB`, this method does not perform
    ///   encryption but returns `zero`.
    /// - If `size_of::<U>()` * `N` is equal to `message.len()`'s next
    ///   multiple of `4` * `NB`, this method performs encryption,
    ///   fills the array `cipher` with the encrypted data, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - If `size_of::<U>()` * `N` is greater than `message.len()`'s next
    ///   multiple of `4` * `NB`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`message.len()` + `1`).next_multiple_of(`4` * `NB`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_string_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_string_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_string_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_string_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_string_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// ```
    pub fn encrypt_string_into_array<U, const N: usize>(&mut self, message: &String, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_vec<U>(&mut self, message: &Vec<U>, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `4` * `NB`,
    ///   and cannot be other than a multiple of `4` * `NB`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`size_of::<U>()` * `message.len()` + `1`).next_multiple_of(`4` * `MB`)
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_vec(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_vec(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_vec(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_vec(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_vec(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// ```
    pub fn encrypt_vec<U>(&mut self, message: &Vec<U>, cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_vec_into_vec<U, V>(&mut self, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode, and
    /// stores the encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `4` * `NB`,
    ///   and cannot be other than a multiple of `4` * `NB`.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_vec_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_vec_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_vec_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_vec_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_vec_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// ```
    pub fn encrypt_vec_into_vec<U, V>(&mut self, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode, and
    /// stores the encrypted data in array `[V; N]`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to `Vec<U>` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `4` * `NB`,
    ///   and cannot be other than a multiple of `4` * `NB`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is an empty `Vec<U>` object `Vec::<U>::new()`, only
    ///   padding bytes will be encrypted, and stored in the array `[U; N]`
    ///   object `cipher`.
    /// - If `size_of::<V>()` * `N` is less than 
    ///   `size_of::<U>() * message.len()`'s next multiple of
    ///   `4` * `NB`, this method does not perform
    ///   encryption but returns `zero`.
    /// - If `size_of::<V>()` * `N` is equal to
    ///   `size_of::<U>() * message.len()`'s next multiple of
    ///   `4` * `NB`, this method performs encryption,
    ///   fills the array `cipher` with the encrypted data, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - If `size_of::<V>()` * `N` is greater than
    ///   `size_of::<U>() * message.len()`'s next multiple of `4` * `NB`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and then fills the rest of the elements of the array
    ///   `cipher` with zeros, and returns the size of the ciphertext including
    ///   padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`size_of::<U>()` * `message.len()` + `1`).next_multiple_of(`4` * `MB`)
    ///   at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_vec_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_vec_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// // Normal case for AES-
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_vec_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
/// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_vec_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// ```
    /// 
    /// # Example 1 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_vec_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// ```
    pub fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_array<U, const N: usize>(&mut self, message: &[U; N], cipher: *mut u8) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #7 in ECB (Electronic CodeBook) mode.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to an array `[U; N]` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable pointer to `u8` which is `*mut u8`, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `4` * `NB`,
    ///   and cannot be other than a multiple of `4` * `NB`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`size_of::<U>()` * `N` + `1`).next_multiple_of(`4` * `MB`)
    ///   at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - If `message` is an empty array `[U; 0]` object, only padding bytes
    ///   will be encrypted, and stored in the memory area that starts from `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// ```
    pub fn encrypt_array<U, const N: usize>(&mut self, message: &[U; N], cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, message: &[U; N], cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode, and stores the
    /// encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to an array `[U; N]` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to `Vec<U>` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `4` * `NB`,
    ///   and cannot be other than a multiple of `4` * `NB`.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_array_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_array_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_array_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_array_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_array_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// ```
    pub fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, message: &[U; N], cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, message: &[U; N], cipher: &mut [V; M]) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode, and stores the
    /// encrypted data in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `message` is an immutable reference to an array `[U; N]` object, and
    ///   is the place where the plaintext to be encrypted is stored.
    /// - `cipher` is a mutable reference to an array `[U; N]` object, and
    ///   is the place where the encrypted data will be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output will be at least `4` * `NB`,
    ///   and cannot be other than a multiple of `4` * `NB`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is an empty array `[U; 0]` object, only padding bytes
    ///   will be encrypted, and stored in the array `[V; M]` object `cipher`.
    /// - If `V::size_in_bytes()` * `M` is less than 
    ///   `U::size_in_bytes()` * `N`'s next multiple of `4` * `NB`,
    ///   this method does not perform encryption and returns `zero`.
    /// - If `V::size_in_bytes()` * `M` is equal to
    ///   `U::size_in_bytes()` * `N`'s next multiple of `4` * `NB`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns the size of the ciphertext including
    ///   padding bits in bytes.
    /// - If `V::size_in_bytes()` * `M` is greater than
    ///   `U::size_in_bytes()` * `N`'s next multiple of `4` * `NB`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and then fills the rest of the elements of the
    ///   array `cipher` with zeros, and returns the size of the ciphertext
    ///   including padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`size_of::<U>()` * `message.len()` + `1`).next_multiple_of(`4` * `MB`)
    ///   at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_array_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_array_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_array_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_array_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_array_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// ```
    pub fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, message: &[U; N], cipher: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64;
    /// Decrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic CodeBook) mode.
    /// 
    /// # Arguments
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
    /// - If `length_in_bytes` is greater than `4` * `NB` (which means
    ///   that the original plaintext is surely not empty data) and it returns
    ///   `zero`, you can interpret it that this method surely failed in
    ///   decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be other than any multiple of `4` * `NB`.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_rijndael.decrypt(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_rijndael.decrypt(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    pub fn decrypt(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_into_vec<U>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    /// Decrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic CodeBook) mode, and stores the decrypted data
    /// in `Vec<U>`.
    /// 
    /// # Arguments
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
    /// - If `length_in_bytes` is greater than `4` * `NB` (which means
    ///   that the original plaintext is surely not empty data) and it returns
    ///   `zero`, you can interpret it that this method surely failed in
    ///   decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be other than any multiple of `4` * `NB`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// println!();
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_aes.decrypt_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// println!();
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_aes.decrypt_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "09 74 04 A7 68 1F 4D 00 ED CA 85 39 90 0E A1 AC E1 AC 90 60 D8 C9 BC AB ED 8D 66 89 3F 85 2B 86 3E CE 84 00 57 F2 B9 14 43 FB DD D9 19 70 46 25 B1 F8 7D 38 27 58 A0 C9 5D AE C0 12 7D 6A 7C D0 ");
    /// println!();
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_aes.decrypt_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// println!();
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_rijndael.decrypt_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_rijndael.decrypt_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    pub fn decrypt_into_vec<U>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_into_array<U, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    /// Decrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic CodeBook) mode, and stores the decrypted data
    /// in array `[U; N]`.
    /// 
    /// # Arguments
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
    /// - If `length_in_bytes` is greater than `4` * `NB` (which means
    ///   that the original plaintext is surely not empty data) and it returns
    ///   `zero`, you can interpret it that this method surely failed in
    ///   decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be other than any multiple of `4` * `NB`.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// 
    /// let mut recovered = [0; 64];
    /// let len = a_aes.decrypt_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut recovered = [0; 64];
    /// a_aes.decrypt_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// print!("Ba =\t");
    /// for b in recovered.clone()
    ///     { print!("{:02X} ", b); }
    /// println!();
    /// let mut txt = String::new();
    /// 
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
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut recovered = [0; 64];
    /// a_aes.decrypt_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// 
    /// let mut recovered = [0; 64];
    /// a_rijndael.decrypt_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// 
    /// let mut recovered = [0; 64];
    /// a_rijndael.decrypt_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    pub fn decrypt_into_array<U, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    /// Decrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic CodeBook) mode, and stores the decrypted data
    /// in a `String`.
    /// 
    /// # Arguments
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
    /// - If `length_in_bytes` is greater than `4` * `NB` (which means
    ///   that the original plaintext is surely not empty data) and it returns
    ///   `zero`, you can interpret it that this method surely failed in
    ///   decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be other than any multiple of `4` * `NB`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// 
    /// let mut converted = String::new();
    /// a_aes.decrypt_into_string(cipher.as_ptr(), cipher.len() as u64, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut converted = String::new();
    /// a_aes.decrypt_into_string(cipher.as_ptr(), cipher.len() as u64, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut converted = String::new();
    /// a_aes.decrypt_into_string(cipher.as_ptr(), cipher.len() as u64, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// 
    /// let mut converted= String::new();
    /// a_rijndael.decrypt_into_string(cipher.as_ptr(), cipher.len() as u64, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// 
    /// let mut converted= String::new();
    /// a_rijndael.decrypt_into_string(cipher.as_ptr(), cipher.len() as u64, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    pub fn decrypt_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_vec<U>(&mut self, cipher: &Vec<U>, message: *mut u8) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode.
    /// 
    /// # Arguments
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
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `4` * `NB`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `size_of::<U>()` * `cipher.len()` cannot be other than any multiple
    ///   of `4` * `NB`.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt_vec(&cipher, recovered.as_mut_ptr());
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt_vec(&cipher, recovered.as_mut_ptr());
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
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt_vec(&cipher, recovered.as_mut_ptr());
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
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_rijndael.decrypt_vec(&cipher, recovered.as_mut_ptr());
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
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_rijndael.decrypt_vec(&cipher, recovered.as_mut_ptr());
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
    pub fn decrypt_vec<U>(&mut self, cipher: &Vec<U>, message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_vec_into_vec<U, V>(&mut self, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode, and
    /// stores the decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
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
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `4` * `NB`
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_aes.decrypt_vec_into_vec(&cipher, &mut recovered);
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_aes.decrypt_vec_into_vec(&cipher, &mut recovered);
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
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_aes.decrypt_vec_into_vec(&cipher, &mut recovered);
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
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_rijndael.decrypt_vec_into_vec(&cipher, &mut recovered);
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
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_rijndael.decrypt_vec_into_vec(&cipher, &mut recovered);
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
    pub fn decrypt_vec_into_vec<U, V>(&mut self, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode, and
    /// stores the decrypted data in array `[V; N]`.
    /// 
    /// # Arguments
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
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `4` * `NB`
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// 
    /// let mut recovered = [0; 64];
    /// let len = a_aes.decrypt_vec_into_array(&cipher, &mut recovered);
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut recovered = [0; 64];
    /// a_aes.decrypt_vec_into_array(&cipher, &mut recovered);
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
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut recovered = [0; 64];
    /// a_aes.decrypt_vec_into_array(&cipher, &mut recovered);
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
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// 
    /// let mut recovered = [0; 64];
    /// a_rijndael.decrypt_vec_into_array(&cipher, &mut recovered);
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
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// 
    /// let mut recovered = [0; 64];
    /// a_rijndael.decrypt_vec_into_array(&cipher, &mut recovered);
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
    pub fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_vec_into_string(&mut self, cipher: &str, message: &mut String) -> u64
    /// Decrypts the data in `str` with the padding defined according to
    /// PKCS #7 in ECB (Electronic CodeBook) mode, and stores the
    /// decrypted data in `String`.
    /// 
    /// # Arguments
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
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `4` * `NB`
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// 
    /// let mut converted= String::new();
    /// a_aes.decrypt_vec_into_string(&cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut converted= String::new();
    /// a_aes.decrypt_vec_into_string(&cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut converted= String::new();
    /// a_aes.decrypt_vec_into_string(&cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// 
    /// let mut converted= String::new();
    /// a_rijndael.decrypt_vec_into_string(&cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// 
    /// let mut converted= String::new();
    /// a_rijndael.decrypt_vec_into_string(&cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    pub fn decrypt_vec_into_string<U>(&mut self, cipher: &Vec<U>, message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_array<U, const N: usize>(&mut self, cipher: &[U; N], message: *mut u8) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #7 in ECB (Electronic CodeBook) mode.
    /// 
    /// # Arguments
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
    /// - If `size_of::<U>()` * `N` is greater than `4` * `NB`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_array_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `size_of::<U>()` * `N` cannot be other than any multiple
    ///   of `4` * `NB`.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt_array(&cipher, recovered.as_mut_ptr());
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt_array(&cipher, recovered.as_mut_ptr());
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
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt_array(&cipher, recovered.as_mut_ptr());
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
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_rijndael.decrypt_array(&cipher, recovered.as_mut_ptr());
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
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_rijndael.decrypt_array(&cipher, recovered.as_mut_ptr());
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
    pub fn decrypt_array<U, const N: usize>(&mut self, cipher: &[U; N], message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, cipher: &[U; N], message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #7 in ECB (Electronic CodeBook) mode,
    /// and stores the decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
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
    /// - If `size_of::<U>()` * `N` is greater than `4` * `NB`
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt_array_into_vec(&cipher, &mut recovered);
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt_array_into_vec(&cipher, &mut recovered);
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
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_aes.decrypt_array_into_vec(&cipher, &mut recovered);
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
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_rijndael.decrypt_array_into_vec(&cipher, &mut recovered);
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
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_rijndael.decrypt_array_into_vec(&cipher, &mut recovered);
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
    pub fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, cipher: &[U; N], message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, cipher: &[U; N], message: &mut [V; M]) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #7 in ECB (Electronic CodeBook) mode,
    /// and stores the decrypted data in array `[V; M]`.
    /// 
    /// # Arguments
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
    /// - If `size_of::<U>()` * `N` is greater than `4` * `NB`
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// 
    /// let mut recovered = [0; 64];
    /// let len = a_aes.decrypt_array_into_array(&cipher, &mut recovered);
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut recovered = [0; 64];
    /// let len = a_aes.decrypt_array_into_array(&cipher, &mut recovered);
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
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut recovered = [0; 64];
    /// let len = a_aes.decrypt_array_into_array(&cipher, &mut recovered);
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
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// 
    /// let mut recovered = [0; 64];
    /// let len = a_rijndael.decrypt_array_into_array(&cipher, &mut recovered);
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
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// 
    /// let mut recovered = [0; 64];
    /// let len = a_rijndael.decrypt_array_into_array(&cipher, &mut recovered);
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
    pub fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, cipher: &[U; N], message: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_array_into_string<U, const N: usize>(&mut self, cipher: &[U; N], message: &mut String) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #7 in ECB (Electronic CodeBook) mode,
    /// and stores the decrypted data in `String`.
    /// 
    /// # Arguments
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
    /// - If `size_of::<U>()` * `N` is greater than `4` * `NB`
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "E8 96 2E 8D BA 45 C6 D9 45 06 DC 98 D9 2F 1F 86 B8 05 A9 B8 D2 B2 1E 82 DA 51 DA B1 F9 81 B7 B3 95 C5 46 72 A8 D9 D8 B0 9E 62 AB 4F F8 36 31 54 5B D2 91 45 68 4B B4 C0 70 D1 8E C0 F3 FF 85 D1 ");
    /// 
    /// let mut converted = String::new();
    /// a_aes.decrypt_array_into_string(&cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut converted = String::new();
    /// a_aes.decrypt_array_into_string(&cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "F2 8F C2 CA 9D 3A AC 96 69 7B 52 0D 5F CB 10 4B 31 26 61 63 CE A4 EB 2B EE 9F C3 4C A1 40 F4 FF 0F 70 8E 7C 6F 42 BC 93 4F CF DA 45 BB F6 6F 6B A9 4F E6 98 36 61 FF 71 9C FF AB 3A 34 D7 BE 3C ");
    /// 
    /// let mut converted = String::new();
    /// a_aes.decrypt_array_into_string(&cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, ECB_PKCS7 };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9A 05 BF D4 01 B1 D9 A1 31 C5 13 A9 D1 50 83 C6 E4 F7 B9 40 E2 29 59 E2 42 96 3A DC 22 65 88 F7 33 86 40 28 99 94 21 6D 08 7F 9B B5 40 C4 D3 63 88 A2 67 DB 24 E1 6F 26 49 13 DE 4C A4 B9 4F 3B ");
    /// 
    /// let mut converted = String::new();
    /// a_rijndael.decrypt_array_into_string(&cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, ECB_PKCS7 };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F E1 B9 6A AB F5 B2 51 9A 0F 19 B3 1A 8C 66 14 5D 16 75 CD 9A 3B 46 5E B3 ED 9D 19 C1 22 D4 86 45 E2 31 AE 0D 1E EF A0 97 6C 2C 45 43 66 D5 16 88 1E 62 0B 06 AB 0B 7A 12 17 F7 B0 C4 93 FB 2F ");
    /// 
    /// let mut converted= String::new();
    /// a_rijndael.decrypt_array_into_string(&cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    pub fn decrypt_array_into_string<U, const N: usize>(&mut self, cipher: &[U; N], message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }
}
