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

use crate::number::{ SmallUInt, LongUnion };
use crate::symmetric::{ CBC_ISO, BigCryptor64 };
use crate::symmetric::{ crypt_cbc_with_padding_iso, crypt_into_something_with_padding,
                        encrypt_into_array, encrypt_into_vec,
                        decrypt_into_array,
                        pre_encrypt_into_array, pre_encrypt_into_vec,
                        pre_decrypt_into_array };


impl CBC_ISO<u64> for BigCryptor64
{
    crypt_cbc_with_padding_iso!{u64}
    crypt_into_something_with_padding!{u64}
}
