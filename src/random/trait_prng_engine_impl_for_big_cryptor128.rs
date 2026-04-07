// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use crate::symmetric::{ BigCryptor128, CTR };
use crate::random::{ PRNG_Engine, Key };

impl PRNG_Engine for BigCryptor128
{
    fn sow_array(&mut self, _: &[u64; 8], original: &[u64; 8])
    {
        if original[0] & 1 == 1
            { self.move_to_next_key(); }
    }

    fn harvest(&mut self, count: u64, message: &[u64; 8]) -> [u64; 8]
    {
        let mut cipher = [0_u64; 8];
        if count == 0
            { self.change_key(message); }
        self.encrypt_array_into_array(count as u128, &message, &mut cipher);
        cipher
    }
}
