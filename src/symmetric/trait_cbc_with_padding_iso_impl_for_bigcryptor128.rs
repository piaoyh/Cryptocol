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

use crate::number::{ SmallUInt, LongerUnion };
use crate::symmetric::{ CBC_ISO, BigCryptor128 };
use crate::symmetric::{ crypt_into_something_with_padding,
                        encrypt_into_array, encrypt_into_vec,
                        decrypt_into_array,
                        pre_encrypt_into_array, pre_encrypt_into_vec,
                        pre_decrypt_into_array };


impl CBC_ISO<u128> for BigCryptor128
{
    fn encrypt(&mut self, iv: u128, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut encoded = iv;
        let mut block = 0_u128;
        for _ in 0..length_in_bytes >> 4    // length_in_bytes >> 4 == length_in_bytes / 16
        {
            unsafe { copy_nonoverlapping(message.add(progress as usize) as *const u8, (&mut block) as *mut u128 as *mut u8, 16); }
            encoded = self.encrypt_u128(block ^ encoded);
            unsafe { copy_nonoverlapping(&encoded as *const u128 as *const u8, cipher.add(progress as usize), 16); }
            progress += 16;
        }
        block = 0_u128;
        let mut block_union = LongerUnion::new_with(0b_1000_0000);
        if progress != length_in_bytes
        {
            let tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u128 as *mut u8, tail); }
            block_union.set(block);
            block_union.set_ubyte_(tail, 0b_1000_0000);
        }
        encoded = self.encrypt_u128(block_union.get() ^ encoded);
        unsafe { copy_nonoverlapping(&encoded as *const u128 as *const u8, cipher.add(progress as usize), 16); }
        self.set_successful();
        progress + 16
    }

    fn decrypt(&mut self, mut iv: u128, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut decoded: u128;
        let mut block = 0_u128;
        for _ in 0..(length_in_bytes >> 4) - 1  // length_in_bytes >> 4 == length_in_bytes / 16
        {
            unsafe { copy_nonoverlapping(cipher.add(progress as usize) as *const u8, (&mut block) as *mut u128 as *mut u8, 16); }
            decoded = iv ^ self.decrypt_u128(block);
            iv = block;
            unsafe { copy_nonoverlapping(&decoded as *const u128 as *const u8, message.add(progress as usize), 16); }
            progress += 8;
        }

        unsafe { copy_nonoverlapping(cipher.add(progress as usize) as *const u8, (&mut block) as *mut u128 as *mut u8, 16); }
        decoded = iv ^ self.decrypt_u128(block);
        let decoded_union = LongerUnion::new_with(decoded);
        for i in 0..16_usize
        {
            if decoded_union.get_ubyte_(15-i) == 0
                { continue; }
            if decoded_union.get_ubyte_(15-i) == 0b_1000_0000_u8
            {
                let message_bytes = 7-i;
                unsafe { copy_nonoverlapping(&decoded as *const u128 as *const u8, message.add(progress as usize), message_bytes); }
                self.set_successful();
                return progress + message_bytes as u64;
            }
            else
            {
                self.set_failed();
                return 0;
            }
        }
        self.set_failed();
        return 0;
    }

    crypt_into_something_with_padding!{u128}
}
