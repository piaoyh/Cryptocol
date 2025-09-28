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



/// This trait PKCS1V15 is based on PKCS #1 ver. 1.5. It is considered not
/// to be cryptographically secure enough. So, you are not encouraged to use
/// this trait. Intead, you are encouraged to use the trait OAEP.
pub trait PKCS1V15
{
    const BT: u8 = 2;
    // const BT: u8 = 1;
    // const PS: u8 = 0xFF_u8;

    fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64;
    fn decrypt(&mut self, cipher: *const u8, cipher: *mut u8) -> u64;
}
