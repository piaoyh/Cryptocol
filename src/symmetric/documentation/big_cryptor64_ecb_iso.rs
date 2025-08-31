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

/// trait_ecb_with_padding_iso.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to big_cryptor64_ecb_iso.rs.
/// And, most of generic parameters are omitted.
/// It is not actual code but dummy code for compilation!!!
#[allow(non_camel_case_types)]
pub struct BigCryptor64
{
    // Dummy struct for documentation
}

/// trait_ecb_with_padding_iso.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to big_cryptor64_ecb_iso.rs.
/// And, most of generic parameters are omitted.
/// It is not actual code but dummy code for compilation!!!
impl BigCryptor64
{
    // fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;
    /// Encrypts the data with the padding defined according to ISO 7816-4
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
    /// - The output will be at least `8`,
    ///   and cannot be other than a multiple of `8`.
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
    ///   (`length_in_bytes` + `1`).next_multiple_of(`8`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt(message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// ```
    pub fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_into_vec<U>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data with the padding defined according to ISO 7816-4
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
    /// - The output will be at least `8`,
    ///   and cannot be other than a multiple of `8`.
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
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_into_vec(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// ```
    pub fn encrypt_into_vec<U>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_into_array<U, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    /// Encrypts the data with the padding defined according to ISO 7816-4
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
    /// - The output will be at least `8`,
    ///   and cannot be other than a multiple of `8`.
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
    ///   multiple of `8`, this method does not perform
    ///   encryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `N` is equal to `length_in_bytes`'s next
    ///   multiple of `8`, this method performs encryption,
    ///   fills the array `cipher` with the encrypted data, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - If `U::size_in_bytes()` * `N` is greater than `length_in_bytes`'s next
    ///   multiple of `8`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`length_in_bytes` + `1`).next_multiple_of(`8`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// ## Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// ```
    pub fn encrypt_into_array<U, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_str(&mut self, message: &str, cipher: *mut u8) -> u64
    /// Encrypts the data in a `str` object with the padding defined
    /// according to ISO 7816-4 in ECB (Electronic CodeBook) mode.
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
    /// - The output will be at least `8`,
    ///   and cannot be other than a multiple of `8`.
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
    ///   (`message.len()` + `1`).next_multiple_of(`8`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_str(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// ```
    pub fn encrypt_str(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_str_into_vec<U>(&mut self, message: &str, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data in `str` with the padding defined according to
    /// ISO 7816-4 in ECB (Electronic CodeBook) mode, and stores the
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// ```
    pub fn encrypt_str_into_vec<U>(&mut self, message: &str, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_str_into_array<U, const N: usize>(&mut self, message: &str, cipher: &mut [U; N]) -> u64
    /// Encrypts the data in a `str` object with the padding defined
    /// according to ISO 7816-4 in ECB (Electronic CodeBook) mode,
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
    /// - The output will be at least `8`,
    ///   and cannot be other than a multiple of `8`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is a null string "", only padding bytes will be encrypted,
    ///   and stored in the array `[U; N]` object `cipher`.
    /// - If `U::size_in_bytes()` * `N` is less than `message.len()`'s next
    ///   multiple of `8`, this method does not perform
    ///   encryption but returns `zero`.
    /// - If `U::size_in_bytes()` * `N` is equal to `message.len()`'s next
    ///   multiple of `8`, this method performs encryption,
    ///   fills the array `cipher` with the encrypted data, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - If `U::size_in_bytes()` * `N` is greater than `message.len()`'s next
    ///   multiple of `8`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`message.len()` + `1`).next_multiple_of(`8`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// ```
    pub fn encrypt_str_into_array<U, const N: usize>(&mut self, message: &str, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_string(&mut self, message: &String, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `String` object with the padding
    /// according to ISO 7816-4 in ECB (Electronic CodeBook) mode.
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
    /// - The output will be at least `8`,
    ///   and cannot be other than a multiple of `8`.
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
    ///   (`message.len()` + `1`).next_multiple_of(`8`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_string(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// ```
    pub fn encrypt_string(&mut self, message: &String, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_string_into_vec<U>(&mut self, message: &String, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data stored in a `String` object with the padding
    /// according to ISO 7816-4 in ECB (Electronic CodeBook) mode,
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
    /// - The output will be at least `8`,
    ///   and cannot be other than a multiple of `8`.
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
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_string_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// ```
    pub fn encrypt_string_into_vec<U>(&mut self, message: &String, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_string_into_array<U, const N: usize>(&mut self, message: &String, cipher: &mut [U; N]) -> u64
    /// Encrypts the data stored in a `String` object with the padding
    /// according to ISO 7816-4 in ECB (Electronic CodeBook) mode,
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
    /// - The output will be at least `8`,
    ///   and cannot be other than a multiple of `8`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is a `String` object that has a null string "", only
    ///   padding bytes will be encrypted, and stored in the array `[U; N]`
    ///   object `cipher`.
    /// - If `size_of::<U>()` * `N` is less than `message.len()`'s next
    ///   multiple of `8`, this method does not perform
    ///   encryption but returns `zero`.
    /// - If `size_of::<U>()` * `N` is equal to `message.len()`'s next
    ///   multiple of `8`, this method performs encryption,
    ///   fills the array `cipher` with the encrypted data, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - If `size_of::<U>()` * `N` is greater than `message.len()`'s next
    ///   multiple of `8`, this method performs encryption, fills
    ///   the array `cipher` with the encrypted data, and then fills the
    ///   rest of the elements of the array `cipher` with zeros, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`message.len()` + `1`).next_multiple_of(`8`) at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_string_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// ```
    pub fn encrypt_string_into_array<U, const N: usize>(&mut self, message: &String, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_vec<U>(&mut self, message: &Vec<U>, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to ISO 7816-4 in ECB (Electronic CodeBook) mode.
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
    /// - The output will be at least `8`,
    ///   and cannot be other than a multiple of `8`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`size_of::<U>()` * `message.len()` + `1`).next_multiple_of(`8`)
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
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_vec(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// ```
    pub fn encrypt_vec<U>(&mut self, message: &Vec<U>, cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_vec_into_vec<U, V>(&mut self, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to ISO 7816-4 in ECB (Electronic CodeBook) mode, and
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
    /// - The output will be at least `8`,
    ///   and cannot be other than a multiple of `8`.
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
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_vec_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// ```
    pub fn encrypt_vec_into_vec<U, V>(&mut self, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to ISO 7816-4 in ECB (Electronic CodeBook) mode, and
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
    /// - The output will be at least `8`,
    ///   and cannot be other than a multiple of `8`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is an empty `Vec<U>` object `Vec::<U>::new()`, only
    ///   padding bytes will be encrypted, and stored in the array `[U; N]`
    ///   object `cipher`.
    /// - If `size_of::<V>()` * `N` is less than 
    ///   `size_of::<U>() * message.len()`'s next multiple of
    ///   `8`, this method does not perform
    ///   encryption but returns `zero`.
    /// - If `size_of::<V>()` * `N` is equal to
    ///   `size_of::<U>() * message.len()`'s next multiple of
    ///   `8`, this method performs encryption,
    ///   fills the array `cipher` with the encrypted data, and returns
    ///   the size of the ciphertext including padding bits in bytes.
    /// - If `size_of::<V>()` * `N` is greater than
    ///   `size_of::<U>() * message.len()`'s next multiple of `8`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted data, and then fills the rest of the elements of the array
    ///   `cipher` with zeros, and returns the size of the ciphertext including
    ///   padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`size_of::<U>()` * `message.len()` + `1`).next_multiple_of(`8`)
    ///   at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_vec_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// ```
    pub fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_array<U, const N: usize>(&mut self, message: &[U; N], cipher: *mut u8) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to ISO 7816-4 in ECB (Electronic CodeBook) mode.
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
    /// - The output will be at least `8`,
    ///   and cannot be other than a multiple of `8`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as encrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The size of the memory area which starts at `cipher` is assumed to be
    ///   enough to store the ciphertext.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`size_of::<U>()` * `N` + `1`).next_multiple_of(`8`)
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
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_array(&message, cipher.as_mut_ptr());
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// ```
    pub fn encrypt_array<U, const N: usize>(&mut self, message: &[U; N], cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, message: &[U; N], cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// according to ISO 7816-4 in ECB (Electronic CodeBook) mode, and stores the
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
    /// - The output will be at least `8`,
    ///   and cannot be other than a multiple of `8`.
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
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_array_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// ```
    pub fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, message: &[U; N], cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, message: &[U; N], cipher: &mut [V; M]) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// according to ISO 7816-4 in ECB (Electronic CodeBook) mode, and stores the
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
    /// - The output will be at least `8`,
    ///   and cannot be other than a multiple of `8`.
    /// - If this method failed in encryption, this method returns `zero`.
    /// 
    /// # Features
    /// - If `message` is an empty array `[U; 0]` object, only padding bytes
    ///   will be encrypted, and stored in the array `[V; M]` object `cipher`.
    /// - If `V::size_in_bytes()` * `M` is less than 
    ///   `U::size_in_bytes()` * `N`'s next multiple of `8`,
    ///   this method does not perform encryption and returns `zero`.
    /// - If `V::size_in_bytes()` * `M` is equal to
    ///   `U::size_in_bytes()` * `N`'s next multiple of `8`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns the size of the ciphertext including
    ///   padding bits in bytes.
    /// - If `V::size_in_bytes()` * `M` is greater than
    ///   `U::size_in_bytes()` * `N`'s next multiple of `8`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and then fills the rest of the elements of the
    ///   array `cipher` with zeros, and returns the size of the ciphertext
    ///   including padding bits in bytes.
    /// - The size of the area for ciphertext should be prepared to be
    ///   (`size_of::<U>()` * `message.len()` + `1`).next_multiple_of(`8`)
    ///   at least.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 56];
    /// tdes.encrypt_array_into_array(&message, &mut cipher);
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// ```
    pub fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, message: &[U; N], cipher: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64;
    /// Decrypts the data with the padding defined according to ISO 7816-4
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
    /// - If `length_in_bytes` is greater than `8` (which means
    ///   that the original plaintext is surely not empty data) and it returns
    ///   `zero`, you can interpret it that this method surely failed in
    ///   decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be other than any multiple of `8`.
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
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// tdes.decrypt(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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

    // fn decrypt_into_vec<U>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    /// Decrypts the data with the padding defined according to ISO 7816-4
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
    /// - If `length_in_bytes` is greater than `8` (which means
    ///   that the original plaintext is surely not empty data) and it returns
    ///   `zero`, you can interpret it that this method surely failed in
    ///   decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be other than any multiple of `8`.
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// tdes.decrypt_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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

    // fn decrypt_into_array<U, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    /// Decrypts the data with the padding defined according to ISO 7816-4
    /// in ECB (Electronic CodeBook) mode, and stores the encrypted data
    /// in array `[U; N]`.
    /// 
    /// # Arguments
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
    /// - The padding bits are composed of the byte `0b_1000_0000` that
    ///   indicates the delimiter one bit `1` followed by seven bits `0`s and
    ///   all padding bits `0`s according to ISO 7816-4.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// 
    /// let mut recovered = [0u8; 56];
    /// let len = tdes.decrypt_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    pub fn decrypt_into_array<U, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    /// Decrypts the data with the padding defined according to ISO 7816-4
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
    /// - If `length_in_bytes` is greater than `8` (which means
    ///   that the original plaintext is surely not empty data) and it returns
    ///   `zero`, you can interpret it that this method surely failed in
    ///   decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_*_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `length_in_bytes` cannot be other than any multiple of `8`.
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
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// 
    /// let mut recovered = String::new();
    /// tdes.decrypt_into_string(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    pub fn decrypt_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_vec<U>(&mut self, cipher: &Vec<U>, message: *mut u8) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to ISO 7816-4 in ECB (Electronic CodeBook) mode.
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
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `8`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `size_of::<U>()` * `cipher.len()` cannot be other than any multiple
    ///   of `8`.
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
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// tdes.decrypt_vec(&cipher, recovered.as_mut_ptr());
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

    // fn decrypt_vec_into_vec<U, V>(&mut self, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to ISO 7816-4 in ECB (Electronic CodeBook) mode, and
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
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `8`
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
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// tdes.decrypt_vec_into_vec(&cipher, &mut recovered);
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

    // fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to ISO 7816-4 in ECB (Electronic CodeBook) mode, and
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
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `8`
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
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// 
    /// let mut recovered = [0u8; 56];
    /// let len = tdes.decrypt_vec_into_array(&cipher, &mut recovered);
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
    pub fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_vec_into_string(&mut self, cipher: &str, message: &mut String) -> u64
    /// Decrypts the data in `str` with the padding defined according to
    /// ISO 7816-4 in ECB (Electronic CodeBook) mode, and stores the
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
    /// - If `size_of::<U>()` * `cipher.len()` is greater than `8`
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
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let mut cipher = Vec::<u8>::new();
    /// tdes.encrypt_str_into_vec(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// 
    /// let mut recovered = String::new();
    /// tdes.decrypt_vec_into_string(&cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    pub fn decrypt_vec_into_string<U>(&mut self, cipher: &Vec<U>, message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_array<U, const N: usize>(&mut self, cipher: &[U; N], message: *mut u8) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to ISO 7816-4 in ECB (Electronic CodeBook) mode.
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
    /// - If `size_of::<U>()` * `N` is greater than `8`
    ///   (which means that the original plaintext is surely not empty data)
    ///   and it returns `zero`, you can interpret it that this method surely
    ///   failed in decryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as decrypt_array_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - `size_of::<U>()` * `N` cannot be other than any multiple
    ///   of `8`.
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
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 56];
    /// tdes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// 
    /// let mut recovered = vec![0; 55];
    /// let len = tdes.decrypt_array(&cipher, recovered.as_mut_ptr());
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
    fn decrypt_array<U, const N: usize>(&mut self, cipher: &[U; N], message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }
    
    // fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, cipher: &[U; N], message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to ISO 7816-4 in ECB (Electronic CodeBook) mode,
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
    /// - If `size_of::<U>()` * `N` is greater than `8`
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
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 56];
    /// tdes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// tdes.decrypt_array_into_vec(&cipher, &mut recovered);
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

    // fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, cipher: &[U; N], message: &mut [V; M]) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to ISO 7816-4 in ECB (Electronic CodeBook) mode,
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
    /// - If `size_of::<U>()` * `N` is greater than `8`
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
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 56];
    /// tdes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// 
    /// let mut recovered = [0u8; 56];
    /// let len = tdes.decrypt_array_into_array(&cipher, &mut recovered);
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
    pub fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, cipher: &[U; N], message: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // fn decrypt_array_into_string<U, const N: usize>(&mut self, cipher: &[U; N], message: &mut String) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to ISO 7816-4 in ECB (Electronic CodeBook) mode,
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
    /// - If `size_of::<U>()` * `N` is greater than `8`
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
    /// # Example 1 for TDES case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ BigCryptor64, DES, ECB_ISO };
    /// 
    /// let mut tdes = DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64)
    ///                 + DES::decryptor_with_key_u64(0x_FEDCBA0987654321_u64)
    ///                 + DES::encryptor_with_key_u64(0x_1234567890ABCDEF_u64);
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);    let mut cipher = [0_u8; 56];
    /// tdes.encrypt_str_into_array(&message, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "99 2D 87 B2 70 5A 21 3B 7C 93 1A 2E D7 B9 3E E1 FD 47 70 51 48 1F 96 06 2E F3 64 AE 43 11 9C 72 E3 90 65 CF 5B FC 08 12 7A 99 5B B1 2F 7F B4 5D 49 94 44 D4 8B 57 00 30 ");
    /// 
    /// let mut recovered = String::new();
    /// tdes.decrypt_array_into_string(&cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    pub fn decrypt_array_into_string<U, const N: usize>(&mut self, cipher: &[U; N], message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }
}
