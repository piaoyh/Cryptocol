// Copyright 2024, 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! various pseudo-random number generators
//! 
//! # Introduction
//! The module that contains a few sub-modules to define various pseudo-random
//! number generators
//! 
//! # Background: Random number generators
//! Generating true random numbers is very difficult. However, if artificial
//! random numbers which are widely called 'pseudo-random numbers' has the
//! same statistical characterisics as the true random numbers, it is
//! considered to be virtually random. For more in detail about randomness,
//! [Read more](https://en.wikipedia.org/wiki/Statistical_randomness).
//! 
//! # Predefined pseudo-random number generators
//! There is name consistancy. For the names of pseudo-random number generators
//! in this module, `Any` indicates cryptographically insecure while `Random`
//! indicates cryptographically secure.
//! 
//! There are provided predefined pseudo-random number generators:
//! ## Pseudo-random number generator
//!   - struct [`Random_Generic`](struct@Random_Generic)
//!   - trait [Random_Engine](trait@Random_Engine)
//! ## Pseudo-random number generator engines using hash algorithms
//!   - [`Random_BIG_KECCAK_1024`](struct@Random_BIG_KECCAK_1024): uses a hash algorithm BIG_KECCAK_1024.
//!   - [`Random_SHA3_512`](struct@Random_SHA3_512): uses a hash algorithm SHA3_512.
//!   - [`Random_SHA2_512`](struct@Random_SHA2_512): uses a hash algorithm SHA2_512.
//!   - [`Any_SHAKE_256`](struct@Any_SHAKE_256)`: uses a hash algorithm SSHAKE_256.
//!   - [`Any_SHAKE_128`](struct@Any_SHAKE_128)`: uses a hash algorithm SSHAKE_128.
//!   - [`Any_SHA3_512`](struct@Any_SHA3_512): uses a hash algorithm SHA3_512.
//!   - [`Any_SHA3_256`](struct@Any_SHA3_256): uses a hash algorithm SHA3_256.
//!   - [`Any_SHA2_512`](struct@Any_SHA2_512): uses a hash algorithm SHA2_512.
//!   - [`Any_SHA2_256`](struct@Any_SHA2_256): uses a hash algorithm SHA2_256.
//!   - [`Any_SHA1`](struct@Any_SHA1): uses a hash algorithm SHA1.
//!   - [`Any_SHA0`](struct@Any_SHA0): uses a hash algorithm SHA0.
//!   - [`Any_MD5`](struct@Any_MD5): uses a hash algorithm MD5.
//!   - [`Any_MD4`](struct@Any_MD4): uses a hash algorithm MD4.
//! ## Pseudo-random number generator engines using symmetric-key encryption
//!   algorithms
//!   - [`Random_Rijndael`](struct@Random_Rijndael): uses a symmetric-key encryption algorithm Rijndael.
//!   - [`Any_Rijndael`](struct@Any_Rijndael): uses a symmetric-key encryption algorithm Rijndael.
//!   - [`Any_DES`](struct@Any_DES): uses a symmetric-key encryption algorithm DES.
//! ## Pseudo-random number generator engines using simple randomization
//!   algorithm
//!   - [`Any_Num_C`](struct@Any_Num_C): uses a pseudo-random number generator
//!     algorithm of the function rand() of C standard library at the moment. 
//! ## Synonyms
//!   - Random: is a synonym of Random_SHA2_512 at the moment and can be
//!     __silently changed__ to have better algorithm in the future. If you want
//!     to keep using SHA2_512 for a pseudo-random number generator, you may
//!     want to use Random_SHA2_512. If you are happy that you will automatically
//!     use the better algotrithm in the future, you may want to use `Random`.
//!     Read [here](type@Random).
//!   - Any: is a synonym of Any_SHA2_256 at the moment and can be __silently
//!     changed__ to have better algorithm in the future. If you want to keep
//!     using SHA2_256 for a pseudo-random number generator, you may want to
//!     use Any_SHA2_256. If you are happy that you will automatically use the
//!     better algotrithm in the future, you may want to use `Any`.
//!     Read [here](type@Any).
//!   - Any_Num: is a synonym of Any_Num_C at the moment and can be __silently
//!     changed__ to have better algorithm in the future. If you want to keep
//!     using the algorithm of C standard libraray for a pseudo-random number
//!     generator, you may want to use Any_Num_C. If you are happy that you
//!     will automatically use the better algotrithm in the future, you may
//!     want to use `Any_Num`.
//!     Read [here](type@Any_Num).
//! 
//! # Quality Issues and Debate
//! The pseudo-random number generators in this module use hash algorithms,
//! encrytion/decryption algorithms, etc. which are not originally designed
//! for pseudo-random number generator. At the Internet, you can find a lot of
//! research results in terms of the possibility to use hash algorithms and/or
//! encryption algorithms for a pseudo-random number generator. This module can
//! also be considered to be a part of the research.
//! 
//! Some people doubt the cryptographical security of the pseudo-random number
//! generator using hash algorithm and/or encryption algorithm though the
//! offical hash algorithms published by NIST such as SHA-3 and SHA-2 are known
//! to have passed all the statistical and cryptographical security tests,
//! which have been done from 2006 to 2015 for SHA-3 and from 2001 to 2012 for
//! SHA-2. The tests included collision attack, preimage attack, and
//! second-preimage attack. It means that a pseudo-random number generator that
//! uses a hash algorithm has the long enough period of its recursively[^note]
//! produced random numbers for most of the cases. According to
//! [security collision chart](https://en.wikipedia.org/wiki/SHA-3#Comparison_of_SHA_functions),
//! the security stregths against collision of SHA-3-512, SHA-3-256, SHA-2-512
//! and SHA-2-256 are 256 bits, 128 bits, 256 bits, and 128 bits, respectively,
//! because of
//! [birthday paradox or birthday problem](https://en.wikipedia.org/wiki/Birthday_problem).
//! So, the period of pseudo-random numbers which is hash values generated by
//! hash algorithms can be theoretically
//! '115792089237316195423570985008687907853269984665640564039457584007913129639935',
//! which is 2^256, for 512-bit hash values of SHA-3-512 and SHA-2-512, and
//! '340282366920938463463374607431768211455', which is 2^128, for 256-bit hash
//! values of SHA-3-256 and SHA-2-256. So, __for non-cryptographical
//! purposes,__ all the pseudo-random number generators in this module are
//! completely fine to use.
//! 
//! So, if you really want one of the best quality pseudo-random number
//! generator rather than this module for serious cryptographical purpose,
//! you are encouraged to use the crate
//! [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)
//! which is well known to be a good pseudo-random number generator for
//! _cryptographical_ security purpose. The module of implementation of
//! `Random_Generic<GenFunc: PRNG + 'static>` to use
//! [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)
//! as a pseudo-random number generator is not implemented in this crate in
//! order to keep small number of dependencies, but how to embed
//! [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)
//! in a module of `Random_Generic<GenFunc: PRNG + 'static>`to use
//! [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)
//! is shown below in the section 'HOW TO EMBED OsRng IN THIS MODULE' in order
//! to help you implement a module to use
//! [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)
//! as a pseudo-random number generator in your project by yourself. 
//! 
//! # QUICK START
//! You can use either struct `Any` or `Random` depending on your purpose.
//! `Any` is for normal non-cryptographical purpose while `Random` is for
//! cryptographical purpose if you are fine to use hash algorithm for
//! pseudo-random number generator for cryptographical purpose.
//! 
//! - For `Random_Generic`, read [here](struct@Random_Generic#quick-start).
//! - For `Random_Generic`, read [here](struct@Random_Generic).
//! 
//! [^note]: Here, 'recursively' means that the output hash value of a hash
//! function is fed back to the hash function as its message, and a new hash
//! value is gotten from it, and then the new hash value is fed back to the
//! hash function as its message again, and this process is repeated.
//! 


/// The module that contains struct `Random_Generic`
mod random;

/// The module that contains `struct`s that construct `Random_Generic` objects with specific engines
mod random_specific;

/// The module that contains struct `AnyMumber_C_Generic`
mod any_number_engine_c_generic;

/// The module that contains trait `Random_Engine`
mod trait_random_engine;

/// The module that contains implementation of trait `Random_Engine` for `MD4`
mod trait_random_engine_impl_for_md4;

/// The module that contains implementation of trait `Random_Engine` for `MD5`
mod trait_random_engine_impl_for_md5;

/// The module that contains implementation of trait `Random_Engine` for `SHA1`
mod trait_random_engine_impl_for_sha1;

/// The module that contains implementation of trait `Random_Engine` for `SHA2_256`
mod trait_random_engine_impl_for_sha2_256;

/// The module that contains implementation of trait `Random_Engine` for `SHA2_512`
mod trait_random_engine_impl_for_sha2_512;

/// The module that contains implementation of trait `Random_Engine` for `SHA3`
mod trait_random_engine_impl_for_sha3;

/// The module that contains implementation of trait `Random_Engine` for `AnyNumber`
mod trait_random_engine_impl_for_any_number;

/// The module that contains implementation of trait `Random_Engine` for `DES`
mod trait_random_engine_impl_for_des;

/// The module that contains implementation of trait `Random_Engine` for `Rijndael`
mod trait_random_engine_impl_for_rijndael;

/// The module that contains implementation of trait `Random_Engine` for `BigCryptor64`
mod trait_random_engine_impl_for_big_cryptor64;

/// The module that contains implementation of trait `Random_Engine` for `BigCryptor128`
mod trait_random_engine_impl_for_big_cryptor128;

mod trait_key;
mod trait_key_impl_for_des;
mod trait_key_impl_for_rijndael;
mod trait_key_impl_for_big_cryptor64;
mod trait_key_impl_for_big_cryptor128;

pub use random::*;
pub use random_specific::*;
pub use any_number_engine_c_generic::{ AnyNumber_Engine_C_Generic, AnyNumber_Engine_C };
pub use trait_random_engine::*;

use trait_key::Key;


/// many *.rs was too big because of documentation and plenty of examples
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to all the *.rs in documentation folder.
mod documentation;
pub use documentation::*;