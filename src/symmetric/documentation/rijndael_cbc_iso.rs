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


/// traits_cbc_with_padding_iso.rs was too big because of documentation and
/// plenty of examples So, in order to provide documentation for rijndael or aes
/// without `docs.rs`'s failing generating documentation, dummy codes were made
/// and documentation and examples were moved to rijndael_cbc.rs. And, most of
/// generic parameters are omitted. It is not actual code but dummy code for
/// compilation!!!
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

/// traits_cbc_with_padding_iso.rs was too big because of documentation and
/// plenty of examples So, in order to provide documentation for rijndael or aes
/// without `docs.rs`'s failing generating documentation, dummy codes were made
/// and documentation and examples were moved to rijndael_cbc.rs. And, most of
/// generic parameters are omitted. It is not actual code but dummy code for
/// compilation!!!
impl <const ROUND: usize, const NB: usize, const NK: usize>
Rijndael_Generic<ROUND, NB, NK>
{
    // fn encrypt(&mut self, iv: [u32; NB], message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;
    /// Encrypts the data with the padding defined according to ISO 7816-4
    /// in CBC (Cipher-Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `message` is a pointer to u8 which is `*const u8`,
    ///   and is the plaintext to be encrypted.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the plaintext `message` in bytes.
    /// - `cipher` is a pointer to u8 which is `*mut u8`,
    ///   and is the ciphertext to be stored.
    /// - The size of the memory area which starts at `cipher` and the
    ///   ciphertext will be stored at is assumed to be enough.
    /// - The size of the area for ciphertext should be prepared to be:
    ///   (`length_in_bytes` + 1).next_multiple_of(32 * `NB`) at least when `T` is `[u32; NB]`.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - When `T` is `[u32; NB]` for Rijndael or AES, the output should be at
    ///   least `32 * NB`, and will be only any multiple of `32 * NB`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as
    ///   encrypt_*_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null string.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
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
    ///     { print!("{:02X}", c); }
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
    /// use cryptocol::symmetric::{ Rijndael_256_256, CBC_ISO };
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
    /// assert_eq!(txt, "C2 C4 1C 91 EE 50 F0 04 B6 73 00 B2 81 05 01 25 C1 87 24 27 7E CE 01 65 C5 CB 87 38 99 9F 5B 0C D1 DF 8D 52 C4 C4 C8 D8 F5 D5 AD F3 FD DA 35 C2 33 F6 5D 83 02 85 F1 20 8C 56 0B 72 9C 91 84 42 ");
    /// ```
    fn encrypt(&mut self, iv: [u32; NB], message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    fn decrypt(&mut self, iv: [u32; NB], cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

}
