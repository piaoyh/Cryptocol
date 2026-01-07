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


/// trait_cbc_with_padding_iso.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to rijndael_cbc_iso.rs. And, most of
/// generic parameters are omitted. It is not actual code but dummy code for
/// compilation!!!
#[allow(non_camel_case_types)]
pub struct Rijndael_Generic<const NB: usize = 4, const NK: usize = 4>
{
    // Dummy struct for documentation
}

/// trait_cbc_with_padding_iso.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to rijndael_cbc_iso.rs. And, most of
/// generic parameters are omitted. It is not actual code but dummy code for
/// compilation!!!
impl <const NB: usize, const NK: usize> Rijndael_Generic<NB, NK>
{
    // pub fn encrypt(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;
    /// Encrypts the data with the padding defined according to ISO 7816-4
    /// in CBC (Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - If this method failed in encryption,
    ///   this method returns `zero`.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
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
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// ```
    pub fn encrypt(&mut self, iv: [u32; NB], message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_into_vec<U>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data with the padding defined according to ISO 7816-4
    /// in CBC (Cipher-Block Chaining) mode, and stores the encrypted data
    /// in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - If this method failed in encryption,
    ///   this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_*_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is a null string.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the `Vec<U>` object which is referred to as `cipher`.
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// ```
    pub fn encrypt_into_vec<U>(&mut self, iv: [u32; NB], message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_into_array<U, const N: usize>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    /// Encrypts the data with the padding defined according to ISO 7816-4
    /// in CBC (Cipher-Block Chaining) mode, and stores the encrypted data
    /// in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// ```
    pub fn encrypt_into_array<U, const N: usize>(&mut self, iv: [u32; NB], message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_str(&mut self, iv: T, message: &str, cipher: *mut u8) -> u64
    /// Encrypts the data in a `str` object with the padding defined
    /// according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// ```
    pub fn encrypt_str(&mut self, iv: [u32; NB], message: &str, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_str_into_vec<U>(&mut self, iv: T, message: &str, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data in `str` with the padding defined according to
    /// ISO 7816-4 in CBC (Cipher-Block Chaining) mode, and stores the
    /// encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// ```
    pub fn encrypt_str_into_vec<U>(&mut self, iv: [u32; NB], message: &str, cipher: &mut Vec<U>) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_str_into_array<U, const N: usize>(&mut self, iv: T, message: &str, cipher: &mut [U; N]) -> u64
    /// Encrypts the data in a `str` object with the padding defined
    /// according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode,
    /// and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
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
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// ```
    pub fn encrypt_str_into_array<U, const N: usize>(&mut self, iv: [u32; NB], message: &str, cipher: &mut [U; N]) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_string(&mut self, iv: T, message: &String, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `String` object with the padding
    /// according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_string(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_string(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// ```
    pub fn encrypt_string(&mut self, iv: [u32; NB], message: &String, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_string_into_vec<U>(&mut self, iv: T, message: &String, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data stored in a `String` object with the padding
    /// according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode,
    /// and stores the encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// a_aes.encrypt_string_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_string_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_string_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// ```
    /// 
    /// # Example 4 for ijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_string_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_string_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// ```
    pub fn encrypt_string_into_vec<U>(&mut self, iv: [u32; NB], message: &String, cipher: &mut Vec<U>) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_string_into_array<U, const N: usize>(&mut self, iv: T, message: &String, cipher: &mut [U; N]) -> u64
    /// Encrypts the data stored in a `String` object with the padding
    /// according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode,
    /// and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, CBC_ISO };
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
    /// a_aes.encrypt_string_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_string_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_aes.encrypt_string_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_string_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_string_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// ```
    pub fn encrypt_string_into_array<U, const N: usize>(&mut self, iv: [u32; NB], message: &String, cipher: &mut [U; N]) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_vec<U>(&mut self, iv: T, message: &Vec<U>, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// ```
    pub fn encrypt_vec<U>(&mut self, iv: [u32; NB], message: &Vec<U>, cipher: *mut u8) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_vec_into_vec<U, V>(&mut self, iv: T, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode, and
    /// stores the encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_vec_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_vec_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// ```
    pub fn encrypt_vec_into_vec<U, V>(&mut self, iv: [u32; NB], message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: T, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode, and
    /// stores the encrypted data in array `[V; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// // Normal case for AES-
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
/// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_vec_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// ```
    /// 
    /// # Example 1 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_vec_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// ```
    pub fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: [u32; NB], message: &Vec<U>, cipher: &mut [V; N]) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_array<U, const N: usize>(&mut self, iv: T, message: &[U; N], cipher: *mut u8) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// ```
    pub fn encrypt_array<U, const N: usize>(&mut self, iv: [u32; NB], message: &[U; N], cipher: *mut u8) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: T, message: &[U; N], cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode, and stores the
    /// encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_array_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_array_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// ```
    pub fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: [u32; NB], message: &[U; N], cipher: &mut Vec<V>) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: T, message: &[U; N], cipher: &mut [V; M]) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode, and stores the
    /// encrypted data in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_array_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(mes.as_bytes());
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_array_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// ```
    pub fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: [u32; NB], message: &[U; N], cipher: &mut [V; M]) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64;
    /// Decrypts the data with the padding defined according to ISO 7816-4
    /// in CBC (Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
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
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
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
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_rijndael.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt(iv.clone(), message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_rijndael.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    pub fn decrypt(&mut self, iv: [u32; NB], cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_into_vec<U>(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    /// Decrypts the data with the padding defined according to ISO 7816-4
    /// in CBC (Cipher-Block Chaining) mode, and stores the decrypted data
    /// in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
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
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
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
    /// println!();
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_rijndael.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt(iv.clone(), message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_rijndael.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    pub fn decrypt_into_vec<U>(&mut self, iv: [u32; NB], cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_into_array<U, const N: usize>(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    /// Decrypts the data with the padding defined according to ISO 7816-4
    /// in CBC (Cipher-Block Chaining) mode, and stores the decrypted data
    /// in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// 
    /// let mut recovered = [0; 64];
    /// a_aes.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// 
    /// let mut recovered = [0; 64];
    /// a_aes.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// 
    /// let mut recovered = [0; 64];
    /// a_rijndael.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// 
    /// let mut recovered = [0; 64];
    /// a_rijndael.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    pub fn decrypt_into_array<U, const N: usize>(&mut self, iv: [u32; NB], cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_into_string(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    /// Decrypts the data with the padding defined according to ISO 7816-4
    /// in CBC (Cipher-Block Chaining) mode, and stores the decrypted data
    /// in a `String`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// 
    /// let mut converted= String::new();
    /// a_aes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// 
    /// let mut converted= String::new();
    /// a_aes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// 
    /// let mut converted= String::new();
    /// a_aes.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// 
    /// let mut converted= String::new();
    /// a_rijndael.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut converted);
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// 
    /// let mut converted= String::new();
    /// a_rijndael.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    pub fn decrypt_into_string(&mut self, iv: [u32; NB], cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_vec<U>(&mut self, iv: T, cipher: &Vec<U>, message: *mut u8) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
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
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
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
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_rijndael.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_rijndael.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    pub fn decrypt_vec<U>(&mut self, iv: [u32; NB], cipher: &Vec<U>, message: *mut u8) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_vec_into_vec<U, V>(&mut self, iv: T, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode, and
    /// stores the decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
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
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
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
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_rijndael.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_rijndael.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    pub fn decrypt_vec_into_vec<U, V>(&mut self, iv: [u32; NB], cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: T, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode, and
    /// stores the decrypted data in array `[V; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// 
    /// let mut recovered = [0; 64];
    /// a_aes.decrypt_vec_into_array(iv, &cipher, &mut recovered);
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
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// 
    /// let mut recovered = [0; 64];
    /// a_aes.decrypt_vec_into_array(iv, &cipher, &mut recovered);
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
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// 
    /// let mut recovered = [0; 64];
    /// a_rijndael.decrypt_vec_into_array(iv, &cipher, &mut recovered);
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// 
    /// let mut recovered = [0; 64];
    /// a_rijndael.decrypt_vec_into_array(iv, &cipher, &mut recovered);
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
    pub fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: [u32; NB], cipher: &Vec<U>, message: &mut [V; N]) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_vec_into_string(&mut self, iv: T, cipher: &str, message: &mut String) -> u64
    /// Decrypts the data in `str` with the padding defined according to
    /// ISO 7816-4 in CBC (Cipher-Block Chaining) mode, and stores the
    /// decrypted data in `String`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// 
    /// let mut converted= String::new();
    /// a_aes.decrypt_vec_into_string(iv, &cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// 
    /// let mut converted= String::new();
    /// a_aes.decrypt_vec_into_string(iv, &cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// 
    /// let mut converted= String::new();
    /// a_aes.decrypt_vec_into_string(iv, &cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// 
    /// let mut converted= String::new();
    /// a_rijndael.decrypt_vec_into_string(iv, &cipher, &mut converted);
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// 
    /// let mut converted= String::new();
    /// a_rijndael.decrypt_vec_into_string(iv, &cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    pub fn decrypt_vec_into_string<U>(&mut self, iv: [u32; NB], cipher: &Vec<U>, message: &mut String) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_array<U, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: *mut u8) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
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
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
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
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_rijndael.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_rijndael.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
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
    pub fn decrypt_array<U, const N: usize>(&mut self, iv: [u32; NB], cipher: &[U; N], message: *mut u8) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode,
    /// and stores the decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
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
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
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
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_rijndael.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_rijndael.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    pub fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: [u32; NB], cipher: &[U; N], message: &mut Vec<V>) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut [V; M]) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode,
    /// and stores the decrypted data in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
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
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
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
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
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
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// 
    /// let mut recovered = [0; 64];
    /// let len = a_rijndael.decrypt_array_into_array(iv, &cipher, &mut recovered);
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// 
    /// let mut recovered = [0; 64];
    /// let len = a_rijndael.decrypt_array_into_array(iv, &cipher, &mut recovered);
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
    pub fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: [u32; NB], cipher: &[U; N], message: &mut [V; M]) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn decrypt_array_into_string<U, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut String) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to ISO 7816-4 in CBC (Cipher-Block Chaining) mode,
    /// and stores the decrypted data in `String`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for CBC mode.
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, CBC_ISO };
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
    /// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 EC D3 12 F1 1A 78 77 A6 A5 CB 73 AB 65 22 5B 7D ");
    /// 
    /// let mut converted= String::new();
    /// a_aes.decrypt_array_into_string(iv, &cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..24
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_192::new_with_key(&key);
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
    /// assert_eq!(txt, "A1 74 C3 56 DD 37 DD D0 56 AD 49 57 09 E8 3E 9C BD 53 61 64 FC 38 20 D9 14 FD 7B 4B C3 49 8C 03 6E 18 D3 28 EC 16 00 CA 36 07 35 6A AD 4F 32 FB 3C 06 AC E0 0A 23 62 86 32 18 30 5B 2D DE 28 9B ");
    /// 
    /// let mut converted= String::new();
    /// a_aes.decrypt_array_into_string(iv, &cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_aes = AES_256::new_with_key(&key);
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
    /// assert_eq!(txt, "6F 4E AB DE A3 9C 7C EA 7D 02 D7 51 22 1E 17 63 5E 41 51 D8 DB 27 21 0E 69 F0 21 49 04 AE B6 F7 D5 BC 1E 2D 1F 77 60 4E 7A 8F 35 C3 6A 57 C1 81 82 00 CA D0 33 66 79 E7 66 57 CB 46 39 93 B6 A4 ");
    /// 
    /// let mut converted= String::new();
    /// a_aes.decrypt_array_into_string(iv, &cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
    /// 
    /// let key = [0x12_u8, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    /// print!("K =\t");
    /// for i in 0..32
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_256_256::new_with_key(&key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be(), iv[4].to_be(), iv[5].to_be(), iv[6].to_be(), iv[7].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A3 73 85 5F B4 73 BC 49 2C 9D D7 22 EE 13 27 99 38 E4 9E 02 CA ED AB 81 81 31 B9 5C F2 3D C2 01 06 0D FD 0A 88 F1 5B 2B 85 93 CB 95 9B 89 8B ED D6 81 9E E4 CA 74 EF B4 BE BE 7D AF 87 81 47 AC ");
    /// 
    /// let mut converted= String::new();
    /// a_rijndael.decrypt_array_into_string(iv, &cipher, &mut converted);
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
    /// 
    /// let mut sha3 = SHA3_512::new();
    /// sha3.absorb_str("Post-quantum");
    /// let key: [u8; 64] = sha3.get_hash_value_in_array();
    /// print!("K =\t");
    /// for i in 0..64
    ///     { print!("{:02X}", key[i]); }
    /// println!();
    /// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
    /// sha3.absorb_str("Initialize");
    /// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
    /// iv.src = sha3.get_hash_value_in_array();
    /// let iv = unsafe { iv.des };
    /// print!("IV =\t");
    /// for i in 0..16
    ///     { print!("{:08X}", iv[i].to_be()); }
    /// println!();
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 64];
    /// a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B F6 E2 EC CA F7 6A 69 44 8E 22 06 B0 0C DD C7 FF 8B BB A7 03 11 E1 9C 41 40 A0 B6 B3 40 5C 4B DF 2C 01 C2 97 E1 3E 71 F4 30 CB 9D B7 8B 6F 67 43 01 1E D5 50 C1 BE 68 14 CE 9C F7 8B 14 61 FB ");
    /// 
    /// let mut converted= String::new();
    /// a_rijndael.decrypt_array_into_string(iv, &cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    pub fn decrypt_array_into_string<U, const N: usize>(&mut self, iv: [u32; NB], cipher: &[U; N], message: &mut String) -> u64
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }
}
