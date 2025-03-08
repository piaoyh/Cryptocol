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
pub trait PCBC_PKCS7<T> : Sized
{
    fn encrypt(&mut self, iv: T, from: *const u8, length_in_bytes: u64, to: *mut u8) -> u64;

    // fn encrypt_into_vec<U>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    /// Encrypts the data without padding.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and pushed into the vector `cipher`.
    fn encrypt_into_vec<U>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        des_pre_encrypt_into_vec!(cipher, length_in_bytes, U);
        let len = self.encrypt(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8);
        cipher.truncate(len as usize);
        len
    }

    // fn encrypt_into_array<U, const N: usize>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    /// Encrypts the data with the padding defined in PKCS #7.
    /// 
    /// # Features
    /// - If `length_in_bytes` is `0`, only padding bytes will be encrypted,
    ///   and stored into the array `cipher`.
    /// - If `N` is less than the next multiple of 8 from `length_in_bytes`,
    ///   this method does not perform encryption and returns `false`.
    /// - If `N` is equal to the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and returns `true`.
    /// - If `N` is greater than the next multiple of 8 from `length_in_bytes`,
    ///   this method performs encryption, fills the array `cipher` with the
    ///   encrypted ciphertext, and then fills the rest of elements of
    ///   the array `cipher`, and returns `true`.
    /// 
    fn encrypt_into_array<U, const N: usize>(&mut self, iv: T, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone;

    #[inline]
    fn encrypt_str(&mut self, iv: T, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_str_into_vec<U>(&mut self, iv: T, message: &str, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_str_into_array<U, const N: usize>(&mut self, iv: T, message: &str, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_string(&mut self, iv: T, message: &String, cipher: *mut u8) -> u64
    {
        self.encrypt(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_string_into_vec<U>(&mut self, iv: T, message: &String, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_string_into_array<U, const N: usize>(&mut self, iv: T, message: &String, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(iv, message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_vec<U>(&mut self, iv: T, message: &Vec<U>, cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt(iv, message.as_ptr() as *const u8, (message.len() * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_vec_into_vec<U, V>(&mut self, iv: T, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(iv, message.as_ptr() as *const u8, (message.len() * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: T, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(iv, message.as_ptr() as *const u8, (message.len() * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_array<U, const N: usize>(&mut self, iv: T, message: &[U; N], cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.encrypt(iv, message.as_ptr() as *const u8, (N * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: T, message: &[U; N], cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_vec(iv, message.as_ptr() as *const u8, (N * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: T, message: &[U; N], cipher: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.encrypt_into_array(iv, message.as_ptr() as *const u8, (N * U::size_in_bytes()) as u64, cipher)
    }


    fn decrypt(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64;

    fn decrypt_into_vec<U>(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        des_pre_decrypt_into_vec!(message, length_in_bytes, U);
        let len = self.decrypt(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8);
        message.truncate(len as usize);
        len
    }

    fn decrypt_into_array<U, const N: usize>(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone;

    #[inline]
    fn decrypt_into_string(&mut self, iv: T, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        self.decrypt_into_vec(iv, cipher, length_in_bytes, unsafe { message.as_mut_vec() })
    }

    #[inline]
    fn decrypt_vec<U>(&mut self, iv: T, cipher: &Vec<U>, message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_vec_into_vec<U, V>(&mut self, iv: T, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: T, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_array(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_vec_into_string<U>(&mut self, iv: T, cipher: &Vec<U>, message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_array<U, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_vec(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        self.decrypt_into_array(iv, cipher.as_ptr() as *const u8, (N * U::size_in_bytes()) as u64, message)
    }

    #[inline]
    fn decrypt_array_into_string<U, const N: usize>(&mut self, iv: T, cipher: &[U; N], message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        self.decrypt_into_string(iv, cipher.as_ptr() as *const u8, (cipher.len() * U::size_in_bytes()) as u64, message)
    }
}
