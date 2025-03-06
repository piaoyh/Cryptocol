// Copyright 2023, 2024 PARK Youngho.
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
// ! (struct@DES_Generic)
//! - NDES encryption/decryption algorithms --- Includes 2DES, 3DES and its expanded versions. `NDES_Generic`
// ! (struct@NDES_Generic)
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
// mod ndes;
// mod aes;
// mod naes;
mod traits_ofb;
mod traits_ctr;

pub use des::*;
// pub use ndes::*;
// pub use aes::*;
// pub use naes::*;
pub use traits_ofb::*;
pub use traits_ctr::*;

/// des.rs was too big because of documentation and plenty of examples
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to all the *.rs in documentation folder.
pub mod documentation;