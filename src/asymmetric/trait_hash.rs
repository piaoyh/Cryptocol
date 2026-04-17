// Copyright 2024, 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use crate::number::SmallUInt;
use crate::hash::{  MD4, MD5, SHA0, SHA1,
                    SHA2_224, SHA2_256, SHA2_384,
                    SHA2_512, SHA2_512_256, SHA2_512_t_224,
                    Keccak_Generic, SHA3_256, SHA3_384, SHA3_512 };


#[allow(non_camel_case_types)]
pub trait Hash
{
    fn box_new() -> Box<Self> where Self: Sized;
    fn get_default_length_in_bytes(&self) -> usize;
    fn calculate_hash_code(&mut self, message: &[u8], counter: u32) -> Vec<u8>;
}

macro_rules! trait_Hash_impl {
    ($($hash_type:ty), *) => {
        $(
            impl Hash for $hash_type
            {
                fn box_new() -> Box<Self> where Self: Sized
                {
                    Box::new(Self::new())
                }

                #[inline]
                fn get_default_length_in_bytes(&self) -> usize
                {
                    Self::DEFUALT_OUTPUT_LENGTH_IN_BYTES
                }

                fn calculate_hash_code(&mut self, message: &[u8], counter: u32) -> Vec<u8>
                {
                    let mut message_array = Vec::<u8>::from(message);
                    let counter = counter.to_be_bytes();
                    for i in 0..4
                        { message_array.push(counter[i]); }
                    self.digest_vec(&message_array);
                    let mut a = self.get_hash_value_in_vec();
                    let b = Vec::<u32>::new();
                    b.
                }
            }
        )*
    };
}

trait_Hash_impl!{ MD4, MD5, SHA0, SHA1,
                  SHA2_224, SHA2_256, SHA2_384, SHA2_512, SHA2_512_256, SHA2_512_t_224,
                  SHA3_256, SHA3_384, SHA3_512 }



impl<const RATE: usize, const PADDING: usize, const ROUNDS: usize, T, const LFSR: u8,
    const THETA_SUB: usize, const THETA_ADD: usize, const THETA_ROT: u32,
    const RHO_MUL_X: usize, const RHO_MUL_Y: usize, const RHO_T: u32,
    const PI_MUL_X: usize, const PI_MUL_Y: usize,
    const CHI_ADD_1: usize, const CHI_ADD_2: usize>
Keccak_Generic<RATE, PADDING, ROUNDS, T, LFSR,
                THETA_SUB, THETA_ADD, THETA_ROT,
                RHO_MUL_X, RHO_MUL_Y, RHO_T,
                PI_MUL_X, PI_MUL_Y, CHI_ADD_1, CHI_ADD_2>
where T: SmallUInt
{
    fn put_hash_value_in_array<const N: usize>(&mut self, hash_value: &mut [u8; N])
    {
        self.get_hash_value(hash_value.as_mut_ptr(), N);
    }
}