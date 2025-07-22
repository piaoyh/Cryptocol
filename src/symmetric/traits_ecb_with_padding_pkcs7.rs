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
use crate::symmetric::{ des_pre_encrypt_into_vec, des_pre_decrypt_into_vec };

/// ECB (Electronic CodeBook) is one of the operation modes for 
/// encryption/decryption.
/// 
/// # Caution
/// For Rijndael or AES, if NB > 64, you are not supposed to use the padding
/// defined in PKCS #7 because its behavior is not defined.
#[allow(non_camel_case_types)]
pub trait ECB_PKCS7<T> : Sized
{
    // fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    /// Encrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic CodeBook) mode.
    /// 
    /// # Arguments
    /// - `message` is a pointer to u8 which is `*const u8`,
    ///   and is the plaintext to be encrypted.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the plaintext `message` in bytes.
    /// - `cipher` is a pointer to u8 which is `*mut u8`,
    ///   and is the ciphertext to be stored.
    /// - The size of the memory area which starts at `cipher` and the
    ///   ciphertext will be stored at is assumed to be enough.
    /// - The size of the area for ciphertext should be prepared to be:
    ///   (`length_in_bytes` + 1).next_multiple_of(8) at least when `T` is `u64`,
    ///   (`length_in_bytes` + 1).next_multiple_of(16) at least when `T` is `u128`, and
    ///   (`length_in_bytes` + 1).next_multiple_of(32 * `NB`) at least when `T` is `[u32; NB]`.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
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
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.encrypt)
    /// 
    /// # For Rijndael or AES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/aes_ecb_pkcs7/struct.Rijndal_Generic.html#method.encrypt)
    fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;

    // fn encrypt_into_vec<U>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic CodeBook) mode, and stores the encrypted data
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
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - When `T` is `[u32; NB]` for Rijndael or AES, the output should be at
    ///   least `32 * NB`, and will be only any multiple of `32 * NB`.
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.encrypt_into_vec)
    fn encrypt_into_vec<U>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        des_pre_encrypt_into_vec!(cipher, length_in_bytes, U);
        let len = self.encrypt(message, length_in_bytes, cipher.as_mut_ptr() as *mut u8);
        cipher.truncate(len as usize);
        len
    }

    // fn encrypt_into_array<U, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    /// Encrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic CodeBook) mode, and stores the encrypted data
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
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - When `T` is `[u32; NB]` for Rijndael or AES, the output should be at
    ///   least `32 * NB`, and will be only any multiple of `32 * NB`.
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.encrypt_into_array)
    fn encrypt_into_array<U, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone;

    // fn encrypt_str(&mut self, message: &str, cipher: *mut u8) -> u64
    /// Encrypts the data in `str` with the padding defined
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode.
    /// 
    /// # Arguments
    /// - `message` is a `str` object, and is the plaintext to be encrypted.
    /// - `cipher` is a pointer to u8 which is `*mut u8`,
    ///   and is the ciphertext to be stored.
    /// - The size of the memory area which starts at `cipher` and the
    ///   ciphertext will be stored at is assumed to be enough.
    /// - The size of the area for ciphertext should be prepared to be:
    ///   (`length_in_bytes` + 1).next_multiple_of(8) at least when `T` is `u64`,
    ///   (`length_in_bytes` + 1).next_multiple_of(16) at least when `T` is `u128`, and
    ///   (`length_in_bytes` + 1).next_multiple_of(32 * `NB`) at least when `T` is `[u32; NB]`.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - When `T` is `[u32; NB]` for Rijndael or AES, the output should be at
    ///   least `32 * NB`, and will be only any multiple of `32 * NB`.
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.encrypt_str)
    #[inline]
    fn encrypt_str(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt(message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_str_into_vec<U>(&mut self, message: &str, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data in `str` with the padding defined according to PKCS #7
    /// in ECB (Electronic CodeBook) mode, and stores the encrypted data
    /// in `Vec<U>`.
    /// 
    /// # Arguments
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
    /// - When `T` is `[u32; NB]` for Rijndael or AES, the output should be at
    ///   least `32 * NB`, and will be only any multiple of `32 * NB`.
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.encrypt_str_into_vec)
    #[inline]
    fn encrypt_str_into_vec<U>(&mut self, message: &str, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_str_into_array<U, const N: usize>(&mut self, message: &str, cipher: &mut [U; N]) -> u64
    /// Encrypts the data in `str` with the padding defined according to PKCS #7
    /// in ECB (Electronic CodeBook) mode, and stores the encrypted data
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
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - When `T` is `[u32; NB]` for Rijndael or AES, the output should be at
    ///   least `32 * NB`, and will be only any multiple of `32 * NB`.
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.encrypt_str_into_array)
    #[inline]
    fn encrypt_str_into_array<U, const N: usize>(&mut self, message: &str, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string(&mut self, message: &String, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a String object with the padding according
    /// to PKCS #7 in ECB (Electronic CodeBook) mode.
    /// 
    /// # Arguments
    /// - `message` is a String object, and is the plaintext to be encrypted.
    /// - `cipher` is a pointer to u8 which is `*mut u8`,
    ///   and is the ciphertext to be stored.
    /// - The size of the memory area which starts at `cipher` and the
    ///   ciphertext will be stored at is assumed to be enough.
    /// - The size of the area for ciphertext should be prepared to be:
    ///   (`length_in_bytes` + 1).next_multiple_of(8) at least when `T` is `u64`,
    ///   (`length_in_bytes` + 1).next_multiple_of(16) at least when `T` is `u128`, and
    ///   (`length_in_bytes` + 1).next_multiple_of(32 * `NB`) at least when `T` is `[u32; NB]`.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - When `T` is `[u32; NB]` for Rijndael or AES, the output should be at
    ///   least `32 * NB`, and will be only any multiple of `32 * NB`.
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.encrypt_string)
    #[inline]
    fn encrypt_string(&mut self, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt(message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string_into_vec<U>(&mut self, message: &String, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data stored in a String object with the padding according
    /// to PKCS #7 in ECB (Electronic CodeBook) mode, and stores the encrypted
    /// data in `Vec<U>`.
    /// 
    /// # Arguments
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
    /// - When `T` is `[u32; NB]` for Rijndael or AES, the output should be at
    ///   least `32 * NB`, and will be only any multiple of `32 * NB`.
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.encrypt_string_into_vec)
    #[inline]
    fn encrypt_string_into_vec<U>(&mut self, message: &String, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string_into_array<U, const N: usize>(&mut self, message: &String, cipher: &mut [U; N]) -> u64
    /// Encrypts the data stored in a String object with the padding according
    /// to PKCS #7 in ECB (Electronic CodeBook) mode, and stores the encrypted
    /// data in array `[U; N]`.
    /// 
    /// # Arguments
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
    /// - When `T` is `[u32; NB]` for Rijndael or AES, the output should be at
    ///   least `32 * NB`, and will be only any multiple of `32 * NB`.
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.encrypt_string_into_array)
    #[inline]
    fn encrypt_string_into_array<U, const N: usize>(&mut self, message: &String, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_vec<U>(&mut self, message: &Vec<U>, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode.
    /// 
    /// # Arguments
    /// - `message` is a `Vec<U>` object, and is the plaintext to be encrypted.
    /// - `cipher` is a pointer to u8 which is `*mut u8`,
    ///   and is the ciphertext to be stored.
    /// - The size of the memory area which starts at `cipher` and the
    ///   ciphertext will be stored at is assumed to be enough.
    /// - The size of the area for ciphertext should be prepared to be:
    ///   (`length_in_bytes` + 1).next_multiple_of(8) at least when `T` is `u64`,
    ///   (`length_in_bytes` + 1).next_multiple_of(16) at least when `T` is `u128`, and
    ///   (`length_in_bytes` + 1).next_multiple_of(32 * `NB`) at least when `T` is `[u32; NB]`.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - When `T` is `[u32; NB]` for Rijndael or AES, the output should be at
    ///   least `32 * NB`, and will be only any multiple of `32 * NB`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as
    ///   encrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `message` is an empty `Vec<U>` object `Vec::<U>::new()`, only padding
    ///   bytes will be encrypted, and stored in the memory area that starts
    ///   from `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_vec(&message, cipher.as_mut_ptr());
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
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.encrypt_vec)
    #[inline]
    fn encrypt_vec<U>(&mut self, message: &Vec<U>, cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt(message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_vec_into_vec<U, V>(&mut self, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding according
    /// to PKCS #7 in ECB (Electronic CodeBook) mode, and stores the encrypted
    /// data in `Vec<V>`.
    /// 
    /// # Arguments
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
    /// - When `T` is `[u32; NB]` for Rijndael or AES, the output should be at
    ///   least `32 * NB`, and will be only any multiple of `32 * NB`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is an empty `Vec<U>` object `Vec::<U>::new()`, only padding
    ///   bytes will be encrypted, and stored in the `Vec<V>` object `cipher`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, DES_Expanded, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_vec_into_vec(&message, &mut cipher);
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
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.encrypt_vec_into_vec)
    #[inline]
    fn encrypt_vec_into_vec<U, V>(&mut self, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding according
    /// to PKCS #7 in ECB (Electronic CodeBook) mode, and stores the encrypted
    /// data in array `[V; N]`.
    /// 
    /// # Arguments
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
    /// - When `T` is `[u32; NB]` for Rijndael or AES, the output should be at
    ///   least `32 * NB`, and will be only any multiple of `32 * NB`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is an empty `Vec<U>` object `Vec::<U>::new()`, only padding
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_vec_into_array(&message, &mut cipher);
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
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.encrypt_vec_into_array)
    #[inline]
    fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array<U, const N: usize>(&mut self, message: &[U; N], cipher: *mut u8) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #7 in ECB (Electronic CodeBook) mode.
    /// 
    /// # Arguments
    /// - `message` is the data stored in an array `[U; N]` object,
    ///   and is the plaintext to be encrypted.
    /// - `cipher` is a pointer to u8 which is `*mut u8`,
    ///   and is the ciphertext to be stored.
    /// - The size of the memory area which starts at `cipher` and the
    ///   ciphertext will be stored at is assumed to be enough.
    /// - The size of the area for ciphertext should be prepared to be:
    ///   (`length_in_bytes` + 1).next_multiple_of(8) at least when `T` is `u64`,
    ///   (`length_in_bytes` + 1).next_multiple_of(16) at least when `T` is `u128`, and
    ///   (`length_in_bytes` + 1).next_multiple_of(32 * `NB`) at least when `T` is `[u32; NB]`.
    ///   So, it is responsible for you to prepare the `cipher` area big enough!
    /// 
    /// # Output
    /// - This method returns the size of ciphertext including padding bits
    ///   in bytes.
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - When `T` is `[u32; NB]` for Rijndael or AES, the output should be at
    ///   least `32 * NB`, and will be only any multiple of `32 * NB`.
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_array(&message, cipher.as_mut_ptr());
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
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.encrypt_array)
    #[inline]
    fn encrypt_array<U, const N: usize>(&mut self, message: &[U; N], cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt(message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, message: &[U; N], cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode, and stores the
    /// encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
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
    /// - When `T` is `[u32; NB]` for Rijndael or AES, the output should be at
    ///   least `32 * NB`, and will be only any multiple of `32 * NB`.
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_array_into_vec(&message, &mut cipher);
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
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.encrypt_array_into_vec)
    #[inline]
    fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, message: &[U; N], cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, message: &[U; N], cipher: &mut [V; M]) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode, and stores the
    /// encrypted data in array `[V; M]`.
    /// 
    /// # Arguments
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
    /// - When `T` is `[u32; NB]` for Rijndael or AES, the output should be at
    ///   least `32 * NB`, and will be only any multiple of `32 * NB`.
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, ECB_PKCS7 };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    /// 
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let mut cipher = [0_u8; 56];
    /// a_des.encrypt_array_into_array(&message, &mut cipher);
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
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.encrypt_array_into_array)
    #[inline]
    fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, message: &[U; N], cipher: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn decrypt(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64;
    /// Decrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic CodeBook) mode.
    /// 
    /// # Arguments
    /// - `cipher` is a pointer to u8 which is `*const u8`,
    ///   and is the ciphertext to be decrypted.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the ciphertext `cipher` in bytes.
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_des.decrypt(cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.decrypt)
    fn decrypt(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64;

    // fn decrypt_into_vec<U>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    /// Decrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic CodeBook) mode, and stores the decrypted data
    /// in `Vec<U>`.
    /// 
    /// # Arguments
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_into_vec(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.decrypt_into_vec)
    fn decrypt_into_vec<U>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        des_pre_decrypt_into_vec!(message, length_in_bytes, U);
        let len = self.decrypt(cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(len as usize);
        len
    }

    // fn decrypt_into_array<U, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    /// Decrypts the data with the padding defined according to PKCS #7
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
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// 
    /// let mut recovered = [0u8; 56];
    /// let len = a_des.decrypt_into_array(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.decrypt_into_array)
    fn decrypt_into_array<U, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone;

    // fn decrypt_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    /// Decrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic CodeBook) mode, and stores the decrypted data
    /// in String object.
    /// 
    /// # Arguments
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
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// 
    /// let mut recovered = String::new();
    /// a_des.decrypt_into_string(cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B (16 rounds) =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.decrypt_into_string)
    #[inline]
    fn decrypt_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_into_vec(cipher, length_in_bytes, unsafe { message.as_mut_vec() })
    }

    // fn decrypt_vec<U>(&mut self, cipher: &Vec<U>, message: *mut u8) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode.
    /// 
    /// # Arguments
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// 
    /// let mut recovered = vec![0; 55];
    /// a_des.decrypt_vec(&cipher, recovered.as_mut_ptr());
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
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.decrypt_vec)
    #[inline]
    fn decrypt_vec<U>(&mut self, cipher: &Vec<U>, message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt(cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_vec_into_vec<U, V>(&mut self, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding according
    /// to PKCS #7 in ECB (Electronic CodeBook) mode, and stores the decrypted
    /// data in `Vec<V>`.
    /// 
    /// # Arguments
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_vec_into_vec(&cipher, &mut recovered);
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
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.decrypt_vec_into_vec)
    #[inline]
    fn decrypt_vec_into_vec<U, V>(&mut self, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_vec(cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding according
    /// to PKCS #7 in ECB (Electronic CodeBook) mode, and stores the decrypted
    /// data in array `[V; N]`.
    /// 
    /// # Arguments
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
    ///   this method does not perform decryption and returns `zero`.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// 
    /// let mut recovered = [0u8; 56];
    /// let len = a_des.decrypt_vec_into_array(&cipher, &mut recovered);
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
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.decrypt_vec_into_array)
    #[inline]
    fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_array(cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_vec_into_string<U>(&mut self, cipher: &Vec<U>, message: &mut String) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding according
    /// to PKCS #7 in ECB (Electronic CodeBook) mode, and stores the decrypted
    /// data in String object.
    /// 
    /// # Arguments
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
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// 
    /// let mut recovered = String::new();
    /// a_des.decrypt_vec_into_string(&cipher, &mut recovered);
    /// println!("B (16 rounds) =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.decrypt_vec_into_string)
    #[inline]
    fn decrypt_vec_into_string<U>(&mut self, cipher: &Vec<U>, message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt_into_string(cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_array<U, const N: usize>(&mut self, cipher: &[U; N], message: *mut u8) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to PKCS #7 in ECB (Electronic CodeBook) mode.
    /// 
    /// # Arguments
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
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as
    ///   decrypt_array_into_*().
    /// - When `T` is `u64`, `N * U::size_in_bytes()`
    ///   can be only any multiple of `8`.
    /// - When `T` is `u128`, `N * U::size_in_bytes()`
    ///   can be only any multiple of `16`.
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - The padding bits are composed of the bytes that indicate the length of
    ///   the padding bits in bytes according to PKCS #7 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS #7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// 
    /// let mut recovered = vec![0; 55];
    /// let len = a_des.decrypt_array(&cipher, recovered.as_mut_ptr());
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
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.decrypt_array)
    #[inline]
    fn decrypt_array<U, const N: usize>(&mut self, cipher: &[U; N], message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt(cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, cipher: &[U; N], message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode, and stores the
    /// decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// 
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_array_into_vec(&cipher, &mut recovered);
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
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.decrypt_array_into_vec)
    #[inline]
    fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, cipher: &[U; N], message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_vec(cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, cipher: &[U; N], message: &mut [V; M]) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// according to PKCS #7 in ECB (Electronic CodeBook) mode, and stores the
    /// decrypted data in array `[V; M]`.
    /// 
    /// # Arguments
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
    /// - When `T` is `u64`, `N * U::size_in_bytes()`
    ///   can be only any multiple of `8`.
    /// - When `T` is `u128`, `N * U::size_in_bytes()`
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
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// 
    /// let mut recovered = [0u8; 56];
    /// let len = a_des.decrypt_array_into_array(&cipher, &mut recovered);
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
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.decrypt_array_into_array)
    #[inline]
    fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, cipher: &[U; N], message: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_array(cipher.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_array_into_string<U, const N: usize>(&mut self, cipher: &[U; N], message: &mut String) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding according
    /// to PKCS #7 in ECB (Electronic CodeBook) mode, and stores the decrypted
    /// data in a String object.
    /// 
    /// # Arguments
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
    /// - This method assumes that the original plaintext is a string
    ///   in the format of UTF-8.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
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
    /// 
    /// let mut recovered = String::new();
    /// a_des.decrypt_array_into_string(&cipher, &mut recovered);
    /// println!("B (16 rounds) =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ecb_pkcs7/struct.DES_Generic.html#method.decrypt_array_into_string)
    #[inline]
    fn decrypt_array_into_string<U, const N: usize>(&mut self, cipher: &[U; N], message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt_into_string(cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }
}
