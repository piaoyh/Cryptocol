// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use crate::number::LongUnion;
use crate::symmetric::Rijndael_64_64;
use crate::random::Random_Engine;

impl Random_Engine for Rijndael_64_64
{
    fn harvest(&mut self, sugar: u64, message: &[u64; 8]) -> [u64; 8]
    {
        let mut key = LongUnion::new_with_uints(self.get_key());
        key.set(key.get() + sugar);
        let mut k = [0_u8; 8];
        for i in 0..8
            { k[i] = key.get_ubyte_(i); }
        self.set_key(&k);
        let mut cipher = [0_u64; 8];
        self.encrypt_array_u64(message, &mut cipher);
        cipher
    }
}
