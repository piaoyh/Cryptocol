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
use crate::number::{ SmallUInt, LongUnion };

/// des.rs was too big because of documentation and plenty of examples
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to des_basic.rs. And, most of generic parameters
/// are omitted. It is not actual code but dummy code for compilation!!!
#[allow(non_camel_case_types)]
pub struct DES_Generic<const ROUND: usize = 16>
{
    // Dummy struct for documentation
    key: LongUnion,
    block: LongUnion,
    round_key: [u64; ROUND],
    enc: fn (s: &mut Self, message: u64) -> u64,
    dec: fn (s: &mut Self, cipher: u64) -> u64,
}

/// des.rs was too big because of documentation and plenty of examples
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to des_basic.rs. And, most of generic parameters
/// are omitted. It is not actual code but dummy code for compilation!!!
impl <const ROUND: usize> DES_Generic<ROUND>
{
    // fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    /// Encrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic Code Book) mode.
    /// 
    /// # Arguments
    /// - `message` is a pointer to u8 which is `*const u8`,
    ///   and is the plaintext to be encrypted.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and the length of the plaintext `message` in bytes.
    /// - `cipher` is a pointer to u8 which is `*mut u8`,
    ///   and is the ciphertext to be stored.
    /// - The size of the memory area which starts at `cipher` and the
    ///   ciphertext will be stored at is assumed to be enough.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`length_in_bytes` + 1).next_multiple_of(8) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output is the size of the ciphertext including padding bits
    ///   in bytes.
    /// - The output should be at least `8`,
    ///   and will be only any multiple of `8`.
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
    /// - The padding bits composed of the bytes that indicate the length of
    ///   the padding bits in bytes accoridng to PKCS#7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher1 = [0_u8; 56];
    /// let mut cipher2 = [0_u8; 56];
    /// c_des.encrypt(message.as_ptr(), message.len() as u64, cipher1.as_mut_ptr());
    /// d_des.encrypt(message.as_ptr(), message.len() as u64, cipher2.as_mut_ptr());
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
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// // Normal case for the message 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 24];
    /// a_des.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");
    /// ```
    pub fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_into_vec<U>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic Code Book) mode, and stores the encrypted data
    /// in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `message` is a pointer to u8 which is `*const u8`,
    ///   and is the plaintext to be encrypted.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the plaintext `message` in bytes.
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output is the size of the ciphertext including padding bits
    ///   in bytes.
    /// - The output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null string.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the `Vec<U>` object `cipher`.
    /// - The padding bits composed of the bytes that indicate the length of
    ///   the padding bits in bytes accoridng to PKCS#7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// println!("K =\t{:#016X}", key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher2);
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
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
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
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
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
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
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
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
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
    /// # Example 8 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// // Normal case for the message of 16 bytes
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");
    /// ```
    pub fn encrypt_into_vec<T>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn encrypt_into_array<T, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    /// Encrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic Code Book) mode, and stores the encrypted data
    /// in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `message` is a pointer to u8 which is `*const u8`,
    ///   and is the plaintext to be encrypted.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the plaintext `message` in bytes.
    /// - `cipher` is an array `[U; N]` object,
    ///   and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null string.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the array `[U; N]` object `cipher`.
    /// - If `N` is less than `length_in_bytes`'s next multiple of 8,
    ///   this method does not perform encryption and returns `zero`.
    /// - If `N` is equal to `length_in_bytes`'s next multiple of 8,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns the size of the ciphertext
    ///   including padding bits in bytes.
    /// - If `N` is greater than `length_in_bytes`'s next multiple of 8,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and then fills the rest of the elements of
    ///   the array `cipher` with zeros, and returns the size of the ciphertext
    ///   including padding bits in bytes.
    /// - The padding bits composed of the bytes that indicate the length of
    ///   the padding bits in bytes accoridng to PKCS#7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher1 = [0_u8; 56];
    /// let mut cipher2 = [0_u8; 56];
    /// c_des.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher2);
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
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
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
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
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
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
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
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
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
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 24];
    /// a_des.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");
    /// ```
    pub fn encrypt_into_array<T, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_str(&mut self, message: &str, cipher: *mut u8) -> u64
    /// Encrypts the data in `str` with the padding defined
    /// according to PKCS #7 in ECB (Electronic Code Book) mode.
    /// 
    /// # Arguments
    /// - `message` is a `str` object, and is the plaintext to be encrypted.
    /// - `cipher` is a pointer to u8 which is `*mut u8`,
    ///   and is the ciphertext to be stored.
    /// - The size of the memory area which starts at `cipher` and the
    ///   ciphertext will be stored at is assumed to be enough.
    /// - The size of the area for ciphertext should be prepared to be:
    ///   (`length_in_bytes` + 1).next_multiple_of(8) at least when `T` is `u64`, and
    ///   (`length_in_bytes` + 1).next_multiple_of(16) at least when `T` is `u128`.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output is the size of the ciphertext including padding bits
    ///   in bytes.
    /// - The output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_str_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - The padding bits composed of the bytes that indicate the length of
    ///   the padding bits in bytes accoridng to PKCS#7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_str(&message, cipher.as_mut_ptr());
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_str(&message, cipher.as_mut_ptr());
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher1 = [0_u8; 56];
    /// let mut cipher2 = [0_u8; 56];
    /// c_des.encrypt_str(&message, cipher1.as_mut_ptr());
    /// d_des.encrypt_str(&message, cipher2.as_mut_ptr());
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
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_str(&message, cipher.as_mut_ptr());
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
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_str(&message, cipher.as_mut_ptr());
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
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_str(&message, cipher.as_mut_ptr());
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
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_str(&message, cipher.as_mut_ptr());
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
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 24];
    /// a_des.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");
    /// ```
    pub fn encrypt_str(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_str_into_vec<U>(&mut self, message: &str, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data in `str` with the padding defined according to PKCS #7
    /// in ECB (Electronic Code Book) mode, and stores the encrypted data
    /// in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `message` is a `str` object, and is the plaintext to be encrypted.
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output is the size of the ciphertext including padding bits
    ///   in bytes.
    /// - The output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - The padding bits composed of the bytes that indicate the length of
    ///   the padding bits in bytes accoridng to PKCS#7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(&message, &mut cipher);
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(&message, &mut cipher);
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_str_into_vec(&message, &mut cipher1);
    /// d_des.encrypt_str_into_vec(&message, &mut cipher2);
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
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(&message, &mut cipher);
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
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(&message, &mut cipher);
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
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(&message, &mut cipher);
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
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(&message, &mut cipher);
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
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");
    /// ```
    pub fn encrypt_str_into_vec<T>(&mut self, message: &str, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_str_into_array<U, const N: usize>(&mut self, message: &str, cipher: &mut [U; N]) -> u64
    /// Encrypts the data in `str` with the padding defined according to PKCS #7
    /// in ECB (Electronic Code Book) mode, and stores the encrypted data
    /// in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `message` is a `str` object, and is the plaintext to be encrypted.
    /// - `cipher` is an array `[U; N]` object,
    ///   and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the array `[U; N]` object `cipher`.
    /// - If `N` is less than `length_in_bytes`'s next multiple of 8,
    ///   this method does not perform encryption and returns `zero`.
    /// - If `N` is equal to `length_in_bytes`'s next multiple of 8,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns the size of the ciphertext
    ///   including padding bits in bytes.
    /// - If `N` is greater than `length_in_bytes`'s next multiple of 8,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and then fills the rest of the elements of
    ///   the array `cipher` with zeros, and returns the size of the ciphertext
    ///   including padding bits in bytes.
    /// - The padding bits composed of the bytes that indicate the length of
    ///   the padding bits in bytes accoridng to PKCS#7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_str_into_array(&message, &mut cipher);
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_str_into_array(&message, &mut cipher);
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher1 = [0_u8; 56];
    /// let mut cipher2 = [0_u8; 56];
    /// c_des.encrypt_str_into_array(&message, &mut cipher1);
    /// d_des.encrypt_str_into_array(&message, &mut cipher2);
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
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_str_into_array(&message, &mut cipher);
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
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_str_into_array(&message, &mut cipher);
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
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_str_into_array(&message, &mut cipher);
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
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_str_into_array(&message, &mut cipher);
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
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 24];
    /// a_des.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.encrypt_str_into_array)
    pub fn encrypt_str_into_array<T, const N: usize>(&mut self, message: &str, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_string(&mut self, message: &String, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `string` object with the padding defined
    /// according to PKCS #7 in ECB (Electronic Code Book) mode.
    /// 
    /// # Arguments
    /// - `message` is a `string` object, and is the plaintext to be encrypted.
    /// - `cipher` is a pointer to u8 which is `*mut u8`,
    ///   and is the ciphertext to be stored.
    /// - The size of the memory area which starts at `cipher` and the
    ///   ciphertext will be stored at is assumed to be enough.
    /// - The size of the area for ciphertext should be prepared to be:
    ///   (`length_in_bytes` + 1).next_multiple_of(8) at least when `T` is `u64`, and
    ///   (`length_in_bytes` + 1).next_multiple_of(16) at least when `T` is `u128`.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output is the size of the ciphertext including padding bits
    ///   in bytes.
    /// - The output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as
    ///   encrypt_string_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `message` is a null string String::new(), only padding bytes will
    ///   be encrypted, and stored in the memory area that starts from `cipher`.
    /// - The padding bits composed of the bytes that indicate the length of
    ///   the padding bits in bytes accoridng to PKCS#7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_string(&message, cipher.as_mut_ptr());
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_string(&message, cipher.as_mut_ptr());
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
    /// # Example 3 for EExpanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher1 = [0_u8; 56];
    /// let mut cipher2 = [0_u8; 56];
    /// c_des.encrypt_string(&message, cipher1.as_mut_ptr());
    /// d_des.encrypt_string(&message, cipher2.as_mut_ptr());
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
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_string(&message, cipher.as_mut_ptr());
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
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// // 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "7 bytes".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_string(&message, cipher.as_mut_ptr());
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
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "I am OK.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_string(&message, cipher.as_mut_ptr());
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
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "PARK Youngho".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_string(&message, cipher.as_mut_ptr());
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
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "고맙습니다.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 24];
    /// a_des.encrypt_string(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");
    /// ```
    pub fn encrypt_string(&mut self, message: &String, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_string_into_vec<U>(&mut self, message: &String, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data stored in a `string` object with the padding according
    /// to PKCS #7 in ECB (Electronic Code Book) mode, and stores the encrypted
    /// data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `message` is a `string` object, and is the plaintext to be encrypted.
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output is the size of the ciphertext including padding bits
    ///   in bytes.
    /// - The output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is a null string String::new(), only padding bytes will
    ///   be encrypted, and stored in the memory area that starts from `cipher`.
    /// - The padding bits composed of the bytes that indicate the length of
    ///   the padding bits in bytes accoridng to PKCS#7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_string_into_vec(&message, &mut cipher);
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_string_into_vec(&message, &mut cipher);
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_string_into_vec(&message, &mut cipher1);
    /// d_des.encrypt_string_into_vec(&message, &mut cipher2);
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
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_string_into_vec(&message, &mut cipher);
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
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "7 bytes".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_string_into_vec(&message, &mut cipher);
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
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "I am OK.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_string_into_vec(&message, &mut cipher);
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
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "PARK Youngho".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_string_into_vec(&message, &mut cipher);
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
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "고맙습니다.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_string_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");
    /// ```
    pub fn encrypt_string_into_vec<T>(&mut self, message: &String, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_string_into_array<U, const N: usize>(&mut self, message: &String, cipher: &mut [U; N]) -> u64
    /// Encrypts the data stored in a `string` object with the padding according
    /// to PKCS #7 in ECB (Electronic Code Book) mode, and stores the encrypted
    /// data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `message` is a `string` object, and is the plaintext to be encrypted.
    /// - `cipher` is an array `[U; N]` object,
    ///   and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - The output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is a null string String::new(), only padding bytes will
    ///   be encrypted, and stored in the array `[U; N]` object `cipher`.
    /// - If `N` is less than `length_in_bytes`'s next multiple of 8,
    ///   this method does not perform encryption and returns `zero`.
    /// - If `N` is equal to `length_in_bytes`'s next multiple of 8,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns the size of the ciphertext
    ///   including padding bits in bytes.
    /// - If `N` is greater than `length_in_bytes`'s next multiple of 8,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and then fills the rest of the elements of
    ///   the array `cipher` with zeros, and returns the size of the ciphertext
    ///   including padding bits in bytes.
    /// - The padding bits composed of the bytes that indicate the length of
    ///   the padding bits in bytes accoridng to PKCS#7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_string_into_array(&message, &mut cipher);
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_string_into_array(&message, &mut cipher);
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
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher1 = [0_u8; 56];
    /// let mut cipher2 = [0_u8; 56];
    /// c_des.encrypt_string_into_array(&message, &mut cipher1);
    /// d_des.encrypt_string_into_array(&message, &mut cipher2);
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
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_string_into_array(&message, &mut cipher);
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
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "7 bytes".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt_string_into_array(&message, &mut cipher);
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
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "I am OK.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_string_into_array(&message, &mut cipher);
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
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "PARK Youngho".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt_string_into_array(&message, &mut cipher);
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
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "고맙습니다.".to_string();
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 24];
    /// a_des.encrypt_string_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "20 83 6B 12 1D 3A 5D BA 4D D6 5F 5A 8E 2E AC E7 41 7F 89 79 08 CD A1 4C ");
    /// ```
    pub fn encrypt_string_into_array<T, const N: usize>(&mut self, message: &String, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    
    pub fn encrypt_vec<T>(&mut self, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }
    
    pub fn encrypt_vec_into_vec<T, U>(&mut self, message: &Vec<T>, cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_vec_into_array<T, U, const N: usize>(&mut self, message: &Vec<T>, cipher: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_array<T, const N: usize>(&mut self, message: &[T; N], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }
    
    pub fn encrypt_array_into_vec<T, U, const N: usize>(&mut self, message: &[T; N], cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_array_into_array<T, U, const N: usize, const M: usize>(&mut self, message: &[T; N], cipher: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }



    pub fn decrypt(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_into_vec<T>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_into_array<T, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec<T>(&mut self, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_into_vec<T, U>(&mut self, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_into_array<T, U, const N: usize>(&mut self, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_into_string<T>(&mut self, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array<T, const N: usize>(&mut self, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_into_vec<T, U, const N: usize>(&mut self, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_into_array<T, U, const N: usize, const M: usize>(&mut self, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_into_string<T, const N: usize>(&mut self, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }
}