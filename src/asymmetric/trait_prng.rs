// Copyright 2024, 2025, 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use crate::hash::BIG_KECCAK_1024;
use crate::random::Random_Generic;

pub trait PRNG
{
    fn new() -> Self;
    fn random_u8(&mut self) -> u8;
}

impl<const COUNT: u128> PRNG for Random_Generic<COUNT>
{
    #[inline] fn new() -> Self { Self::new_with(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new())}
    #[inline] fn random_u8(&mut self) -> u8 { self.random_u8() }
}
