// Copyright 2024, 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use crate::hash::{  MD4, MD5, SHA0, SHA1, SHA2_224, SHA2_256, SHA2_384, SHA2_512,
                    SHA2_512_256, SHA2_512_t_224, SHA3_256, SHA3_384, SHA3_512 };

#[allow(non_camel_case_types)]
pub trait Hash
{
    fn get_default_length_in_bytes(&self) -> usize;
    fn get_hash_code<const N: usize>(&mut self, message: &[u8; N], counter: u32) -> [u8; N];
}

macro_rules! trait_Hash_impl {
    ($($hash_type:ty), *) => {
        $(
            impl Hash for $hash_type
            {
                #[inline]
                fn get_default_length_in_bytes(&self) -> usize
                    { Self::DEFUALT_OUTPUT_LENGTH_IN_BYTES }

                fn get_hash_code<const N: usize>(&mut self, message: &[u8; N], counter: u32) -> [u8; N]
                {
                    let mut message_array = Vec::<u8>::from(message);
                    let counter = counter.to_be_bytes();
                    for i in 0..4
                        { message_array.push(counter[i]); }
                    let mut hash = [0_u8; N];
                    self.digest_vec(&message_array);
                    self.put_hash_value_in_array(&mut hash);
                    hash
                }
            }
        )*
    };
}

trait_Hash_impl!{ MD4, MD5, SHA0, SHA1,
                  SHA2_224, SHA2_256, SHA2_384, SHA2_512, SHA2_512_256, SHA2_512_t_224,
                  SHA3_256, SHA3_384, SHA3_512 }
