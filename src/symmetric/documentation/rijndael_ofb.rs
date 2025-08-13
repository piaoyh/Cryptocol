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
use crate::number::{ SmallUInt, IntUnion };


/// trait_ofb.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to rijndael_ofb.rs. And, most of generic parameters
/// are omitted. It is not actual code but dummy code for compilation!!!
#[allow(non_camel_case_types)]
pub struct Rijndael_Generic<const ROUND: usize = 10, const NB: usize = 4, const NK: usize = 4>
{
    // Dummy struct for documentation
    key:        [IntUnion; NK],
    block:      [[u8; NB]; 4],
    round_key:  Vec<[IntUnion; NB]>,
    enc:        fn (s: &mut Self, message: &[IntUnion; NB]) -> [IntUnion; NB],
    dec:        fn (s: &mut Self, cipher: &[IntUnion; NB]) -> [IntUnion; NB],
}

/// trait_ofb.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to rijndael_ofb.rs. And, most of generic parameters
/// are omitted. It is not actual code but dummy code for compilation!!!
impl <const ROUND: usize, const NB: usize, const NK: usize>
Rijndael_Generic<ROUND, NB, NK>
{
    // fn encrypt(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;
    /// Encrypts the data without any padding in OFB (Output feedback) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// ```
    pub fn encrypt(&mut self, iv: [u32; NB], message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_into_vec<U>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data without any padding in OFB (Output feedback) mode,
    /// and stores the encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
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
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// ```
    pub fn encrypt_into_vec<U>(&mut self, iv: [u32; NB], message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_into_array<U, const N: usize>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    /// Encrypts the data without any padding in OFB (Output feedback) mode,
    /// and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// ```
    pub fn encrypt_into_array<U, const N: usize>(&mut self, iv: [u32; NB], message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_str(&mut self, iv: T, message: &str, cipher: *mut u8) -> u64
    /// Encrypts the data in a `str` object without any padding in CFB (Cipher
    /// FeedBack) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// ```
    pub fn encrypt_str(&mut self, iv: [u32; NB], message: &str, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_str_into_vec<U>(&mut self, iv: T, message: &str, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data in `str` without any padding in OFB (Output feedback)
    /// mode, and stores the encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
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
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// ```
    pub fn encrypt_str_into_vec<U>(&mut self, iv: [u32; NB], message: &str, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_str_into_array<U, const N: usize>(&mut self, iv: T, message: &str, cipher: &mut [U; N]) -> u64
    /// Encrypts the data in a `str` object without any padding in CFB (Cipher
    /// FeedBack) mode, and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// ```
    pub fn encrypt_str_into_array<U, const N: usize>(&mut self, iv: [u32; NB], message: &str, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_string(&mut self, iv: T, message: &String, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `String` object without any padding
    /// in OFB (Output feedback) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_string(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_string(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_string(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_string(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_string(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// ```
    pub fn encrypt_string(&mut self, iv: [u32; NB], message: &String, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_string_into_vec<U>(&mut self, iv: T, message: &String, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data stored in a `String` object without any padding in
    /// OFB (Output feedback) mode, and stores the encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
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
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// a_aes.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
    /// ```
    /// 
    /// # Example 4 for ijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// a_rijndael.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// ```
    pub fn encrypt_string_into_vec<U>(&mut self, iv: [u32; NB], message: &String, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_string_into_array<U, const N: usize>(&mut self, iv: T, message: &String, cipher: &mut [U; N]) -> u64
    /// Encrypts the data stored in a `String` object without any padding in CFB
    /// (Cipher FeedBack) mode, and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, AES_192, AES_256, Rijndael_256_256, Rijndael_512_512, OFB };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// ```
    pub fn encrypt_string_into_array<U, const N: usize>(&mut self, iv: [u32; NB], message: &String, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_vec<U>(&mut self, iv: T, message: &Vec<U>, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `Vec<U>` object without any padding
    /// in OFB (Output feedback) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// ```
    pub fn encrypt_vec<U>(&mut self, iv: [u32; NB], message: &Vec<U>, cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_vec_into_vec<U, V>(&mut self, iv: T, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in a `Vec<U>` object without any padding in
    /// OFB (Output feedback) mode, and stores the encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
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
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// ```
    pub fn encrypt_vec_into_vec<U, V>(&mut self, iv: [u32; NB], message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: T, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    /// Encrypts the data stored in a `Vec<U>` object without any padding in CFB
    /// (Cipher FeedBack) mode, and stores the encrypted data in array `[V; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_vec_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_vec_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_vec_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_vec_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
    /// ```
    /// 
    /// # Example 1 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_vec_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// ```
    pub fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: [u32; NB], message: &Vec<U>, cipher: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_array<U, const N: usize>(&mut self, iv: T, message: &[U; N], cipher: *mut u8) -> u64
    /// Encrypts the data stored in an array `[U; N]` object without any
    /// padding in OFB (Output feedback) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// ```
    pub fn encrypt_array<U, const N: usize>(&mut self, iv: [u32; NB], message: &[U; N], cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: T, message: &[U; N], cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in an array `[U; N]` object without any padding
    /// in OFB (Output feedback) mode, and stores the encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_array_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_array_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = Vec::<u8>::new();
    /// a_aes.encrypt_array_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_array_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = Vec::<u8>::new();
    /// a_rijndael.encrypt_array_into_vec(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// ```
    pub fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: [u32; NB], message: &[U; N], cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: T, message: &[U; N], cipher: &mut [V; M]) -> u64
    /// Encrypts the data stored in an array `[U; N]` object without any padding
    /// in OFB (Output feedback) mode, and stores the encrypted data
    /// in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_array_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
    /// ```
    /// 
    /// # Example 2 for AES-192
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_array_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
    /// ```
    /// 
    /// # Example 3 for AES-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_array_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
    /// ```
    /// 
    /// # Example 4 for Rijndael-256-256
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_array_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
    /// ```
    /// 
    /// # Example 5 for Rijndael-512-512 for post-quantum
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::number::SharedArrays;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_array_into_array(iv, &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// ```
    pub fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: [u32; NB], message: &[U; N], cipher: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64;
    /// Decrypts the data without any padding in OFB (Output feedback) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
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
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
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
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
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
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt(iv.clone(), message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
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

    // fn decrypt_into_vec<U>(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    /// Decrypts the data without any padding in OFB (Output feedback) mode,
    /// and stores the decrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
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
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
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
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
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
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt(iv.clone(), message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
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
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_into_array<U, const N: usize>(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    /// Decrypts the data without any padding in OFB (Output feedback) mode,
    /// and stores the decrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
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
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
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
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
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
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
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
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_into_string(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    /// Decrypts the data without any padding in OFB (Output feedback) mode,
    /// and stores the decrypted data in a `String`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
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
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
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
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
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
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str(iv.clone(), &message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
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

    // fn decrypt_vec<U>(&mut self, iv: T, cipher: &Vec<U>, message: *mut u8) -> u64
    /// Decrypts the data stored in a `Vec<U>` object without any padding
    /// in OFB (Output feedback) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
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
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
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
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
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
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
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
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
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
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_vec_into_vec<U, V>(&mut self, iv: T, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in a `Vec<U>` object without any padding in
    /// OFB (Output feedback) mode, and stores the decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
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
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
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
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
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
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
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
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
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
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: T, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    /// Decrypts the data stored in a `Vec<U>` object without any padding in CFB
    /// (Cipher FeedBack) mode, and stores the decrypted data in array `[V; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    ///   `size_of::<U>()` * `cipher.len()` - `1`, this method performs
    ///   decryption, fills the array `message` with the decrypted data, and
    ///   then fills the rest of the elements of the array `message` with zeros,
    ///   and returns the size of the plaintext.
    /// - It is responsible for you to prepare the `message` area big enough!
    /// - The size of the area for plaintext does not have to be prepared more
    ///   than `size_of::<U>()` * `cipher.len()`.
    /// 
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
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
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
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
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
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
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
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
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
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
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_vec_into_string(&mut self, iv: T, cipher: &str, message: &mut String) -> u64
    /// Decrypts the data in `str` without any padding in OFB (Output feedback)
    /// mode, and stores the decrypted data in `String`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
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
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
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
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
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
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
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
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// 
    /// let mut converted= String::new();
    /// a_rijndael.decrypt_vec_into_string(iv, &cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    pub fn decrypt_vec_into_string<U>(&mut self, iv: [u32; NB], cipher: &Vec<U>, message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_array<U, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: *mut u8) -> u64
    /// Decrypts the data stored in an array `[U; N]` object without any padding
    /// in OFB (Output feedback) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
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
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
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
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
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
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
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
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in an array `[U; N]` object without any padding
    /// in OFB (Output feedback) mode, and stores the decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
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
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
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
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
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
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
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
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut [V; M]) -> u64
    /// Decrypts the data stored in an array `[U; N]` object without any padding
    /// in OFB (Output feedback) mode, and stores the decrypted data
    /// in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
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
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
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
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
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
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
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
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_array_into_string<U, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut String) -> u64
    /// Decrypts the data stored in an array `[U; N]` object without any padding
    /// in OFB (Output feedback) mode, and stores the decrypted data in `String`.
    /// 
    /// # Arguments
    /// - `iv` is an initialization vector for OFB mode.
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
    /// # Example 1 for AES-128
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, OFB };
    /// 
    /// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
    /// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B9 AD 7F CB 45 2A B9 31 89 15 16 47 4C A9 F3 D1 37 0B 09 FA 85 70 29 5C 99 3A DB F3 A9 A2 B8 C6 5B 39 CF A6 07 29 23 FF 8B 1E B6 26 29 D6 C8 19 41 A5 4C A0 49 82 F6 ");
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
    /// use cryptocol::symmetric::{ AES_192, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "AF 45 AD 45 36 8B 38 2A 69 F1 50 31 1D F6 90 88 31 2D 9D 2C FD E3 21 61 10 F7 18 81 AB 72 7F 21 37 D0 34 87 EE 62 70 BC 45 89 B2 F7 F3 9F 1D 27 D1 2D 8F FC 73 0D 68 ");
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
    /// use cryptocol::symmetric::{ AES_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_aes.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "9B 2F 4A C9 65 B3 7C 61 E7 DE 9B 97 F1 4C A2 B6 41 17 10 CE 43 B6 36 4A 7C 4C DD E9 73 E5 8F 5E C5 AC B0 0A 00 02 4F 85 A9 37 2C 2D D3 C7 29 F7 45 F1 5C C7 7A D3 B8 ");
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
    /// use cryptocol::symmetric::{ Rijndael_256_256, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A0 84 D5 64 D1 8C FD B4 67 7F 5F 01 DA FF 1F A3 39 F8 F4 66 5C D4 54 87 2C CA 6C 2B AB C3 FD CC A5 C8 17 32 0C 84 F6 A6 71 D4 DA C4 DB 82 8C C7 67 BF CD 28 7C 52 DA ");
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
    /// use cryptocol::symmetric::{ Rijndael_512_512, OFB };
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
    /// let mut cipher = [0_u8; 55];
    /// a_rijndael.encrypt_str_into_array(iv.clone(), &message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "B7 50 B4 3D 3C 4A C7 58 C2 1E 11 33 FD A8 BE AC 3A 56 2F DF 89 F4 57 4F A6 E8 98 61 A6 30 EF 7C 2B 09 2A 18 59 AF AC 5E 31 22 9A E1 8C 4A 63 6F 06 C5 F2 41 23 60 54 ");
    /// 
    /// let mut converted= String::new();
    /// a_rijndael.decrypt_array_into_string(iv, &cipher, &mut converted);
    /// println!("B =\t{}", converted);
    /// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(converted, message);
    /// ```
    pub fn decrypt_array_into_string<U, const N: usize>(&mut self, iv: [u32; NB], cipher: &[U; N], message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }
}
