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


use crate::number::TraitsBigUInt;


pub trait OAEP
{
    const BT: u8 = 2;
    fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;

    fn encrypt_into_vec<U>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    where U: TraitsBigUInt<U>;

    fn encrypt_into_array<U, const N: usize>(&mut self, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: TraitsBigUInt<U>;

    #[inline]
    fn encrypt_str(&mut self, message: &str, cipher: *mut u8) -> u64
    {
        self.encrypt(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_str_into_vec<U>(&mut self, message: &str, cipher: &mut Vec<U>) -> u64
    where U: TraitsBigUInt<U>
    {
        self.encrypt_into_vec(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_str_into_array<U, const N: usize>(&mut self, message: &str, cipher: &mut [U; N]) -> u64
    where U: TraitsBigUInt<U>
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
    where U: TraitsBigUInt<U>
    {
        self.encrypt_into_vec(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_string_into_array<U, const N: usize>(&mut self, message: &String, cipher: &mut [U; N]) -> u64
    where U: TraitsBigUInt<U>
    {
        self.encrypt_into_array(message.as_ptr(), message.len() as u64, cipher)
    }

    #[inline]
    fn encrypt_vec<U>(&mut self, message: &Vec<U>, cipher: *mut u8) -> u64
    where U: TraitsBigUInt<U>
    {
        self.encrypt(message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_vec_into_vec<U, V>(&mut self, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        self.encrypt_into_vec(message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        self.encrypt_into_array(message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_array<U, const N: usize>(&mut self, message: &[U; N], cipher: *mut u8) -> u64
    where U: TraitsBigUInt<U>
    {
        self.encrypt(message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, message: &[U; N], cipher: &mut Vec<V>) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        self.encrypt_into_vec(message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
    }

    #[inline]
    fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, message: &[U; N], cipher: &mut [V; M]) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        self.encrypt_into_array(message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64, cipher)
    }

    fn decrypt(&mut self, cipher: *const u8, message: *mut u8) -> u64;

    fn decrypt_into_vec<U>(&mut self, cipher: *const u8, message: &mut Vec<U>) -> u64
    where U: TraitsBigUInt<U>;

    fn decrypt_into_array<U, const N: usize>(&mut self, cipher: *const u8, message: &mut [U; N]) -> u64
    where U: TraitsBigUInt<U>;

    #[inline]
    fn decrypt_into_string(&mut self, cipher: *const u8, message: &mut String) -> u64
    {
        self.decrypt_into_vec(cipher, unsafe { message.as_mut_vec() })
    }

    #[inline]
    fn decrypt_vec<U>(&mut self, cipher: &Vec<U>, message: *mut u8) -> u64
    where U: TraitsBigUInt<U>
    {
        self.decrypt(cipher.as_ptr() as *const u8, message)
    }

    fn decrypt_vec_into_vec<U, V>(&mut self, cipher: &Vec<U>, message: &mut Vec<V>) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>;

    #[inline]
    fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, cipher: &Vec<U>, message: &mut [V; N]) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        self.decrypt_into_array(cipher.as_ptr() as *const u8, message)
    }

    #[inline]
    fn decrypt_vec_into_string<U>(&mut self, cipher: &Vec<U>, message: &mut String) -> u64
    where U: TraitsBigUInt<U>
    {
        self.decrypt_into_string(cipher.as_ptr() as *const u8, message)
    }

    #[inline]
    fn decrypt_array<U, const N: usize>(&mut self, cipher: &[U; N], message: *mut u8) -> u64
    where U: TraitsBigUInt<U>
    {
        self.decrypt(cipher.as_ptr() as *const u8, message)
    }

    #[inline]
    fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, cipher: &[U; N], message: &mut Vec<V>) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        self.decrypt_into_vec(cipher.as_ptr() as *const u8, message)
    }

    #[inline]
    fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, cipher: &[U; N], message: &mut [V; M]) -> u64
    where U: TraitsBigUInt<U>, V: TraitsBigUInt<V>
    {
        self.decrypt_into_array(cipher.as_ptr() as *const u8, message)
    }

    #[inline]
    fn decrypt_array_into_string<U, const N: usize>(&mut self, cipher: &[U; N], message: &mut String) -> u64
    where U: TraitsBigUInt<U>
    {
        self.decrypt_into_string(cipher.as_ptr() as *const u8, message)
    }
}
