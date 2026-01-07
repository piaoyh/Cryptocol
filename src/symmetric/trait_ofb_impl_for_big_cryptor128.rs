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


use std::ptr::copy_nonoverlapping;

use crate::number::TraitsBigUInt;
use crate::symmetric::{ BigCryptor128, OFB };
use crate::symmetric::{ crypt_ofb, crypt_into_something_without_padding,
                        pre_encrypt_into_array, pre_encrypt_into_vec,
                        pre_decrypt_into_array_without_padding,
                        encrypt_into_vec, encrypt_into_array_without_padding,
                        decrypt_into_array_without_padding };


impl OFB<u128> for BigCryptor128
{
    crypt_ofb!{u128}
    crypt_into_something_without_padding!{u128}
}
