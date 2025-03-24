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


#[allow(non_camel_case_types)]
pub trait CBC_ISO<T> : Sized
{
    // fn encrypt(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;
    /// Encrypts the data with the padding defined according to ISO 7816-4
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.encrypt)
    fn encrypt(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;

    // fn encrypt_into_vec<U>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data with the padding defined according to ISO 7816-4
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.encrypt_into_vec)
    fn encrypt_into_vec<U>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        des_pre_encrypt_into_vec!(cipher, length_in_bytes, U);
        let len = self.encrypt(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8);
        cipher.truncate(len as usize);
        len
    }

    // fn encrypt_into_array<U, const N: usize>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    /// Encrypts the data with the padding defined according to ISO 7816-4
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.encrypt_into_array)
    fn encrypt_into_array<U, const N: usize>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone;

    // fn encrypt_str(&mut self, iv: T, message: &str, cipher: *mut u8) -> u64
    /// Encrypts the data in `str` with the padding defined
    /// according to ISO 7816-4 in CBC (Cipher Block Chaining) mode.
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.encrypt_str)
    #[inline]
    fn encrypt_str(&mut self, iv: T, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_str_into_vec<U>(&mut self, iv: T, message: &str, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data in `str` with the padding defined according to ISO 7816-4
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.encrypt_str_into_vec)
    #[inline]
    fn encrypt_str_into_vec<U>(&mut self, iv: T, message: &str, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_str_into_array<U, const N: usize>(&mut self, iv: T, message: &str, cipher: &mut [U; N]) -> u64
    /// Encrypts the data in `str` with the padding defined according to ISO 7816-4
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.encrypt_str_into_array)
    #[inline]
    fn encrypt_str_into_array<U, const N: usize>(&mut self, iv: T, message: &str, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string(&mut self, iv: T, message: &String, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a String object with the padding according
    /// to ISO 7816-4 in CBC (Cipher Block Chaining) mode.
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.encrypt_string)
    #[inline]
    fn encrypt_string(&mut self, iv: T, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string_into_vec<U>(&mut self, iv: T, message: &String, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data stored in a String object with the padding according
    /// to ISO 7816-4 in CBC (Cipher Block Chaining) mode, and stores the encrypted
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.encrypt_string_into_vec)
    #[inline]
    fn encrypt_string_into_vec<U>(&mut self, iv: T, message: &String, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string_into_array<U, const N: usize>(&mut self, iv: T, message: &String, cipher: &mut [U; N]) -> u64
    /// Encrypts the data stored in a String object with the padding according
    /// to ISO 7816-4 in CBC (Cipher Block Chaining) mode, and stores the encrypted
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.encrypt_string_into_array)
    #[inline]
    fn encrypt_string_into_array<U, const N: usize>(&mut self, iv: T, message: &String, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_vec<U>(&mut self, iv: T, message: &Vec<U>, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to ISO 7816-4 in CBC (Cipher Block Chaining) mode.
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.encrypt_vec)
    #[inline]
    fn encrypt_vec<U>(&mut self, iv: T, message: &Vec<U>, cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt(iv, message.as_ptr() as *const u8, (message.len() * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_vec_into_vec<U, V>(&mut self, iv: T, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding according
    /// to ISO 7816-4 in CBC (Cipher Block Chaining) mode, and stores the encrypted
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.encrypt_vec_into_vec)
    #[inline]
    fn encrypt_vec_into_vec<U, V>(&mut self, iv: T, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(iv, message.as_ptr() as *const u8, (message.len() * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: T, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    /// Encrypts the data stored in a `Vec<U>` object with the padding according
    /// to ISO 7816-4 in CBC (Cipher Block Chaining) mode, and stores the encrypted
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.encrypt_vec_into_array)
    #[inline]
    fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: T, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(iv, message.as_ptr() as *const u8, (message.len() * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array<U, const N: usize>(&mut self, iv: T, message: &[U; N], cipher: *mut u8) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to ISO 7816-4 in CBC (Cipher Block Chaining) mode.
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.encrypt_array)
    #[inline]
    fn encrypt_array<U, const N: usize>(&mut self, iv: T, message: &[U; N], cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt(iv, message.as_ptr() as *const u8, (N * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: T, message: &[U; N], cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// according to ISO 7816-4 in CBC (Cipher Block Chaining) mode, and stores the
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.encrypt_array_into_vec)
    #[inline]
    fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: T, message: &[U; N], cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(iv, message.as_ptr() as *const u8, (N * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: T, message: &[U; N], cipher: &mut [V; M]) -> u64
    /// Encrypts the data stored in an array `[U; N]` object with the padding
    /// according to ISO 7816-4 in CBC (Cipher Block Chaining) mode, and stores the
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.encrypt_array_into_array)
    #[inline]
    fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: T, message: &[U; N], cipher: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(iv, message.as_ptr() as *const u8, (N * U::size_in_bytes()) as u64, cipher)
    }

    // fn decrypt(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64;
    /// Decrypts the data with the padding defined according to ISO 7816-4
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.decrypt)
    fn decrypt(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64;

    // fn decrypt_into_vec<U>(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    /// Decrypts the data with the padding defined according to ISO 7816-4
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.decrypt_into_vec)
    fn decrypt_into_vec<U>(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        des_pre_decrypt_into_vec!(message, length_in_bytes, U);
        let len = self.decrypt(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(len as usize);
        len
    }

    // fn decrypt_into_array<U, const N: usize>(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    /// Decrypts the data with the padding defined according to ISO 7816-4
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.decrypt_into_array)
    fn decrypt_into_array<U, const N: usize>(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone;

    // fn decrypt_into_string(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    /// Decrypts the data with the padding defined according to ISO 7816-4
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.decrypt_into_string)
    #[inline]
    fn decrypt_into_string(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_into_vec(iv, cipher, length_in_bytes, unsafe { message.as_mut_vec() })
    }

    // fn decrypt_vec<U>(&mut self, iv: T, cipher: &Vec<U>, message: *mut u8) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding defined
    /// according to ISO 7816-4 in CBC (Cipher Block Chaining) mode.
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.decrypt_vec)
    #[inline]
    fn decrypt_vec<U>(&mut self, iv: T, cipher: &Vec<U>, message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_vec_into_vec<U, V>(&mut self, iv: T, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding according
    /// to ISO 7816-4 in CBC (Cipher Block Chaining) mode, and stores the decrypted
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.decrypt_vec_into_vec)
    #[inline]
    fn decrypt_vec_into_vec<U, V>(&mut self, iv: T, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: T, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding according
    /// to ISO 7816-4 in CBC (Cipher Block Chaining) mode, and stores the decrypted
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.decrypt_vec_into_array)
    #[inline]
    fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: T, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_array(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_vec_into_string<U>(&mut self, iv: T, cipher: &Vec<U>, message: &mut String) -> u64
    /// Decrypts the data stored in a `Vec<U>` object with the padding according
    /// to ISO 7816-4 in CBC (Cipher Block Chaining) mode, and stores the decrypted
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.decrypt_vec_into_string)
    #[inline]
    fn decrypt_vec_into_string<U>(&mut self, iv: T, cipher: &Vec<U>, message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_array<U, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: *mut u8) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// defined according to ISO 7816-4 in CBC (Cipher Block Chaining) mode.
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.decrypt_array)
    #[inline]
    fn decrypt_array<U, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// according to ISO 7816-4 in CBC (Cipher Block Chaining) mode, and stores the
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.decrypt_array_into_vec)
    #[inline]
    fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut [V; M]) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding
    /// according to ISO 7816-4 in CBC (Cipher Block Chaining) mode, and stores the
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.decrypt_array_into_array)
    #[inline]
    fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_array(iv, cipher.as_ptr() as *const u8, (N * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_array_into_string<U, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut String) -> u64
    /// Decrypts the data stored in an array `[U; N]` object with the padding according
    /// to ISO 7816-4 in CBC (Cipher Block Chaining) mode, and stores the decrypted
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
    ///   the padding bits in bytes according to ISO 7816-4 defined in RFC 5652.
    /// - For more information about the padding bits according to ISO 7816-4,
    ///   Read [here](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CBC_ISO };
    /// 
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_cbc_iso/struct.DES_Generic.html#method.decrypt_array_into_string)
    #[inline]
    fn decrypt_array_into_string<U, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }
}
