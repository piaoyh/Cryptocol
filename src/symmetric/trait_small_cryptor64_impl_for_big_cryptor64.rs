// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


// #![allow(missing_docs)]
// #![allow(unused_must_use)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![warn(rustdoc::missing_doc_code_examples)]


use crate::symmetric::{ SmallCryptor,  BigCryptor64 };


impl SmallCryptor<u64, 8> for BigCryptor64
{
    #[inline] fn move_to_next_key(&mut self)    { self.move_to_next_key(); }
    #[inline] fn encrypt_unit(&mut self, message: u64) -> u64   { self._encrypt(message) }
    #[inline] fn decrypt_unit(&mut self, cipher: u64) -> u64    { self._decrypt(cipher) }
    #[inline] fn turn_inverse(&mut self)    { self.turn_inverse(); }
    #[inline] fn turn_encryptor(&mut self)  { self.turn_encryptor(); }
    #[inline] fn turn_decryptor(&mut self)  { self.turn_decryptor(); }
}