// Copyright 2023, 2024, 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! various symmetric-key algorithms for the encryption/decryption of digital data
//! 
//! # Introduction
//! The module that contains a few sub-modules to define symmetric-key
//! algorithms for the encryption/decryption of digital data
//! 
//! # Background: Symmetric encryption/decryption algorithms
//! // Todo
//! 
// ! Read [this article](https://en.wikipedia.org/wiki/Cryptographic_hash_function)
// ! and/or Watch [this lecture](https://www.youtube.com/watch?v=kPBJIhpcZgE)
// ! to learn symmetric encryption/decryption algorithms more in detail.
//! 
//! # The symmetric-key algorithms for the encryption/decryption of digital data
//! This module provides several kinds of symmetric-key algorithms for the encryption/decryption of digital data:
//! - DES encryption/decryption algorithms --- Includes DES and its expanded versions. `DES_Generic`
//! (struct@DES_Generic)
//! - NDES encryption/decryption algorithms --- Includes 2DES, 3DES and its expanded versions. `NDES_Generic`
//! (struct@NDES_Generic)
//! - AES encryption/decryption algorithms --- Includes AES and its expanded versions. `AES_Generic`
// ! (struct@AES_Generic)
//! - NAES encryption/decryption algorithms --- Includes 2AES, 3AES and its expanded versions. `NAES_Generic`
// ! (struct@NAES_Generic)
//! 
//! # QUICK START
//! - For `DES`, read [here](struct@DES_Generic#quick-start).
// ! - For `NDES`, read [here](struct@NDES_Generic#quick-start).
// ! - For `AES`, read [here](struct@AES_Generic#quick-start).
// ! - For `NAES`, read [here](struct@NAES_Generic#quick-start).



mod des;
mod rijndael;
mod big_cryptor64;
mod big_cryptor128;

mod operation_mode_macros;
mod trait_ecb_with_padding_pkcs7;
mod trait_ecb_with_padding_iso;
mod trait_cbc_with_padding_pkcs7;
mod trait_cbc_with_padding_iso;
mod trait_pcbc_with_padding_pkcs7;
mod trait_pcbc_with_padding_iso;
mod trait_cfb;
mod trait_ofb;
mod trait_ctr;
mod trait_small_cryptor;

mod trait_ecb_with_padding_pkcs7_impl_for_des;
mod trait_ecb_with_padding_pkcs7_impl_for_rijndael;
mod trait_ecb_with_padding_pkcs7_impl_for_big_cryptor64;
mod trait_ecb_with_padding_pkcs7_impl_for_big_cryptor128;
mod trait_ecb_with_padding_iso_impl_for_des;
mod trait_ecb_with_padding_iso_impl_for_rijndael;
mod trait_ecb_with_padding_iso_impl_for_big_cryptor64;
mod trait_ecb_with_padding_iso_impl_for_big_cryptor128;
mod trait_cbc_with_padding_pkcs7_impl_for_des;
mod trait_cbc_with_padding_pkcs7_impl_for_rijndael;
mod trait_cbc_with_padding_pkcs7_impl_for_big_cryptor64;
mod trait_cbc_with_padding_pkcs7_impl_for_big_cryptor128;
mod trait_cbc_with_padding_iso_impl_for_des;
mod trait_cbc_with_padding_iso_impl_for_rijndael;
mod trait_cbc_with_padding_iso_impl_for_big_cryptor64;
mod trait_cbc_with_padding_iso_impl_for_big_cryptor128;
mod trait_pcbc_with_padding_pkcs7_impl_for_des;
mod trait_pcbc_with_padding_pkcs7_impl_for_rijndael;
mod trait_pcbc_with_padding_pkcs7_impl_for_big_cryptor64;
mod trait_pcbc_with_padding_pkcs7_impl_for_big_cryptor128;
mod trait_pcbc_with_padding_iso_impl_for_des;
mod trait_pcbc_with_padding_iso_impl_for_rijndael;
mod trait_pcbc_with_padding_iso_impl_for_big_cryptor64;
mod trait_pcbc_with_padding_iso_impl_for_big_cryptor128;
mod trait_cfb_impl_for_des;
mod trait_cfb_impl_for_rijndael;
mod trait_cfb_impl_for_big_cryptor64;
mod trait_cfb_impl_for_big_cryptor128;
mod trait_ofb_impl_for_des;
mod trait_ofb_impl_for_rijndael;
mod trait_ofb_impl_for_big_cryptor64;
mod trait_ofb_impl_for_big_cryptor128;
mod trait_ctr_impl_for_des;
mod trait_ctr_impl_for_rijndael;
mod trait_ctr_impl_for_big_cryptor64;
mod trait_ctr_impl_for_big_cryptor128;
mod trait_small_cryptor64_impl_for_des;
mod trait_small_cryptor64_impl_for_rijndael;
mod trait_small_cryptor64_impl_for_big_cryptor64;
mod trait_small_cryptor128_impl_for_rijndael;
mod trait_small_cryptor128_impl_for_big_cryptor128;
mod trait_for_big_cryptor_impl;

pub use des::*;
pub use rijndael::*;
pub use big_cryptor64::*;
pub use big_cryptor128::*;
use operation_mode_macros::*;

pub use trait_ecb_with_padding_pkcs7::ECB_PKCS7;
pub use trait_ecb_with_padding_iso::ECB_ISO;
pub use trait_cbc_with_padding_pkcs7::CBC_PKCS7;
pub use trait_cbc_with_padding_iso::CBC_ISO;
pub use trait_pcbc_with_padding_pkcs7::PCBC_PKCS7;
pub use trait_pcbc_with_padding_iso::PCBC_ISO;
pub use trait_cfb::CFB;
pub use trait_ofb::OFB;
pub use trait_ctr::CTR;
pub use trait_small_cryptor::SmallCryptor;


/// des.rs was too big because of documentation and plenty of examples
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to all the *.rs in documentation folder.
pub mod documentation;
