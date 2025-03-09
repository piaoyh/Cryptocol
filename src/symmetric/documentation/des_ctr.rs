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
    
    pub fn encrypt_ctr(&mut self, nonce: u64, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_ctr_into_vec<T>(&mut self, nonce: u64, message: *const u8, length_in_bytes: u64, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_ctr_into_array<T, const N: usize>(&mut self, nonce: u64, message: *const u8, length_in_bytes: u64, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_str_with_padding_pkcs7_pcbc(&mut self, nonce: u64, message: &str, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_str_ctr_into_vec<T>(&mut self, nonce: u64, message: &str, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_str_ctr_into_array<T, const N: usize>(&mut self, nonce: u64, message: &str, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_string_with_padding_pkcs7_pcbc(&mut self, nonce: u64, message: &String, cipher: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_string_ctr_into_vec<T>(&mut self, nonce: u64, message: &String, cipher: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_string_ctr_into_array<T, const N: usize>(&mut self, nonce: u64, message: &String, cipher: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_vec_with_padding_pkcs7_pcbc<T>(&mut self, nonce: u64, message: &Vec<T>, cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }
    
    pub fn encrypt_vec_ctr_into_vec<T, U>(&mut self, nonce: u64, message: &Vec<T>, cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_vec_ctr_into_array<T, U, const N: usize>(&mut self, nonce: u64, message: &Vec<T>, cipher: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_array_with_padding_pkcs7_pcbc<T, const N: usize>(&mut self, nonce: u64, message: &[T; N], cipher: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }
    
    pub fn encrypt_array_ctr_into_vec<T, U, const N: usize>(&mut self, nonce: u64, message: &[T; N], cipher: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn encrypt_array_ctr_into_array<T, U, const N: usize, const M: usize>(&mut self, nonce: u64, message: &[T; N], cipher: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }



    pub fn decrypt_with_padding_pkcs7_pcbc(&mut self, nonce: u64, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_ctr_into_vec<T>(&mut self, nonce: u64, cipher: *const u8, length_in_bytes: u64, message: &mut Vec<T>) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_ctr_into_array<T, const N: usize>(&mut self, nonce: u64, cipher: *const u8, length_in_bytes: u64, message: &mut [T; N]) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_ctr_into_string(&mut self, nonce: u64, cipher: *const u8, length_in_bytes: u64, message: &mut String) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_with_padding_pkcs7_pcbc<T>(&mut self, nonce: u64, cipher: &Vec<T>, message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_ctr_into_vec<T, U>(&mut self, nonce: u64, cipher: &Vec<T>, message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_ctr_into_array<T, U, const N: usize>(&mut self, nonce: u64, cipher: &Vec<T>, message: &mut [U; N]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_vec_ctr_into_string<T>(&mut self, nonce: u64, cipher: &Vec<T>, message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_with_padding_pkcs7_pcbc<T, const N: usize>(&mut self, nonce: u64, cipher: &[T; N], message: *mut u8) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_ctr_into_vec<T, U, const N: usize>(&mut self, nonce: u64, cipher: &[T; N], message: &mut Vec<U>) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_ctr_into_array<T, U, const N: usize, const M: usize>(&mut self, nonce: u64, cipher: &[T; N], message: &mut [U; M]) -> u64
    where T: SmallUInt + Copy + Clone, U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    pub fn decrypt_array_ctr_into_string<T, const N: usize>(&mut self, nonce: u64, cipher: &[T; N], message: &mut String) -> u64
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }
}