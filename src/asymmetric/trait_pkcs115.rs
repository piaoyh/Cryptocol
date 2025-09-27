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

use std::ptr::{ copy_nonoverlapping, copy };

use crate::number::{ BigUInt, SmallUInt, IntUnion, LongUnion, LongerUnion, BigUInt_Prime, BigUInt_Modular };
use crate::random::Random;
use crate::define_utypes_with;

pub trait PKCS115
{
    fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> bool;
    fn decrypt(&mut self, cipher: *const u8, cipher: *mut u8) -> bool;
}
