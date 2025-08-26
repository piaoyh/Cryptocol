// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(missing_docs)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_variables)]
// #![warn(rustdoc::missing_doc_code_examples)]


use std::ptr::copy_nonoverlapping;
use crate::number::{IntUnion, LongUnion};
use crate::symmetric::{ SmallCryptor,  Rijndael_Generic };


impl <const ROUND: usize, const NK: usize,
        const IRREDUCIBLE: u8, const AFFINE_MUL: u64, const AFFINE_ADD: u8,
        const SR0: usize, const SR1: usize, const SR2: usize, const SR3: usize,
        const MC00: u8, const MC01: u8, const MC02: u8, const MC03: u8,
        const MC10: u8, const MC11: u8, const MC12: u8, const MC13: u8,
        const MC20: u8, const MC21: u8, const MC22: u8, const MC23: u8,
        const MC30: u8, const MC31: u8, const MC32: u8, const MC33: u8,
        const RC0: u32, const RC1: u32, const RC2: u32, const RC3: u32, const RC4: u32,
        const RC5: u32, const RC6: u32, const RC7: u32, const RC8: u32, const RC9: u32, const ROT: u32>
SmallCryptor<u64, 8> for Rijndael_Generic<ROUND, 2, NK, IRREDUCIBLE, AFFINE_MUL, AFFINE_ADD, SR0, SR1, SR2, SR3,
        MC00, MC01, MC02, MC03, MC10, MC11, MC12, MC13, MC20, MC21, MC22, MC23, MC30, MC31, MC32, MC33,
        RC0, RC1, RC2, RC3, RC4, RC5, RC6, RC7, RC8, RC9, ROT>
{
    #[inline]
    fn set_key(&mut self, key: [u8; 8])
    {
        self.set_key(&key);
    }

    fn set_key_unit(&mut self, key: u64)
    {
        let mut k = [0_u8; 8];
        unsafe {
            copy_nonoverlapping(&key as *const u64 as *const u8,
                                 k.as_mut_ptr() as *mut u8, 8);
        }
        self.set_key(&k);
    }

    fn encrypt_unit(&mut self, message: u64) -> u64
    {
        let long = LongUnion::new_with(message);
        let m = [IntUnion::new_with(long.get_uint_(0)),
                                IntUnion::new_with(long.get_uint_(1))];
        let res = self._encrypt(&m);
        LongUnion::new_with_uints([res[0].get(), res[1].get()]).get()
    }
    
    #[inline] fn decrypt_unit(&mut self, cipher: u64) -> u64
    {
        let long = LongUnion::new_with(cipher);
        let c = [IntUnion::new_with(long.get_uint_(0)),
                                IntUnion::new_with(long.get_uint_(1))];
        let res = self._decrypt(&c);
        LongUnion::new_with_uints([res[0].get(), res[1].get()]).get()
    }

    #[inline] fn turn_inverse(&mut self)    { self.turn_inverse(); }
    #[inline] fn turn_encryptor(&mut self)  { self.turn_encryptor(); }
    #[inline] fn turn_decryptor(&mut self)  { self.turn_decryptor(); }
    // #[inline] fn clone_cryptor(&self) -> SmallCryptor<u64, 8>   { self.clone() }
}