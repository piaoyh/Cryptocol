// Copyright 2024, 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use crate::random::{ Random_Engine, AnyNumber_Engine_C_Generic, SALT };

impl<const MULTIPLIER: u64, const ADDER: u64>
Random_Engine for AnyNumber_Engine_C_Generic<MULTIPLIER, ADDER>
{
    fn sow_array(&mut self, _: &[u64; 8], _: &[u64; 8])
    {
    }

    fn harvest(&mut self, count: u128, message: &[u64; 8]) -> [u64; 8]
    {
        let mut any_numbers = [0_u64; 8];
        let salt = if count == 0 { SALT } else { 0 };
        for i in 0..8
        {
            any_numbers[i] = message[i].wrapping_add(salt)
                                        .wrapping_mul(MULTIPLIER)
                                        .wrapping_add(ADDER)
                                        ^ salt;
        }
        any_numbers
    }
}
