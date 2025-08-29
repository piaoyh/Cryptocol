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


/// SmallCryptor<u64, 8> and SmallCryptor<u128, 16> are the traits for the
/// components for BigCryptor128 and BigCryptor64, respectively.
pub trait SmallCryptor<T, const N: usize>
{
    fn encrypt_unit(&mut self, message: T) -> T;
    fn decrypt_unit(&mut self, cipher: T) -> T;
    fn turn_inverse(&mut self);
    fn turn_encryptor(&mut self);
    fn turn_decryptor(&mut self);
    // fn clone_cryptor(&self) -> Self;
}

// trait SmallCryptor64 = SmallCryptor<u64, 8>;
// trait SmallCryptor128 = SmallCryptor<u128, 16>;