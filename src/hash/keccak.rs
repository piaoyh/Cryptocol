// Copyright 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]
// #![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]


use std::ptr::{ copy_nonoverlapping, write_bytes };
use std::fmt::{ self, Debug, Display, Formatter };
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign,
                BitXor, BitXorAssign, Not, Shl };

use crate::number::{ SmallUInt, LongUnion };


#[allow(non_camel_case_types)]
pub struct KECCAK_CONST {}

impl KECCAK_CONST
{
    pub const SHA3: usize   = 0;    // SHA-3
    pub const SHAKE: usize  = 1;    // SHAKE
    pub const CSHAKE: usize = 2;    // cSHAKE
    pub const KECCAK: usize = 3;    // KECCAK
}


macro_rules! run_register {
    ($STATE:expr, $LFSR:expr) => {
        (($STATE >> 1) | ((($STATE & $LFSR).count_ones() as u8) << 7), $STATE & 1 == 1)
        // {
        //     let mut state = $STATE << 1;
        //     let msb = $STATE & 0b_1000_0000;
        //     if msb != 0
        //         { state ^= $LFSR }
        //     (state, state & 0b_0000_0001 == 1)
        // }
    };
}

macro_rules! make_RC {
    ($T:ty, $ROUNDS:expr, $LFSR:expr) => {
        {
            #[allow(non_snake_case)] let mut union_A = A::<$T, $ROUNDS> { U128: [0_u128; ROUNDS] };
            #[allow(non_snake_case)] let WIDTH = <$T>::BITS as usize;
            #[allow(non_snake_case)] let mut RC = [<$T>::MIN; ROUNDS];
            let mut bit = [0_usize; 7];
            let mut j = 0_usize;
            while j < 7_usize
            {
                bit[j] = ((1_usize << j) - 1) % WIDTH;
                j += 1;
            }
            let mut state = 1_u8;
            let mut output: bool;
            let mut i = 0_usize;
            while i < $ROUNDS
            {
                j = 0_usize;
                while j < 7_usize
                {
                    (state, output) = run_register!(state, $LFSR);
                    if output
                    {
                        unsafe {
                            match WIDTH
                            {
                                128 =>  { union_A.U128[i] |= 1_u128 << bit[j]; },
                                64 =>   { union_A.U64[i] |= 1_u64 << bit[j]; },
                                32 =>   { union_A.U32[i] |= 1_u32 << bit[j]; },
                                16 =>   { union_A.U16[i] |= 1_u16 << bit[j]; },
                                8 =>    { union_A.U8[i] |= 1_u8 << bit[j]; },
                                _ =>    {},
                            }
                        }
                    }
                    j += 1;
                }
                unsafe { RC[i] = union_A.RC[i]; }
                i += 1;
            }
            RC
        }
    }
}


#[allow(non_snake_case)]
union A<T, const ROUNDS: usize>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + Shl<Output = T>
{
    RC:    [T; ROUNDS],
    U128:  [u128; ROUNDS],
    U64:   [u64; ROUNDS],
    U32:   [u32; ROUNDS],
    U16:   [u16; ROUNDS],
    U8:    [u8; ROUNDS],
}



/// BIG_KECCAK_224 is non-standard hash alogrithm
/// but one of the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 224) = 448 bits
/// - The parameter R = B (= 3200) - C (= 448) = 2752 bits (= 344 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_KECCAK_224 = Keccak_Generic<344, {KECCAK_CONST::KECCAK}, 26, u128>;

/// BIG_KECCAK_256 is non-standard hash alogrithm
/// but one of the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 256) = 512 bits
/// - The parameter R = B (= 3200) - C (= 512) = 2688 bits (= 336 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_KECCAK_256 = Keccak_Generic<336, {KECCAK_CONST::KECCAK}, 26, u128>;

/// BIG_KECCAK_384 is non-standard hash alogrithm
/// but one of the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 384) = 768 bits
/// - The parameter R = B (= 3200) - C (= 768) = 2432 bits (= 304 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_KECCAK_384 = Keccak_Generic<304, {KECCAK_CONST::KECCAK}, 26, u128>;

/// BIG_KECCAK_512 is non-standard hash alogrithm
/// but one of the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 512) = 1024 bits
/// - The parameter R = B (= 3200) - C (= 1024) = 2176 bits (= 272 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_KECCAK_512 = Keccak_Generic<272, {KECCAK_CONST::KECCAK}, 26, u128>;

/// BIG_KECCAK_768 is non-standard hash alogrithm
/// but one of the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 768) = 1536 bits
/// - The parameter R = B (= 3200) - C (= 1536) = 1664 bits (= 208 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_KECCAK_768 = Keccak_Generic<208, {KECCAK_CONST::KECCAK}, 26, u128>;

/// BIG_KECCAK_1024 is non-standard hash alogrithm
/// but one of the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 1024) = 2048 bits
/// - The parameter R = B (= 3200) - C (= 2048) = 1152 bits (= 144 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_KECCAK_1024 = Keccak_Generic<144, {KECCAK_CONST::KECCAK}, 26, u128>;

/// BIG_KECCAK_1536 is non-standard hash alogrithm
/// but one of the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 1536) = 3072 bits
/// - The parameter R = B (= 3200) - C (= 3072) = 128 bits (= 16 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_KECCAK_1536 = Keccak_Generic<16, {KECCAK_CONST::KECCAK}, 26, u128>;

/// BIG_cSHAKE_128 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X security level (= 128) = 256 bits
/// - The parameter R = B (= 3200) - C (= 256) = 2944 bits (= 368 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_cSHAKE_128 = Keccak_Generic<368, {KECCAK_CONST::CSHAKE}, 26, u128>;

/// BIG_cSHAKE_224 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 224) = 448 bits
/// - The parameter R = B (= 3200) - C (= 448) = 2752 bits (= 344 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_cSHAKE_224 = Keccak_Generic<344, {KECCAK_CONST::CSHAKE}, 26, u128>;

/// BIG_cSHAKE_256 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X security level (= 256) = 512 bits
/// - The parameter R = B (= 3200) - C (= 512) = 2688 bits (= 336 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_cSHAKE_256 = Keccak_Generic<336, {KECCAK_CONST::CSHAKE}, 26, u128>;

/// BIG_cSHAKE_384 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X security level (= 384) = 768 bits
/// - The parameter R = B (= 3200) - C (= 768) = 2432 bits (= 304 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_cSHAKE_384 = Keccak_Generic<304, {KECCAK_CONST::CSHAKE}, 26, u128>;

/// BIG_cSHAKE_512 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X security level (= 512) = 1024 bits
/// - The parameter R = B (= 3200) - C (= 1024) = 2176 bits (= 272 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_cSHAKE_512 = Keccak_Generic<272, {KECCAK_CONST::CSHAKE}, 26, u128>;

/// BIG_cSHAKE_768 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X security level (= 768) = 1536 bits
/// - The parameter R = B (= 3200) - C (= 1536) = 1664 bits (= 208 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_cSHAKE_768 = Keccak_Generic<208, {KECCAK_CONST::CSHAKE}, 26, u128>;

/// BIG_cSHAKE_1024 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X security level (= 1024) = 2048 bits
/// - The parameter R = B (= 3200) - C (= 2048) = 1152 bits (= 144 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_cSHAKE_1024 = Keccak_Generic<144, {KECCAK_CONST::CSHAKE}, 26, u128>;

/// BIG_cSHAKE_1536 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X security level (= 1536) = 3072 bits
/// - The parameter R = B (= 3200) - C (= 3072) = 128 bits (= 16 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_cSHAKE_1536 = Keccak_Generic<16, {KECCAK_CONST::CSHAKE}, 26, u128>;

/// BIG_SHAKE_128 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X security level (= 128) = 256 bits
/// - The parameter R = B (= 3200) - C (= 256) = 2944 bits (= 368 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_SHAKE_128 = Keccak_Generic<368, {KECCAK_CONST::SHAKE}, 26, u128>;

/// BIG_SHAKE_224 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 224) = 448 bits
/// - The parameter R = B (= 3200) - C (= 448) = 2752 bits (= 344 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_SHAKE_224 = Keccak_Generic<344, {KECCAK_CONST::SHAKE}, 26, u128>;

/// BIG_SHAKE_256 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X security level (= 256) = 512 bits
/// - The parameter R = B (= 3200) - C (= 512) = 2688 bits (= 336 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_SHAKE_256 = Keccak_Generic<336, {KECCAK_CONST::SHAKE}, 26, u128>;

/// BIG_SHAKE_384 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X security level (= 384) = 768 bits
/// - The parameter R = B (= 3200) - C (= 768) = 2432 bits (= 304 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_SHAKE_384 = Keccak_Generic<304, {KECCAK_CONST::SHAKE}, 26, u128>;

/// BIG_SHAKE_512 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X security level (= 512) = 1024 bits
/// - The parameter R = B (= 3200) - C (= 1024) = 2176 bits (= 272 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_SHAKE_512 = Keccak_Generic<272, {KECCAK_CONST::SHAKE}, 26, u128>;

/// BIG_SHAKE_768 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 768) = 1536 bits
/// - The parameter R = B (= 3200) - C (= 1536) = 1664 bits (= 208 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_SHAKE_768 = Keccak_Generic<208, {KECCAK_CONST::SHAKE}, 26, u128>;

/// BIG_SHAKE_1024 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X security level (= 1024) = 2048 bits
/// - The parameter R = B (= 3200) - C (= 2048) = 1152 bits (= 144 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_SHAKE_1024 = Keccak_Generic<144, {KECCAK_CONST::SHAKE}, 26, u128>;

/// BIG_SHAKE_1536 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X security level (= 1536) = 3072 bits
/// - The parameter R = B (= 3200) - C (= 3072) = 128 bits (= 16 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_SHAKE_1536 = Keccak_Generic<16, {KECCAK_CONST::SHAKE}, 26, u128>;

/// BIG_SHA3_224 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 224) = 448 bits
/// - The parameter R = B (= 3200) - C (= 448) = 2752 bits (= 344 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_SHA3_224 = Keccak_Generic<344, {KECCAK_CONST::SHA3}, 26, u128>;

/// BIG_SHA3_256 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 256) = 512 bits
/// - The parameter R = B (= 3200) - C (= 512) = 2688 bits (= 336 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_SHA3_256 = Keccak_Generic<336, {KECCAK_CONST::SHA3}, 26, u128>;

/// BIG_SHA3_384 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 384) = 768 bits
/// - The parameter R = B (= 3200) - C (= 768) = 2432 bits (= 304 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_SHA3_384 = Keccak_Generic<304, {KECCAK_CONST::SHA3}, 26, u128>;

/// BIG_SHA3_512 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 512) = 1024 bits
/// - The parameter R = B (= 3200) - C (= 1024) = 2176 bits (= 272 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_SHA3_512 = Keccak_Generic<272, {KECCAK_CONST::SHA3}, 26, u128>;

/// BIG_SHA3_768 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 768) = 1536 bits
/// - The parameter R = B (= 3200) - C (= 1536) = 1664 bits (= 208 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_SHA3_768 = Keccak_Generic<208, {KECCAK_CONST::SHA3}, 26, u128>;

/// BIG_SHA3_1024 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 1024) = 2048 bits
/// - The parameter R = B (= 3200) - C (= 2048) = 1152 bits (= 144 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_SHA3_1024 = Keccak_Generic<144, {KECCAK_CONST::SHA3}, 26, u128>;

/// BIG_SHA3_1536 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 7
/// - The parameter W = 2 ^ L (= 7) = 128 bits which means `u128`
/// - The parameter B = 25 X W (= 128) = 3200 bits (= 400 bytes)
///   which is the whole size of the state, ```[[u128; 5]; 5]```
/// - The parameter C = 2 X output length (= 1536) = 3072 bits
/// - The parameter R = B (= 3200) - C (= 3072) = 128 bits (= 16 bytes)
/// - The number of Rounds is 12 + 2 X L (= 7) = 26 rounds
#[allow(non_camel_case_types)]
pub type BIG_SHA3_1536 = Keccak_Generic<16, {KECCAK_CONST::SHA3}, 26, u128>;

/// Keccak-224 is non-standard hash alogrithm but one of the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X output length (= 224) = 448 bits
/// - The parameter R = B (= 1600) - C (= 448) = 1152 bits (= 144 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
/// 
/// # Simple Example
/// ```
/// use cryptocol::hash::KECCAK_224;
/// let mut keccak = KECCAK_224::new();
/// keccak.absorb_str("");
/// let txt = keccak.get_hash_value_in_string();
/// println!("keccak = {}", txt);
/// assert_eq!(txt, "F71837502BA8E10837BDD8D365ADB85591895602FC552B48B7390ABD");
/// ```
#[allow(non_camel_case_types)]
pub type KECCAK_224 = Keccak_Generic<144, {KECCAK_CONST::KECCAK}>;

/// Keccak-256 is non-standard hash alogrithm but one of the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X output length (= 256) = 512 bits
/// - The parameter R = B (= 1600) - C (= 512) = 1088 bits (= 136 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
/// 
/// # Simple Example
/// ```
/// use cryptocol::hash::KECCAK_256;
/// 
/// let mut keccak = KECCAK_256::new();
/// keccak.absorb_str("");
/// let txt = keccak.get_hash_value_in_string();
/// println!("keccak = {}", txt);
/// assert_eq!(txt, "C5D2460186F7233C927E7DB2DCC703C0E500B653CA82273B7BFAD8045D85A470");
/// ```
#[allow(non_camel_case_types)]
pub type KECCAK_256 = Keccak_Generic<136, {KECCAK_CONST::KECCAK}>;

/// Keccak-384 is non-standard hash alogrithm but one of the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X output length (= 384) = 768 bits
/// - The parameter R = B (= 1600) - C (= 768) = 832 bits (= 104 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
/// 
/// # Simple Example
/// ```
/// use cryptocol::hash::KECCAK_384;
/// let mut keccak = KECCAK_384::new();
/// keccak.absorb_str("");
/// let txt = keccak.get_hash_value_in_string();
/// println!("keccak = {}", txt);
/// assert_eq!(txt, "2C23146A63A29ACF99E73B88F8C24EAA7DC60AA771780CCC006AFBFA8FE2479B2DD2B21362337441AC12B515911957FF");
/// ```
#[allow(non_camel_case_types)]
pub type KECCAK_384 = Keccak_Generic<104, {KECCAK_CONST::KECCAK}>;

/// Keccak-512 is non-standard hash alogrithm but one of the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X output length (= 512) = 1024 bits
/// - The parameter R = B (= 1600) - C (= 1024) = 576 bits (= 72 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
/// 
/// # Simple Example
/// ```
/// use cryptocol::hash::KECCAK_512;
/// let mut keccak = KECCAK_512::new();
/// keccak.absorb_str("");
/// let txt = keccak.get_hash_value_in_string();
/// println!("keccak = {}", txt);
/// assert_eq!(txt, "0EAB42DE4C3CEB9235FC91ACFFE746B29C29A8C366B7C60E4E67C466F36A4304C00FA9CAF9D87976BA469BCBE06713B435F091EF2769FB160CDAB33D3670680E");
/// ```
#[allow(non_camel_case_types)]
pub type KECCAK_512 = Keccak_Generic<72, {KECCAK_CONST::KECCAK}>;

/// Keccak-768 is non-standard hash alogrithm but one of the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X output length (= 768) = 1536 bits
/// - The parameter R = B (= 1600) - C (= 1536) = 64 bits (= 8 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
#[allow(non_camel_case_types)]
pub type KECCAK_768 = Keccak_Generic<8, {KECCAK_CONST::KECCAK}>;

/// cSHAKE-128 is one of the SHA-3 family members
/// which is standard hash alogrithms.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X security level (= 128) = 256 bits
/// - The parameter R = B (= 1600) - C (= 256) = 1344 bits (= 168 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
/// 
/// # Simple Example
/// ```
/// use cryptocol::hash::cSHAKE_128;
/// 
/// let mut shake = cSHAKE_128::new();
/// shake.absorb_vec_customized(&"".to_string().into_bytes(), &"Email Signature".to_string().into_bytes(), &vec![0_u8, 1, 2, 3]);
/// let txt = shake.get_hash_value_in_string();
/// println!("cSHAKE = {}", txt);
/// assert_eq!(txt, "C1C36925B6409A04F1B504FCBCA9D82B4017277CB5ED2B2065FC1D3814D5AAF5");
/// 
/// let mut data = Vec::<u8>::new();
/// for i in 0..200
///     { data.push(i as u8); }
/// shake.absorb_vec_customized(&"".to_string().into_bytes(), &"Email Signature".to_string().into_bytes(), &data);
/// let txt = shake.get_hash_value_in_string();
/// println!("cSHAKE = {}", txt);
/// assert_eq!(txt, "C5221D50E4F822D96A2E8881A961420F294B7B24FE3D2094BAED2C6524CC166B");
///
/// shake.absorb_str_customized("PARK", "Youngho", "In the beginning God created the heavens and the earth.");
/// let txt = shake.get_hash_value_in_string();
/// println!("cSHAKE = {}", txt);
/// assert_eq!(txt, "25685E3E59672612F86AB24E418EB610B6F5F7D299E1B315FD9B59BD698A4AC9");
/// ```
#[allow(non_camel_case_types)]
pub type cSHAKE_128 = Keccak_Generic<168, {KECCAK_CONST::CSHAKE}>;

/// cSHAKE-224 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X security level (= 224) = 448 bits
/// - The parameter R = B (= 1600) - C (= 448) = 1152 bits (= 144 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
#[allow(non_camel_case_types)]
pub type cSHAKE_224 = Keccak_Generic<144, {KECCAK_CONST::CSHAKE}>;

/// cSHAKE-256 is one of the SHA-3 family members
/// which is standard hash alogrithms.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X security level (= 256) = 512 bits
/// - The parameter R = B (= 1600) - C (= 512) = 1088 bits (= 136 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
/// 
/// # Simple Example
/// ```
/// use cryptocol::hash::cSHAKE_256;
/// let mut shake = cSHAKE_256::new();
/// shake.absorb_str_customized("PARK", "Youngho", "In the beginning God created the heavens and the earth.");
/// let txt = shake.get_hash_value_in_string();
/// println!("cSHAKE = {}", txt);
/// assert_eq!(txt, "490561E775E7CCF90084522B6D8F9DDFAF087C198ABC788737DAC198795242A29B520951D09E8096C441EF88EB53B48AC43B2E7FF9416CF9F32A897BA2EE99FC");
/// ```
#[allow(non_camel_case_types)]
pub type cSHAKE_256 = Keccak_Generic<136, {KECCAK_CONST::CSHAKE}>;

/// cSHAKE_384 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X security level (= 384) = 768 bits
/// - The parameter R = B (= 1600) - C (= 768) = 832 bits (= 104 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
#[allow(non_camel_case_types)]
pub type cSHAKE_384 = Keccak_Generic<104, {KECCAK_CONST::CSHAKE}>;

/// cSHAKE_512 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X security level (= 512) = 1024 bits
/// - The parameter R = B (= 1600) - C (= 1024) = 576 bits (= 72 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
#[allow(non_camel_case_types)]
pub type cSHAKE_512 = Keccak_Generic<72, {KECCAK_CONST::CSHAKE}>;

/// cSHAKE_768 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X security level (= 768) = 1536 bits
/// - The parameter R = B (= 1600) - C (= 1536) = 64 bits (= 8 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
#[allow(non_camel_case_types)]
pub type cSHAKE_768 = Keccak_Generic<8, {KECCAK_CONST::CSHAKE}>;

/// SHAKE-128 is one of the SHA-3 family members
/// which is standard hash alogrithms.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X security level (= 128) = 256 bits
/// - The parameter R = B (= 1600) - C (= 256) = 1344 bits (= 168 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
/// 
/// # Simple Example
/// ```
/// use cryptocol::hash::SHAKE_128;
/// let mut shake = SHAKE_128::new();
/// shake.absorb_str("");
/// let txt = shake.get_hash_value_in_string();
/// println!("shake = {}", txt);
/// assert_eq!(txt, "7F9C2BA4E88F827D616045507605853ED73B8093F6EFBC88EB1A6EACFA66EF26");
/// ```
#[allow(non_camel_case_types)]
pub type SHAKE_128 = Keccak_Generic<168, {KECCAK_CONST::SHAKE}>;

/// SHAKE-224 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X security level (= 224) = 448 bits
/// - The parameter R = B (= 1600) - C (= 448) = 1152 bits (= 144 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
#[allow(non_camel_case_types)]
pub type SHAKE_224 = Keccak_Generic<144, {KECCAK_CONST::SHAKE}>;

/// SHAKE-256 is one of the SHA-3 family members
/// which is standard hash alogrithms.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X security level (= 256) = 512 bits
/// - The parameter R = B (= 1600) - C (= 512) = 1088 bits (= 136 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
/// 
/// # Simple Example
/// ```
/// use cryptocol::hash::SHAKE_256;
/// let mut shake = SHAKE_256::new();
/// shake.absorb_str("");
/// let txt = shake.get_hash_value_in_string();
/// println!("shake = {}", txt);
/// assert_eq!(txt, "46B9DD2B0BA88D13233B3FEB743EEB243FCD52EA62B81B82B50C27646ED5762FD75DC4DDD8C0F200CB05019D67B592F6FC821C49479AB48640292EACB3B7C4BE");
/// ```
#[allow(non_camel_case_types)]
pub type SHAKE_256 = Keccak_Generic<136, {KECCAK_CONST::SHAKE}>;

/// SHAKE_384 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X security level (= 384) = 768 bits
/// - The parameter R = B (= 1600) - C (= 768) = 832 bits (= 104 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
#[allow(non_camel_case_types)]
pub type SHAKE_384 = Keccak_Generic<104, {KECCAK_CONST::SHAKE}>;

/// SHAKE_512 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X security level (= 512) = 1024 bits
/// - The parameter R = B (= 1600) - C (= 1024) = 576 bits (= 72 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
#[allow(non_camel_case_types)]
pub type SHAKE_512 = Keccak_Generic<72, {KECCAK_CONST::SHAKE}>;

/// SHAKE_768 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X security level (= 768) = 1536 bits
/// - The parameter R = B (= 1600) - C (= 1536) = 64 bits (= 8 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
#[allow(non_camel_case_types)]
pub type SHAKE_768 = Keccak_Generic<8, {KECCAK_CONST::SHAKE}>;

/// SHA-3-224 is one of the SHA-3 family members
/// which is standard hash alogrithms.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X output length (= 224) = 448 bits
/// - The parameter R = B (= 1600) - C (= 448) = 1152 bits (= 144 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
/// 
/// # Simple Example
/// ```
/// use cryptocol::hash::SHA3_224;
/// let mut sha3 = SHA3_224::new();
/// let data = [0xA3_u8; 8 * 25];
/// sha3.absorb_array(&data);
/// let txt = sha3.get_hash_value_in_string();
/// println!("sha3 = {}", txt);
/// assert_eq!(txt, "9376816ABA503F72F96CE7EB65AC095DEEE3BE4BF9BBC2A1CB7E11E0");
/// let mut sha3 = SHA3_224::new();
/// sha3.absorb_array(&data);
/// let v = sha3.get_hash_value_in_vec();
/// print!("sha3 = ");
/// for vv in v
/// {
///     print!("{:02X}", vv);
/// }
/// println!("\n");
/// ```
#[allow(non_camel_case_types)]
pub type SHA3_224 = Keccak_Generic<144>;

/// SHA-3-256 is one of the SHA-3 family members
/// which is standard hash alogrithms.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X output length (= 256) = 512 bits
/// - The parameter R = B (= 1600) - C (= 512) = 1088 bits (= 136 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
/// 
/// # Simple Example
/// ```
/// use cryptocol::hash::SHA3_256;
/// let mut sha3 = SHA3_256::new();
/// sha3.absorb_str("");
/// let txt = sha3.get_hash_value_in_string();
/// println!("sha3 = {}", txt);
/// assert_eq!(txt, "A7FFC6F8BF1ED76651C14756A061D662F580FF4DE43B49FA82D80A4B80F8434A");
/// ```
#[allow(non_camel_case_types)]
pub type SHA3_256 = Keccak_Generic<136>;

/// SHA-3-384 is one of the SHA-3 family members
/// which is standard hash alogrithms.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X output length (= 384) = 768 bits
/// - The parameter R = B (= 1600) - C (= 768) = 832 bits (= 104 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
/// 
/// # Simple Example
/// ```
/// use cryptocol::hash::SHA3_384;
/// let mut sha3 = SHA3_384::new();
/// sha3.absorb_str("");
/// let txt = sha3.get_hash_value_in_string();
/// println!("sha3 = {}", txt);
/// assert_eq!(txt, "0C63A75B845E4F7D01107D852E4C2485C51A50AAAA94FC61995E71BBEE983A2AC3713831264ADB47FB6BD1E058D5F004");
/// ```
#[allow(non_camel_case_types)]
pub type SHA3_384 = Keccak_Generic<104>;

/// SHA-3-512 is one of the SHA-3 family members
/// which is standard hash alogrithms.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X output length (= 512) = 1024 bits
/// - The parameter R = B (= 1600) - C (= 1024) = 576 bits (= 72 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
/// 
/// # Simple Example
/// ```
/// use cryptocol::hash::SHA3_512;
/// let mut sha3 = SHA3_512::new();
/// sha3.absorb_str("");
/// let txt = sha3.get_hash_value_in_string();
/// println!("sha3 = {}", txt);
/// assert_eq!(txt, "A69F73CCA23A9AC5C8B567DC185A756E97C982164FE25859E0D1DCC1475C80A615B2123AF1F5F94C11E3E9402C3AC558F500199D95B6D3E301758586281DCD26");
/// ```
#[allow(non_camel_case_types)]
pub type SHA3_512 = Keccak_Generic;

/// SHA3_768 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 6
/// - The parameter W = 2 ^ L (= 6) = 64 bits which means `u64`
/// - The parameter B = 25 X W (=64) = 1600 bits (= 200 bytes)
///   which is the whole size of the state, ```[[u64; 5]; 5]```
/// - The parameter C = 2 X output length (= 768) = 1536 bits
/// - The parameter R = B (= 1600) - C (= 1536) = 64 bits (= 8 bytes)
/// - The number of Rounds is 12 + 2 X L (= 6) = 24 rounds
#[allow(non_camel_case_types)]
pub type SHA3_768 = Keccak_Generic<8>;

/// SMALL_KECCAK_224 is non-standard hash alogrithm
/// but one of the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 5
/// - The parameter W = 2 ^ L (= 5) = 32 bits which means `u32`
/// - The parameter B = 25 X W (=32) = 800 bits (= 100 bytes)
///   which is the whole size of the state, ```[[u32; 5]; 5]```
/// - The parameter C = 2 X output length (= 224) = 448 bits
/// - The parameter R = B (= 800) - C (= 448) = 352 bits (= 44 bytes)
/// - The number of Rounds is 12 + 2 X L (= 5) = 22 rounds
#[allow(non_camel_case_types)]
pub type SMALL_KECCAK_224 = Keccak_Generic<39, {KECCAK_CONST::KECCAK}, 22, u32>;

/// SMALL_KECCAK_256 is non-standard hash alogrithm
/// but one of the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 5
/// - The parameter W = 2 ^ L (= 5) = 32 bits which means `u32`
/// - The parameter B = 25 X W (=32) = 800 bits (= 100 bytes)
///   which is the whole size of the state, ```[[u32; 5]; 5]```
/// - The parameter C = 2 X output length (= 256) = 512 bits
/// - The parameter R = B (= 800) - C (= 512) = 288 bits (= 36 bytes)
/// - The number of Rounds is 12 + 2 X L (= 5) = 22 rounds
#[allow(non_camel_case_types)]
pub type SMALL_KECCAK_256 = Keccak_Generic<36, {KECCAK_CONST::KECCAK}, 22, u32>;

/// SMALL_KECCAK_384 is non-standard hash alogrithm
/// but one of the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 5
/// - The parameter W = 2 ^ L (= 5) = 32 bits which means `u32`
/// - The parameter B = 25 X W (=32) = 800 bits (= 100 bytes)
///   which is the whole size of the state, ```[[u32; 5]; 5]```
/// - The parameter C = 2 X output length (= 384) = 768 bits
/// - The parameter R = B (= 800) - C (= 768) = 32 bits (= 4 bytes)
/// - The number of Rounds is 12 + 2 X L (= 5) = 22 rounds
#[allow(non_camel_case_types)]
pub type SMALL_KECCAK_384 = Keccak_Generic<4, {KECCAK_CONST::KECCAK}, 22, u32>;

/// SMALL_cSHAKE_128 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 5
/// - The parameter W = 2 ^ L (= 5) = 32 bits which means `u32`
/// - The parameter B = 25 X W (=32) = 800 bits (= 100 bytes)
///   which is the whole size of the state, ```[[u32; 5]; 5]```
/// - The parameter C = 2 X security level (= 128) = 256 bits
/// - The parameter R = B (= 800) - C (= 256) = 544 bits (= 68 bytes)
/// - The number of Rounds is 12 + 2 X L (= 5) = 22 rounds
#[allow(non_camel_case_types)]
pub type SMALL_cSHAKE_128 = Keccak_Generic<68, {KECCAK_CONST::CSHAKE}, 22, u32>;

/// SMALL_cSHAKE_224 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 5
/// - The parameter W = 2 ^ L (= 5) = 32 bits which means `u32`
/// - The parameter B = 25 X W (=32) = 800 bits (= 100 bytes)
///   which is the whole size of the state, ```[[u32; 5]; 5]```
/// - The parameter C = 2 X output length (= 224) = 448 bits
/// - The parameter R = B (= 800) - C (= 448) = 352 bits (= 44 bytes)
/// - The number of Rounds is 12 + 2 X L (= 5) = 22 rounds
#[allow(non_camel_case_types)]
pub type SMALL_cSHAKE_224 = Keccak_Generic<39, {KECCAK_CONST::CSHAKE}, 22, u32>;

/// SMALL_cSHAKE_256 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 5
/// - The parameter W = 2 ^ L (= 5) = 32 bits which means `u32`
/// - The parameter B = 25 X W (=32) = 800 bits (= 100 bytes)
///   which is the whole size of the state, ```[[u32; 5]; 5]```
/// - The parameter C = 2 X security level (= 256) = 512 bits
/// - The parameter R = B (= 800) - C (= 512) = 288 bits (= 36 bytes)
/// - The number of Rounds is 12 + 2 X L (= 5) = 22 rounds
#[allow(non_camel_case_types)]
pub type SMALL_cSHAKE_256 = Keccak_Generic<36, {KECCAK_CONST::CSHAKE}, 22, u32>;

/// SMALL_SHAKE_128 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 5
/// - The parameter W = 2 ^ L (= 5) = 32 bits which means `u32`
/// - The parameter B = 25 X W (=32) = 800 bits (= 100 bytes)
///   which is the whole size of the state, ```[[u32; 5]; 5]```
/// - The parameter C = 2 X security level (= 128) = 256 bits
/// - The parameter R = B (= 800) - C (= 256) = 544 bits (= 68 bytes)
/// - The number of Rounds is 12 + 2 X L (= 5) = 22 rounds
#[allow(non_camel_case_types)]
pub type SMALL_SHAKE_128 = Keccak_Generic<68, {KECCAK_CONST::SHAKE}, 22, u32>;

/// SMALL_SHAKE_224 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 5
/// - The parameter W = 2 ^ L (= 5) = 32 bits which means `u32`
/// - The parameter B = 25 X W (=32) = 800 bits (= 100 bytes)
///   which is the whole size of the state, ```[[u32; 5]; 5]```
/// - The parameter C = 2 X output length (= 224) = 448 bits
/// - The parameter R = B (= 800) - C (= 448) = 352 bits (= 44 bytes)
/// - The number of Rounds is 12 + 2 X L (= 5) = 22 rounds
#[allow(non_camel_case_types)]
pub type SMALL_SHAKE_224 = Keccak_Generic<39, {KECCAK_CONST::SHAKE}, 22, u32>;

/// SMALL_SHAKE_256 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 5
/// - The parameter W = 2 ^ L (= 5) = 32 bits which means `u32`
/// - The parameter B = 25 X W (=32) = 800 bits (= 100 bytes)
///   which is the whole size of the state, ```[[u32; 5]; 5]```
/// - The parameter C = 2 X security level (= 256) = 512 bits
/// - The parameter R = B (= 800) - C (= 512) = 288 bits (= 36 bytes)
/// - The number of Rounds is 12 + 2 X L (= 5) = 22 rounds
#[allow(non_camel_case_types)]
pub type SMALL_SHAKE_256 = Keccak_Generic<36, {KECCAK_CONST::SHAKE}, 22, u32>;

/// SMALL_SHA3_224 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 5
/// - The parameter W = 2 ^ L (= 5) = 32 bits which means `u32`
/// - The parameter B = 25 X W (=32) = 800 bits (= 100 bytes)
///   which is the whole size of the state, ```[[u32; 5]; 5]```
/// - The parameter C = 2 X output length (= 224) = 448 bits
/// - The parameter R = B (= 800) - C (= 448) = 352 bits (= 44 bytes)
/// - The number of Rounds is 12 + 2 X L (= 5) = 22 rounds
#[allow(non_camel_case_types)]
pub type SMALL_SHA3_224 = Keccak_Generic<39, {KECCAK_CONST::SHA3}, 22, u32>;

/// SMALL_SHA3_256 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 5
/// - The parameter W = 2 ^ L (= 5) = 32 bits which means `u32`
/// - The parameter B = 25 X W (=32) = 800 bits (= 100 bytes)
///   which is the whole size of the state, ```[[u32; 5]; 5]```
/// - The parameter C = 2 X output length (= 256) = 448 bits
/// - The parameter R = B (= 800) - C (= 512) = 288 bits (= 36 bytes)
/// - The number of Rounds is 12 + 2 X L (= 5) = 22 rounds
#[allow(non_camel_case_types)]
pub type SMALL_SHA3_256 = Keccak_Generic<36, {KECCAK_CONST::SHA3}, 22, u32>;

/// SMALL_SHA3_384 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 5
/// - The parameter W = 2 ^ L (= 5) = 32 bits which means `u32`
/// - The parameter B = 25 X W (=32) = 800 bits (= 100 bytes)
///   which is the whole size of the state, ```[[u32; 5]; 5]```
/// - The parameter C = 2 X output length (= 384) = 768 bits
/// - The parameter R = B (= 800) - C (= 768) = 32 bits (= 4 bytes)
/// - The number of Rounds is 12 + 2 X L (= 5) = 22 rounds
#[allow(non_camel_case_types)]
pub type SMALL_SHA3_384 = Keccak_Generic<4, {KECCAK_CONST::SHA3}, 22, u32>;

/// SMALLER_KECCAK_128 is non-standard hash alogrithm
/// but one of the Keccak family members.
/// __This hash algorithm is NOT cryptographically secure so that you are not
/// encouraged to use this hash algorithm for serious purpose.__
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 4
/// - The parameter W = 2 ^ L (= 4) = 16 bits which means `u16`
/// - The parameter B = 25 X W (= 16) = 400 bits (= 50 bytes)
///   which is the whole size of the state, ```[[u16; 5]; 5]```
/// - The parameter C = 2 X output length (= 128) = 256 bits
/// - The parameter R = B (= 400) - C (= 256) = 144 bits (= 18 bytes)
/// - The number of Rounds is 12 + 2 X L (= 4) = 20 rounds
#[allow(non_camel_case_types)]
pub type SMALLER_KECCAK_128 = Keccak_Generic<18, {KECCAK_CONST::KECCAK}, 20, u16>;

/// SMALLER_cSHAKE_128 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 4
/// - The parameter W = 2 ^ L (= 4) = 16 bits which means `u16`
/// - The parameter B = 25 X W (= 16) = 400 bits (= 50 bytes)
///   which is the whole size of the state, ```[[u16; 5]; 5]```
/// - The parameter C = 2 X security level (= 128) = 256 bits
/// - The parameter R = B (= 400) - C (= 256) = 144 bits (= 18 bytes)
/// - The number of Rounds is 12 + 2 X L (= 4) = 20 rounds
#[allow(non_camel_case_types)]
pub type SMALLER_cSHAKE_128 = Keccak_Generic<18, {KECCAK_CONST::CSHAKE}, 20, u16>;

/// SMALLER_SHAKE_128 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 4
/// - The parameter W = 2 ^ L (= 4) = 16 bits which means `u16`
/// - The parameter B = 25 X W (= 16) = 400 bits (= 50 bytes)
///   which is the whole size of the state, ```[[u16; 5]; 5]```
/// - The parameter C = 2 X security level (= 128) = 256 bits
/// - The parameter R = B (= 400) - C (= 256) = 144 bits (= 18 bytes)
/// - The number of Rounds is 12 + 2 X L (= 4) = 20 rounds
#[allow(non_camel_case_types)]
pub type SMALLER_SHAKE_128 = Keccak_Generic<18, {KECCAK_CONST::SHAKE}, 20, u16>;

/// SMALLER_SHA3_128 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members.
/// __This hash algorithm is NOT cryptographically secure so that you are not
/// encouraged to use this hash algorithm for serious purpose.__
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 4
/// - The parameter W = 2 ^ L (= 4) = 16 bits which means `u16`
/// - The parameter B = 25 X W (= 16) = 400 bits (= 50 bytes)
///   which is the whole size of the state, ```[[u16; 5]; 5]```
/// - The parameter C = 2 X output length (= 128) = 256 bits
/// - The parameter R = B (= 400) - C (= 256) = 144 bits (= 18 bytes)
/// - The number of Rounds is 12 + 2 X L (= 4) = 20 rounds
#[allow(non_camel_case_types)]
pub type SMALLER_SHA3_128 = Keccak_Generic<18, {KECCAK_CONST::SHA3}, 20, u16>;

/// TINY_KECCAK_64 is non-standard hash alogrithm
/// but one of the Keccak family members. It is a toy version of Keccak or for
/// education purpose.
/// __This hash algorithm is NOT cryptographically secure so that you are not
/// encouraged to use this hash algorithm for serious purpose.__ It won't be
/// hard for you to find the pre-image collision that has the same hash code.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 3
/// - The parameter W = 2 ^ L (= 3) = 8 bits which means `u8`
/// - The parameter B = 25 X W (= 8) = 200 bits (= 25 bytes)
///   which is the whole size of the state, ```[[u8; 5]; 5]```
/// - The parameter C = 2 X output length (= 64) = 128 bits
/// - The parameter R = B (= 200) - C (= 128) = 72 bits (= 9 bytes)
/// - The number of Rounds is 12 + 2 X L (= 3) = 18 rounds
#[allow(non_camel_case_types)]
pub type TINY_KECCAK_64 = Keccak_Generic<9, {KECCAK_CONST::KECCAK}, 18, u8>;

/// TINY_cSHAKE_64 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members. It is a toy version of cSHAKE or for
/// education purpose.
/// __This hash algorithm is NOT cryptographically secure so that you are not
/// encouraged to use this hash algorithm for serious purpose.__ It won't be
/// hard for you to find the pre-image collision that has the same hash code.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 3
/// - The parameter W = 2 ^ L (= 3) = 8 bits which means `u8`
/// - The parameter B = 25 X W (= 8) = 200 bits (= 25 bytes)
///   which is the whole size of the state, ```[[u8; 5]; 5]```
/// - The parameter C = 2 X security level (= 64) = 128 bits
/// - The parameter R = B (= 200) - C (= 128) = 72 bits (= 9 bytes)
/// - The number of Rounds is 12 + 2 X L (= 3) = 18 rounds
#[allow(non_camel_case_types)]
pub type TINY_cSHAKE_64 = Keccak_Generic<9, {KECCAK_CONST::CSHAKE}, 18, u8>;

/// TINY_SHAKE_64 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members. It is a toy version of SHAKE or for
/// education purpose.
/// __This hash algorithm is NOT cryptographically secure so that you are not
/// encouraged to use this hash algorithm for serious purpose.__ It won't be
/// hard for you to find the pre-image collision that has the same hash code.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 3
/// - The parameter W = 2 ^ L (= 3) = 8 bits which means `u8`
/// - The parameter B = 25 X W (= 8) = 200 bits (= 25 bytes)
///   which is the whole size of the state, ```[[u8; 5]; 5]```
/// - The parameter C = 2 X security level (= 64) = 128 bits
/// - The parameter R = B (= 200) - C (= 128) = 72 bits (= 9 bytes)
/// - The number of Rounds is 12 + 2 X L (= 3) = 18 rounds
#[allow(non_camel_case_types)]
pub type TINY_SHAKE_64 = Keccak_Generic<9, {KECCAK_CONST::SHAKE}, 18, u8>;

/// TINY_SHA3_64 is non-standard hash alogrithm which is created by
/// expanding the Keccak family members. It is a toy version of SHA3 or for
/// education purpose.
/// __This hash algorithm is NOT cryptographically secure so that you are not
/// encouraged to use this hash algorithm for serious purpose.__ It won't be
/// hard for you to find the pre-image collision that has the same hash code.
/// According to the
/// [descryption](https://keccak.team/keccak_specs_summary.html) about Keccak:
/// - The parameter L = 3
/// - The parameter W = 2 ^ L (= 3) = 8 bits which means `u8`
/// - The parameter B = 25 X W (= 8) = 200 bits (= 25 bytes)
///   which is the whole size of the state, ```[[u8; 5]; 5]```
/// - The parameter C = 2 X output length (= 64) = 128 bits
/// - The parameter R = B (= 200) - C (= 128) = 72 bits (= 9 bytes)
/// - The number of Rounds is 12 + 2 X L (= 3) = 18 rounds
/// 
/// 
#[allow(non_camel_case_types)]
pub type TINY_SHA3_64 = Keccak_Generic<9, {KECCAK_CONST::SHA3}, 18, u8>;




/// A Keccak message-digest algorithm that lossily compresses data of arbitrary
/// length into a any-bit hash value, and its flexible variants that allows
/// you to develop your own `Keccak`-based hash algorithms
/// 
/// # Introduction
/// SHA stands for Secure Hash Algorithm, and SHA-3 means the third version of
/// SHA. The National Institute of Standards and Technology (NIST) released
/// SHA-3 as one of the standards of secure hash algorithms on August 5, 2015.
/// NIST is an agency of the United States Department of Commerce whose mission
/// is to promote American innovation and industrial competitiveness.
/// SHA-1 has been broken by Google on 23rd, February, 2017. SHA-1 uses
/// Merkle–Damgård construction. And, SHA-2 also uses same construction. So,
/// NIST wanted to provide another secure hash algorithm that uses the
/// construction other than Merkle–Damgård construction just in case SHA-2
/// will be broken too. In 2006, NIST started to organize the NIST hash
/// function competition to create a new hash standard so-called SHA-3. Finally,
/// Keccak was chosen as SHA-3 after changing Keccak algorithm a little bit.
/// SHA-3 uses a sponge construction, which consists of two phases: an absortion
/// phase and a squeezing phase. It digests a message in the absortion phase
/// while it produces the hash value of an arbitrary length of the message in
/// the squeezing phase. You can repeatedly squeeze the hash values and use them
/// as pseudo-random numbers, too.
/// SHA-2 has not been broken yet. As of 2022, NIST has no plan to withdraw
/// or remove SHA-2. You can directly substitute SHA-3 for SHA-2 in current
/// applications, if necessary, to significantly improve the robustness.
/// 
/// # Use of Keccak or SHA3 and their variants
/// You can use Keccak or SHA3 and its variants for cryptograpic purposes
/// such as:
/// - Generating IDs
/// - Integrity test
/// - Storing passwords
/// - Digital Signature
/// - Key generation
/// - Generating the cryptographically secure pseudo-random numbers
/// - Implementing proof of work for block chain.
/// - Study of hash algorithms
/// - Cryptanalysis Research to find the weakness of the sponge construction
///   of SHA-3 and Keccak which SHA3 family uses
/// 
/// # Generic Parameters
/// You can create your own expanded version of Keccak by changing the generic
/// parameters.
/// - RATE : The parameter `RATE` is in bytes though it is usually written in
///   bits in most of the documents. `RATE` determines how many bytes the
///   message will be absorbed at once.
///   If `RATE` is `72`, it is for SHA3-512.
///   If `RATE` is `104`, it is for SHA3-384.
///   If `RATE` is `136`, it is for either SHA3-256 or SHAKE-256.
///   If `RATE` is `144`, it is for SHA3-224.
///   If `RATE` is `168`, it is for SHAKE-128.
///   However, you can freely choose any number for `RATE` to create your own
///   hash algorithms. The default value is `72` for SHA3-512.
/// - PADDING : The parameter `PADDING` determines padding way. Only
///   KECCAK_CONST::SHA3 (= `0`), KECCAK_CONST::SHAKE (= `1`),
///   KECCAK_CONST::CSHAKE (= `2`), and KECCAK_CONST::KECCAK (= `3`) are
///   available for the parameter `PADDING`.
///   If `PADDING` is KECCAK_CONST::SHA3 or `0`, domain separator bits `01`
///   are appended, and the start bits of padding `1` is appended, and the
///   necessary `0`s are added, and then `1` is appended in order to make the
///   length of the message be a multiple of `RATE`. It is for SHA-3 standard.
///   If `PADDING` is KECCAK_CONST::SHAKE or `1`, domain separator bits `1111`
///   are appended, and the start bits of padding `1` is appended, and the
///   necessary `0`s are added, and then `1` is appended in order to make the
///   length of the message be a multiple of `RATE`. It is for SHAKE standard.
///   If `PADDING` is KECCAK_CONST::CSHAKE or `2`, domain separator bits `00`
///   are appended, and the start bits of padding `1` is appended, and the
///   necessary `0`s are added, and then `1` is appended in order to make the
///   length of the message be a multiple of `RATE`. It is for cSHAKE standard.
///   If `PADDING` is KECCAK_CONST::KECCAK or `3`, the start bits of padding `1`
///   is appended without domain separator bits, and the necessary `0`s are
///   appended, and then `1` is appended in order to make the length of the
///   message be a multiple of `RATE`. It is for Keccak standard.
///   The default value is `0` for for SHA-3 standard.
/// - ROUNDS : The parameter `ROUNDS` determines how many rounds the digesting
///   steps are repeated. By changing this parameter, you can change this
///   Keccak-based hash algorithm. Usually, for the official SHA3, SHAKE, and
///   cSHAKE, `ROUNDS` is 24. So, the default value is `24` for SHA3, SHAKE,
///   and cSHAKE.
/// - T : The parameter `T` is the datatype of the unit block to process. It
///   is one of `u8`, `u16`, `u32`, `u64`, and `u128`.
///   Usually, for the official SHA3, SHAKE, and cSHAKE, `T` is `u64`.
/// - LFSR : The parameter `LFSR` is the 8-bit linear feedback shift register.
///   It is expressed in the form of polynormial such as x^8 + x^6 + x^5 + x^4 +
///   1 (= x^0). The highest order term x^8 indicates 8-bit register. The
///   exponents 6, 5, 4, and 0 means the bits of the register to be taken, such
///   as 6th, 5th, 4th and 0th bits. The taken bits will be eXclusive ORed with
///   one another to get one-bit result. And then, the LSB (Least Segnificant
///   bit) is taken and the linear feedback shift register is rotated right with
///   feeding the one-bit result to the MSB (Most Segnificant bit). The taken
///   LSB is the output of the linear feedback shift register. You can change
///   which bits will be XORed. The the behaviour of the linear feedback shift
///   register will be changed. For example, if the polinormial of the linear
///   feedback shift register is x^8 + x^6 + x^5 + x^4 + 1, the value of the
///   parameter `LFSR` is 0b_0111_0001. And, if `LFSR` is 0b_1101_0100, the
///   linear feedback shift register has the polinormial x^7 + x^6 + x^4 + x^2.
///   If `LFSR` is 0x0, the step iota becomes virtually equivalent not to exist.
///   The default value of `LFSR` is 0b_0111_0001 for official SHA3, SHAKE,
///   cSHAKE, and Keccak.
/// - THETA_SUB : The parameter `THETA_SUB` is what will be subtracted from an
///   index (modular subtraction) of the state register in a theta step.
///   By changing this parameter, you can change the theta step. If THETA_SUB,
///   THETA_ADD, and THETA_ROT are all `zero`, the step theta becomes virtually
///   equivalent not to exist. The default value of `THETA_SUB` is 1.
/// - THETA_ADD : The parameter `THETA_ADD` is what will be added to an
///   index (modular addition) of the state register in a theta step.
///   By changing this parameter, you can change the theta step. If THETA_SUB,
///   THETA_ADD, and THETA_ROT are all `zero`, the step theta becomes virtually
///   equivalent not to exist. The default value of `THETA_ADD` is 1.
/// - THETA_ROT : The parameter `THETA_ROT` is how many bits the state register
///   will be rotated to the left (in little endianness) in a theta step.
///   By changing this parameter, you can change the theta step. If THETA_SUB,
///   THETA_ADD, and THETA_ROT are all `zero`, the step theta becomes virtually
///   equivalent not to exist. The default value of `THETA_ROT` is 1.
/// - RHO_MUL_X : The parameter `RHO_MUL_X` is what will be multiplied to an
///   index (modular multiplication) of the state register in a rho step.
///   By changing this parameter, you can change the rho step.
///   The default value of `RHO_MUL_X` is 2.
/// - RHO_MUL_Y : The parameter `RHO_MUL_Y` is what will be multiplied to an
///   index (modular multiplication) of the state register in a rho step.
///   By changing this parameter, you can change the rho step.
///   The default value of `RHO_MUL_Y` is 3.
/// - RHO_T : The parameter `RHO_T` is how many bits according to the formular
///   `(t + 1) * (t + 2) / 2` and how many times to rotate the state register
///   in a rho step. By changing this parameter, you can change the rho step.
///   The default value of `RHO_T` is 24,
/// - PI_MUL_X : The parameter `PI_MUL_X` is what will be multiplied to an
///   index (modular multiplication) of the state register in a pi step.
///   By changing this parameter, you can change the pi step.
///   The default value of `PI_MUL_X` is 1.
/// - PI_MUL_Y : The parameter `PI_MUL_Y` is what will be multiplied to an
///   index (modular multiplication) of the state register in a pi step.
///   By changing this parameter, you can change the pi step.
///   The default value of `PI_MUL_Y` is 3.
/// - CHI_ADD_1 : The parameter `CHI_ADD_1` is what will be added to an
///   index (modular addition) of the state register in a chi step.
///   By changing this parameter, you can change the chi step. If CHI_ADD_1
///   and CHI_ADD_2 are all `zero`, the step chi becomes virtually equivalent
///   not to exist. The default value of `THETA_ROT` is 1. 
/// - CHI_ADD_2 : The parameter `THETA_ADD` is what will be added to an
///   index (modular addition) of the state register in a chi step.
///   By changing this parameter, you can change the chi step. If CHI_ADD_1
///   and CHI_ADD_2 are all `zero`, the step chi becomes virtually equivalent
///   not to exist. The default value of `CHI_ADD_2` is 2.
/// 
/// Watch [this video](https://www.youtube.com/watch?v=JWskjzgiIa4)
/// to learn SHA-3 more in detail.
/// 
/// # Security of your own expanded version
/// Your own algrorithm based on Keccak may be stronger or weaker than official
/// SHA3-224, SHA3-256, SHA3-384, SHA3-512, SHAKE-128, SHAKE-256, cSHAKE-128,
/// cSHAKE-256, and other Keccak variants. Unless you seriously checked the
/// cryptographic security of your own algorithms, it is hard to assume that
/// your own alogrithms are stronger than the official SHA3-224, SHA3-256,
/// SHA3-384, SHA3-512, SHAKE-128, SHAKE-256, cSHAKE-128, cSHAKE-256, and other
/// Keccak variants.
/// 
/// # Reference
/// Read [more](https://en.wikipedia.org/wiki/SHA-3) about SHA-3 in detail.
/// 
/// # Quick Start
/// In order to use the module Keccak, you don't have to import (or use)
/// cryptocol::hash::keccak::* directly because the module
/// cryptocol::hash::keccak is re-exported. All you have to do is only import
/// SHA3_512, SHA3_384, SHA3_256, SHA3_224, SHAKE_256, SHAKE_128, cSHAKE_256,
/// cSHAKE_128, Keccak_512, Keccak_384, Keccak_256, and Keccak_224, and/or other
/// variants of Keccak_Generic in the module cryptocol::hash.
/// 
/// Example 1 shows how to import types SHA3_512, SHA3_384, SHA3_256,
/// SHA3_224, SHAKE_256, SHAKE_128, cSHAKE_256, cSHAKE_128, Keccak_512,
/// Keccak_384, Keccak_256, and Keccak_224, and/or other variants of
/// Keccak_Generic. Plus, what you have to know is these. All the types (or
/// structs) are the specific versions of Keccak_Generic.
/// Actually, SHA3_512 is a specific versions of Keccak_Generic, for example.
/// 
/// ## Example 1 for SHA-3 standard family
/// ```
/// use cryptocol::hash::SHA3_512;
/// use cryptocol::hash::SHA3_384;
/// use cryptocol::hash::SHA3_256;
/// use cryptocol::hash::SHA3_224;
/// use cryptocol::hash::SHAKE_256;
/// use cryptocol::hash::SHAKE_128;
/// use cryptocol::hash::cSHAKE_256;
/// use cryptocol::hash::cSHAKE_128;
/// use cryptocol::hash::KECCAK_512;
/// use cryptocol::hash::KECCAK_384;
/// use cryptocol::hash::KECCAK_256;
/// use cryptocol::hash::KECCAK_224;
/// ```
/// 
/// ## Example 2 for Keccak family
/// ```
/// use cryptocol::hash::KECCAK_512;
/// use cryptocol::hash::KECCAK_384;
/// use cryptocol::hash::KECCAK_256;
/// use cryptocol::hash::KECCAK_224;
/// use cryptocol::hash::BIG_KECCAK_1536;
/// use cryptocol::hash::BIG_KECCAK_1024;
/// use cryptocol::hash::BIG_KECCAK_768;
/// use cryptocol::hash::BIG_KECCAK_512;
/// use cryptocol::hash::BIG_KECCAK_384;
/// use cryptocol::hash::BIG_KECCAK_256;
/// use cryptocol::hash::BIG_KECCAK_224;
/// use cryptocol::hash::BIG_KECCAK_128;
/// use cryptocol::hash::SMALL_KECCAK_224;
/// use cryptocol::hash::SMALL_KECCAK_256;
/// use cryptocol::hash::SMALL_KECCAK_384;
/// use cryptocol::hash::SMALLER_KECCAK_128;
/// use cryptocol::hash::TINY_KECCAK_64;
/// ```
/// 
/// ## Example 3 for Keccak-expanded versions
/// ```
/// use cryptocol::hash::SHA3_768;
/// use cryptocol::hash::SHAKE_224;
/// use cryptocol::hash::SHAKE_384;
/// use cryptocol::hash::SHAKE_768;
/// use cryptocol::hash::cSHAKE_224;
/// use cryptocol::hash::cSHAKE_384;
/// use cryptocol::hash::cSHAKE_768;
/// use cryptocol::hash::BIG_SHA3_128;
/// use cryptocol::hash::BIG_SHA3_256;
/// use cryptocol::hash::BIG_SHA3_384;
/// use cryptocol::hash::BIG_SHA3_512;
/// use cryptocol::hash::BIG_SHA3_768;
/// use cryptocol::hash::BIG_SHA3_1024;
/// use cryptocol::hash::BIG_SHA3_1536;
/// use cryptocol::hash::BIG_SHAKE_128;
/// use cryptocol::hash::BIG_SHAKE_256;
/// use cryptocol::hash::BIG_SHAKE_384;
/// use cryptocol::hash::BIG_SHAKE_512;
/// use cryptocol::hash::BIG_SHAKE_768;
/// use cryptocol::hash::BIG_SHAKE_1024;
/// use cryptocol::hash::BIG_SHAKE_1536;
/// use cryptocol::hash::BIG_cSHAKE_128;
/// use cryptocol::hash::BIG_cSHAKE_256;
/// use cryptocol::hash::BIG_cSHAKE_384;
/// use cryptocol::hash::BIG_cSHAKE_512;
/// use cryptocol::hash::BIG_cSHAKE_768;
/// use cryptocol::hash::BIG_cSHAKE_1024;
/// use cryptocol::hash::BIG_cSHAKE_1536;
/// use cryptocol::hash::SMALL_SHA3_224;
/// use cryptocol::hash::SMALL_SHA3_256;
/// use cryptocol::hash::SMALL_SHA3_384;
/// use cryptocol::hash::SMALL_SHAKE_128;
/// use cryptocol::hash::SMALL_SHAKE_224;
/// use cryptocol::hash::SMALL_SHAKE_256;
/// use cryptocol::hash::SMALL_cSHAKE_128;
/// use cryptocol::hash::SMALL_cSHAKE_224;
/// use cryptocol::hash::SMALL_cSHAKE_256;
/// use cryptocol::hash::SMALLER_SHA3_128;
/// use cryptocol::hash::SMALLER_SHAKE_128;
/// use cryptocol::hash::SMALLER_cSHAKE_128;
/// use cryptocol::hash::TINY_SHA3_64;
/// use cryptocol::hash::TINY_SHAKE_64;
/// use cryptocol::hash::TINY_cSHAKE_64;
/// ```
/// 
/// Then, you create SHA3_512 object by the method SHA3_512::new(), for example.
/// Now, you are ready to use all provided methods to hash any data. If you want
/// to hash a string, for example, you can use the method absorb_str(). Then,
/// the SHA3_512 object that you created will contain its hash value. You can
/// use the macro println!(), for instance, to print on a commandline screen by
/// `println!("{}", hash)` where hash is the SHA3_512 object. Example 4 shows
/// how to use SHA3_512.
/// 
/// ## Example 4
/// ```
/// use cryptocol::hash::SHA3_512;
/// let mut hash = SHA3_512::new();
/// 
/// let mut txt = "";
/// hash.digest_str(txt);
/// let hash_value = hash.get_hash_value_in_string();
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash_value);
/// assert_eq!(hash_value, "A69F73CCA23A9AC5C8B567DC185A756E97C982164FE25859E0D1DCC1475C80A615B2123AF1F5F94C11E3E9402C3AC558F500199D95B6D3E301758586281DCD26");
/// 
/// let txt_stirng = String::from("A");
/// hash.digest_string(&txt_stirng);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
/// assert_eq!(hash.to_string(), "F5F0EAA9CA3FD0C4E0D72A3471E4B71EDAABE2D01C4B25E16715004ED91E663A1750707CC9F04430F19B995F4ABA21B0EC878FC5C4EB838A18DF5BF9FDC949DF");
/// 
/// let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
/// hash.digest_array(&txt_array);
/// println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "4D8225A3EC677F44F3489B04925989BB18A9873446C8C122AC76019527E7A2324BD07D3CE5404649050F9DA05EEE8A6F2B64FDB05EA98BB77770A668D167EE0D");
/// 
/// txt = "The length of this message is forty-eight bytes.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "D3409ADAF35CF0D99EA0742BF50F84C6000F4B8CE84C76920CDADA6A077F4D274834AFADC43480D063CFD42E71860319F8436B7EDDFB03D682222A1AE1EA7B0E");
/// 
/// txt = "The unit of the message length is not byte but bit.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "1D617BEE99571A8725B1D349F005B306B34637FDCF3D672D9311CA24083161697CDFFCC959C9FAFB7D75994653D37A8A097011C3F7700A0A4173364CAD6CB65A");
/// 
/// txt = "This algorithm SHA3-512 is being tested with this message the length of which is one hundred eleven bytes long.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.to_string(), "A78FF3E9862D7962D20A350F60D67F6DC56D4996DDEFEEF5F23962F89B55F6E26140741D1ADE9C99F1CC2828F4A2CBD8D1E3EE17B1A07E28BD764667BF09DA72");
/// 
/// txt = "This algorithm SHA3-512 is being tested for this message the length of which is one hundred sixty-four long so as to check whether or not this algorithm works well.";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
/// assert_eq!(hash.get_hash_value_in_string(), "3DD5B04FF485CA54787706E8CFCF6DBB57DD84452520575348960639EAE467D235858E00D038A0C88F56CF04FD39EC5B889B9D54B3F5F1C31D36A2050CF7C0CB");
/// 
/// txt = "This algorithm SHA3-512 is being tested with this message the length of which is two hundred ninety-one long so that whether or not this algorithm works well is checked.  The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
/// hash.digest_str(txt);
/// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
/// assert_eq!(hash.to_string(), "0A77A1191A768A1D0D7BF280F105D6399C2B8DE2280EF4BE307F99DDFC6A8895C8262A536405680DD01CC3699D5186043E406AE3FE01287A977EA4121F85BF53");
/// ```
/// 
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for Big Endian CPUs for serious purpose. Only use this crate
/// for Big-endian CPUs with your own full responsibility.
/// 
/// # A Simple but Useful Application using cryptocol
/// The following is the source code of the commandline SHA3 hash value 
/// extractor using the struct SHA3 of this module. You can get the hash
/// value from a text or a file. The following source code assumes its
/// executable file name will be "sha3_app". You can find all the examples
/// including the following source code in the folder "examples" of this crate.
/// ```
/// use std::{ io, env, fs };
/// use std::io::BufRead;
/// use std::convert::From;
/// use cryptocol::hash::SHA3_256;
/// 
/// type HASH = SHA3_256;
/// 
/// fn main()
/// {
///     let args: Vec<String> = env::args().collect();
///     if args.len() < 3
///     {
///         help();
///         return;
///     }
/// 
///     let arg = &args[1][..];
///     match arg
///     {
///         "--text" | "-t" =>  { get_hash_value_from_text(&args[2][..]); },
///         "--file" | "-f" =>  { get_hash_value_from_file(&args[2][..]); },
///         "--check" | "-c" => { check_files(&args[2][..]) },
///         _ =>  { help(); },
///     }
/// }
/// 
/// fn get_hash_value_from_text(txt: &str)
/// {
///     let mut hash = HASH::new();
///     hash.digest_str(txt);
///     println!("Hash value:\t{}", hash.get_hash_value_in_string());
/// }
/// 
/// fn get_hash_value_from_file(file: &str)
/// {
///     if let Ok(contents) = fs::read(file)
///     {
///         let mut hash = HASH::new();
///         hash.digest_vec(&contents);
///         println!("Hash value:\t{}", hash.get_hash_value_in_string());
///     }
///     else
///     {
///         println!("File Error!");
///     }
/// }
/// 
/// fn check_files(file_list: &str)
/// {
///     let mut reader;
///     match fs::File::open(file_list)
///     {
///         Ok(file) => {
///                 reader = io::BufReader::new(file);
///                 let mut line = String::new();
///                 while let Ok(n) = reader.read_line(&mut line)
///                 {
///                     if n == 0
///                         { break; }
///                     let txt = line.trim();
///                     if txt.chars().nth(0).unwrap() == '#'
///                     { 
///                         line.clear();
///                         continue;
///                     }
///                     let elem: Vec<&str> = txt.split_whitespace().collect();
///                     let item = elem[0];
///                     let h = String::from(elem[1]).to_uppercase();
///                     if let Ok(contents) = fs::read(item)
///                     {
///                         let mut hash = HASH::new();
///                         hash.digest_vec(&contents);
///                         if hash.to_string() == h
///                             { println!("{} ---> OK", item); }
///                         else
///                             { println!("{} ---> Corrupted", item); }
///                     }
///                     line.clear();
///                 }
///             },
///         _ => {
///                 println!("File open error");
///                 return;
///             }
///     }
/// }
/// 
/// fn help()
/// {
///     println!("This is an SHA3_256 hash value extractor from a text or a file, using cryptocol.");
///     println!("Usage: sha3_app <option> <source>");
///     println!("options       description");
///     println!("--text, -t    : <source> is a text to get a hash code.");
///     println!("                The text should be enclosed by ' or \".");
///     println!("--file, -f    : <source> is the name of the file to get a hash code.");
///     println!("--check, -c   : <source> is the name of the file that contains pairs");
///     println!("                of file and its hash code.");
///     println!("--help, -h    : print this help message on screen\n");
///     println!("Examples:");
///     println!("\tsha3_app -t 'How are you doing?'");
///     println!("\tsha3_app -f linuxmint-21.3-cinnamon-64bit.iso");
///     println!("\tsha3_app -c CHECKSUM");
/// }
/// ```
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct Keccak_Generic<const RATE: usize = 72, const PADDING: usize = 0,
        const ROUNDS: usize = 24, T = u64, const LFSR: u8 = 0b_0111_0001,
        const THETA_SUB: usize = 1, const THETA_ADD: usize = 1, const THETA_ROT: u32 = 1,
        const RHO_MUL_X: usize = 2, const RHO_MUL_Y: usize = 3, const RHO_T: u32 = 24,
        const PI_MUL_X: usize = 1, const PI_MUL_Y: usize = 3,
        const CHI_ADD_1: usize = 1, const CHI_ADD_2: usize = 2>
where T: SmallUInt
{
    state: [[T; 5]; 5],
}

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
    const RC: [T; ROUNDS] = make_RC!(T, ROUNDS, LFSR);
    const SEPARATOR: u8 = match PADDING
                            {
                                KECCAK_CONST::SHA3 => { 0b_0000_0110 },  // SHA-3
                                KECCAK_CONST::SHAKE => { 0b_0001_1111 },  // SHAKE
                                KECCAK_CONST::CSHAKE => { 0b_0000_0100 },  // cSHAKE
                                _ => { 0b_0000_0001 },  // KECCAK
                            };
    const TAIL: u8 = 0b_1000_0000;
    /// Default output length of the hash value in byte 
    pub(crate) const DEFUALT_OUTPUT_LENGTH_IN_BYTES: usize = if (PADDING == KECCAK_CONST::SHAKE) || (PADDING == KECCAK_CONST::CSHAKE)
                                    { T::BITS as usize / 8 * 25 - RATE }
                                else
                                    { (T::BITS as usize / 8 * 25 - RATE) / 2 };

    // pub fn new() -> Self
    /// Creates the new object of `Self`.
    /// 
    /// # Output
    /// A new object of `Self`.
    /// 
    /// # Example 1 for SHA3_512
    /// ```
    /// use cryptocol::hash::SHA3_512;
    /// let hash = SHA3_512::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.new)
    #[inline]
    pub fn new() -> Self
    {
        Self { state: [[T::MIN; 5]; 5] }
    }

    #[inline]
    pub(crate) fn box_new() -> Box<Self>
    {
        Box::new(Self::new())
    }

    // pub fn digest(&mut self, message: *const u8, length_in_bytes: u64)
    /// Computes hash value.
    /// 
    /// # Arguments
    /// - `message` is pointer to `const u8`.
    /// - `length_in_bytes` is the size of message in the unit of bytes, and
    ///   its data type is `u64`.
    /// 
    /// # Features
    /// - This function has the generalized interface (pointer, `*const u8`)
    ///   so as to enable other functions to wrap this function with any
    ///   convenient interface for uses. So, this function is usually not called
    ///   directly in Rust. This function is provided to be called from other
    ///   programming languages such as C/C++.
    /// - This method is the wrapper of the method `absorb()` for consistancy
    ///   with other hash functions.
    /// 
    /// # Warning
    /// You can use this method even for the type `cSHAKE_*` but it is not
    /// in accordance with the standard. It is desirable that You use
    /// the method `digest_customized()` instead. If you use this method for
    /// the type `cSHAKE_*`, others may think that you do not fully understand
    /// `SHAKE-*` and `cSHAKE-*`. However, if you intentionally write your code
    /// in non-standard way, you can use this method even for the type
    /// `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [digest_str()](struct@Keccak_Generic#method.digest_str)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [digest_string()](struct@Keccak_Generic#method.digest_string)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [digest_array()](struct@Keccak_Generic#method.digest_array)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec()](struct@Keccak_Generic#method.digest_array)
    ///   rather than this method.
    ///
    /// # Example 1 for SHA3_256
    /// ```
    /// use cryptocol::hash::SHA3_256;
    /// let mut hash = SHA3_256::new();
    /// let txt = "This is an example of the method digest().";
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "065B40EFFE93C55937ACA0C23D7A35387E0FDCA478C49D13255A59F685A2A53C");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.digest)
    #[inline]
    pub fn digest(&mut self, message: *const u8, length_in_bytes: u64)
    {
        self.absorb(message, length_in_bytes);
    }

    // pub fn digest_str(&mut self, message: &str)
    /// Computes hash value.
    /// 
    /// # Features
    /// This method is the wrapper of the method `absorb_str()`
    /// for consistancy with other hash functions.
    /// 
    /// # Arguments
    /// - `message` is of type `&str`.
    /// 
    /// # Warning
    /// You can use this method even for the type `cSHAKE_*` but it is not
    /// in accordance with the standard. It is desirable that You use
    /// the method `digest_str_customized()` instead. If you use this method for
    /// the type `cSHAKE_*`, others may think that you do not fully understand
    /// `SHAKE-*` and `cSHAKE-*`. However, if you intentionally write your code
    /// in non-standard way, you can use this method even for the type
    /// `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [digest_string()](struct@Keccak_Generic#method.digest_string)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [digest_array()](struct@Keccak_Generic#method.digest_array)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec()](struct@Keccak_Generic#method.digest_array)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [digest()](struct@Keccak_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA3_224
    /// ```
    /// use cryptocol::hash::SHA3_224;
    /// let mut hash = SHA3_224::new();
    /// let txt = "This is an example of the method digest_str().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "BA8399261A38A097A69A072A9DE74FEAB248E5E2C93E622AC7E3381A");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.digest_str)
    #[inline]
    pub fn digest_str(&mut self, message: &str)
    {
        self.absorb_str(message);
    }

    // pub fn digest_string(&mut self, message: &String)
    /// Computes hash value.
    /// 
    /// # Features
    /// This method is the wrapper of the method `absorb_string()`
    /// for consistancy with other hash functions.
    /// 
    /// # Arguments
    /// - `message` is of the type `&String`.
    /// 
    /// # Warning
    /// You can use this method even for the type `cSHAKE_*` but it is not
    /// in accordance with the standard. It is desirable that You use
    /// the method `digest_string_customized()` instead. If you use this method
    /// for the type `cSHAKE_*`, others may think that you do not fully
    /// understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally write
    /// your code in non-standard way, you can use this method even for the type
    /// `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [digest_str()](struct@Keccak_Generic#method.digest_str)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [digest_array()](struct@Keccak_Generic#method.digest_array)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec()](struct@Keccak_Generic#method.digest_array)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [digest()](struct@Keccak_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA3_384
    /// ```
    /// use cryptocol::hash::SHA3_384;
    /// let mut hash = SHA3_384::new();
    /// let txt = String::from("This is an example of the method digest_string().");
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "34721672060C3F72C8FFD207E6D7ABA63CAA7A5BFEE0A695C7A11C423E8B14A27A61A967E3BACD041C4449F127533247");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.digest_string)
    #[inline]
    pub fn digest_string(&mut self, message: &String)
    {
        self.absorb_string(message);
    }

    // pub fn digest_array<U, const N: usize>(&mut self, message: &[U; N])
    /// Computes hash value.
    /// 
    /// # Features
    /// This method is the wrapper of the method `absorb_array()`
    /// for consistancy with other hash functions.
    /// 
    /// # Arguments
    /// - `message` is of the type `&[U; N]`.
    /// 
    /// # Warning
    /// You can use this method even for the type `cSHAKE_*` but it is not
    /// in accordance with the standard. It is desirable that You use
    /// the method `digest_array_customized()` instead. If you use this method
    /// for the type `cSHAKE_*`, others may think that you do not fully
    /// understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally write
    /// your code in non-standard way, you can use this method even for the type
    /// `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [digest_str()](struct@Keccak_Generic#method.digest_str)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [digest_string()](struct@Keccak_Generic#method.digest_string)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec()](struct@Keccak_Generic#method.digest_vec)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [digest()](struct@Keccak_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA3_256
    /// ```
    /// use cryptocol::hash::SHA3_256;
    /// let mut hash = SHA3_256::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_array(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "2FA65DD00903E850AD14E00D13ACBE9C2CA2E7B140419B8C7EA2742900586B14");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.digest_array)
    #[inline]
    pub fn digest_array<U, const N: usize>(&mut self, message: &[U; N])
    where U: SmallUInt + Copy + Clone
    {
        self.absorb_array(message);
    }

    // pub fn digest_vec<U>(&mut self, message: &Vec<U>)
    /// Computes hash value.
    /// 
    /// # Features
    /// This method is the wrapper of the method `absorb_array()`
    /// for consistancy with other hash functions.
    /// 
    /// # Arguments
    /// - `message` is of the type `&Vec<U>`.
    /// 
    /// # Warning
    /// You can use this method even for the type `cSHAKE_*` but it is not
    /// in accordance with the standard. It is desirable that You use
    /// the method `digest_vec_customized()` instead. If you use this method
    /// for the type `cSHAKE_*`, others may think that you do not fully
    /// understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally write
    /// your code in non-standard way, you can use this method even for the type
    /// `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [digest_str()](struct@Keccak_Generic#method.digest_str)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [digest_string()](struct@Keccak_Generic#method.digest_string)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [digest_array()](struct@Keccak_Generic#method.digest_array)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [digest()](struct@Keccak_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA3_256
    /// ```
    /// use cryptocol::hash::SHA3_256;
    /// let mut hash = SHA3_256::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "2FA65DD00903E850AD14E00D13ACBE9C2CA2E7B140419B8C7EA2742900586B14");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.digest_vec)
    #[inline]
    pub fn digest_vec<U>(&mut self, message: &Vec<U>)
    where U: SmallUInt + Copy + Clone
    {
        self.absorb_vec(message);
    }

    // pub fn digest_customized(&mut self, function_name: *const u8, function_name_length_in_bytes: u64, user_defined: *const u8, user_defined_length_in_bytes: u64, message: *const u8, length_in_bytes: u64)
    /// Digests the message with filename string and user-defined string.
    /// 
    /// # Arguments
    /// - `function_name` is pointer to const u8, which contains function name,
    ///   but it is reserved for NIST use. You are supposed to give null string
    ///   for `function name`. You may want to use `user_defined` instead.
    ///   However, for all the types other than cSHAKE, you can freely use
    ///   this method of the expanded versions of struct `Keccak_Generic` for
    ///   your own purpose.
    /// - `function_name_length_in_bytes` is the size of `function_name` in the
    ///   unit of bytes, and its data type is `u64`, but it is reserved
    ///   for NIST use. You are supposed to give `0` for
    ///   `function_name_length_in_bytes`. You may want to use
    ///   `user_defined_length_in_bytes` instead. However, for all the types other
    ///   than cSHAKE, you can you `function_name_length_in_bytes` because NIST
    ///   reserved `function_name` and `function_name_length_in_bytes` only for
    ///   cSHAKE. You can freely use this method for expanded versions of struct
    ///   `Keccak_Generic`.
    /// - `user_defined` is pointer to const u8, which contains a string for
    ///   your special use.
    /// - `user_defined_length_in_bytes` is the size of `user_defined` in the
    ///   unit of bytes, and its data type is `u64`.
    /// - `message` is pointer to const u8, which contains the actual date to
    ///   hash.
    /// - `length_in_bytes` is the size of message in the unit of bytes, and
    ///   its data type is `u64`.
    /// 
    /// # Features
    /// - This function has the generalized interface (pointer, `*const u8`)
    ///   so as to enable other functions to wrap this function with any
    ///   convenient interface for uses. So, this function is usually not called
    ///   directly in Rust. This function is provided to be called from other
    ///   programming languages such as C/C++.
    /// - This method is the wrapper of the method `absorb_customized()`.
    /// 
    /// # Warning
    /// You can use this method even for the types other than `cSHAKE_*` but
    /// it is not in accordance with the standard. It is desirable that You
    /// use the method `digest()` instead. If you use this method for the
    /// types other than `cSHAKE_*`, others may think that you do not fully
    /// understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally
    /// write your code in non-standard way, you can use this method even for
    /// the types other than `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [digest_str_customized()](struct@Keccak_Generic#method.digest_str_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [digest_string_customized()](struct@Keccak_Generic#method.digest_string_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [digest_array_customized()](struct@Keccak_Generic#method.digest_array_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec_customized()](struct@Keccak_Generic#method.digest_vec_customized)
    ///   rather than this method.
    ///
    /// # Example 1 for cSHAKE_128
    /// ```
    /// use cryptocol::hash::cSHAKE_128;
    /// let mut hash = cSHAKE_128::new();
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_customized().";
    /// hash.digest_customized("".as_ptr(), 0, user_defined.as_ptr(), user_defined.len() as u64,  txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "4C3793B9B1CBA98DA30F71F0ABEEB6DB7D5B35318F17E5445BAEC565FADCB003");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.digest_customized)
    #[inline]
    pub fn digest_customized(&mut self, function_name: *const u8, function_name_length_in_bytes: u64, user_defined: *const u8, user_defined_length_in_bytes: u64, message: *const u8, length_in_bytes: u64)
    {
        self.absorb_customized(function_name, function_name_length_in_bytes, user_defined, user_defined_length_in_bytes, message, length_in_bytes);
    }

    // pub fn digest_str_customized(&mut self, function_name: &str, user_defined: &str, message: &str)
    /// Digests the message with filename string and user-defined string.
    /// 
    /// # Arguments
    /// - `function_name` is of type `&str`, which contains function name,
    ///   but it is reserved for NIST use. You are supposed to give null string
    ///   for `function name`. You may want to use `user_defined` instead.
    ///   However, for all the types other than cSHAKE, you can freely use
    ///   this method of the expanded versions of struct `Keccak_Generic` for
    ///   your own purpose.
    /// - `user_defined` is of type `&str`, which contains a string for
    ///   your special use.
    /// - `message` is of type `&str`.
    /// 
    /// # Features
    /// This method is the wrapper of the method `absorb_str_customized()`.
    /// 
    /// # Warning
    /// You can use this method even for the types other than `cSHAKE_*` but it
    /// is not in accordance with the standard. It is desirable that You use
    /// the method `digest_str()` instead. If you use this method for the
    /// types other than `cSHAKE_*`, others may think that you do not
    /// fully understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally
    /// write your code in non-standard way, you can use this method even for
    /// the types other than `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [digest_string_customized()](struct@Keccak_Generic#method.digest_string_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [digest_array_customized()](struct@Keccak_Generic#method.digest_array_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec_customized()](struct@Keccak_Generic#method.digest_vec_customized)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [digest()](struct@Keccak_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA3_224
    /// ```
    /// use cryptocol::hash::cSHAKE_256;
    /// let mut hash = cSHAKE_256::new();
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_str_customized().";
    /// hash.digest_str_customized("", user_defined, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "3BB260278648858A59A25EE45AEA4E17A8DD7FAF51E32AEF4D3EA11739E38D4C9D22B7AE394D79E2A88BD2EFA4385E490836D0C6ED9D9087A3229F17F5E50EC9");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.digest_str_customized)
    #[inline]
    pub fn digest_str_customized(&mut self, function_name: &str, user_defined: &str, message: &str)
    {
        self.absorb_str_customized(function_name, user_defined, message);
    }

    // pub fn digest_string_customized(&mut self, function_name: &String, user_defined: &String, message: &String)
    /// Digests the message with filename string and user-defined string.
    /// 
    /// # Arguments
    /// - `function_name` is of type `&String`, which contains function name,
    ///   but it is reserved for NIST use. You are supposed to give null string
    ///   for `function name`. You may want to use `user_defined` instead.
    ///   However, for all the types other than cSHAKE, you can freely use
    ///   this method of the expanded versions of struct `Keccak_Generic` for
    ///   your own purpose.
    /// - `user_defined` is of type `&String`, which contains a string for
    ///   your special use.
    /// - `message` is of type `&String`.
    /// 
    /// # Features
    /// This method is the wrapper of the method `absorb_string_customized()`.
    /// 
    /// # Warning
    /// You can use this method even for the types other than `cSHAKE_*` but it
    /// is not in accordance with the standard. It is desirable that You use
    /// the method `digest_string()` instead. If you use this method for the
    /// types other than `cSHAKE_*`, others may think that you do not
    /// fully understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally
    /// write your code in non-standard way, you can use this method even for
    /// the types other than `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [digest_str_customized()](struct@Keccak_Generic#method.digest_str_customized)
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [digest_array_customized()](struct@Keccak_Generic#method.digest_array_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec_customized()](struct@Keccak_Generic#method.digest_vec_customized)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [digest()](struct@Keccak_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for cSHAKE_128
    /// ```
    /// use cryptocol::hash::cSHAKE_128;
    /// let mut hash = cSHAKE_128::new();
    /// let user_defined = "on my own purpose".to_string();
    /// let txt = String::from("This is an example of the method digest_string_customized().");
    /// hash.digest_string_customized(&"".to_string(), &user_defined, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "411E09A6E5CA61E99546226582C89FE0D6C3A57992173476C95F8BA1089EDF6D");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.digest_string_customized)
    #[inline]
    pub fn digest_string_customized(&mut self, function_name: &String, user_defined: &String, message: &String)
    {
        self.absorb_string_customized(function_name, user_defined, message);
    }

    // pub fn digest_array_customized<U, V, W, const L: usize, const M: usize, const N: usize>(&mut self, function_name: &[U; L], user_defined: &[V; M], message: &[W; N])
    /// Digests the message with filename string and user-defined string.
    /// 
    /// # Features
    /// This method is the wrapper of the method `absorb_array_customized()`
    /// .
    /// # Arguments
    /// - `function_name` is of type `&[U; L]`, which contains function name,
    ///   but it is reserved for NIST use. You are supposed to give null string
    ///   for `function name`. You may want to use `user_defined` instead.
    ///   However, for all the types other than cSHAKE, you can freely use
    ///   this method for expanded versions of struct `Keccak_Generic`.
    /// - `user_defined` is of type `&[V; M]`, which contains a string for
    ///   your special use.
    /// - `message` is of the type `&[W; N]`.
    /// 
    /// # Warning
    /// You can use this method even for the types other than `cSHAKE_*` but it
    /// is not in accordance with the standard. It is desirable that You use
    /// the method `digest_array()` instead. If you use this method for the
    /// types other than `cSHAKE_*`, others may think that you do not
    /// fully understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally
    /// write your code in non-standard way, you can use this method even for
    /// the types other than `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [digest_str_customized()](struct@Keccak_Generic#method.digest_str_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [digest_string_customized()](struct@Keccak_Generic#method.digest_string_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec_customized()](struct@Keccak_Generic#method.digest_vec_customized)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [digest_customized()](struct@Keccak_Generic#method.digest_customized)
    ///   rather than this method.
    ///
    /// # Example 1 for cSHAKE_256
    /// ```
    /// use cryptocol::hash::cSHAKE_256;
    /// let mut hash = cSHAKE_256::new();
    /// let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_array_customized(&[0_u8; 0], &user_defined, &data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "FE5EBE3F5007EDCE865F7DC425531F0C493E18168835722CCC0F11CE66F1423061C202BE0B1D0528DE0D763A3097A090B62115392769305D1FF32588A78CCEE9");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.digest_array_customized)
    #[inline]
    pub fn digest_array_customized<U, V, W, const L: usize, const M: usize, const N: usize>(&mut self, function_name: &[U; L], user_defined: &[V; M], message: &[W; N])
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone, W: SmallUInt + Copy + Clone
    {
        self.absorb_array_customized(function_name, user_defined, message);
    }

    // pub fn digest_vec_customized<U, V, W>(&mut self, function_name: &Vec<U>, user_defined: &Vec<V>, message: &Vec<W>)
    /// Digests the message with filename string and user-defined string.
    /// 
    /// # Features
    /// This method is the wrapper of the method `absorb_vec_customized()`
    /// 
    /// # Arguments
    /// - `function_name` is of type `&[U; L]`, which contains function name,
    ///   but it is reserved for NIST use. You are supposed to give null string
    ///   for `function name`. You may want to use `user_defined` instead.
    ///   However, for all the types other than cSHAKE, you can freely use
    ///   this method for expanded versions of struct `Keccak_Generic`.
    /// - `user_defined` is of type `&[V; M]`, which contains a string for
    ///   your special use.
    /// - `message` is of the type `&[W; N]`.
    /// 
    /// # Warning
    /// You can use this method even for the types other than `cSHAKE_*` but it
    /// is not in accordance with the standard. It is desirable that You use
    /// the method `digest_vec()` instead. If you use this method for the
    /// types other than `cSHAKE_*`, others may think that you do not
    /// fully understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally
    /// write your code in non-standard way, you can use this method even for
    /// the types other than `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [digest_str_customized()](struct@Keccak_Generic#method.digest_str_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [digest_string_customized()](struct@Keccak_Generic#method.digest_string_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_array_customized()](struct@Keccak_Generic#method.digest_array_customized)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [digest_customized()](struct@Keccak_Generic#method.digest_customized)
    ///   rather than this method.
    ///
    /// # Example 1 for cSHAKE_256
    /// ```
    /// use cryptocol::hash::cSHAKE_256;
    /// let mut hash = cSHAKE_256::new();
    /// let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec_customized(&vec![0_u8; 0], &user_defined, &data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "FE5EBE3F5007EDCE865F7DC425531F0C493E18168835722CCC0F11CE66F1423061C202BE0B1D0528DE0D763A3097A090B62115392769305D1FF32588A78CCEE9");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.digest_vec_customized)
    #[inline]
    pub fn digest_vec_customized<U, V, W>(&mut self, function_name: &Vec<U>, user_defined: &Vec<V>, message: &Vec<W>)
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone, W: SmallUInt + Copy + Clone
    {
        self.absorb_vec_customized(function_name, user_defined, message);
    }

    // pub fn get_hash_value_in_array<const N: usize>(&mut self) -> [u8; N]
    /// Returns a hash value in the form of array object.
    /// 
    /// # Output
    /// A hash value in the form of array object [u8; N] where `N` is a generic
    /// parameter.
    /// 
    /// # Features
    /// This method depends on the generic parameter `N`.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    ///   you are highly recommended to use the method
    ///   [get_hash_value_string()](struct@Keccak_Generic#method.get_hash_value_string)
    ///   rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    ///   you are highly recommended to use the method
    ///   [get_hash_value_in_vec()](struct@Keccak_Generic#method.get_hash_value_in_vec)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [get_hash_value()](struct@Keccak_Generic#method.get_hash_value)
    ///   rather than this method.
    /// 
    /// # Example 1 for SHA3_512
    /// ```
    /// use std::io::Write;
    /// use cryptocol::hash::SHA3_512;
    /// let mut hash = SHA3_512::new();
    /// let txt = "This is an example of the method get_hash_value_in_array().";
    /// hash.digest_str(txt);
    /// let hash_value = hash.get_hash_value_in_array::<64>();
    /// let mut hs = String::new();
    /// for h in hash_value
    ///     { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    /// assert_eq!(hs, "4387084DCE64950435EC8196096E6F64FBAEC92755840886F67F6FC60D18A519C02A20614DD4D6218AB6837D3CE46288A2BF1AA17ECDD63117F908161989A90D");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.get_hash_value_in_array)
    pub fn get_hash_value_in_array<const N: usize>(&mut self) -> [u8; N]
    {
        let mut out = [0_u8; N];
        self.push_hash_value_in_array(&mut out);
        out
    }

    // pub fn get_hash_value_in_vec(&mut self) -> Vec<u8>
    /// Returns a hash value in the form of `Vec<u8>` object.
    /// 
    /// # Output
    /// A hash value in the form of `Vec<u8>` object.
    /// 
    /// # Features
    /// The length of output hash value is automatically determined to be:
    /// - `T::BITS as usize / 8 * 25 - RATE`
    ///   if `Self` is SHAKE family or cSHAKE family
    /// - `(T::BITS as usize / 8 * 25 - RATE) / 2`
    ///   if `Self` is SHA3 family or Keccak family.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    ///   you are highly recommended to use the method
    ///   [get_hash_value_string()](struct@Keccak_Generic#method.get_hash_value_string)
    ///   rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    ///   you are highly recommended to use the method
    ///   [get_hash_value_in_array()](struct@Keccak_Generic#method.get_hash_value_in_array)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [get_hash_value()](struct@Keccak_Generic#method.get_hash_value)
    ///   rather than this method.
    /// 
    /// # Example 1 for SHA3_512
    /// ```
    /// use std::io::Write;
    /// use cryptocol::hash::SHA3_224;
    /// let mut hash = SHA3_224::new();
    /// let txt = "This is an example of the method get_hash_value_in_vec().";
    /// hash.digest_str(txt);
    /// let hash_value = hash.get_hash_value_in_vec();
    /// let mut hs = String::new();
    /// for h in hash_value
    ///     { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    /// assert_eq!(hs, "49FA84569FC01A4FD42DC7D6A892A61539AA14761FDC679A14A8A365");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.get_hash_value_in_vec)
    pub fn get_hash_value_in_vec(&mut self) -> Vec<u8>
    {
        let mut out = Vec::<u8>::new();
        out.resize(Self::DEFUALT_OUTPUT_LENGTH_IN_BYTES, 0);
        self.get_hash_value(out.as_mut_ptr(), Self::DEFUALT_OUTPUT_LENGTH_IN_BYTES);
        out
    }

    // pub fn get_hash_code_in_vec<const N: usize>(&mut self) -> Vec<u8>
    /// Returns a hash value in the form of `Vec<u8>` object with the length
    /// indicated by generic parameter.
    /// 
    /// # Output
    /// A hash value in the form of `Vec<u8>` object.
    /// 
    /// # Features
    /// The length of output hash value should be manually determined
    /// by generic parameter.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    ///   you are highly recommended to use the method
    ///   [get_hash_value_string()](struct@Keccak_Generic#method.get_hash_value_string)
    ///   rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    ///   you are highly recommended to use the method
    ///   [get_hash_value_in_array()](struct@Keccak_Generic#method.get_hash_value_in_array)
    ///   rather than this method.
    /// - If you want to get the hash value in the form of Vec object
    ///   of predetermined length, you are highly recommended to use the method
    ///   [get_hash_value_in_vec()](struct@Keccak_Generic#method.get_hash_value_in_vec)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [get_hash_value()](struct@Keccak_Generic#method.get_hash_value)
    ///   rather than this method.
    /// 
    /// # Example 1 for SHA3_384
    /// ```
    /// use std::io::Write;
    /// use cryptocol::hash::SHA3_384;
    /// 
    /// let mut hash = SHA3_384::new();
    /// let txt = "This is an example of the method sha3_get_hash_code_in_vec().";
    /// hash.digest_str(txt);
    /// let hash_value = hash.get_hash_code_in_vec::<48>();
    /// let mut hs = String::new();
    /// for h in hash_value
    ///     { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    /// assert_eq!(hs, "4DF9726A50546589EDED01B0D6CAF4DB022B382C6B0B6229EAD2F75B743940A0993891C6E38DB84931AAC1EB2CFAC9F8");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.get_hash_code_in_vec)
    pub fn get_hash_code_in_vec<const N: usize>(&mut self) -> Vec<u8>
    {
        let mut out = Vec::<u8>::new();
        let arr: [u8; N] = self.get_hash_value_in_array();
        for it in arr
            { out.push(it); }
        out
    }

    // pub fn get_hash_value_in_string(&mut self) -> String
    /// Returns the hash value in the form of String object.
    /// 
    /// # Output
    /// A hash value in the form of String object.
    /// 
    /// # Features
    /// The length of output hash value is automatically determined to be:
    /// - `T::BITS as usize / 8 * 25 - RATE`
    ///   if `Self` is SHAKE family or cSHAKE family
    /// - `(T::BITS as usize / 8 * 25 - RATE) / 2`
    ///   if `Self` is SHA3 family or Keccak family.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of Vec object,
    ///   you are highly recommended to use the method
    ///   [get_hash_value_in_array()](struct@Keccak_Generic#method.get_hash_value_in_array)
    ///   rather than this method.
    /// - If you want to get the hash value in the form of Vec object
    ///   of predetermined length, you are highly recommended to use the method
    ///   [get_hash_value_in_vec()](struct@Keccak_Generic#method.get_hash_value_in_vec)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [get_hash_value()](struct@Keccak_Generic#method.get_hash_value)
    ///   rather than this method.
    /// 
    /// # Example 1 for SHA3_256
    /// ```
    /// use std::io::Write;
    /// use cryptocol::hash::SHA3_256;
    /// 
    /// let mut hash = SHA3_256::new();
    /// let txt = "This is an example of the method get_hash_value_in_string().";
    /// hash.digest_str(txt);
    /// let hs = hash.get_hash_value_in_string();
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    /// assert_eq!(hs, "2075C0B4930865BA252F5BA2A7DF5AC4AF587B9E054B8BCC249CED216AFAA459");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.get_hash_value_in_string)
    #[inline]
    pub fn get_hash_value_in_string(&mut self) -> String
    {
        self.get_hash_code_in_string(Self::DEFUALT_OUTPUT_LENGTH_IN_BYTES)
    }

    // pub fn get_hash_code_in_string(&mut self, length_in_bytes: usize) -> String
    /// Returns the hash value in the form of String object with the length
    /// indicated by generic parameter.
    /// 
    /// # Output
    /// A hash value in the form of String object.
    /// 
    /// # Features
    /// The length of output hash value should be manually determined
    /// by generic parameter.
    /// 
    /// # Arguments
    /// - `length_in_bytes` is the size of the hash value in the unit of bytes,
    ///   and its data type is `usize`.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    ///   you are highly recommended to use the method
    ///   [get_hash_value_string()](struct@Keccak_Generic#method.get_hash_value_string)
    ///   rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    ///   you are highly recommended to use the method
    ///   [get_hash_value_in_array()](struct@Keccak_Generic#method.get_hash_value_in_array)
    ///   rather than this method.
    /// - If you want to get the hash value in the form of Vec object
    ///   of predetermined length, you are highly recommended to use the method
    ///   [get_hash_value_in_vec()](struct@Keccak_Generic#method.get_hash_value_in_vec)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [get_hash_value()](struct@Keccak_Generic#method.get_hash_value)
    ///   rather than this method.
    /// 
    /// # Example 1 for SHA3_512
    /// ```
    /// use std::io::Write;
    /// use cryptocol::hash::SHA3_512;
    /// 
    /// let mut hash = SHA3_512::new();
    /// let txt = "This is an example of the method get_hash_code_in_string().";
    /// hash.digest_str(txt);
    /// let hs = hash.get_hash_code_in_string(64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    /// assert_eq!(hs, "F9DA1EAB4A2F7204BDA5E06DACDC23D98491CB5E313F7F74594F9852F8122DAFB03A715BE6836B17F6ACD9EC6A1BA12AD8F0C8C221A9BD20D0834AB78C6FB6A7");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.get_hash_code_in_string)
    pub fn get_hash_code_in_string(&mut self, length_in_bytes: usize) -> String
    {
        let chunk_num = length_in_bytes / RATE;
        let rest_num = length_in_bytes % RATE;
        let mut txt = String::new();
        for _ in 0..chunk_num
        {
            let hash_code = self.squeeze();
            txt.push_str(Self::read_hash_value_in_hexadecimal(&hash_code).as_str());
        }
        if rest_num != 0
        {
            let hash_code = self.squeeze();
            let mut chs = Self::read_hash_value_in_hexadecimal(&hash_code);
            chs.truncate(rest_num << 1);
            txt.push_str(chs.as_str());
        }
        txt
    }

    // pub fn push_hash_value_in_array<const N: usize>(&mut self, hash_value: &mut [u8; N])
    /// Stores a hash value into the given array object with the length
    /// indicated by generic parameter.
    /// 
    /// # Features
    /// The length of output hash value should be manually determined
    /// by generic parameter.
    /// 
    /// # Arguments
    /// - `hash_value` is the array of `u8` with `N` elements.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    ///   you are highly recommended to use the method
    ///   [get_hash_value_string()](struct@Keccak_Generic#method.get_hash_value_string)
    ///   rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    ///   you are highly recommended to use the method
    ///   [get_hash_value_in_array()](struct@Keccak_Generic#method.get_hash_value_in_array)
    ///   rather than this method.
    /// - If you want to get the hash value in the form of Vec object
    ///   of predetermined length, you are highly recommended to use the method
    ///   [get_hash_value_in_vec()](struct@Keccak_Generic#method.get_hash_value_in_vec)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [get_hash_value()](struct@Keccak_Generic#method.get_hash_value)
    ///   rather than this method.
    /// 
    /// # Example 1 for SHA3_224
    /// ```
    /// use std::io::Write;
    /// use cryptocol::hash::SHA3_224;
    /// 
    /// let mut hash = SHA3_224::new();
    /// let txt = "This is an example of the method push_hash_value_in_array().";
    /// hash.digest_str(txt);
    /// let mut hash_value = [0_u8; 28];
    /// hash.push_hash_value_in_array(&mut hash_value);
    /// let mut hs = String::new();
    /// for h in hash_value.iter()
    ///     { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    /// assert_eq!(hs, "84C1FA767D0DB1DF9F886333681641A55253AB934A16B51376A5403C");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.push_hash_value_in_array)
    #[inline]
    pub fn push_hash_value_in_array<const N: usize>(&mut self, hash_value: &mut [u8; N])
    {
        self.get_hash_value(hash_value.as_mut_ptr(), N);
    }

    // pub fn get_hash_value(&mut self, hash_value: *mut u8, length_in_bytes: usize)
    /// Stores the hash value into a certain memory area.
    /// 
    /// # Features
    /// This function has the generalized interface (pointer, `*mut u8`)
    /// so as to enable other functions to wrap this function with any
    /// convenient interface for uses. So, this function is usually not called
    /// directly in Rust. This function is provided to be called from other
    /// programming languages such as C/C++.
    /// 
    /// # Arguments
    /// - `hash_value` is the pointer that points to the memory area that will
    ///   contain the hash value.
    /// - `length_in_bytes` is the length of the memory area that the argument
    ///   `hash_value` is pointing to.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    ///   you are highly recommended to use the method
    ///   [get_hash_value_string()](struct@Keccak_Generic#method.get_hash_value_string)
    ///   rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    ///   you are highly recommended to use the method
    ///   [get_hash_value_in_array()](struct@Keccak_Generic#method.get_hash_value_in_array)
    ///   rather than this method.
    /// - If you want to get the hash value in the form of Vec object
    ///   of predetermined length, you are highly recommended to use the method
    ///   [get_hash_value_in_vec()](struct@Keccak_Generic#method.get_hash_value_in_vec)
    ///   rather than this method.
    /// 
    /// # Example 1 for SHA3_384
    /// ```
    /// use std::io::Write;
    /// use cryptocol::hash::SHA3_384;
    /// 
    /// let mut hash = SHA3_384::new();
    /// let txt = "This is an example of the method get_hash_value().";
    /// hash.digest_str(txt);
    /// let mut hash_value = [0_u8; 48];
    /// hash.get_hash_value(hash_value.as_mut_ptr(), 48);
    /// let mut hs = String::new();
    /// for h in hash_value.iter()
    ///     { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    /// assert_eq!(hs, "96298277B7B1EB85520425DD38DA75B9C1CC5D4CF34FCFAD681C17D0BD9BCEEE02C4D3ED06E9575579BBCD4A2B8614AC");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.get_hash_value)
    pub fn get_hash_value(&mut self, hash_value: *mut u8, length_in_bytes: usize)
    {
        let chunk_num = length_in_bytes / RATE;
        let rest_num = length_in_bytes % RATE;
        for i in 0..chunk_num
        {
            let hash_code = self.squeeze();
            unsafe { copy_nonoverlapping(hash_code.as_ptr() as *const u8, hash_value.add(i * RATE), RATE); }
        }
        if rest_num != 0
        {
            let hash_code = self.squeeze();
            unsafe { copy_nonoverlapping(hash_code.as_ptr() as *const u8, hash_value.add(chunk_num * RATE), rest_num); }
        }
    }

    // pub fn read_hash_value_in_hexadecimal<const N: usize>(hash: &[u8; N]) -> String
    /// Reads the hash value, and returns it in hexadecimal format
    /// in the form of a String object.
    /// 
    /// # Features
    /// - This method is so useful to transform a hash value to hexadecimal
    ///   string.
    /// - This method can be used without instantiating this struct.
    /// 
    /// # Arguments
    /// - `hash` is the array object that contains hash value.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::hash::SHA3_256;
    /// let hash_value = [01u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x00, 0x11, 0x22, 0x33,0x44, 0x55, 0x66, 0x77];
    /// let hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    /// println!("Hash =\t{}", hs);
    /// assert_eq!(hs, "0123456789ABCDEF0011223344556677");
    /// ```
    // / 
    // / # For more examples,
    // / click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.read_hash_value_in_hexadecimal)
    pub fn read_hash_value_in_hexadecimal<const N: usize>(hash: &[u8; N]) -> String
    {
        let mut txt = String::new();
        for i in 0..N
        {
            let byte = hash[i];
            txt.push(Self::to_char(byte >> 4));
            txt.push(Self::to_char(byte & 0b1111));
        }
        txt
    }

    // pub fn squeeze(&mut self) -> [u8; RATE]
    /// Returns the resulting hash value after squeezing.
    /// 
    /// # Output
    /// A hash value in the form of array object of `u8` with `RATE` elements.
    /// 
    /// # Features
    /// The length of output hash value is automatically determined
    /// by the generic parameter `RATE`.
    /// 
    /// # Example 1 for SHA3_256
    /// ```
    /// use cryptocol::hash::SHA3_256;
    /// let mut hash = SHA3_256::new();
    /// let txt = "This is an example of the method squeeze().";
    /// hash.absorb_str(txt);
    /// let mut hash_value = hash.squeeze();
    /// let mut hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    /// assert_eq!(hs, "D035239B93FBE534A4B1C0923EB844D689E55ACFB1E13144E6FE7D69B75290F68502392D6138336847C2D52AB1AC84BC0F66B7501ED3705FE4811B106604A2A3704C563BB2BAC52B7FAE628AE96D7E38312B5779FE5E154FAD2116C8E58F97F42C202BBA92310B5DA0758BC2F4549B1EA1C43393FC0BD4BCF392BFB0B4B59883AC6B1D702B9563BA");
    /// 
    /// hash_value = hash.squeeze();
    /// hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    /// println!("First random number =\t{}", hs);
    /// assert_eq!(hs, "BFC8C6A5B32F55DB7FE8A060D63F1872BC8F98D2279567E92F0DC58FCB30808CB673FAB3C69AFA064D6081FB8349563AF2B1580CE458147D34C52A1CE54CA6F7C49EECB06DACCEA31926186455348AB9980F556C4764563D4AABE3C5EAF575767C7CA18DB2FD5651D567EE6D127196F424D1AC61A7B7DB60EE5E211EF290B7AF8A015E6D07C51139");
    /// hash_value = hash.squeeze();
    /// hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    /// println!("Second random number =\t{}", hs);
    /// assert_eq!(hs, "631720A8E88A894A34177078646CA9D081C65732E0D6587EBCC5E1B902E4D5B57ADCA15B8A55173A322DD552D28D2EB244AB60C63D6BE0C72206F0CB44F74F16FAA1CE1647426892424E82F7468C00B007C79859A858AC12C92FC8068ACF14EFD0825B3E1F3200EB922B652F2B1B34D2247051F77C5B2B050A1DB5BFA92F7D2C5951D9328DBFD3D3");
    /// hash_value = hash.squeeze();
    /// hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    /// println!("Third random number =\t{}", hs);
    /// assert_eq!(hs, "DC064A78C927A328F555A29B2654151A336D28B13F35A427DC0BB58BDA500089ED5EC7045CA3CF71B8121F3B77AC5861613E2F4EF49A23A9D737C78B7256B03AB7F9C5FF7A40814294F465EC5A0F646152FC84CF62B150B4B7CC5246E28DF429EDB483B7B34070092C27537B6AB365A6D6F77DDC2A910BD2513229C9043661F62874912EF88F7984");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.squeeze)
    pub fn squeeze(&mut self) -> [u8; RATE]
    {
        let mut block = [0_u8; RATE];
        unsafe { copy_nonoverlapping(self.state.as_ptr() as *mut u8, block.as_mut_ptr(), RATE); }
        for round in 0..ROUNDS
        {
            self.theta();
            self.rho();
            self.pi();
            self.chi();
            self.iota(round);
        }
        block
    }

    // pub fn absorb(&mut self, message: *const u8, length_in_bytes: u64)
    /// Absorbs the message.
    /// 
    /// # Arguments
    /// - `message` is pointer to `const u8`.
    /// - `length_in_bytes` is the size of message in the unit of bytes, and
    ///   its data type is `u64`.
    /// 
    /// # Features
    /// - Absorbing is the same as digesting.
    /// - This function has the generalized interface (pointer, `*const u8`)
    ///   so as to enable other functions to wrap this function with any
    ///   convenient interface for uses. So, this function is usually not called
    ///   directly in Rust. This function is provided to be called from other
    ///   programming languages such as C/C++.
    /// 
    /// # Warning
    /// You can use this method even for the type `cSHAKE_*` but it
    /// is not in accordance with the standard. It is desirable that You use
    /// the method `absorb_customized()` instead. If you use this method
    /// for the type `cSHAKE_*`, others may think that you do not
    /// fully understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally
    /// write your code in non-standard way, you can use this method even for
    /// the type `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [absorb_str()](struct@Keccak_Generic#method.absorb_str)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [absorb_string()](struct@Keccak_Generic#method.absorb_string)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [absorb_array()](struct@Keccak_Generic#method.absorb_array)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [absorb_vec()](struct@Keccak_Generic#method.absorb_array)
    ///   rather than this method.
    ///
    /// # Example 1 for SHA3_384
    /// ```
    /// use cryptocol::hash::SHA3_384;
    /// let mut hash = SHA3_384::new();
    /// let txt = "This is an example of the method absorb().";
    /// hash.absorb(txt.as_ptr(), txt.len() as u64);
    /// let hash_value = hash.squeeze();
    /// let hs = SHA3_384::read_hash_value_in_hexadecimal(&hash_value);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    /// assert_eq!(hs, "50739E69E4848F1AAB0192BD08899F5FDCF194EA91CFB51FCFA53BBF01749E81515503A363D961300434B7DA1BC3E2BA2A56E39A33D7D797AE3694D42027F52F594FEB9EA93684CAE961EC23B7BD8E586D7A79A9A80BE24175F5C169CBC4B0FC0C9E73C8D08EF06D");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.absorb)
    pub fn absorb(&mut self, message: *const u8, length_in_bytes: u64)
    {
        if PADDING == KECCAK_CONST::CSHAKE
        {
            self.absorb_customized([0u8;0].as_ptr(), 0, [0u8;0].as_ptr(), 0, message, length_in_bytes);
            return;
        }
        self.initialize_state();
        let mut count = 0_usize;
        let rest = length_in_bytes as usize % RATE;
        let length = length_in_bytes as usize - rest;
        while count < length
        {
            self.absorb_block(unsafe { message.add(count) });
            count += RATE;
        }

        let mut padded = [0_u8; RATE];
        unsafe { copy_nonoverlapping(message.add(count), padded.as_mut_ptr(), rest); }
        padded[rest] = Self::SEPARATOR;
        padded[RATE - 1] |= Self::TAIL;
        self.absorb_block(padded.as_ptr());
    }

    // pub fn absorb_str(&mut self, message: &str)
    /// Absorbs a string slice.
    /// 
    /// # Arguments
    /// - `message` is a string slice to be absorbed, and
    ///   its data type is `&str`.
    /// 
    /// # Features
    /// Absorbing is the same as digesting.
    /// 
    /// # Warning
    /// You can use this method even for the type `cSHAKE_*` but it
    /// is not in accordance with the standard. It is desirable that You use
    /// the method `absorb_str_customized()` instead. If you use this method
    /// for the type `cSHAKE_*`, others may think that you do not
    /// fully understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally
    /// write your code in non-standard way, you can use this method even for
    /// the type `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [absorb_string()](struct@Keccak_Generic#method.absorb_string)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [absorb_array()](struct@Keccak_Generic#method.absorb_array)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [absorb_vec()](struct@Keccak_Generic#method.absorb_array)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [absorb()](struct@Keccak_Generic#method.absorb) rather than this method.
    /// 
    /// # Example 1 for SHA3_512
    /// ```
    /// use cryptocol::hash::SHA3_512;
    /// let mut hash = SHA3_512::new();
    /// let txt = "This is an example of the method absorb_str().";
    /// hash.absorb_str(txt);
    /// let hash_value = hash.squeeze();
    /// let hs = SHA3_512::read_hash_value_in_hexadecimal(&hash_value);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    /// assert_eq!(hs, "625AFE67C854BAB869A7C18798402BDB136E2A320DF953F0AF0F6AD7CA10C3A4020BB0951AD8CAFD36249266B23D6AC41F2D8A65DCEA8FF0B643A17E16F01CFCE51A73747941EECD");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.absorb_str)
    pub fn absorb_str(&mut self, message: &str)
    {
        self.absorb(message.as_ptr(), message.len() as u64);
    }

    // pub fn absorb_string(&mut self, message: &String)
    /// Absorbs a String object.
    /// 
    /// # Arguments
    /// - `message` is a a string to be absorbed, and
    ///    its data type is `String` object.
    /// 
    /// # Features
    /// Absorbing is the same as digesting.
    /// 
    /// # Warning
    /// You can use this method even for the type `cSHAKE_*` but it
    /// is not in accordance with the standard. It is desirable that You use
    /// the method `absorb_string_customized()` instead. If you use this method
    /// for the type `cSHAKE_*`, others may think that you do not
    /// fully understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally
    /// write your code in non-standard way, you can use this method even for
    /// the type `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of the content of string
    ///   slice, you are highly recommended to use the method
    ///   [absorb_str()](struct@Keccak_Generic#method.absorb_str)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [absorb_array()](struct@Keccak_Generic#method.absorb_array)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [absorb_vec()](struct@Keccak_Generic#method.absorb_array)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [absorb()](struct@Keccak_Generic#method.absorb) rather than this method.
    /// 
    /// # Example 1 for SHA3_224
    /// ```
    /// use cryptocol::hash::SHA3_224;
    /// let mut hash = SHA3_224::new();
    /// let txt = "This is an example of the method absorb_string().".to_string();
    /// hash.absorb_string(&txt);
    /// let hash_value = hash.squeeze();
    /// let hs = SHA3_224::read_hash_value_in_hexadecimal(&hash_value);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    /// assert_eq!(hs, "D3B139242B2B6EF8B399CF0E6A4F6D993AC83EDE2CEA3894D4EDE8AAB69ADA5DB232289E62D752349FC3F621B2DA118400142D5DDBA6897633F712A1B320FC4F1F7F3D5BE186FE75F8E3602D344FDA1A5AAB343E8FC68B918CC3C2DC01DA3DDF7AE0CBF0C855D6463EBDCAD81F9CABED5580EF6C652E786DED50EBC72DD827E4129A368A823E8B6F7FBCFA6F9FBB72F9");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.absorb_string)
    #[inline]
    pub fn absorb_string(&mut self, message: &String)
    {
        self.absorb(message.as_ptr(), message.len() as u64);
    }

    // pub fn absorb_array<U, const N: usize>(&mut self, message: &[U; N])
    /// Absorbs an array object.
    /// 
    /// # Arguments
    /// - `message` is an array of data type `U` with `N` elements
    ///   to be absorbed.
    /// 
    /// # Features
    /// Absorbing is the same as digesting.
    /// 
    /// # Warning
    /// You can use this method even for the type `cSHAKE_*` but it
    /// is not in accordance with the standard. It is desirable that You use
    /// the method `absorb_array_customized()` instead. If you use this method
    /// for the type `cSHAKE_*`, others may think that you do not
    /// fully understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally
    /// write your code in non-standard way, you can use this method even for
    /// the type `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [absorb_str()](struct@Keccak_Generic#method.absorb_str)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [absorb_string()](struct@Keccak_Generic#method.absorb_string)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [absorb_vec()](struct@Keccak_Generic#method.absorb_array)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [absorb()](struct@Keccak_Generic#method.absorb) rather than this method.
    /// 
    /// # Example 1 for SHA3_256
    /// ```
    /// use cryptocol::hash::SHA3_256;
    /// let mut hash = SHA3_256::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.absorb_array(&data);
    /// let hash_value = hash.squeeze();
    /// let hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    /// println!("Hash =\t{}", hs);
    /// assert_eq!(hs, "2FA65DD00903E850AD14E00D13ACBE9C2CA2E7B140419B8C7EA2742900586B149F41975DC994F8BD506DA5460FE855267CBD0C02D4DB595C78697D2D80FCE659B1F7ED187BEC70EDC00666A545DDBC8836B11D9410F544FD3A0A0288DAEBB92F86C654AC20336A9A7343180DBB8FC342E024DA4627C80697D78BD365AEEBC58353F684ACEAE89ED4");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.absorb_array)
    #[inline]
    pub fn absorb_array<U, const N: usize>(&mut self, message: &[U; N])
    where U: SmallUInt + Copy + Clone
    {
        self.absorb(message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64);
    }

    // pub fn absorb_vec<U>(&mut self, message: &Vec<U>)
    /// Absorbs a `Vec` object.
    /// 
    /// # Arguments
    /// - `message` is a `Vec` object to be absorbed.
    /// 
    /// # Features
    /// Absorbing is the same as digesting.
    /// 
    /// # Warning
    /// You can use this method even for the type `cSHAKE_*` but it
    /// is not in accordance with the standard. It is desirable that You use
    /// the method `absorb_vec_customized()` instead. If you use this method
    /// for the type `cSHAKE_*`, others may think that you do not
    /// fully understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally
    /// write your code in non-standard way, you can use this method even for
    /// the type `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [absorb_str()](struct@Keccak_Generic#method.absorb_str)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [absorb_string()](struct@Keccak_Generic#method.absorb_string)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [absorb_array()](struct@Keccak_Generic#method.absorb_array)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [absorb()](struct@Keccak_Generic#method.absorb) rather than this method.
    /// 
    /// # Example 1 for SHA3_512
    /// ```
    /// use cryptocol::hash::SHA3_512;
    /// let mut hash = SHA3_512::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.absorb_vec(&data);
    /// let hash_value = hash.squeeze();
    /// let hs = SHA3_512::read_hash_value_in_hexadecimal(&hash_value);
    /// println!("Hash =\t{}", hs);
    /// assert_eq!(hs, "8714760A0421335BF23FE3C6F436686983A04B055729161041E717E1E964AD2B4E0DB3EEBBE35BC1BAE2999EF1E0DF48F4A60FBAFA1F5B96E3A20A24D44A02989646C598D1857681");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.absorb_vec)
    #[inline]
    pub fn absorb_vec<U>(&mut self, message: &Vec<U>)
    where U: SmallUInt + Copy + Clone
    {
        self.absorb(message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64);
    }

    // pub fn absorb_customized(&mut self, filename: *const u8, filename_length_in_bytes: u64, user: *const u8, user_length_in_bytes: u64, message: *const u8, length_in_bytes: u64)
    /// Absorbs the message with filename string and user-defined string.
    /// 
    /// # Arguments
    /// - `function_name` is pointer to const u8, which contains function name,
    ///   but it is reserved for NIST use. You are supposed to give null string
    ///   for `function name`. You may want to use `user_defined` instead.
    ///   However, for all the types other than cSHAKE, you can freely use
    ///   this method of the expanded versions of struct `Keccak_Generic` for
    ///   your own purpose.
    /// - `function_name_length_in_bytes` is the size of `function_name` in the
    ///   unit of bytes, and its data type is `u64`, but it is reserved
    ///   for NIST use. You are supposed to give `0` for
    ///   `function_name_length_in_bytes`. You may want to use
    ///   `user_defined_length_in_bytes` instead. However, for all the types other
    ///   than cSHAKE, you can you `function_name_length_in_bytes` because NIST
    ///   reserved `function_name` and `function_name_length_in_bytes` only for
    ///   cSHAKE. You can freely use this method for expanded versions of struct
    ///   `Keccak_Generic`.
    /// - `user_defined` is pointer to const u8, which contains a string for
    ///   your special use.
    /// - `user_defined_length_in_bytes` is the size of `user_defined` in the
    ///   unit of bytes, and its data type is `u64`.
    /// - `message` is pointer to const u8, which contains the actual date to
    ///   hash.
    /// - `length_in_bytes` is the size of message in the unit of bytes, and
    ///   its data type is `u64`.
    /// 
    /// # Features
    /// This function has the generalized interface (pointer, `*const u8`)
    /// so as to enable other functions to wrap this function with any
    /// convenient interface for uses. So, this function is usually not called
    /// directly in Rust. This function is provided to be called from other
    /// programming languages such as C/C++.
    /// 
    /// # Warning
    /// You can use this method even for the types other than `cSHAKE_*` but
    /// it is not in accordance with the standard. It is desirable that You
    /// use the method `absorb()` instead. If you use this method for the
    /// types other than `cSHAKE_*`, others may think that you do not fully
    /// understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally
    /// write your code in non-standard way, you can use this method even for
    /// the types other than `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [absorb_str_customized()](struct@Keccak_Generic#method.absorb_str_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [absorb_string_customized()](struct@Keccak_Generic#method.absorb_string_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [absorb_array_customized()](struct@Keccak_Generic#method.absorb_array_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [absorb_vec_customized()](struct@Keccak_Generic#method.absorb_vec_customized)
    ///   rather than this method.
    /// 
    /// # Example 1 for cSHAKE_128
    /// ```
    /// use cryptocol::hash::cSHAKE_128;
    /// let mut hash = cSHAKE_128::new();
    /// let txt = "This is an example of the method absorb_customized().";
    /// let user_defined = "On my own purpose";
    /// hash.absorb_customized("".as_ptr(), 0, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    /// let hash_value = hash.squeeze();
    /// let hs = cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    /// assert_eq!(hs, "C72A5C005749C1EEFC7776965FE81E519E769A026D5FA9321928D7881B77A46A1C44A949E1636846D29771076EE02E40C1A684F0575D3F618DA25C6025153AE2F4E416844E1676F885116B879B9E53356A5C18D7E5216D32BEAF07EBD652477A2AA03A6A31408C5153730084218FFFF9B5CD5F1AC1F1949A4732231CA7364F223516B3871384048695ECC60079CB18B25963643CA8468EA3BC1DD020E66B75C6E13AF900C61DD9E4");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.absorb_customized)
    pub fn absorb_customized(&mut self, function_name: *const u8, function_name_length_in_bytes: u64,
                                user_defined: *const u8, user_defined_length_in_bytes: u64,
                                message: *const u8, length_in_bytes: u64)
    {
        if function_name_length_in_bytes == 0 && user_defined_length_in_bytes == 0
        {
            if PADDING == KECCAK_CONST::CSHAKE
            {
                let mut shake 
                    = Keccak_Generic::<RATE, {KECCAK_CONST::SHAKE}, ROUNDS, T, LFSR,
                                        THETA_SUB, THETA_ADD, THETA_ROT,
                                        RHO_MUL_X, RHO_MUL_Y, RHO_T,
                                        PI_MUL_X, PI_MUL_Y, CHI_ADD_1, CHI_ADD_2>::new();
                shake.absorb(message, length_in_bytes);
                self.state = shake.state;
            }
            else
            {
                self.absorb(message, length_in_bytes);
            }
            return;
        }

        let mut prefix = Self::encode_left(RATE as u64);
        prefix.append(&mut Self::encode_string(function_name, function_name_length_in_bytes));
        prefix.append(&mut Self::encode_string(user_defined, user_defined_length_in_bytes));

        self.initialize_state();
        let mut count = 0_usize;
        let mut length = prefix.len();
        let mut rest = length % RATE;
        length -= rest;
        while count < length
        {
            self.absorb_block( unsafe { prefix.as_ptr().add(count) } );
            count += RATE;
        }
        if rest != 0
        {
            let mut block = [0_u8; RATE];
            unsafe { copy_nonoverlapping(prefix.as_ptr().add(count), block.as_mut_ptr(), rest); }
            self.absorb_block(block.as_ptr());
        }

        count = 0_usize;
        rest = length_in_bytes as usize % RATE;
        length = length_in_bytes as usize - rest;
        while count < length
        {
            self.absorb_block(unsafe { message.add(count) });
            count += RATE;
        }

        let mut padded = [0_u8; RATE];
        unsafe { copy_nonoverlapping(message.add(count), padded.as_mut_ptr(), rest); }
        padded[rest] = Self::SEPARATOR;
        padded[RATE - 1] |= Self::TAIL;
        self.absorb_block(padded.as_ptr());
    }

    // pub fn absorb_str_customized(&mut self, function_name: &str, user_defined: &str, message: &str)
    /// Absorbs the message with filename string slice and user-defined string slice.
    /// 
    /// # Arguments
    /// - `function_name` is of type `&str`, which contains function name,
    ///   but it is reserved for NIST use. You are supposed to give null string
    ///   for `function name`. You may want to use `user_defined` instead.
    ///   However, for all the types other than cSHAKE, you can freely use
    ///   this method of the expanded versions of struct `Keccak_Generic` for
    ///   your own purpose.
    /// - `user_defined` is of type `&str`, which contains a string for
    ///   your special use.
    /// - `message` is of type `&str`.
    /// 
    /// # Warning
    /// You can use this method even for the types other than `cSHAKE_*` but
    /// it is not in accordance with the standard. It is desirable that You
    /// use the method `absorb_strr()` instead. If you use this method for the
    /// types other than `cSHAKE_*`, others may think that you do not fully
    /// understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally
    /// write your code in non-standard way, you can use this method even for
    /// the types other than `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [absorb_string_customized()](struct@Keccak_Generic#method.absorb_string_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [absorb_array_customized()](struct@Keccak_Generic#method.absorb_array_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [absorb_vec_customized()](struct@Keccak_Generic#method.absorb_vec_customized)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [absorb_customized()](struct@Keccak_Generic#method.absorb_customized)
    ///   rather than this method.
    /// 
    /// # Example 1 for cSHAKE_128
    /// ```
    /// use cryptocol::hash::cSHAKE_128;
    /// let mut hash = cSHAKE_128::new();
    /// let txt = "This is an example of the method absorb_str_customized().";
    /// let user_defined = "On my own purpose";
    /// hash.absorb_str_customized("", user_defined, txt);
    /// let hash_value = hash.squeeze();
    /// let hs = cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    /// assert_eq!(hs, "8C4C069400F3D84F48DCF19F3C38136BA5C7ACADE522359FB44085A39C3909A41E2D953B9454BC3BF5794DEE14B65F0B801FC72EE593850F63552441E08E328D7EDFDD0F7C84A48C699A1C01C331002D938422DAFDEC9BA210E0F0CA8828C0EF89038FE0995BEBFC21D96AB4EE1599E847427B465F3911BA6631A4FF85585663F255ECA5459E090B4FE3E9B199086BF33E4F1221A8128BBE9487DE178EE497B088B69F80F0C024AE");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.absorb_str_customized)
    #[inline]
    pub fn absorb_str_customized(&mut self, function_name: &str, user_defined: &str, message: &str)
    {
        self.absorb_customized(function_name.as_ptr(), function_name.len() as u64,
                                user_defined.as_ptr(), user_defined.len() as u64,
                                message.as_ptr(), message.len() as u64);
    }

    // pub fn absorb_string_customized(&mut self, function_name: &String, user_defined: &String, message: &String)
    /// Absorbs the message with filename string and user-defined string.
    /// 
    /// # Arguments
    /// - `function_name` is of type `&String`, which contains function name,
    ///   but it is reserved for NIST use. You are supposed to give null string
    ///   for `function name`. You may want to use `user_defined` instead.
    ///   However, for all the types other than cSHAKE, you can freely use
    ///   this method of the expanded versions of struct `Keccak_Generic` for
    ///   your own purpose.
    /// - `user_defined` is of type `&String`, which contains a string for
    ///   your special use.
    /// - `message` is of type `&String`.
    /// 
    /// # Warning
    /// You can use this method even for the types other than `cSHAKE_*` but
    /// it is not in accordance with the standard. It is desirable that You
    /// use the method `absorb_strr()` instead. If you use this method for the
    /// types other than `cSHAKE_*`, others may think that you do not fully
    /// understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally
    /// write your code in non-standard way, you can use this method even for
    /// the types other than `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [absorb_str_customized()](struct@Keccak_Generic#method.absorb_str_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [absorb_array_customized()](struct@Keccak_Generic#method.absorb_array_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [absorb_vec_customized()](struct@Keccak_Generic#method.absorb_vec_customized)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [absorb_customized()](struct@Keccak_Generic#method.absorb_customized)
    ///   rather than this method.
    /// 
    /// # Example 1 for cSHAKE_256
    /// ```
    /// use cryptocol::hash::cSHAKE_256;
    /// let mut hash = cSHAKE_256::new();
    /// let txt = "This is an example of the method absorb_string_customized().".to_string();
    /// let user_defined = "On my own purpose".to_string();
    /// hash.absorb_string_customized(&"".to_string(), &user_defined, &txt);
    /// let hash_value = hash.squeeze();
    /// let hs = cSHAKE_256::read_hash_value_in_hexadecimal(&hash_value);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    /// assert_eq!(hs, "E9D77C6EEF37DEB01E8B8A8F250A4F5791A3B2BBF17D9C5A729DF38ADBF80742F14C44E7CE17CAFEF6A847C5403D0D420BD47E38E24E6B8D3946B825D96165BD5D81373195218031E915DCC744F7CA244F600D1D7E2B6195890BD2F0B5CE05BB2D877958675D0E0395150CF7E94A0036200A620049595530D123C077CAC1749E4ACE0098F92DA6BB");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.absorb_string_customized)
    #[inline]
    pub fn absorb_string_customized(&mut self, function_name: &String, user_defined: &String, message: &String)
    {
        self.absorb_customized(function_name.as_ptr(), function_name.len() as u64,
                                user_defined.as_ptr(), user_defined.len() as u64,
                                message.as_ptr(), message.len() as u64);
    }

    // pub fn absorb_array_customized<U, const K: usize, const L: usize, const M: usize>(&mut self, function_name: &[U; K], user_defined: &[U; L], message: &[U; M])
    /// Absorbs the message with filename array and user-defined array.
    /// 
    /// # Arguments
    /// - `function_name` is of type `&[U; L]`, which contains function name,
    ///   but it is reserved for NIST use. You are supposed to give null string
    ///   for `function name`. You may want to use `user_defined` instead.
    ///   However, for all the types other than cSHAKE, you can freely use
    ///   this method for expanded versions of struct `Keccak_Generic`.
    /// - `user_defined` is of type `&[V; M]`, which contains a string for
    ///   your special use.
    /// - `message` is of the type `&[W; N]`.
    /// 
    /// # Warning
    /// You can use this method even for the types other than `cSHAKE_*` but
    /// it is not in accordance with the standard. It is desirable that You
    /// use the method `absorb_strr()` instead. If you use this method for the
    /// types other than `cSHAKE_*`, others may think that you do not fully
    /// understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally
    /// write your code in non-standard way, you can use this method even for
    /// the types other than `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [absorb_str_customized()](struct@Keccak_Generic#method.absorb_str_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [absorb_string_customized()](struct@Keccak_Generic#method.absorb_string_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [absorb_vec_customized()](struct@Keccak_Generic#method.absorb_vec_customized)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [absorb_customized()](struct@Keccak_Generic#method.absorb_customized)
    ///   rather than this method.
    /// 
    /// # Example 1 for cSHAKE_256
    /// ```
    /// use cryptocol::hash::cSHAKE_256;
    /// let mut hash = cSHAKE_256::new();
    /// let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.absorb_array_customized(&[0_u8; 0], &user_defined, &data);
    /// let hash_value = hash.squeeze();
    /// let hs = cSHAKE_256::read_hash_value_in_hexadecimal(&hash_value);
    /// println!("Hash =\t{}", hs);
    /// assert_eq!(hs, "FE5EBE3F5007EDCE865F7DC425531F0C493E18168835722CCC0F11CE66F1423061C202BE0B1D0528DE0D763A3097A090B62115392769305D1FF32588A78CCEE990286ABF615452E4DBFF3915D64673E725C5BADF965FC06A80F90C865C65EAEA7A107228B80AC49C9C89F98F1AF4C3563BE243207D6970219264ACB9420E97BF802E862D32D18F31");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.absorb_array_customized)
    #[inline]
    pub fn absorb_array_customized<U, V, W, const L: usize, const M: usize, const N: usize>(&mut self, function_name: &[U; L], user_defined: &[V; M], message: &[W; N])
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone, W: SmallUInt + Copy + Clone
    {
        self.absorb_customized(function_name.as_ptr() as *const u8, L as u64 * U::size_in_bytes() as u64,
                                user_defined.as_ptr() as *const u8, M as u64 * V::size_in_bytes() as u64,
                                message.as_ptr() as *const u8, N as u64 * W::size_in_bytes() as u64);
    }

    // pub fn absorb_vec_customized<U>(&mut self, function_name: &Vec<U>, user_defined: &Vec<U>, message: &Vec<U>)
    /// Absorbs the message with filename Vec object and user-defined Vec object.
    /// 
    /// # Arguments
    /// - `function_name` is of type `&[U; L]`, which contains function name,
    ///   but it is reserved for NIST use. You are supposed to give null string
    ///   for `function name`. You may want to use `user_defined` instead.
    ///   However, for all the types other than cSHAKE, you can freely use
    ///   this method for expanded versions of struct `Keccak_Generic`.
    /// - `user_defined` is of type `&[V; M]`, which contains a string for
    ///   your special use.
    /// - `message` is of the type `&[W; N]`.
    /// 
    /// # Warning
    /// You can use this method even for the types other than `cSHAKE_*` but
    /// it is not in accordance with the standard. It is desirable that You
    /// use the method `absorb_strr()` instead. If you use this method for the
    /// types other than `cSHAKE_*`, others may think that you do not fully
    /// understand `SHAKE-*` and `cSHAKE-*`. However, if you intentionally
    /// write your code in non-standard way, you can use this method even for
    /// the types other than `cSHAKE_*`, of course.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [absorb_str_customized()](struct@Keccak_Generic#method.absorb_str_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [absorb_string_customized()](struct@Keccak_Generic#method.absorb_string_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [absorb_array_customized()](struct@Keccak_Generic#method.absorb_array_customized)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [absorb_customized()](struct@Keccak_Generic#method.absorb_customized)
    ///   rather than this method.
    /// 
    /// # Example 1 for cSHAKE_128
    /// ```
    /// use cryptocol::hash::cSHAKE_128;
    /// let mut hash = cSHAKE_128::new();
    /// let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.absorb_vec_customized(&vec![0_u8; 0], &user_defined, &data);
    /// let hash_value = hash.squeeze();
    /// let hs = cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    /// println!("Hash =\t{}", hs);
    /// assert_eq!(hs, "1307CF49A76814AC1D48ADE39AEBC3FADC746604B8C0376B0F76AA01BF4395C097C316B00278F63B080CA5662B4588D38D25A8599414064D91661F2F5B850266C4436230F8557E5CA45A5E295205B99B00EA3756700E85A5C4C3EAFD7948E7C9AFF5015930DFCEF8DCC22E36A1CA896AB4AA501E849047DACF8702644AE746011D63ADB8A3CC1F1446B8028182485C081B87523FBE0BA5E70F3AD020829E9293767E34CE275D30CC");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.absorb_vec_customized)
    #[inline]
    pub fn absorb_vec_customized<U, V, W>(&mut self, function_name: &Vec<U>, user_defined: &Vec<V>, message: &Vec<W>)
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone, W: SmallUInt + Copy + Clone
    {
        self.absorb_customized(function_name.as_ptr() as *const u8, function_name.len() as u64 * U::size_in_bytes() as u64,
                                user_defined.as_ptr() as *const u8, user_defined.len() as u64 * V::size_in_bytes() as u64,
                                message.as_ptr() as *const u8, message.len() as u64 * W::size_in_bytes() as u64);
    }

    // pub fn tangle(&mut self, tangling: u64)
    /// Tangles the hash value
    /// 
    /// # Arguments
    /// u64 constants to tangle the hash value
    /// 
    /// # Features
    /// It is for using this struct as random number generator.
    /// 
    /// # Example 1 for SHA3_256
    /// ```
    /// use cryptocol::hash::SHA3_256;
    /// let mut hash = SHA3_256::new();
    /// let txt = "TANGLING";
    /// hash.digest_str(txt);
    /// let hs = hash.get_hash_value_in_string();
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    /// assert_eq!(hs, "FA126A570281297F1B8F4075BECD6CD4263AB45A746D37CE2323560E876021A3");
    /// hash.tangle(1);
    /// let hs = hash.get_hash_value_in_string();
    /// println!("Hash =\t{}", hs);
    /// assert_eq!(hs, "762C52E75427AD19FD29FF6606CFC1E09DD9038C2B23489591A6288AA69F0182");
    /// hash.tangle(1);
    /// let hs = hash.get_hash_value_in_string();
    /// println!("Hash =\t{}", hs);
    /// assert_eq!(hs, "1959308B6378EB3613A1F9A6FFBB77534549A5FDD0BE06A9D3A7988D92F8B0CB");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.tangle)
    pub fn tangle(&mut self, tangling: u64)
    {
        let mut rate = [0_u8; RATE];
        let count = if RATE < 8 { RATE } else { 8 }; 
        unsafe { copy_nonoverlapping(&tangling as *const u64 as *const u8, rate.as_mut_ptr(), count); }
        self.absorb_block(rate.as_ptr());
    }

    fn encode_string(txt: *const u8, txt_length_in_bytes: u64) -> Vec<u8>
    {
        let mut res = Self::encode_left(txt_length_in_bytes << 3);
        for i in 0..txt_length_in_bytes as usize
            { res.push( unsafe { *txt.add(i) } ) }
            // { res.push( unsafe { (*txt.add(i)).reverse_bits() } ) }
        res
    }

    fn encode_left(length: u64) -> Vec<u8>
    {
        let size = LongUnion::new_with(length);
        let mut prefix = Vec::<u8>::new();
        if length <= u8::MAX as u64
        {
            prefix.push(1_u8);
            prefix.push(size.get_ubyte_(0));
        }
        else if length <= u16::MAX as u64
        {
            prefix.push(2_u8);
            prefix.push(size.get_ubyte_(1));
            prefix.push(size.get_ubyte_(0));
        }
        else if length <= u32::MAX as u64
        {
            prefix.push(4_u8);
            prefix.push(size.get_ubyte_(3));
            prefix.push(size.get_ubyte_(2));
            prefix.push(size.get_ubyte_(1));
            prefix.push(size.get_ubyte_(0));
        }
        else
        {
            prefix.push(8_u8);
            prefix.push(size.get_ubyte_(7));
            prefix.push(size.get_ubyte_(6));
            prefix.push(size.get_ubyte_(5));
            prefix.push(size.get_ubyte_(4));
            prefix.push(size.get_ubyte_(3));
            prefix.push(size.get_ubyte_(2));
            prefix.push(size.get_ubyte_(1));
            prefix.push(size.get_ubyte_(0));
        };
        prefix        
    }

    // fn initialize_state(&mut self)
    /// Initialize state array to be all zeros.
    fn initialize_state(&mut self)
    {
        unsafe { write_bytes(self.state.as_mut_ptr() as *mut T, 0, 5 * 5); }
        // for y in 0..5
        // {
        //     for x in 0..5
        //         { self.state[x][y] = T::zero(); }
        // }
    }

    // fn absorb_block(&mut self, block: *const u8)
    /// The block of length, `RATE` bytes, will be absorbed.
    fn absorb_block(&mut self, block: *const u8)
    {
        self.feed_block_to_state(block);
        for round in 0..ROUNDS
        {
            self.theta();       // self._check(round, "theta");
            self.rho();         // self._check(round, "rho");
            self.pi();          // self._check(round, "pi");
            self.chi();         // self._check(round, "chi");
            self.iota(round);   // self._check(round, "iota");
        }
    }


    // fn feed_message_to_state(&mut self, message: *const u8)
    /// The message will be absorbed `RATE` bytes by `RATE` bytes.
    #[inline]
    fn feed_block_to_state(&mut self, block: *const u8)
    {
        let mut state = [[T::MIN; 5]; 5];
        unsafe { copy_nonoverlapping(block, state.as_mut_ptr() as *mut u8, RATE); }

        let nominator = (T::BITS / 8) as usize * 5;
        let limit_y = RATE / nominator;
        for y in 0..limit_y
        {
            for x in 0..5_usize
                { self.state[y][x] ^= state[y][x]; }
        }
        let mut limit_x = (RATE % nominator) / (T::BITS as usize / 8);
        if (RATE % nominator) % (T::BITS as usize / 8) != 0
            { limit_x += 1; }
        for x in 0..limit_x
            { self.state[limit_y][x] ^= state[limit_y][x]; }
    }

    fn theta(&mut self)
    {
        let mut c = [T::zero(); 5];
        for x in 0..5_usize
        {
            for y in 0..5_usize
                { c[x] ^= self.state[y][x]; }
        }

        let mut d = [T::zero(); 5];
        for i in 0..5_usize
        {
            d[i] = c[i.modular_sub(THETA_SUB, 5)]
                 ^ c[i.modular_add(THETA_ADD, 5)].rotate_left(THETA_ROT);
        }

        for y in 0..5_usize
        {
            for x in 0..5_usize
                { self.state[y][x] ^= d[x]; }
        }
    }

    fn rho(&mut self)
    {
        let (mut x, mut y) = (1_usize, 0_usize);
        for t in 0..RHO_T
        {
            let rot = (t + 1) * (t + 2) / 2;
            self.state[y][x] = self.state[y][x].rotate_left(rot);
            let x2 = x.modular_mul(RHO_MUL_X, 5);
            let y3 = y.modular_mul(RHO_MUL_Y, 5);
            (x, y) = (y, x2.modular_add(y3, 5));
        }
    }

    fn pi(&mut self)
    {
        let mut array= [[T::MIN; 5]; 5];
        for y in 0..5_usize
        {
            for x in 0..5_usize
            {
                let x1 = x.modular_mul(PI_MUL_X, 5);
                let y3 = y.modular_mul(PI_MUL_Y, 5);
                let xy = x1.modular_add(y3, 5);
                array[y][x] = self.state[x][xy];
            }
        }
        self.state = array;
    }

    fn chi(&mut self)
    {
        let mut array= [[T::MIN; 5]; 5];
        for y in 0..5_usize
        {
            for x in 0..5_usize
            {
                let a = self.state[y][x];
                let b = !self.state[y][x.modular_add(CHI_ADD_1, 5)];
                let c = self.state[y][x.modular_add(CHI_ADD_2, 5)];
                array[y][x] = a ^ (b & c);
            }
        }
        self.state = array;
    }

    #[inline]
    fn iota(&mut self, round: usize)
    {
        self.state[0][0] ^= Self::RC[round];
    }

    // pub fn get_desirable_l() -> usize
    /// Returns the desirable `L` according to the size of `T`.
    /// 
    /// # Output
    /// This method returns the desirable `L` in the type `usize`
    /// calculated based on the size of `T`.
    /// 
    /// # Example 1 for SHA3_224
    /// ```
    /// use cryptocol::hash::SHA3_224;
    /// let L = SHA3_224::get_desirable_l();
    /// println!("The desirable L of SHA3-244 is {}", L);
    /// assert_eq!(L, 6);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.get_desirable_l)
    #[inline]
    pub fn get_desirable_l() -> usize
    {
        T::BITS.trailing_zeros() as usize
    }

    // pub fn get_desirable_rounds() -> usize
    /// Returns the desirable number of rounds according to the size of `T`.
    /// 
    /// # Output
    /// This method returns the desirable number of rounds in the type `usize`
    /// calculated based on the size of `T`.
    /// 
    /// # Example 1 for SHA3_512
    /// ```
    /// use cryptocol::hash::SHA3_512;
    /// let ROUND = SHA3_512::get_desirable_rounds();
    /// println!("The desirable ROUND of SHA3-512 is {}", ROUND);
    /// assert_eq!(ROUND, 24);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.get_desirable_rounds)
    #[inline]
    pub fn get_desirable_rounds() -> usize
    {
        12 + 2 * Self::get_desirable_l() as usize
    }

    // pub fn get_desirable_b() -> usize
    /// Returns the desiable `B` according to the size of `T`. 
    /// 
    /// # Output
    /// This method returns the desiable `B` in the type `usize`
    /// calculated based on the size of `T`.
    /// 
    /// # Features
    /// The desiable `B` is expressed not in bits but in bytes here.
    /// 
    /// # Example 1 for SHA3_384
    /// ```
    /// use cryptocol::hash::SHA3_384;
    /// let B = SHA3_384::get_desirable_b();
    /// println!("The desirable B of SHA3-384 is {}", B);
    /// assert_eq!(B, 200);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.get_desirable_b)
    #[inline]
    pub fn get_desirable_b() -> usize
    {
        25 * (1 << (Self::get_desirable_l() - 3))
    }

    // pub fn get_desirable_output_length() -> usize
    /// Returns the desiable `DEFUALT_OUTPUT_LENGTH_IN_BYTES` of specific algorithm.
    /// The desiable `DEFUALT_OUTPUT_LENGTH_IN_BYTES` is expressed
    /// not in bits but in bytes here.
    /// 
    /// # Output
    /// This method returns the desiable `DEFUALT_OUTPUT_LENGTH_IN_BYTES` of specific
    /// algorithm in the type `usize`.
    /// 
    /// # Example 1 for SHA3_256
    /// ```
    /// use cryptocol::hash::SHA3_256;
    /// let LENGTH = SHA3_256::get_desirable_output_length();
    /// println!("The desirable output length of SHA3-256 is {}", LENGTH);
    /// assert_eq!(LENGTH, 32);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.get_desirable_output_length)
    #[inline]
    pub fn get_desirable_output_length() -> usize
    {
        Self::DEFUALT_OUTPUT_LENGTH_IN_BYTES
    }

    #[inline]
    fn to_char(nibble: u8) -> char
    {
        if nibble < 10 
            { ('0' as u8 + nibble) as u8 as char }
        else
            { ('A' as u8 - 10 + nibble) as char }
    }



    ///////////////////// For test
    // #[inline] pub fn _initialize_state(&mut self)   { self.initialize_state(); }
    // #[inline] pub fn _feed_block_to_state(&mut self, block: *const u8)  { self.feed_block_to_state(block); }
    // #[inline] pub fn _absorb_block(&mut self, block: *const u8)         { self.absorb_block(block); }
    // #[inline] pub fn _theta(&mut self)  { self.theta(); }
    // #[inline] pub fn _rho(&mut self)    { self.rho(); }
    // #[inline] pub fn _pi(&mut self)     { self.pi(); }
    // #[inline] pub fn _chi(&mut self)    { self.chi(); }
    // #[inline] pub fn _iota(&mut self, round: usize) { self.iota(round); }
    
    // fn convert_state_to_message(&mut self) -> Vec<T>
    // {
    //     let mut message = Vec::<T>::new();
    //     for y in 0..5
    //     {
    //         for x in 0..5
    //             { message.push(self.state[x][y]); }
    //     }
    //     message
    // }
    
    // pub fn _show_state(&self)
    // {
    //     println!("State: ");
    //     for y in 0..5
    //     {
    //         for x in 0..5
    //         {
    //             let u = LongUnion::new_with(self.state[y][x].into_u64());
    //             for i in 0..8
    //             {
    //                 print!("{:02X} ", u.get_ubyte_(i));
    //             }
    //             print!("  ");
    //             if (x + y * 5)  & 1 == 1
    //                 { println!(); }
    //         }
    //     }
    //     println!();println!();
    // }
    
    // pub fn _check(&self, round: usize, txt: &str)
    // {
    //     let mut bblock = [0_u8; 25 * 8];
    //     unsafe { copy_nonoverlapping(self.state.as_ptr() as *const u8, bblock.as_mut_ptr() as *mut u8, bblock.len()); }
    //     print!("{} {} state = ", round, txt);
    //     for j in 0..12
    //     {
    //         for i in 0..16
    //             { print!("{:02X?} ", bblock[j * 16 + i]); }
    //         println!();
    //     }
    //     for i in 0..8
    //         { print!("{:02X?} ", bblock[12 * 16 + i]); }
    //     println!();
    // }
    
    // pub fn _check_pad(&self, pad: &[u8; RATE])
    // {
    //     let mut bblock = [0_u8; 25 * 8];
    //     unsafe { copy_nonoverlapping(pad.as_ptr() as *const u8, bblock.as_mut_ptr() as *mut u8, pad.len()); }
    //     print!("PAD = ");
    //     for j in 0..12
    //     {
    //         for i in 0..16
    //             { print!("{:02X?} ", bblock[j * 16 + i]); }
    //         println!();
    //     }
    //     for i in 0..8
    //         { print!("{:02X?} ", bblock[12 * 16 + i]); }
    //     println!();
    // }
}





impl<const RATE: usize, const PADDING: usize, const ROUNDS: usize, T, const LFSR: u8,
    const THETA_SUB: usize, const THETA_ADD: usize, const THETA_ROT: u32,
    const RHO_MUL_X: usize, const RHO_MUL_Y: usize, const RHO_T: u32,
    const PI_MUL_X: usize, const PI_MUL_Y: usize,
    const CHI_ADD_1: usize, const CHI_ADD_2: usize>
Display for Keccak_Generic<RATE, PADDING, ROUNDS, T, LFSR,
                THETA_SUB, THETA_ADD, THETA_ROT,
                RHO_MUL_X, RHO_MUL_Y, RHO_T,
                PI_MUL_X, PI_MUL_Y, CHI_ADD_1, CHI_ADD_2>
where T: SmallUInt
{
    /// Formats the value using the given formatter.
    /// You will hardly use this method directly.
    /// Automagically the function `to_string()` will be implemented. So, you
    /// can use the function `to_string()`, and you can also print the SHA-1
    /// object in the macro `println!()` directly for example.
    /// `f` is a buffer, this method must write the formatted string into it.
    /// [Read more](https://doc.rust-lang.org/core/fmt/trait.Display.html#tymethod.fmt)
    ///
    /// # Example 1 for SHA2_256 for to_string()
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    /// assert_eq!(hash.to_string(), "46577469D8A5CBCA1AC874A5350E4FACD318A468046816B066117D51DB9D1640");
    /// ```
    ///
    /// # Example 2 for SHA2_256_Expanded for to_string()
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    /// assert_eq!(my_hash.to_string(), "6DED905D80768EE8F19D76233902E6CA1417B23A89845C2DA9127FEDD7CCDB5C");
    /// ```
    ///
    /// # Example 3 for SHA2_224 for to_string()
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    /// assert_eq!(hash.to_string(), "979DB3C5F63C2FBB32A72804A991534EB38884EB5B9131AE0EE3A496");
    /// ```
    ///
    /// # Example 4 for SHA2_224_Expanded for to_string()
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    /// assert_eq!(my_hash.to_string(), "136C899347821BCC7529F3B42C0A9E3E997E156B1E5E081F57BBB15E");
    /// ```
    ///
    /// # Example 5 for SHA2_256 for the macro println!()
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let mut hash = SHA2_256::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "B8338443431AB13309330A8064AF039E39F90CAC334CF8EA1FF0640646AB121C");
    /// ```
    ///
    /// # Example 6 for SHA2_256_Expanded for the macro println!()
    /// ```
    /// use cryptocol::hash::SHA2_256_Expanded;
    /// type MySHA2 = SHA2_256_Expanded<0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "EF27B2954B124469ACD614F1DE4E99B30C418194B614EE19361674F64F60189C");
    /// ```
    ///
    /// # Example 7 for SHA2_224 for the macro println!()
    /// ```
    /// use cryptocol::hash::SHA2_224;
    /// let mut hash = SHA2_224::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "E333EE19A56FCDCCB05957F2B6FB0AD1EA11D7B6258DF28DCE3B526B");
    /// ```
    ///
    /// # Example 8 for SHA2_224_Expanded for the macro println!()
    /// ```
    /// use cryptocol::hash::SHA2_224_Expanded;
    /// type MySHA2 = SHA2_224_Expanded<128>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "849F654BAFF41D3025DE982EC410F8EC6991FFD6E5BF4047F45082F6");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        let mut me = self.clone();
        write!(f, "{}", me.get_hash_value_in_string())
    }
}
