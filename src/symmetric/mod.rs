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
mod big_cryptor;
// mod aes;
// mod naes;

mod traits_ecb_with_padding_pkcs7;
mod traits_ecb_with_padding_iso;
mod traits_cbc_with_padding_pkcs7;
mod traits_cbc_with_padding_iso;
mod traits_pcbc_with_padding_pkcs7;
mod traits_pcbc_with_padding_iso;
mod traits_cfb;
mod traits_ofb;
mod traits_ctr;
mod trait_small_cryptor;

mod traits_ecb_with_padding_pkcs7_impl;
mod traits_ecb_with_padding_iso_impl;
mod traits_cbc_with_padding_pkcs7_impl;
mod traits_cbc_with_padding_iso_impl;
mod traits_pcbc_with_padding_pkcs7_impl;
mod traits_pcbc_with_padding_iso_impl;
mod traits_cfb_impl;
mod traits_ofb_impl;
mod traits_ctr_impl;
mod trait_small_cryptor_impl;
mod traits_for_big_cryptor_impl;

pub use des::*;
pub use big_cryptor::*;
// pub use aes::*;
// pub use naes::*;

pub use traits_ecb_with_padding_pkcs7::ECB_PKCS7;
pub use traits_ecb_with_padding_iso::ECB_ISO;
pub use traits_cbc_with_padding_pkcs7::CBC_PKCS7;
pub use traits_cbc_with_padding_iso::CBC_ISO;
pub use traits_pcbc_with_padding_pkcs7::PCBC_PKCS7;
pub use traits_pcbc_with_padding_iso::PCBC_ISO;
pub use traits_cfb::CFB;
pub use traits_ofb::OFB;
pub use traits_ctr::CTR;
pub use trait_small_cryptor::{ SmallCryptor64, SmallCryptor128 };

/// des.rs was too big because of documentation and plenty of examples
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to all the *.rs in documentation folder.
pub mod documentation;
