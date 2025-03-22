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
    // fn encrypt(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;
    /// Encrypts the data with the padding defined according to PKCS #7
    /// in CBC (Cipher Block Chaining) mode.
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
    ///   (`length_in_bytes` + 1).next_multiple_of(8) at least when `T` is `u64`, and
    ///   (`length_in_bytes` + 1).next_multiple_of(16) at least when `T` is `u128`.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
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
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =\t{}", iv);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
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
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =\t{}", iv);
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 C6 9D C6 BD A6 9B 58 8B ");
    /// ```
    /// 
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CBC_PKCS7 };
    /// 
    /// let key1 = 0x_1234567890ABCDEF_u64;
    /// let key2 = 0_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut c_des = DES_Expanded::<0, 0>::new_with_key_u64(key1);
    /// let mut d_des = DES_Expanded::<0, 0>::new_with_key_u64(key2);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =\t{}", iv);
    /// let mut cipher1 = [0_u8; 56];
    /// let mut cipher2 = [0_u8; 56];
    /// c_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher1.as_mut_ptr());
    /// d_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher2.as_mut_ptr());
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 20 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "94 1E 8A F3 92 EF FC 6C F2 B6 D3 6E FC 49 61 07 E1 F2 7C 05 EC 15 23 91 40 49 26 92 CC 92 87 F8 90 12 83 F3 75 FB D6 47 70 B3 DE 6B AA 4F 7D 11 A0 E9 7F 26 ED 1B A3 20 ");
    /// ```
    /// 
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =\t{}", iv);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "A5 21 CF D8 13 A4 20 36 ");
    /// ```
    /// 
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =\t{}", iv);
    /// let mut cipher = [0_u8; 8];
    /// a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "D8 F8 E5 E8 6B 44 98 16 ");
    /// ```
    /// 
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =\t{}", iv);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "ED 4F CD B8 C1 A4 48 47 AB 12 20 DA B4 A6 F1 F1 ");
    /// ```
    /// 
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =\t{}", iv);
    /// let mut cipher = [0_u8; 16];
    /// a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "5F 4F BB 12 C8 FB 7D 4B 91 E7 5E CF CC C0 44 5A ");
    /// ```
    /// 
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =\t{}", iv);
    /// let mut cipher = [0_u8; 24];
    /// a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "C3 60 23 7C BD E7 0E 6E F4 0C B1 6A 69 67 13 26 91 D0 EB 51 AF F5 1F 5A ");
    /// ```
    pub fn encrypt(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_into_vec<U>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data with the padding defined according to PKCS #7
    /// in CBC (Cipher Block Chaining) mode, and stores the encrypted data
    /// in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `message` is a pointer to u8 which is `*const u8`,
    ///   and is the plaintext to be encrypted.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the plaintext `message` in bytes.
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null string.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the `Vec<U>` object `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn encrypt_into_vec<T>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_into_array<U, const N: usize>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    /// Encrypts the data with the padding defined according to PKCS #7
    /// in CBC (Cipher Block Chaining) mode, and stores the encrypted data
    /// in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
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
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null data.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the array `[U; N]` object `cipher`.
    /// - If `U::size_in_bytes() * N` is less than `length_in_bytes`'s next
    ///   multiple of 8, this method does not perform encryption and returns
    ///   `zero`.
    /// - If `U::size_in_bytes() * N` is equal to `length_in_bytes`'s next
    ///   multiple of 8, this method performs encryption, fills the array
    ///   `cipher` with the encrypted ciphertext, and returns the size of the
    ///   ciphertext including padding bits in bytes.
    /// - If `U::size_in_bytes() * N` is greater than `length_in_bytes`'s next
    ///   multiple of 8, this method performs encryption, fills the array
    ///   `cipher` with the encrypted ciphertext, and then fills the rest of the
    ///   elements of the array `cipher` with zeros, and returns the size of the
    ///   ciphertext including padding bits in bytes.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn encrypt_into_array<T, const N: usize>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_str(&mut self, iv: u64, message: &str, cipher: *mut u8) -> u64
    /// Encrypts the data in `str` with the padding defined
    /// according to PKCS #7 in CBC (Cipher Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
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
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_str_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn encrypt_str(&mut self, iv: u64, message: &str, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_str_into_vec<U>(&mut self, iv: u64, message: &str, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data in `str` with the padding defined according to PKCS #7
    /// in CBC (Cipher Block Chaining) mode, and stores the encrypted data
    /// in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `message` is a `str` object, and is the plaintext to be encrypted.
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the `Vec<U>` object `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn encrypt_str_into_vec<T>(&mut self, iv: u64, message: &str, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_str_into_array<U, const N: usize>(&mut self, iv: u64, message: &str, cipher: &mut [U; N]) -> u64
    /// Encrypts the data in `str` with the padding defined according to PKCS #7
    /// in CBC (Cipher Block Chaining) mode, and stores the encrypted data
    /// in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `message` is a `str` object, and is the plaintext to be encrypted.
    /// - `cipher` is an array `[U; N]` object,
    ///   and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is a null string "", only padding bytes will be
    ///   encrypted, and stored in the array `[U; N]` object `cipher`.
    /// - If `U::size_in_bytes() * N` is less than `message.len()`'s next
    ///   multiple of 8,
    ///   this method does not perform encryption and returns `zero`.
    /// - If `U::size_in_bytes() * N` is equal to `message.len()`'s next
    ///   multiple of 8, this method performs encryption, fills the array
    ///   `cipher` with the encrypted ciphertext, and returns the size of
    ///   the ciphertext including padding bits in bytes.
    /// - If `U::size_in_bytes() * N` is greater than `message.len()`'s next
    ///   multiple of 8, this method performs encryption, fills the array
    ///   `cipher` with the encrypted ciphertext, and then fills the rest of
    ///   the elements of the array `cipher` with zeros, and returns the size
    ///   of the ciphertext including padding bits in bytes.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn encrypt_str_into_array<T, const N: usize>(&mut self, iv: u64, message: &str, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_string(&mut self, iv: u64, message: &String, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a String object with the padding according
    /// to PKCS #7 in CBC (Cipher Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `message` is a String object, and is the plaintext to be encrypted.
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
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
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
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn encrypt_string(&mut self, iv: u64, message: &String, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_string_into_vec<U>(&mut self, iv: u64, message: &String, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data stored in a String object with the padding according
    /// to PKCS #7 in CBC (Cipher Block Chaining) mode, and stores the encrypted
    /// data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `message` is a String object, and is the plaintext to be encrypted.
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is a null string String::new(), only padding bytes will
    ///   be encrypted, and stored in the `Vec<U>` object `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn encrypt_string_into_vec<T>(&mut self, iv: u64, message: &String, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_string_into_array<U, const N: usize>(&mut self, iv: u64, message: &String, cipher: &mut [U; N]) -> u64
    /// Encrypts the data stored in a String object with the padding according
    /// to PKCS #7 in CBC (Cipher Block Chaining) mode, and stores the encrypted
    /// data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `message` is a String object, and is the plaintext to be encrypted.
    /// - `cipher` is an array `[U; N]` object,
    ///   and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is a null string String::new(), only padding bytes will
    ///   be encrypted, and stored in the array `[U; N]` object `cipher`.
    /// - If `U::size_in_bytes() * N` is less than `message.len()`'s next
    ///   multiple of 8,
    ///   this method does not perform encryption and returns `zero`.
    /// - If `U::size_in_bytes() * N` is equal to `message.len()`'s next
    ///   multiple of 8, this method performs encryption, fills the array
    ///   `cipher` with the encrypted ciphertext, and returns the size of
    ///   the ciphertext including padding bits in bytes.
    /// - If `U::size_in_bytes() * N` is greater than `message.len()`'s next
    ///   multiple of 8, this method performs encryption, fills the array
    ///   `cipher` with the encrypted ciphertext, and then fills the rest of
    ///   the elements of the array `cipher` with zeros, and returns the size
    ///   of the ciphertext including padding bits in bytes.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn encrypt_string_into_array<T, const N: usize>(&mut self, iv: u64, message: &String, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_vec<U>(&mut self, iv: u64, message: &Vec<U>, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #7 in CBC (Cipher Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `message` is a `Vec<U>` object, and is the plaintext to be encrypted.
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
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as
    ///   encrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `message` is an empty Vec<U> object Vec::<U>::new(), only padding
    ///   bytes will be encrypted, and stored in the memory area that starts
    ///   from `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn encrypt_vec<T>(&mut self, iv: u64, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_vec_into_vec<U, V>(&mut self, iv: u64, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding according
    /// to PKCS #7 in CBC (Cipher Block Chaining) mode, and stores the encrypted
    /// data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `message` is a `Vec<U>` object, and is the plaintext to be encrypted.
    /// - `cipher` is a `Vec<V>` object, and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is an empty Vec<U> object Vec::<U>::new(), only padding
    ///   bytes will be encrypted, and stored in the `Vec<V>` object `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, DES_Expanded, ECB_PKCS7 };
    /// 
    /// ```
    pub fn encrypt_vec_into_vec<T, U>(&mut self, iv: u64, message: &Vec<T>, cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: u64, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding according
    /// to PKCS #7 in CBC (Cipher Block Chaining) mode, and stores the encrypted
    /// data in array `[V; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `message` is a `Vec<U>` object, and is the plaintext to be encrypted.
    /// - `cipher` is an array `[V; N]` object,
    ///   and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is an empty Vec<U> object Vec::<U>::new(), only padding
    ///   bytes will be encrypted, and stored in the array `[V; N]` object
    ///   `cipher`.
    /// - If `V::size_in_bytes() * N` is less than 
    ///   `U::size_in_bytes() * message.len()`'s next multiple of 8,
    ///   this method does not perform encryption and returns `zero`.
    /// - If `V::size_in_bytes() * N` is equal to
    ///   `U::size_in_bytes() * message.len()`'s next multiple of 8, this method
    ///   performs encryption, fills the array `cipher` with the encrypted
    ///   ciphertext, and returns the size of the ciphertext including padding
    ///   bits in bytes.
    /// - If `V::size_in_bytes() * N` is greater than
    ///   `U::size_in_bytes() * message.len()`'s next multiple of 8, this method
    ///   performs encryption, fills the array `cipher` with the encrypted
    ///   ciphertext, and then fills the rest of the elements of the array
    ///   `cipher` with zeros, and returns the size of the ciphertext including
    ///   padding bits in bytes.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn encrypt_vec_into_array<T, U, const N: usize>(&mut self, iv: u64, message: &Vec<T>, cipher: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_array<U, const N: usize>(&mut self, iv: u64, message: &[U; N], cipher: *mut u8) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #7 in CBC (Cipher Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `message` is the data stored in an array `[U; N]` object,
    ///   and is the plaintext to be encrypted.
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
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as
    ///   encrypt_array_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `message.len()` is `0`, it means the message is empty data.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn encrypt_array<T, const N: usize>(&mut self, iv: u64, message: &[T; N], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: u64, message: &[U; N], cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// according to PKCS #7 in CBC (Cipher Block Chaining) mode, and stores the
    /// encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `message` is an array `[U; N]` object, and is the plaintext to be
    ///   encrypted.
    /// - `cipher` is a `Vec<V>` object, and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is an empty array `[U; N]` object [U; 0], only padding
    ///   bytes will be encrypted, and stored in the `Vec<U>` object `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn encrypt_array_into_vec<T, U, const N: usize>(&mut self, iv: u64, message: &[T; N], cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: u64, message: &[U; N], cipher: &mut [V; M]) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// according to PKCS #7 in CBC (Cipher Block Chaining) mode, and stores the
    /// encrypted data in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `message` is an array `[U; N]` object,
    ///   and is the plaintext to be encrypted.
    /// - `cipher` is an array `[V; M]` object,
    ///   and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is an empty array `[U; N]` object [U; 0],
    ///   only padding bytes will be encrypted,
    ///   and stored in the array `[V; M]` object `cipher`.
    /// - If `V::size_in_bytes() * M` is less than 
    ///   `U::size_in_bytes() * N`'s next multiple of 8,
    ///   this method does not perform encryption and returns `zero`.
    /// - If `V::size_in_bytes() * M` is equal to
    ///   `U::size_in_bytes() * N`'s next multiple of 8, this method
    ///   performs encryption, fills the array `cipher` with the encrypted
    ///   ciphertext, and returns the size of the ciphertext including padding
    ///   bits in bytes.
    /// - If `V::size_in_bytes() * M` is greater than
    ///   `U::size_in_bytes() * N`'s next multiple of 8, this method performs
    ///   encryption, fills the array `cipher` with the encrypted ciphertext,
    ///   and then fills the rest of the elements of the array `cipher`
    ///   with zeros, and returns the size of the ciphertext including
    ///   padding bits in bytes.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn encrypt_array_into_array<T, U, const N: usize, const M: usize>(&mut self, iv: u64, message: &[T; N], cipher: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64;
    /// Decrypts the data with the padding defined according to PKCS #7
    /// in CBC (Cipher Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `cipher` is a pointer to u8 which is `*mut u8`,
    ///   and is the ciphertext to be stored.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the plaintext `message` in bytes.
    /// - `message` is a pointer to u8 which is `*const u8`,
    ///   and is the plaintext to be encrypted.
    /// - The size of the memory area which starts at `message` and the
    ///   plaintext will be stored at is assumed to be enough.
    /// - The size of the area for plaintext should be prepared to be:
    ///   `length_in_bytes` - 1.
    ///   So, it is responsible for you to prepare the `message` area big enough!
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `length_in_bytes` is greater than
    ///   `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `length_in_bytes` is `8` for `T` =
    ///   `u64` or `16` for `T` = `u128`, it means either that this method
    ///   failed in decryption or that the original plaintext is empty data.
    ///   Then, you will have to check whether or not it failed by the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as
    ///   decrypt_*_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - When `T` is `u64`, `length_in_bytes` can be only any multiple of `8`.
    /// - When `T` is `u128`, `length_in_bytes` can be only any multiple of `16`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn decrypt(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_into_vec<U>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    /// Decrypts the data with the padding defined according to PKCS #7
    /// in CBC (Cipher Block Chaining) mode, and stores the decrypted data
    /// in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `cipher` is a pointer to u8 which is `*const u8`,
    ///   and is the ciphertext to be decrypted.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the ciphertext `cipher` in bytes.
    /// - `message` is a `Vec<U>` object, and is the plaintext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `length_in_bytes` is greater than
    ///   `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `length_in_bytes` is `8` for `T` =
    ///   `u64` or `16` for `T` = `u128`, it means either that this method
    ///   failed in decryption or that the original plaintext is empty data.
    ///   Then, you will have to check whether or not it failed by the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - When `T` is `u64`, `length_in_bytes` can be only any multiple of `8`.
    /// - When `T` is `u128`, `length_in_bytes` can be only any multiple of `16`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn decrypt_into_vec<T>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_into_array<U, const N: usize>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    /// Decrypts the data with the padding defined according to PKCS #7
    /// in CBC (Cipher Block Chaining) mode, and stores the encrypted data
    /// in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `cipher` is a pointer to u8 which is `*const u8`,
    ///   and is the ciphertext to be encrypted.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the ciphertext `message` in bytes.
    /// - `message` is an array `[U; N]` object,
    ///   and is the plaintext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `length_in_bytes` is greater than
    ///   `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `length_in_bytes` is `8` for `T` =
    ///   `u64` or `16` for `T` = `u128`, it means either that this method
    ///   failed in decryption or that the original plaintext is empty data.
    ///   Then, you will have to check whether or not it failed by the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - When `T` is `u64`, `length_in_bytes` can be only any multiple of `8`.
    /// - When `T` is `u128`, `length_in_bytes` can be only any multiple of `16`.
    /// - If `U::size_in_bytes() * N` is less than `length_in_bytes` - 1,
    ///   this method does not perform decryption and returns `zero`.
    /// - If `U::size_in_bytes() * N` is greater than or equal to
    ///   `length_in_bytes` - 1, this method performs decryption, fills the
    ///   array `message` with the derypted plaintext, and then fills the rest
    ///   of the elements of the array `message` with zeros if any, and returns
    ///   the size of the plaintext.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn decrypt_into_array<T, const N: usize>(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_into_string(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    /// Decrypts the data with the padding defined according to PKCS #7
    /// in CBC (Cipher Block Chaining) mode, and stores the decrypted data
    /// in String object.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `cipher` is a pointer to u8 which is `*const u8`,
    ///   and is the ciphertext to be decrypted.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the ciphertext `cipher` in bytes.
    /// - `message` is a String object, and is the plaintext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `length_in_bytes` is greater than
    ///   `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `length_in_bytes` is `8` for `T` =
    ///   `u64` or `16` for `T` = `u128`, it means either that this method
    ///   failed in decryption or that the original plaintext is empty String.
    ///   Then, you will have to check whether or not it failed by the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - When `T` is `u64`, `length_in_bytes` can be only any multiple of `8`.
    /// - When `T` is `u128`, `length_in_bytes` can be only any multiple of `16`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn decrypt_into_string(&mut self, iv: u64, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_vec<U>(&mut self, iv: u64, cipher: &Vec<U>, message: *mut u8) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #7 in CBC (Cipher Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be decrypted.
    /// - `message` is a pointer to u8 which is `*mut u8`,
    ///   and is the plaintext to be stored.
    /// - The size of the memory area which starts at `message` and the
    ///   plaintext will be stored at is assumed to be enough.
    /// - The size of the area for plaintext should be prepared to be:
    ///   `length_in_bytes` - 1.
    ///   So, it is responsible for you to prepare the `message` area big enough!
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `cipher.len() * U::size_in_bytes()`
    ///   is greater than `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `cipher.len() * U::size_in_bytes()`
    ///   is `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means either that this method failed in decryption
    ///   or that the original plaintext is empty data.
    ///   Then, you will have to check whether or not it failed by the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as
    ///   decrypt_vec_into_*().
    /// - When `T` is `u64`, `cipher.len() * U::size_in_bytes()`
    ///   can be only any multiple of `8`.
    /// - When `T` is `u128`, `cipher.len() * U::size_in_bytes()`
    ///   can be only any multiple of `16`.
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn decrypt_vec<T>(&mut self, iv: u64, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_vec_into_vec<U, V>(&mut self, iv: u64, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding according
    /// to PKCS #7 in CBC (Cipher Block Chaining) mode, and stores the decrypted
    /// data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be decrypted.
    /// - `message` is a `Vec<V>` object, and is the plaintext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `cipher.len() * U::size_in_bytes()`
    ///   is greater than `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `cipher.len() * U::size_in_bytes()`
    ///   is `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means either that this method failed in decryption
    ///   or that the original plaintext is empty data.
    ///   Then, you will have to check whether or not it failed by the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - When `T` is `u64`, `cipher.len() * U::size_in_bytes()`
    ///   can be only any multiple of `8`.
    /// - When `T` is `u128`, `cipher.len() * U::size_in_bytes()`
    ///   can be only any multiple of `16`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn decrypt_vec_into_vec<T, U>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: u64, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding according
    /// to PKCS #7 in CBC (Cipher Block Chaining) mode, and stores the decrypted
    /// data in array `[V; N]`.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be decrypted.
    /// - `message` is an array `[V; N]` object,
    ///   and is the plaintext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `cipher.len() * U::size_in_bytes()`
    ///   is greater than `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `cipher.len() * U::size_in_bytes()`
    ///   is `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means either that this method failed in decryption
    ///   or that the original plaintext is empty data.
    ///   Then, you will have to check whether or not it failed by the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - When `T` is `u64`, `cipher.len() * U::size_in_bytes()`
    ///   can be only any multiple of `8`.
    /// - When `T` is `u128`, `cipher.len() * U::size_in_bytes()`
    ///   can be only any multiple of `16`.
    /// - If `V::size_in_bytes() * N` is greater than or equal to
    ///   `U::size_in_bytes() * cipher.len() - 1`, this method performs
    ///   decryption, fills the array `message` with the derypted plaintext,
    ///   and then fills the rest of the elements of the array `message`
    ///   with zeros if any, and returns the size of the plaintext.
    /// - If `V::size_in_bytes() * N` is less than 
    ///   `U::size_in_bytes() * cipher.len() - 1`,
    ///   this method does not perform encryption and returns `zero`.
     /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn decrypt_vec_into_array<T, U, const N: usize>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_vec_into_string<U>(&mut self, iv: u64, cipher: &Vec<U>, message: &mut String) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding according
    /// to PKCS #7 in CBC (Cipher Block Chaining) mode, and stores the decrypted
    /// data in String object.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be decrypted.
    /// - `message` is an String object, and is the plaintext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `cipher.len() * U::size_in_bytes()`
    ///   is greater than `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `cipher.len() * U::size_in_bytes()`
    ///   is `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means either that this method failed in decryption
    ///   or that the original plaintext is empty String.
    ///   Then, you will have to check whether or not it failed by the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - When `T` is `u64`, `cipher.len() * U::size_in_bytes()`
    ///   can be only any multiple of `8`.
    /// - When `T` is `u128`, `cipher.len() * U::size_in_bytes()`
    ///   can be only any multiple of `16`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn decrypt_vec_into_string<T>(&mut self, iv: u64, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_array<U, const N: usize>(&mut self, iv: u64, cipher: &[U; N], message: *mut u8) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #7 in CBC (Cipher Block Chaining) mode.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `cipher` is the data stored in an array `[U; N]` object,
    ///   and is the ciphertext to be decrypted.
    /// - `message` is a pointer to u8 which is `*mut u8`,
    ///   and is the plaintext to be stored.
    /// - The size of the memory area which starts at `message` and the
    ///   plaintext will be stored at is assumed to be enough.
    /// - The size of the area for plaintext should be prepared to be:
    ///   `N * U::size_in_bytes()` - 1.
    ///   So, it is responsible for you to prepare the `message` area big enough!
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `length_in_bytes` is greater than
    ///   `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `length_in_bytes` is `8` for `T` =
    ///   `u64` or `16` for `T` = `u128`, it means either that this method
    ///   failed in decryption or that the original plaintext is empty array
    ///   [U; 0]. Then, you will have to check whether or not it failed by the
    ///   method `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as
    ///   decrypt_array_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - When `T` is `u64`, `N` can be only any multiple of `8`.
    /// - When `T` is `u128`, `N` can be only any multiple of `16`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn decrypt_array<T, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: u64, cipher: &[U; N], message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// according to PKCS #7 in CBC (Cipher Block Chaining) mode, and stores the
    /// decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `cipher` is an array `[U; N]` object, and is the ciphertext to be
    ///   decrypted.
    /// - `message` is a `Vec<V>` object, and is the plaintext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `cipher.len() * U::size_in_bytes()`
    ///   is greater than `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `cipher.len() * U::size_in_bytes()`
    ///   is `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means either that this method failed in decryption
    ///   or that the original plaintext is empty array [U; 0].
    ///   Then, you will have to check whether or not it failed by the
    ///   method `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - When `T` is `u64`, `cipher.len() * U::size_in_bytes()`
    ///   can be only any multiple of `8`.
    /// - When `T` is `u128`, `cipher.len() * U::size_in_bytes()`
    ///   can be only any multiple of `16`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn decrypt_array_into_vec<T, U, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: u64, cipher: &[U; N], message: &mut [V; M]) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// according to PKCS #7 in CBC (Cipher Block Chaining) mode, and stores the
    /// decrypted data in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `cipher` is an array `[U; N]` object,
    ///   and is the ciphertext to be decrypted.
    /// - `message` is an array `[V; M]` object,
    ///   and is the plaintext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `cipher.len() * U::size_in_bytes()`
    ///   is greater than `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `cipher.len() * U::size_in_bytes()`
    ///   is `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means either that this method failed in decryption
    ///   or that the original plaintext is empty array [U; 0].
    ///   Then, you will have to check whether or not it failed by the
    ///   method `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - When `T` is `u64`, `cipher.len() * U::size_in_bytes()`
    ///   can be only any multiple of `8`.
    /// - When `T` is `u128`, `cipher.len() * U::size_in_bytes()`
    ///   can be only any multiple of `16`.
    /// - If `V::size_in_bytes() * M` is less than `U::size_in_bytes() * N - 1`,
    ///   this method does not perform decryption and returns `zero`.
    /// - If `V::size_in_bytes() * M` is greater than or qual to
    ///   `U::size_in_bytes() * N - 1`, this method performs decryption,
    ///   fills the array `message` with the derypted plaintext, and then
    ///   fills the rest of the elements of the array `message` with zeros
    ///   if any, and returns the size of the plaintext.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn decrypt_array_into_array<T, U, const N: usize, const M: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_array_into_string<U, const N: usize>(&mut self, iv: u64, cipher: &[U; N], message: &mut String) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding according
    /// to PKCS #7 in CBC (Cipher Block Chaining) mode, and stores the decrypted
    /// data in a String object.
    /// 
    /// # Arguments
    /// - `iv` is an initial value for CBC mode.
    /// - `cipher` is an array `[U; N]` object,
    ///   and is the ciphertext to be decrypted.
    /// - `message` is a String object, and is the plaintext to be encrypted.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `N * U::size_in_bytes()`
    ///   is greater than `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `N * U::size_in_bytes()`
    ///   is `8` for `T` = `u64` or `16` for `T` = `u128`,
    ///   it means either that this method failed in decryption
    ///   or that the original plaintext is empty array [U; 0].
    ///   Then, you will have to check whether or not it failed by the
    ///   method `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - When `T` is `u64`, `N * U::size_in_bytes()`
    ///   can be only any multiple of `8`.
    /// - When `T` is `u128`, `N * U::size_in_bytes()`
    ///   can be only any multiple of `16`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_PKCS7 };
    /// 
    /// ```
    pub fn decrypt_array_into_string<T, const N: usize>(&mut self, iv: u64, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }
}