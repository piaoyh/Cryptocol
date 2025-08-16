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

use crate::number::{ SmallUInt, IntUnion };
use crate::symmetric::{ Rijndael_Generic, CTR };
use crate::symmetric::{ crypt_into_something_without_padding,
                        encrypt_into_array_without_padding, encrypt_into_vec,
                        decrypt_into_array_without_padding,
                        pre_encrypt_into_array, pre_encrypt_into_vec,
                        pre_decrypt_into_array_without_padding };


impl <const ROUND: usize, const NB: usize, const NK: usize, const IRREDUCIBLE: u8,
        const AFFINE_MUL: u64, const AFFINE_ADD: u8,
        const SR0: usize, const SR1: usize, const SR2: usize, const SR3: usize,
        const MC00: u8, const MC01: u8, const MC02: u8, const MC03: u8,
        const MC10: u8, const MC11: u8, const MC12: u8, const MC13: u8,
        const MC20: u8, const MC21: u8, const MC22: u8, const MC23: u8,
        const MC30: u8, const MC31: u8, const MC32: u8, const MC33: u8,
        const RC0: u32, const RC1: u32, const RC2: u32, const RC3: u32,
        const RC4: u32, const RC5: u32, const RC6: u32, const RC7: u32,
        const RC8: u32, const RC9: u32, const ROT: u32>
CTR<[u32; NB]> for Rijndael_Generic<ROUND, NB, NK, IRREDUCIBLE,
                                            AFFINE_MUL, AFFINE_ADD, SR0, SR1, SR2, SR3,
                                            MC00, MC01, MC02, MC03, MC10, MC11, MC12, MC13,
                                            MC20, MC21, MC22, MC23, MC30, MC31, MC32, MC33,
                                            RC0, RC1, RC2, RC3, RC4, RC5, RC6, RC7, RC8, RC9, ROT>
{
    fn encrypt(&mut self, nonce: [u32; NB], message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_usize;
        let mut block = [IntUnion::new(); NB];
        let mut nonce_union = [IntUnion::new(); NB];
        for i in 0..NB
            { nonce_union[i].set(nonce[i]); }

        for _ in 0..length_in_bytes / Self::BLOCK_SIZE as u64
        {
            unsafe { copy_nonoverlapping(message.add(progress as usize), block.as_mut_ptr() as *mut u8, Self::BLOCK_SIZE); }
            let mut coded = self.encrypt_unit(&nonce_union);
            for i in 0..NB
                { coded[i] ^= block[i]; }
            unsafe { copy_nonoverlapping(coded.as_ptr() as *const u8, cipher.add(progress as usize), Self::BLOCK_SIZE); }
            for i in 0..NB
            {
                let old = nonce_union[i];
                nonce_union[i].set(nonce_union[i].get().wrapping_add(1));
                if nonce_union[i] > old
                    { break; }
            }
            progress += Self::BLOCK_SIZE;
        }

        if progress as u64 == length_in_bytes
        {
            self.set_successful();
            progress as u64
        }
        else
        {
            block = [IntUnion::new(); NB];
            let tail = (length_in_bytes - progress as u64) as usize;
            let addr = unsafe { message.add(progress) as *const u8 };
            unsafe { copy_nonoverlapping(addr, block.as_mut_ptr() as *mut u8, tail); }
            let mut coded = self.encrypt_unit(&nonce_union);
            for i in 0..NB
                { coded[i] ^= block[i]; }
            unsafe { copy_nonoverlapping(coded.as_ptr() as *const u8, cipher.add(progress as usize), tail); }
            self.set_successful();
            progress as u64 + tail as u64
        }
    }

    crypt_into_something_without_padding! {[u32; NB]}
}