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
pub trait ECB_PKCS7<T> : Sized
{
    // fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    /// Encrypts the data with the padding defined according to PKCS #7
    /// in ECB (Electronic Code Book) mode.
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
    ///   encrypt_*_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `length_in_bytes` is `0`, it means the message is null string.
    ///   So, only padding bytes will be encrypted,
    ///   and stored in the memory area that starts from `cipher`.
    /// - The padding bits composed of the bytes that indicate the length of
    ///   the padding bits.
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
    fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;

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
    ///   the padding bits.
    /// - For more information about the padding bits according to PKCS#7,
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
    ///   the padding bits.
    /// - For more information about the padding bits according to PKCS#7,
    ///   Read [here](https://node-security.com/posts/cryptography-pkcs-7-padding/).
    /// 
    /// # Features
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

    #[inline]
    fn encrypt_str(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_str_into_vec<U>(&mut self, message: &str, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_str_into_array<U, const N: usize>(&mut self, message: &str, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_string(&mut self, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_string_into_vec<U>(&mut self, message: &String, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_string_into_array<U, const N: usize>(&mut self, message: &String, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_vec<U>(&mut self, message: &Vec<U>, cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt(message.as_ptr() as *const u8, (message.len() * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_vec_into_vec<U, V>(&mut self, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(message.as_ptr() as *const u8, (message.len() * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(message.as_ptr() as *const u8, (message.len() * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_array<U, const N: usize>(&mut self, message: &[U; N], cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt(message.as_ptr() as *const u8, (N * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, message: &[U; N], cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(message.as_ptr() as *const u8, (N * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, message: &[U; N], cipher: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(message.as_ptr() as *const u8, (N * U::size_in_bytes()) as u64, cipher)
    }


    fn decrypt(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64;

    fn decrypt_into_vec<U>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        des_pre_decrypt_into_vec!(message, length_in_bytes, U);
        let len = self.decrypt(cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(len as usize);
        len
    }

    fn decrypt_into_array<U, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone;

    #[inline]
    fn decrypt_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_into_vec(cipher, length_in_bytes, unsafe { message.as_mut_vec() })
    }

    #[inline]
    fn decrypt_vec<U>(&mut self, cipher: &Vec<U>, message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt(cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_vec_into_vec<U, V>(&mut self, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_vec(cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_array(cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_vec_into_string<U>(&mut self, cipher: &Vec<U>, message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt_into_string(cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_array<U, const N: usize>(&mut self, cipher: &[U; N], message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt(cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, cipher: &[U; N], message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_vec(cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, cipher: &[U; N], message: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_array(cipher.as_ptr() as *const u8, (N * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_array_into_string<U, const N: usize>(&mut self, cipher: &[U; N], message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt_into_string(cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }
}
