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
use crate::symmetric::{ crypt_ecb_with_padding_pkcs7, BigCryptor64, ECB_PKCS7 };
use crate::symmetric::{ crypt_into_something_with_padding_without_iv,
                        encrypt_into_array, encrypt_into_vec,
                        decrypt_into_array,
                        pre_encrypt_into_array, pre_encrypt_into_vec,
                        pre_decrypt_into_array };


impl ECB_PKCS7<u64> for BigCryptor64
{
    crypt_ecb_with_padding_pkcs7!{u64}
    crypt_into_something_with_padding_without_iv!{}
}
