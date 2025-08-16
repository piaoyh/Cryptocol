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

use crate::number::{ SmallUInt, LongUnion };
use crate::symmetric::{ ECB_PKCS7, BigCryptor64 };
use crate::symmetric::{ crypt_into_something_with_padding_without_iv,
                        encrypt_into_array, encrypt_into_vec,
                        decrypt_into_array,
                        pre_encrypt_into_array, pre_encrypt_into_vec,
                        pre_decrypt_into_array };


impl ECB_PKCS7<u64> for BigCryptor64
{
    fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let mut progress = 0_u64;
        let mut encoded: u64;
        let mut block = 0_u64;
        for _ in 0..length_in_bytes >> 3    // length_in_bytes >> 3 == length_in_bytes / 8
        {
            unsafe { copy_nonoverlapping(message.add(progress as usize) as *const u8, (&mut block) as *mut u64 as *mut u8, 8); }
            encoded = self.encrypt_u64(block);
            unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
            progress += 8;
        }

        let mut block_union = LongUnion::new_with(0x_08_08_08_08__08_08_08_08);
        block = 0_u64;
        if progress != length_in_bytes
        {
            let tail = (length_in_bytes - progress) as usize;
            let addr = unsafe { message.add(progress as usize) as *const u8 };
            unsafe { copy_nonoverlapping(addr, &mut block as *mut u64 as *mut u8, tail); }
            let padding = 8 - tail as u8;
            block_union.set(block);
            for i in tail..8
                { block_union.set_ubyte_(i, padding); }
        }
        encoded = self.encrypt_u64(block_union.get());
        unsafe { copy_nonoverlapping(&encoded as *const u64 as *const u8, cipher.add(progress as usize), 8); }
        self.set_successful();
        progress + 8
    }

    fn decrypt(&mut self, cipher: *const u8, length_in_bytes: u64, message: *mut u8) -> u64
    {
        if (length_in_bytes < Self::BLOCK_SIZE as u64) || (length_in_bytes % Self::BLOCK_SIZE as u64 != 0)
        {
            self.set_failed();
            return 0;
        }
        let mut progress = 0_u64;
        let mut decoded: u64;
        let mut block: u64;
        if length_in_bytes > 8
        {
            for _ in 0..(length_in_bytes >> 3) - 1 // length_in_bytes >> 3 == length_in_bytes / 8
            {
                block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
                decoded = self.decrypt_u64(block);
                unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), 8); }
                progress += 8;
            }
        }
        block = unsafe { *(cipher.add(progress as usize) as *const u64 ) };
        decoded = self.decrypt_u64(block);
        let decoded_union = LongUnion::new_with(decoded);
        let padding_bytes = decoded_union.get_ubyte_(7);
        let message_bytes = 8 - padding_bytes as usize;
        for i in message_bytes..8
        {
            if decoded_union.get_ubyte_(i) != padding_bytes
            {
                self.set_failed();
                return 0;
            }
        }
        unsafe { copy_nonoverlapping(&decoded as *const u64 as *const u8, message.add(progress as usize), message_bytes); }
        self.set_successful();
        progress + message_bytes as u64
    }
    
    crypt_into_something_with_padding_without_iv! {}
}
