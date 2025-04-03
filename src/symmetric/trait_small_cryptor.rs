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


pub trait SmallCryptor128
{
    fn set_key(&mut self, key: [u8; 16]);
    fn set_key_unit(&mut self, key: u128);
    fn encrypt_unit(&mut self, message: u128) -> u128;
    fn decrypt_unit(&mut self, cipher: u128) -> u128;
    fn turn_inverse(&mut self);
    fn turn_encryptor(&mut self);
    fn turn_decryptor(&mut self);
    // fn clone_cryptor(&self) -> Self;
}


pub trait SmallCryptor64
{
    fn set_key(&mut self, key: [u8; 8]);
    fn set_key_unit(&mut self, key: u64);
    fn encrypt_unit(&mut self, message: u64) -> u64;
    fn decrypt_unit(&mut self, cipher: u64) -> u64;
    fn turn_inverse(&mut self);
    fn turn_encryptor(&mut self);
    fn turn_decryptor(&mut self);
    // fn clone_cryptor(&self) -> Self;
}
