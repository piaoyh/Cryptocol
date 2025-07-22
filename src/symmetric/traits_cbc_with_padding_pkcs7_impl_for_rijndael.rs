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
use crate::symmetric::{ CBC_PKCS7, Rijndael_Generic };
use crate::symmetric::{ des_pre_encrypt_into_array, des_pre_decrypt_into_array };



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
CBC_PKCS7<[u32; NB]> for Rijndael_Generic<ROUND, NB, NK, IRREDUCIBLE,
                                                AFFINE_MUL, AFFINE_ADD, SR0, SR1, SR2, SR3,
                                                MC00, MC01, MC02, MC03, MC10, MC11, MC12, MC13,
                                                MC20, MC21, MC22, MC23, MC30, MC31, MC32, MC33,
                                                RC0, RC1, RC2, RC3, RC4, RC5, RC6, RC7, RC8, RC9, ROT>
{
    fn encrypt(&mut self, iv: [u32; NB], message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let size = NB * 4;
        let mut progress = 0_usize;
        let mut block = [IntUnion::new(); NB];
        let mut encoded = [IntUnion::new(); NB];
        for i in 0..NB
            { encoded[i].set(iv[i]); }
        for _ in 0..length_in_bytes / size as u64
        {
            unsafe { copy_nonoverlapping(message.add(progress as usize), block.as_mut_ptr() as *mut u8, size); }
            for i in 0..NB
                { encoded[i] ^= block[i]; }
            encoded = self.encrypt_unit(&encoded);
            unsafe { copy_nonoverlapping(encoded.as_ptr() as *const u8, cipher.add(progress as usize), size); }
            progress += size;
        }

        block.fill(IntUnion::new_with_ubytes([size as u8, size as u8, size as u8, size as u8]));
        if progress as u64 != length_in_bytes
        {
            let tail = (length_in_bytes - progress as u64) as usize;
            let padding = (size - tail) as u8;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, block.as_mut_ptr() as *mut u8, tail); }
            let nb = tail >> 2;     // tail >> 2 == tail / 4
            let ii = tail & 0b11;   // tail & 0b11 == tail % 4
            for i in ii..4
                { block[nb].set_ubyte_(i, padding); }
            for row in (nb + 1)..NB
            {
                for i in 0..4
                    { block[row].set_ubyte_(i, padding); }
            }
        }
        for i in 0..NB
            { encoded[i] ^= block[i]; }
        encoded = self.encrypt_unit(&encoded);
        unsafe { copy_nonoverlapping(encoded.as_ptr() as *const u8, cipher.add(progress as usize), size); }
        self.set_successful();
        progress as u64 + size as u64
    }

    fn encrypt_into_array<U, const N: usize>(&mut self, iv: [u32; NB], message: *const u8, length_in_bytes: u64, cipher: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        if length_in_bytes as u128 > U::size_in_bytes() as u128 * N as u128
        {
            self.set_failed();
            return 0;
        }
        des_pre_encrypt_into_array!(cipher, length_in_bytes, U);
        self.encrypt(iv, message, length_in_bytes, cipher.as_mut_ptr() as *mut u8)
    }

    fn decrypt(&mut self, iv: [u32; NB], cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        let size = NB * 4;
        if length_in_bytes % size as u64 != 0
        {
            self.set_failed();
            return 0;
        }
        let mut progress = 0_usize;
        let mut decoded: [IntUnion; NB];
        let mut block = [IntUnion::new(); NB];
        let mut iivv = [IntUnion::new(); NB];
        for i in 0..NB
            { iivv[i].set(iv[i]); }

        if length_in_bytes > size as u64
        {
            for _ in 0..(length_in_bytes / size as u64 - 1)
            {
                unsafe { copy_nonoverlapping(cipher.add(progress as usize), block.as_mut_ptr() as *mut u8, size); }
                decoded = self.decrypt_unit(&block);
                for i in 0..NB
                    { decoded[i] ^= iivv[i]; }
                iivv = block.clone();
                unsafe { copy_nonoverlapping(decoded.as_ptr() as *const u8, message.add(progress as usize), size); }
                progress += size;
            }
        }

        unsafe { copy_nonoverlapping(cipher.add(progress as usize), block.as_mut_ptr() as *mut u8, size); }
        decoded = self.decrypt_unit(&block);
        for i in 0..NB
            { decoded[i] ^= iivv[i]; }
        let padding_bytes = decoded[NB-1].get_ubyte_(3);
        if padding_bytes as usize > size
        {
            self.set_failed();
            return 0;
        }
        let message_bytes = size - padding_bytes as usize;
        for i in message_bytes..size
        {
            if unsafe { *((decoded.as_ptr() as *const u8).add(i)) } != padding_bytes
            {
                self.set_failed();
                return 0;
            }
        }
        unsafe { copy_nonoverlapping(decoded.as_ptr() as *const u8, message.add(progress as usize), message_bytes); }
        self.set_successful();
        progress as u64 + message_bytes as u64
    }

    fn decrypt_into_array<U, const N: usize>(&mut self, iv: [u32; NB], cipher: *const u8, length_in_bytes: u64, message: &mut [U; N]) -> u64
    where U: SmallUInt + Copy + Clone
    {
        if length_in_bytes as u128 > U::size_in_bytes() as u128 * N as u128
        {
            self.set_failed();
            return 0;
        }
        des_pre_decrypt_into_array!(message, length_in_bytes, U);
        self.decrypt(iv, cipher, length_in_bytes, message.as_mut_ptr() as *mut u8)
    }
}
