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
    
    pub fn encrypt(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_into_vec<U>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_into_array<U, const N: usize>(&mut self, iv: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_str(&mut self, iv: u64, message: &str, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_str_into_vec<U>(&mut self, iv: u64, message: &str, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_str_into_array<U, const N: usize>(&mut self, iv: u64, message: &str, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_string(&mut self, iv: u64, message: &String, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_string_into_vec<U>(&mut self, iv: u64, message: &String, cipher: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_string_into_array<U, const N: usize>(&mut self, iv: u64, message: &String, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_vec<U>(&mut self, iv: u64, message: &Vec<U>, cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }
    
    pub fn encrypt_vec_into_vec<U, V>(&mut self, iv: u64, message: &Vec<U>, cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_vec_into_array<U, V, const N: usize>(&mut self, iv: u64, message: &Vec<U>, cipher: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_array<U, const N: usize>(&mut self, iv: u64, message: &[U; N], cipher: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }
    
    pub fn encrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: u64, message: &[U; N], cipher: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: u64, message: &[U; N], cipher: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }



    pub fn decrypt(&mut self, cipher: *const u8, length_in_bytes: u64, iv: u64, message: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_into_vec<U>(&mut self, cipher: *const u8, length_in_bytes: u64, iv: u64, message: &mut Vec<U>) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_into_array<U, const N: usize>(&mut self, cipher: *const u8, length_in_bytes: u64, iv: u64, message: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_into_string(&mut self, cipher: *const u8, length_in_bytes: u64, iv: u64, message: &mut String) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec<U>(&mut self, cipher: &Vec<U>, iv: u64, message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_into_vec<U, V>(&mut self, cipher: &Vec<U>, iv: u64, message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_into_array<U, V, const N: usize>(&mut self, cipher: &Vec<U>, iv: u64, message: &mut [V; N]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_into_string<U>(&mut self, cipher: &Vec<U>, iv: u64, message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array<U, const N: usize>(&mut self, iv: u64, cipher: &[U; N], message: *mut u8) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_into_vec<U, V, const N: usize>(&mut self, iv: u64, cipher: &[U; N], message: &mut Vec<V>) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_into_array<U, V, const N: usize, const M: usize>(&mut self, iv: u64, cipher: &[U; N], message: &mut [V; M]) -> u64
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_into_string<U, const N: usize>(&mut self, iv: u64, cipher: &[U; N], message: &mut String) -> u64
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }
}