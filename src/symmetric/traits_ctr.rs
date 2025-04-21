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
use crate::symmetric::{ des_pre_encrypt_into_vec, des_pre_decrypt_into_vec_no_padding };


pub trait CTR<T> : Sized
{
    // fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    /// Encrypts the data in CTR (CounTeR) mode.
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
    /// - This method returns the size of ciphertext
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
    ///   So, only null string will be stored in the memory area that starts
    ///   from `cipher`.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt(iv, message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt)
    fn encrypt(&mut self, nonce: T, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;

    // fn encrypt_into_vec<U>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data in CTR (CounTeR) mode,
    /// and stores the encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `message` is a pointer to u8 which is `*const u8`,
    ///   and is the plaintext to be encrypted.
    /// - `length_in_bytes` is of `u64`-type,
    ///   and is the length of the plaintext `message` in bytes.
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext
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
    ///   and stored in the `Vec<U>` object `cipher`.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_into_vec)
    fn encrypt_into_vec<U>(&mut self, nonce: T, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        des_pre_encrypt_into_vec!(cipher, length_in_bytes, U);
        let len = self.encrypt(nonce, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8);
        cipher.truncate(len as usize);
        len
    }

    // fn encrypt_into_array<U, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    /// Encrypts the data in CTR (CounTeR) mode,
    /// and stores the encrypted data in array `[U; N]`.
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
    /// - This method returns the size of ciphertext
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
    ///   and stored in the array `[U; N]` object `cipher`.
    /// - If `U::size_in_bytes() * N` is less than `length_in_bytes`'s next
    ///   multiple of 8, this method does not perform encryption and returns
    ///   `zero`.
    /// - If `U::size_in_bytes() * N` is equal to `length_in_bytes`, this
    ///   method performs encryption, fills the array `cipher` with the
    ///   (encrypted) ciphertext, and returns the size of the ciphertext.
    /// - If `U::size_in_bytes() * N` is greater than `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   (encrypted) ciphertext, and then fills the rest of the elements of
    ///   the array `cipher` with zeros, and returns the size of the ciphertext.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_into_array)
    fn encrypt_into_array<U, const N: usize>(&mut self, nonce: T, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone;

    // fn encrypt_str(&mut self, message: &str, cipher: *mut u8) -> u64
    /// Encrypts the data in `str` in CTR (CounTeR) mode.
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
    /// - This method returns the size of ciphertext
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
    /// - If `message` is a null string "", nothing will be stored
    ///   in the memory area that starts from `cipher`.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_str(iv, &message, cipher.as_mut_ptr());
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_str)
    #[inline]
    fn encrypt_str(&mut self, nonce: T, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_str_into_vec<U>(&mut self, message: &str, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data in `str` in CTR (CounTeR) mode,
    /// and stores the encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `message` is a `str` object, and is the plaintext to be encrypted.
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is a null string "", nothing will be stored
    ///   in the `Vec<U>` object `cipher`.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_str_into_vec(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_str_into_vec)
    #[inline]
    fn encrypt_str_into_vec<U>(&mut self, nonce: T, message: &str, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_str_into_array<U, const N: usize>(&mut self, message: &str, cipher: &mut [U; N]) -> u64
    /// Encrypts the data in `str` in CTR (CounTeR) mode,
    /// and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `message` is a `str` object, and is the plaintext to be encrypted.
    /// - `cipher` is an array `[U; N]` object,
    ///   and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is a null string "", nothing will be stored
    ///   in the array `[U; N]` object `cipher`.
    /// - If `U::size_in_bytes() * N` is less than `message.len()`'s next
    ///   multiple of 8,
    ///   this method does not perform encryption and returns `zero`.
    /// - If `U::size_in_bytes() * N` is equal to `message.len()`, this
    ///   method performs encryption, fills the array `cipher` with the
    ///   (encrypted) ciphertext, and returns the size of the ciphertext.
    /// - If `U::size_in_bytes() * N` is greater than `message.len()`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   (encrypted) ciphertext, and then fills the rest of the elements of
    ///   the array `cipher` with zeros, and returns the size of the ciphertext.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_str_into_array(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_str_into_array)
    #[inline]
    fn encrypt_str_into_array<U, const N: usize>(&mut self, nonce: T, message: &str, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string(&mut self, message: &String, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a String object in CTR (CounTeR) mode.
    /// 
    /// # Arguments
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
    /// - This method returns the size of ciphertext
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
    /// - If `message` is a null string String::new(), nothing will be stored
    ///   in the memory area that starts from `cipher`.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_string(iv, &message, cipher.as_mut_ptr());
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_string)
    #[inline]
    fn encrypt_string(&mut self, nonce: T, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt(nonce, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string_into_vec<U>(&mut self, message: &String, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data stored in a String object in CTR (CounTeR) mode,
    /// and stores the encrypted data in `Vec<U>`.
    /// 
    /// # Arguments
    /// - `message` is a String object, and is the plaintext to be encrypted.
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is a null string String::new(), nothing will be stored
    ///   in the `Vec<U>` object `cipher`.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_string_into_vec(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_string_into_vec)
    #[inline]
    fn encrypt_string_into_vec<U>(&mut self, nonce: T, message: &String, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(nonce, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_string_into_array<U, const N: usize>(&mut self, message: &String, cipher: &mut [U; N]) -> u64
    /// Encrypts the data stored in a String object in CTR (CounTeR) mode,
    /// and stores the encrypted data in array `[U; N]`.
    /// 
    /// # Arguments
    /// - `message` is a String object, and is the plaintext to be encrypted.
    /// - `cipher` is an array `[U; N]` object,
    ///   and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is a null string String::new(), nothing will be stored
    ///   in the array `[U; N]` object `cipher`.
    /// - If `U::size_in_bytes() * N` is less than `message.len()`'s next
    ///   multiple of 8,
    ///   this method does not perform encryption and returns `zero`.
    /// - If `U::size_in_bytes() * N` is equal to `message.len()`, this
    ///   method performs encryption, fills the array `cipher` with the
    ///   (encrypted) ciphertext, and returns the size of the ciphertext.
    /// - If `U::size_in_bytes() * N` is greater than `message.len()`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   (encrypted) ciphertext, and then fills the rest of the elements of
    ///   the array `cipher` with zeros, and returns the size of the ciphertext.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.".to_string();
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_string_into_array(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_string_into_array)
    #[inline]
    fn encrypt_string_into_array<U, const N: usize>(&mut self, nonce: T, message: &String, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(nonce, message.as_ptr(), message.len() as u64, cipher)
    }

    // fn encrypt_vec<U>(&mut self, message: &Vec<U>, cipher: *mut u8) -> u64
    /// Encrypts the data stored in a `Vec<U>` object in CTR (CounTeR) mode.
    /// 
    /// # Arguments
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
    /// - This method returns the size of ciphertext
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
    /// - If `message` is an empty Vec<U> object Vec::<U>::new(),
    ///   nothing will be stored in the memory area that starts from `cipher`.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_vec(iv, &message, cipher.as_mut_ptr());
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_vec)
    #[inline]
    fn encrypt_vec<U>(&mut self, nonce: T, message: &Vec<U>, cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt(nonce, message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_vec_into_vec<U, V>(&mut self, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in a `Vec<U>` object in CTR (CounTeR) mode,
    /// and stores the encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `message` is a `Vec<U>` object, and is the plaintext to be encrypted.
    /// - `cipher` is a `Vec<V>` object, and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is an empty Vec<U> object Vec::<U>::new(),
    ///   nothing will be stored in the `Vec<V>` object `cipher`.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_vec_into_vec(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_vec_into_vec)
    #[inline]
    fn encrypt_vec_into_vec<U, V>(&mut self, nonce: T, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(nonce, message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    /// Encrypts the data stored in a `Vec<U>` object in CTR (CounTeR) mode,
    /// and stores the encrypted data in array `[V; N]`.
    /// 
    /// # Arguments
    /// - `message` is a `Vec<U>` object, and is the plaintext to be encrypted.
    /// - `cipher` is an array `[V; N]` object,
    ///   and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is an empty Vec<U> object Vec::<U>::new(),
    ///   nothing will be stored in the array `[V; N]` object `cipher`.
    /// - If `V::size_in_bytes() * N` is less than 
    ///   `U::size_in_bytes() * message.len()`'s next multiple of 8,
    ///   this method does not perform encryption and returns `zero`.
    /// - If `V::size_in_bytes() * N` is equal to
    ///   `U::size_in_bytes() * message.len()`, this method performs encryption,
    ///   fills the array `cipher` with the (encrypted) ciphertext, and returns
    ///   the size of the ciphertext.
    /// - If `V::size_in_bytes() * N` is greater than
    ///   `U::size_in_bytes() * message.len()`, this method performs encryption,
    ///   fills the array `cipher` with the (encrypted) ciphertext, and then
    ///   fills the rest of the elements of the array `cipher` with zeros,
    ///   and returns the size of the ciphertext.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let message = unsafe { message.to_string().as_mut_vec().clone() };
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_vec_into_array(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_vec_into_array)
    #[inline]
    fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, nonce: T, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(nonce, message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array<U, const N: usize>(&mut self, message: &[U; N], cipher: *mut u8) -> u64
    /// Encrypts the data stored in an array `[U; N]` object
    /// in CTR (CounTeR) mode.
    /// 
    /// # Arguments
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
    /// - This method returns the size of ciphertext
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
    ///   and stored in the memory area that starts from `cipher`.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_array(iv, &message, cipher.as_mut_ptr());
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_array)
    #[inline]
    fn encrypt_array<U, const N: usize>(&mut self, nonce: T, message: &[U; N], cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt(nonce, message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, message: &[U; N], cipher: &mut Vec<V>) -> u64
    /// Encrypts the data stored in an array `[U; N]` object in CTR (CounTeR)
    /// mode, and stores the encrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `message` is an array `[U; N]` object, and is the plaintext to be
    ///   encrypted.
    /// - `cipher` is a `Vec<V>` object, and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is an empty array `[U; N]` object [U; 0],
    ///   nothing will be stored in the `Vec<U>` object `cipher`.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_array_into_vec(iv, &message, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_array_into_vec)
    #[inline]
    fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, nonce: T, message: &[U; N], cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(nonce, message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, message: &[U; N], cipher: &mut [V; M]) -> u64
    /// Encrypts the data stored in an array `[U; N]` object in CTR (CounTeR)
    /// mode, and stores the encrypted data in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `message` is an array `[U; N]` object,
    ///   and is the plaintext to be encrypted.
    /// - `cipher` is an array `[V; M]` object,
    ///   and is the ciphertext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of ciphertext
    /// - When `T` is `u64`, the output should be at least `8`,
    ///   and will be only any multiple of `8`.
    /// - When `T` is `u128`, the output should be at least `16`,
    ///   and will be only any multiple of `16`.
    /// - If this method returns `zero`,
    ///   it means this method failed in encryption.
    /// 
    /// # Features
    /// - If `message` is an empty array `[U; N]` object [U; 0],
    ///   nothing will be stored in the array `[V; M]` object `cipher`.
    /// - If `V::size_in_bytes() * M` is less than 
    ///   `U::size_in_bytes() * N`'s next multiple of 8,
    ///   this method does not perform encryption and returns `zero`.
    /// - If `V::size_in_bytes() * M` is equal to
    ///   `U::size_in_bytes() * N`, this method performs encryption,
    ///   fills the array `cipher` with the (encrypted) ciphertext, and returns
    ///   the size of the ciphertext.
    /// - If `V::size_in_bytes() * M` is greater than
    ///   `U::size_in_bytes() * N`, this method performs encryption,
    ///   fills the array `cipher` with the (encrypted) ciphertext, and then
    ///   fills the rest of the elements of the array `cipher` with zeros,
    ///   and returns the size of the ciphertext.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let mes = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", mes);
    /// let mut message = [0_u8; 55];
    /// message.copy_from_slice(unsafe { mes.to_string().as_mut_vec() });
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_array_into_array(iv, &message, &mut cipher);
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.encrypt_array_into_array)
    #[inline]
    fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, nonce: T, message: &[U; N], cipher: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(nonce, message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
    }

    // fn decrypt(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64;
    /// Decrypts the data in CTR (CounTeR) mode.
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
    ///   `length_in_bytes`.
    ///   So, it is responsible for you to prepare the `message` area big enough!
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `length_in_bytes` is greater than
    ///   `0`, it means that this method failed in decryption.
    /// - If this method returns `zero`, and `length_in_bytes` is `0`,
    ///   it means either that this method failed in decryption or
    ///   that the original plaintext is empty data.
    ///   Then, you will have to check whether or not it failed by the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as
    ///   decrypt_*_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = vec![0; 55];
    /// a_des.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
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
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt)
    #[inline]
    fn decrypt(&mut self, nonce: T, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        self.encrypt(nonce, cipher, length_in_bytes, message)
    }

    // fn decrypt_into_vec<U>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    /// Decrypts the data in CTR (CounTeR) mode,
    /// and stores the decrypted data in `Vec<U>`.
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
    ///   `0`, it means that this method failed in decryption.
    /// - If this method returns `zero`, and `length_in_bytes` is `0`,
    ///   it means either that this method failed in decryption or
    ///   that the original plaintext is empty data.
    ///   Then, you will have to check whether or not it failed by the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the plaintext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_into_vec(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_into_vec)
    fn decrypt_into_vec<U>(&mut self, nonce: T, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        des_pre_decrypt_into_vec_no_padding!(message, length_in_bytes, U);
        let len = self.decrypt(nonce, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(len as usize);
        len
    }

    // fn decrypt_into_array<U, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    /// Decrypts the data in CTR (CounTeR) mode,
    /// and stores the encrypted data in array `[U; N]`.
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
    ///   `0`, it means that this method failed in decryption.
    /// - If this method returns `zero`, and `length_in_bytes` is `0`,
    ///   it means either that this method failed in decryption or
    ///   that the original plaintext is empty data.
    ///   Then, you will have to check whether or not it failed by the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - This method is useful to use in hybrid programming with C/C++.
    /// - If `U::size_in_bytes() * N` is less than `length_in_bytes` - 1,
    ///   this method does not perform decryption and returns `zero`.
    /// - If `U::size_in_bytes() * N` is greater than or equal to
    ///   `length_in_bytes` - 1, this method performs decryption, fills the
    ///   array `message` with the derypted plaintext, and then fills the rest
    ///   of the elements of the array `message` with zeros if any, and returns
    ///   the size of the plaintext.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = [0u8; 56];
    /// let len = a_des.decrypt_into_array(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
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
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_into_array)
    fn decrypt_into_array<U, const N: usize>(&mut self, nonce: T, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone;

    // fn decrypt_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    /// Decrypts the data in CTR (CounTeR) mode,
    /// and stores the decrypted data in String object.
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
    ///   `0`, it means that this method failed in decryption.
    /// - If this method returns `zero`, and `length_in_bytes` is `0`,
    ///   it means either that this method failed in decryption or
    ///   that the original plaintext is empty String.
    ///   Then, you will have to check whether or not it failed by the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - This method is useful to use in hybrid programming with C/C++.
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
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_into_string(iv, cipher.as_ptr(), cipher.len() as u64, &mut recovered);
    /// println!("B (16 rounds) =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_into_string)
    #[inline]
    fn decrypt_into_string(&mut self, nonce: T, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_into_vec(nonce, cipher, length_in_bytes, unsafe { message.as_mut_vec() })
    }

    // fn decrypt_vec<U>(&mut self, cipher: &Vec<U>, message: *mut u8) -> u64
    /// Decrypts the data stored in a `Vec<U>` object in CTR (CounTeR) mode.
    /// 
    /// # Arguments
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be decrypted.
    /// - `message` is a pointer to u8 which is `*mut u8`,
    ///   and is the plaintext to be stored.
    /// - The size of the memory area which starts at `message` and the
    ///   plaintext will be stored at is assumed to be enough.
    /// - The size of the area for plaintext should be prepared to be:
    ///   `length_in_bytes`.
    ///   So, it is responsible for you to prepare the `message` area big enough!
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `cipher.len()` is greater than `0`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `cipher.len()` is `0`,
    ///   it means either that this method failed in decryption
    ///   or that the original plaintext is empty data.
    ///   Then, you will have to check whether or not it failed by the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as
    ///   decrypt_vec_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = vec![0; 55];
    /// a_des.decrypt_vec(iv, &cipher, recovered.as_mut_ptr());
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
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_vec)
    #[inline]
    fn decrypt_vec<U>(&mut self, nonce: T, cipher: &Vec<U>, message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_vec_into_vec<U, V>(&mut self, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in a `Vec<U>` object in CTR (CounTeR) mode,
    /// and stores the decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be decrypted.
    /// - `message` is a `Vec<V>` object, and is the plaintext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `cipher.len()` is greater than `0`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `cipher.len()` is `0`,
    ///   it means either that this method failed in decryption
    ///   or that the original plaintext is empty data.
    ///   Then, you will have to check whether or not it failed by the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// You don't have to worry about whether or not the size of the memory
    /// area where the plaintext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_vec_into_vec(iv, &cipher, &mut recovered);
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
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_vec_into_vec)
    #[inline]
    fn decrypt_vec_into_vec<U, V>(&mut self, nonce: T, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_vec(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    /// Decrypts the data stored in a `Vec<U>` object in CTR (CounTeR) mode,
    /// and stores the decrypted data in array `[V; N]`.
    /// 
    /// # Arguments
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be decrypted.
    /// - `message` is an array `[V; N]` object,
    ///   and is the plaintext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `cipher.len()` is greater than `0`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `cipher.len()` is `0`,
    ///   it means either that this method failed in decryption
    ///   or that the original plaintext is empty data.
    ///   Then, you will have to check whether or not it failed by the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - If `V::size_in_bytes() * N` is greater than or equal to
    ///   `U::size_in_bytes() * cipher.len()`, this method performs
    ///   decryption, fills the array `message` with the derypted plaintext,
    ///   and then fills the rest of the elements of the array `message`
    ///   with zeros if any, and returns the size of the plaintext.
    /// - If `V::size_in_bytes() * N` is less than 
    ///   `U::size_in_bytes() * cipher.len()`,
    ///   this method does not perform decryption and returns `zero`.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = [0u8; 56];
    /// let len = a_des.decrypt_vec_into_array(iv, &cipher, &mut recovered);
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
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_vec_into_array)
    #[inline]
    fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, nonce: T, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_array(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_vec_into_string<U>(&mut self, cipher: &Vec<U>, message: &mut String) -> u64
    /// Decrypts the data stored in a `Vec<U>` object in CTR (CounTeR) mode,
    /// and stores the decrypted data in String object.
    /// 
    /// # Arguments
    /// - `cipher` is a `Vec<U>` object, and is the ciphertext to be decrypted.
    /// - `message` is an String object, and is the plaintext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `cipher.len()` is greater than `0`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `cipher.len()` is `0`,
    ///   it means either that this method failed in decryption
    ///   or that the original plaintext is an empty String.
    ///   Then, you will have to check whether or not it failed by the method
    ///   `is_successful()` or `is_failed()`.
    /// 
    /// # Features
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
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    /// println!("B (16 rounds) =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 2 for Expanded case for 128 rounds
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES_Expanded::<128, 0x_8103_8103_8103_8103_8103_8103_8103_8103_u128>::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (128 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "FA 29 57 1F C9 60 9F 98 4C 48 14 62 7B 72 B4 D6 5D 09 1F C8 FB CE 1C 86 92 DF E2 3E 3F 91 75 62 F8 47 77 BB 86 8A 7D F0 BF E9 E4 52 EC 4D 42 F6 D4 7B 41 19 43 C5 5B ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    /// println!("B (128 rounds) =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 3 for Expanded case for 0 rounds which means that key is meaningless
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES_Expanded, CTR };
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
    /// println!("IV =	{}", iv);
    /// let mut cipher1 = Vec::<u8>::new();
    /// let mut cipher2 = Vec::<u8>::new();
    /// c_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher1);
    /// d_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher2);
    /// print!("C (0 rounds) =\t");
    /// for c in cipher1.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher1.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    /// print!("D (0 rounds) =\t");
    /// for c in cipher2.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher2.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "58 ED BA 3F 6E 10 CC 9F 76 E4 F3 25 68 1C 82 9A 38 C4 F5 2F 26 16 9E 98 7B F7 FF 2F 26 01 84 98 39 EB FF 2A 70 10 82 8E 3B E2 F4 2F 26 01 84 98 34 E6 FB 39 72 1D C2 ");
    ///
    /// let mut recovered1 = String::new();
    /// let mut recovered2 = String::new();
    /// c_des.decrypt_vec_into_string(iv, &cipher1, &mut recovered1);
    /// d_des.decrypt_vec_into_string(iv, &cipher2, &mut recovered2);
    /// println!("B1 (0 rounds) =\t{}", recovered1);
    /// println!("B2 (0 rounds) =\t{}", recovered2);
    /// assert_eq!(recovered1, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered2, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered1, message);
    /// assert_eq!(recovered2, message);
    /// assert_eq!(recovered1, recovered2);
    /// ```
    ///
    /// # Example 4 for Normal case for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 5 for Normal case for the message shorter than 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "7 bytes";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "4E 1E 2D 3C 7C BA C2 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "7 bytes");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 6 for Normal case for the message of 8 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "I am OK.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 1E 2E 28 28 90 FA 32 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "I am OK.");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 7 for Normal case for the message longer than 8 bytes and shorter than 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "PARK Youngho";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "29 7F 1D 0E 28 86 DE 69 DB DE 39 A7 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "PARK Youngho");
    /// assert_eq!(recovered, message);
    /// ```
    ///
    /// # Example 8 for Normal case for the message of 16 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "고맙습니다.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = Vec::<u8>::new();
    /// a_des.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "93 8D EF AE AF 46 5D 96 00 52 DA 40 78 B2 14 F5 ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
    /// println!("B =\t{}", recovered);
    /// assert_eq!(recovered, "고맙습니다.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_vec_into_string)
    #[inline]
    fn decrypt_vec_into_string<U>(&mut self, nonce: T, cipher: &Vec<U>, message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt_into_string(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_array<U, const N: usize>(&mut self, cipher: &[U; N], message: *mut u8) -> u64
    /// Decrypts the data stored in an array `[U; N]` object
    /// in CTR (CounTeR) mode.
    /// 
    /// # Arguments
    /// - `cipher` is the data stored in an array `[U; N]` object,
    ///   and is the ciphertext to be decrypted.
    /// - `message` is a pointer to u8 which is `*mut u8`,
    ///   and is the plaintext to be stored.
    /// - The size of the memory area which starts at `message` and the
    ///   plaintext will be stored at is assumed to be enough.
    /// - The size of the area for plaintext should be prepared to be:
    ///   `N * U::size_in_bytes()`.
    ///   So, it is responsible for you to prepare the `message` area big enough!
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `N` is greater than `0`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `N` is `0`,
    ///   it means either that this method failed in decryption
    ///   or that the original plaintext is empty array [U; 0].
    ///   Then, you will have to check whether or not it failed by the
    ///   method `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - You are not encouraged to use this method in pure Rust programming.
    ///   Instead, use other safer methods such as
    ///   decrypt_array_into_*().
    /// - This method is useful to use in hybrid programming with C/C++.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = vec![0; 55];
    /// let len = a_des.decrypt_array(iv, &cipher, recovered.as_mut_ptr());
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
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_array)
    #[inline]
    fn decrypt_array<U, const N: usize>(&mut self, nonce: T, cipher: &[U; N], message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, cipher: &[U; N], message: &mut Vec<V>) -> u64
    /// Decrypts the data stored in an array `[U; N]` object in CTR (CounTeR)
    /// mode, and stores the decrypted data in `Vec<V>`.
    /// 
    /// # Arguments
    /// - `cipher` is an array `[U; N]` object, and is the ciphertext to be
    ///   decrypted.
    /// - `message` is a `Vec<V>` object, and is the plaintext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `N` is greater than `0`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `N` is `0`,
    ///   it means either that this method failed in decryption
    ///   or that the original plaintext is empty array [U; 0].
    ///   Then, you will have to check whether or not it failed by the
    ///   method `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// You don't have to worry about whether or not the size of the memory
    /// area where the ciphertext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = Vec::<u8>::new();
    /// a_des.decrypt_array_into_vec(iv, &cipher, &mut recovered);
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
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_array_into_vec)
    #[inline]
    fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, nonce: T, cipher: &[U; N], message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_vec(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, cipher: &[U; N], message: &mut [V; M]) -> u64
    /// Decrypts the data stored in an array `[U; N]` object in CTR (CounTeR)
    /// mode, and stores the decrypted data in array `[V; M]`.
    /// 
    /// # Arguments
    /// - `cipher` is an array `[U; N]` object,
    ///   and is the ciphertext to be decrypted.
    /// - `message` is an array `[V; M]` object,
    ///   and is the plaintext to be stored.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `N` is greater than `0`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `N` is `0`,
    ///   it means either that this method failed in decryption
    ///   or that the original plaintext is empty array [U; 0].
    ///   Then, you will have to check whether or not it failed by the
    ///   method `is_successful()` or `is_failed()`.
    /// 
    /// # Features
    /// - If `V::size_in_bytes() * M` is less than `U::size_in_bytes() * N`,
    ///   this method does not perform decryption and returns `zero`.
    /// - If `V::size_in_bytes() * M` is greater than or qual to
    ///   `U::size_in_bytes() * N`, this method performs decryption,
    ///   fills the array `message` with the derypted plaintext, and then
    ///   fills the rest of the elements of the array `message` with zeros
    ///   if any, and returns the size of the plaintext.
    /// - You don't have to worry about whether or not the size of the memory
    ///   area where the ciphertext will be stored is enough.
    /// 
    /// # For DES and its variants
    /// ## Example 1 for Normal case
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = [0u8; 56];
    /// let len = a_des.decrypt_array_into_array(iv, &cipher, &mut recovered);
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
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_array_into_array)
    #[inline]
    fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, nonce: T, cipher: &[U; N], message: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_array(nonce, cipher.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, message)
    }

    // fn decrypt_array_into_string<U, const N: usize>(&mut self, cipher: &[U; N], message: &mut String) -> u64
    /// Decrypts the data stored in an array `[U; N]` object in CTR (CounTeR)
    /// mode, and stores the decrypted data in a String object.
    /// 
    /// # Arguments
    /// - `cipher` is an array `[U; N]` object,
    ///   and is the ciphertext to be decrypted.
    /// - `message` is a String object, and is the plaintext to be encrypted.
    /// 
    /// # Output
    /// - This method returns the size of plaintext in bytes.
    /// - If this method returns `zero`, and `N` is greater than `0`,
    ///   it means that this method failed in decryption.
    /// - If this method returns `zero`, and `N` is `0`,
    ///   it means either that this method failed in decryption
    ///   or that the original plaintext is empty array [U; 0].
    ///   Then, you will have to check whether or not it failed by the
    ///   method `is_successful()` or `is_failed()`.
    /// 
    /// # Features
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
    /// use cryptocol::symmetric::{ DES, CTR };
    /// 
    /// let key = 0x_1234567890ABCDEF_u64;
    /// println!("K =\t{:#016X}", key);
    /// let mut a_des = DES::new_with_key_u64(key);
    ///
    /// let message = "In the beginning God created the heavens and the earth.";
    /// println!("M =\t{}", message);
    /// let iv = 0x_FEDCBA0987654321_u64;
    /// println!("IV =	{}", iv);
    /// let mut cipher = [0_u8; 55];
    /// a_des.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, &mut cipher);
    /// print!("C (16 rounds) =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "30 50 6F 31 60 BA 91 7E D0 DE 38 A6 FD 50 DE BC F5 BF CA 3D A4 15 03 C5 2A 8B 35 94 F9 1B 0B 64 FE C4 32 98 5B 3B 20 FC DE B6 88 E4 BD 4E 7D 8E 5A E8 41 79 F0 DC 2E ");
    ///
    /// let mut recovered = String::new();
    /// a_des.decrypt_array_into_string(iv, &cipher, &mut recovered);
    /// println!("B (16 rounds) =\t{}", recovered);
    /// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
    /// assert_eq!(recovered, message);
    /// ```
    /// 
    /// ## For more examples,
    /// click [here](./documentation/des_ctr/struct.DES_Generic.html#method.decrypt_array_into_string)
    #[inline]
    fn decrypt_array_into_string<U, const N: usize>(&mut self, nonce: T, cipher: &[U; N], message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt_into_string(nonce, cipher.as_ptr() as *const u8, (cipher.len() as u32 * U::size_in_bytes()) as u64, message)
    }
}
