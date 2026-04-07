// Copyright 2024, 2025, 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use crate::hash::{ BIG_KECCAK_1024, SHA3_512, SHA3_256, SHAKE_256, SHAKE_128,
                   SHA2_512, SHA2_256, SHA1, SHA0, MD5, MD4 };
use crate::symmetric::{ AES_128, DES };
use crate::random::{ Random_Generic, CPRNG_Engine, PRNG_Creator, PRNG_Creator_methods, LESS_SECURE_COUNT };


pub trait PRNG
{
    fn new() -> Self;
    fn random_u8(&mut self) -> u8;
}

impl PRNG for Random_Generic<LESS_SECURE_COUNT>
{
    #[inline] fn new() -> Self { Self::new_with(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new())}
    #[inline] fn random_u8(&mut self) -> u8 { self.random_u8() }
}



PRNG_Creator!{ ASYMMETRIC_PRNG_Creator_BIG_KECCAK_1024, BIG_KECCAK_1024, LESS_SECURE_COUNT }
PRNG_Creator!{ ASYMMETRIC_PRNG_Creator_SHA3_512, SHA3_512, LESS_SECURE_COUNT }
PRNG_Creator!{ ASYMMETRIC_PRNG_Creator_SHA3_256, SHA3_256, LESS_SECURE_COUNT }
PRNG_Creator!{ ASYMMETRIC_PRNG_Creator_SHAKE_256, SHAKE_256, LESS_SECURE_COUNT }
PRNG_Creator!{ ASYMMETRIC_PRNG_Creator_SHAKE_128, SHAKE_128, LESS_SECURE_COUNT }
PRNG_Creator!{ ASYMMETRIC_PRNG_Creator_SHA2_512, SHA2_512, LESS_SECURE_COUNT }
PRNG_Creator!{ ASYMMETRIC_PRNG_Creator_SHA2_256, SHA2_256, LESS_SECURE_COUNT }
PRNG_Creator!{ ASYMMETRIC_PRNG_Creator_SHA1, SHA1, LESS_SECURE_COUNT }
PRNG_Creator!{ ASYMMETRIC_PRNG_Creator_SHA0, SHA0, LESS_SECURE_COUNT }
PRNG_Creator!{ ASYMMETRIC_PRNG_Creator_MD5, MD5, LESS_SECURE_COUNT }
PRNG_Creator!{ ASYMMETRIC_PRNG_Creator_MD4, MD4, LESS_SECURE_COUNT }
PRNG_Creator!{ ASYMMETRIC_PRNG_Creator_Rijndael, AES_128, LESS_SECURE_COUNT }
PRNG_Creator!{ ASYMMETRIC_PRNG_Creator_DES, DES, LESS_SECURE_COUNT }
PRNG_Creator!{ ASYMMETRIC_PRNG_Creator_CPRNG_Engine, CPRNG_Engine, LESS_SECURE_COUNT }
