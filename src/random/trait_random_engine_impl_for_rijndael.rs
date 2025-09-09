// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use std::ptr::copy_nonoverlapping;

use crate::number::IntUnion;
use crate::symmetric::Rijndael_Generic;
use crate::random::Random_Engine;

impl <const ROUND: usize, const NB: usize, const NK: usize, const IRREDUCIBLE: u8, const AFFINE_MUL: u64,
        const AFFINE_ADD: u8, const SR0: usize, const SR1: usize, const SR2: usize, const SR3: usize,
        const MC00: u8, const MC01: u8, const MC02: u8, const MC03: u8,
        const MC10: u8, const MC11: u8, const MC12: u8, const MC13: u8,
        const MC20: u8, const MC21: u8, const MC22: u8, const MC23: u8,
        const MC30: u8, const MC31: u8, const MC32: u8, const MC33: u8,
        const RC0: u32, const RC1: u32, const RC2: u32, const RC3: u32, const RC4: u32,
        const RC5: u32, const RC6: u32, const RC7: u32, const RC8: u32, const RC9: u32, const ROT: u32>
Random_Engine for Rijndael_Generic<ROUND, NB, NK, IRREDUCIBLE, AFFINE_MUL, AFFINE_ADD, SR0, SR1, SR2, SR3,
        MC00, MC01, MC02, MC03, MC10, MC11, MC12, MC13, MC20, MC21, MC22, MC23, MC30, MC31, MC32, MC33,
        RC0, RC1, RC2, RC3, RC4, RC5, RC6, RC7, RC8, RC9, ROT>
{
    fn harvest(&mut self, sugar: u64, message: &[u64; 8]) -> [u64; 8]
    {
        // let mut key = LongUnion::new_with_uints(self.get_key());
        let mut key = self.get_key();
        let mut salt = [0_u32; NK];
        salt[0] = sugar as u32;
        salt[1] = (sugar >> 32) as u32;
        let mut old = key[0];
        let mut carry = 0;
        for i in 0..NK
        {
            key[i] = key[i].wrapping_add(salt[i]).wrapping_add(carry);
            carry = if key[i] < old {1} else {0};
            old = key[i];
        }
        self.set_original_key(&salt);

        let mut cipher = [0_u64; 8];
        if NB >= 16
        {
            let mut m = [IntUnion::new(); NB];
            unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr() as *mut u32, 16); }
            let c = self.encrypt_unit(&m);
            unsafe { copy_nonoverlapping(c.as_ptr() as *const u32, cipher.as_mut_ptr() as *mut u32, 16); }
        }
        else if NB >= 8
        {
            let mut m = [[IntUnion::new(); NB]; 2];
            let mut c = [[IntUnion::new(); NB]; 2];
            unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr() as *mut u32, 16); }
            self.encrypt_array_unit(&m, &mut c);
            unsafe { copy_nonoverlapping(c.as_ptr() as *const u32, cipher.as_mut_ptr() as *mut u32, 16); }
        }
        else if NB >= 4
        {
            let mut m = [[IntUnion::new(); NB]; 4];
            let mut c = [[IntUnion::new(); NB]; 4];
            unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr() as *mut u32, 16); }
            self.encrypt_array_unit(&m, &mut c);
            unsafe { copy_nonoverlapping(c.as_ptr() as *const u32, cipher.as_mut_ptr() as *mut u32, 16); }
        }
        else if NB >= 2
        {
            let mut m = [[IntUnion::new(); NB]; 8];
            let mut c = [[IntUnion::new(); NB]; 8];
            unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr() as *mut u32, 16); }
            self.encrypt_array_unit(&m, &mut c);
            unsafe { copy_nonoverlapping(c.as_ptr() as *const u32, cipher.as_mut_ptr() as *mut u32, 16); }
        }
        else
        {
            let mut m = [[IntUnion::new(); NB]; 16];
            let mut c = [[IntUnion::new(); NB]; 16];
            unsafe { copy_nonoverlapping(message.as_ptr() as *const u32, m.as_mut_ptr() as *mut u32, 16); }
            self.encrypt_array_unit(&m, &mut c);
            unsafe { copy_nonoverlapping(c.as_ptr() as *const u32, cipher.as_mut_ptr() as *mut u32, 16); }
        }
        cipher
    }
}

