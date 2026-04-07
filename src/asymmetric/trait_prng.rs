// Copyright 2024, 2025, 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use crate::hash::BIG_KECCAK_1024;
use crate::random::{ Random_Generic, LESS_SECURE_COUNT };

/*
pub type PRNG_BIG_KECCAK_1024 ,
                            Random_PRNG_Creator_BIG_KECCAK_1024, Random_PRNG_Creator_SHA3_512, 
                            Random_PRNG_Creator_SHA2_512, Random_PRNG_Creator_Rijndael,
                        Any_PRNG_Creator,
                            Any_PRNG_Creator_SHA3_512, Any_PRNG_Creator_SHA3_256,
                            Any_PRNG_Creator_SHAKE_256, Any_PRNG_Creator_SHAKE_128, 
                            Any_PRNG_Creator_SHA2_512, Any_PRNG_Creator_SHA2_256, Any_PRNG_Creator_Rijndael,
                        Slapdash_PRNG_Creator,
                            Slapdash_PRNG_Creator_SHA1, Slapdash_PRNG_Creator_SHA0,
                            Slapdash_PRNG_Creator_MD5, Slapdash_PRNG_Creator_MD4,
                            Slapdash_PRNG_Creator_DES, Slapdash_PRNG_Creator_CPRNG_Engine
*/

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

