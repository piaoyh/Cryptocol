// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use std::ptr::copy_nonoverlapping;
use crate::symmetric::BigCryptor128;
use crate::random::{ Random_Engine, Key, SALT };

impl Random_Engine for BigCryptor128
{
    fn harvest(&mut self, sugar: bool, message: &[u64; 8]) -> [u64; 8]
    {
        let mut m = [0_u128; 4];
        let mut c = [0_u128; 4];
        let mut cipher = [0_u64; 8];

        unsafe { copy_nonoverlapping(message.as_ptr(), m.as_mut_ptr() as *mut u64, 8); }
        if !sugar
        {
            self.encrypt_array_u128(&m, &mut c);
        }
        else
        {
            self.change_key(sugar);
            for i in 0..m.len()
                { m[i] = message[i].wrapping_add(SALT) as u128 + (message[i+1].wrapping_add(SALT) as u128) << 64; }
            self.encrypt_array_u128(&m, &mut c);
        }
        unsafe { copy_nonoverlapping(c.as_ptr() as *const u64, cipher.as_mut_ptr(), 8); }
        cipher
    }
}
