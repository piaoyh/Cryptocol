// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use crate::symmetric::{ BigCryptor128, CTR };
use crate::random::{ Random_Engine, Key };

impl Random_Engine for BigCryptor128
{
    fn harvest(&mut self, count: u128, message: &[u64; 8]) -> [u64; 8]
    {
        let mut cipher = [0_u64; 8];
        self.change_key(count == 0);
        self.encrypt_array_into_array(count, &message, &mut cipher);
        cipher
    }
}
