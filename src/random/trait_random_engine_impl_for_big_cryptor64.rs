// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use crate::symmetric::BigCryptor64;
use crate::random::{ Random_Engine, Key, SALT };

impl Random_Engine for BigCryptor64
{
    fn harvest(&mut self, sugar: bool, message: &[u64; 8]) -> [u64; 8]
    {
        let mut cipher = [0_u64; 8];
        if !sugar
        {
            self.encrypt_array_u64(message, &mut cipher);
        }
        else
        {
            self.change_key(sugar);
            let mut m = [0_u64; 8];
            for i in 0..message.len()
                { m[i] = message[i].wrapping_add(SALT); }
            self.encrypt_array_u64(&m, &mut cipher);
        }
        cipher
    }
}
