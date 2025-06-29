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
/// - Cryptanalysis Research to find the weakness of SHA-3 and Keccak
///   construction which SHA3 family uses
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
///   So, the default value is `24` for SHA3, SHAKE, and cSHAKE.
/// - LFSR : The parameter `LFSR` is the 8-bit linear feedback shift register.
///   It is expressed in the form of polynormial such as x^8 + x^6 + x^5 + x^4
///   + 1 (= x^0). The highest order term x^8 indicates 8-bit register. The
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
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct Keccak_Generic<const RATE: usize = 72, const PADDING: usize = 0,
        const ROUNDS: usize = 24, T = u64, const LFSR: u8 = 0b_0111_0001,
        const THETA_SUB: usize = 1, const THETA_ADD: usize = 1, const THETA_ROT: u32 = 1,
        const RHO_MUL_X: usize = 2, const RHO_MUL_Y: usize = 3, const RHO_T: u32 = 24,
        const PI_MUL_X: usize = 1, const PI_MUL_Y: usize = 3,
        const CHI_ADD_1: usize = 1, const CHI_ADD_2: usize = 2>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + Shl<Output = T>
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
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + Shl<Output = T>
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
    const OUPUT_LENGTH: usize = if (PADDING == KECCAK_CONST::SHAKE) || (PADDING == KECCAK_CONST::CSHAKE)
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

    // pub fn digest(&mut self, message: *const u8, length_in_bytes: u64)
    /// Computes hash value.
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
    /// # Arguments
    /// - `message` is pointer to const u8.
    /// - `length_in_bytes` is the size of message in the unit of bytes, and
    ///   its data type is `u64`.
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
    /// Computes hash value.
    /// 
    /// # Features
    /// - This function has the generalized interface (pointer, `*const u8`)
    ///   so as to enable other functions to wrap this function with any
    ///   convenient interface for uses. So, this function is usually not called
    ///   directly in Rust. This function is provided to be called from other
    ///   programming languages such as C/C++.
    /// - This method is the wrapper of the method `absorb_customized()`.
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
    /// Computes hash value.
    /// 
    /// # Features
    /// This method is the wrapper of the method `absorb_str_customized()`
    /// .
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
    /// Computes hash value.
    /// 
    /// # Features
    /// This method is the wrapper of the method `absorb_string_customized()`
    /// .
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
    /// Computes hash value.
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
    /// Computes hash value.
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
    /// Returns a hash value in the form of Vec<u8> object.
    /// 
    /// # Output
    /// A hash value in the form of Vec<u8> object.
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
        out.resize(Self::OUPUT_LENGTH, 0);
        self.get_hash_value(out.as_mut_ptr(), Self::OUPUT_LENGTH);
        out
    }

    // pub fn get_hash_code_in_vec<const N: usize>(&mut self) -> Vec<u8>
    /// Returns a hash value in the form of Vec<u8> object with the length
    /// indicated by generic parameter.
    /// 
    /// # Output
    /// A hash value in the form of Vec<u8> object.
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
        self.get_hash_code_in_string(Self::OUPUT_LENGTH)
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
    /// # Example 1
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
    /// Returns the resulting hash value.
    /// 
    /// # Output
    /// A hash value in the form of array object of `u8` with `RATE` elements.
    /// 
    /// # Features
    /// The length of output hash value is automatically determined
    /// by the generic parameter `RATE`.
    /// 
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

    pub fn absorb_str(&mut self, message: &str)
    {
        self.absorb(message.as_ptr(), message.len() as u64);
    }

    #[inline]
    pub fn absorb_string(&mut self, message: &String)
    {
        self.absorb(message.as_ptr(), message.len() as u64);
    }

    #[inline]
    pub fn absorb_array<U, const N: usize>(&mut self, message: &[U; N])
    where U: SmallUInt + Copy + Clone
    {
        self.absorb(message.as_ptr() as *const u8, (N as u32 * U::size_in_bytes()) as u64);
    }

    #[inline]
    pub fn absorb_vec<U>(&mut self, message: &Vec<U>)
    where U: SmallUInt + Copy + Clone
    {
        self.absorb(message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64);
    }

    // pub fn absorb(&mut self, message: *const u8, length_in_bytes: usize)
    /// Digests the message.
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

    // pub fn absorb_str_customized(&mut self, function_name: &str, user_defined: &str, message: &str)
    /// Digests the message with filename string and user-defined string.
    #[inline]
    pub fn absorb_str_customized(&mut self, function_name: &str, user_defined: &str, message: &str)
    {
        self.absorb_customized(function_name.as_ptr(), function_name.len() as u64,
                                user_defined.as_ptr(), user_defined.len() as u64,
                                message.as_ptr(), message.len() as u64);
    }

    // pub fn absorb_string_customized(&mut self, function_name: &String, user_defined: &String, message: &String)
    /// Digests the message with filename string and user-defined string.
    #[inline]
    pub fn absorb_string_customized(&mut self, function_name: &String, user_defined: &String, message: &String)
    {
        self.absorb_customized(function_name.as_ptr(), function_name.len() as u64,
                                user_defined.as_ptr(), user_defined.len() as u64,
                                message.as_ptr(), message.len() as u64);
    }

    // pub fn absorb_array_customized<U, const K: usize, const L: usize, const M: usize>(&mut self, function_name: &[U; K], user_defined: &[U; L], message: &[U; M])
    /// Digests the message with filename string and user-defined string.
    #[inline]
    pub fn absorb_array_customized<U, V, W, const L: usize, const M: usize, const N: usize>(&mut self, function_name: &[U; L], user_defined: &[V; M], message: &[W; N])
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone, W: SmallUInt + Copy + Clone
    {
        self.absorb_customized(function_name.as_ptr() as *const u8, L as u64 * U::size_in_bytes() as u64,
                                user_defined.as_ptr() as *const u8, M as u64 * V::size_in_bytes() as u64,
                                message.as_ptr() as *const u8, N as u64 * W::size_in_bytes() as u64);
    }

    // pub fn absorb_vec_customized<U>(&mut self, function_name: &Vec<U>, user_defined: &Vec<U>, message: &Vec<U>)
    /// Digests the message with filename string and user-defined string.
    #[inline]
    pub fn absorb_vec_customized<U, V, W>(&mut self, function_name: &Vec<U>, user_defined: &Vec<V>, message: &Vec<W>)
    where U: SmallUInt + Copy + Clone, V: SmallUInt + Copy + Clone, W: SmallUInt + Copy + Clone
    {
        self.absorb_customized(function_name.as_ptr() as *const u8, function_name.len() as u64 * U::size_in_bytes() as u64,
                                user_defined.as_ptr() as *const u8, user_defined.len() as u64 * V::size_in_bytes() as u64,
                                message.as_ptr() as *const u8, message.len() as u64 * W::size_in_bytes() as u64);
    }

    // pub fn absorb_customized(&mut self, filename: *const u8, filename_length_in_bytes: u64, user: *const u8, user_length_in_bytes: u64, message: *const u8, length_in_bytes: u64)
    /// Digests the message with filename string and user-defined string.
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

    // pub fn tangle(&mut self, tangling: u64)
    /// Tangles the hash value
    /// 
    /// # Arguments
    /// u64 constants to tangle the hash value
    /// 
    /// # Features
    /// It is for using this struct as random number generator.
    /// 
    /// # Example 1 for SHA2_512
    /// ```
    /// use cryptocol::hash::SHA2_512;
    /// let mut hash = SHA2_512::new();
    /// let txt = "TANGLING";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:016X?}", txt, hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array()), "[070B6A9457F65DD9, A7D2C2326CE14E8A, E870D6939FE02E39, 5CFEEDCA96BF3BA3, 013FFB332B3F51F3, B1D4E16355DBE0A9, E998240787066535, 1D5F597F04F84820]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:016X?}", hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array()), "[4780AEEAD19D5962, C55EAFBA7590FB70, CA6587899B2B276F, 55361EC5C9568667, FFD38C58FF62C288, 5E96A9FFC6B17704, 6D3885C75FE9B667, BFDA80D1514F38E5]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:016X?}", hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:016X?}", hash.get_hash_value_in_array()), "[D7FFE2BEEB81D532, EA420969761C4DAA, 8EE930740ABBBE3E, 0DC90C0705AE5F38, E91531243615F994, 174C4F96168FBFC4, 06373FFDD9C66A16, 910560A5898E3728]");
    /// ```
    /// 
    /// # Example 2 for SHA2_512_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_Expanded;
    /// type MySHA2 = SHA2_512_Expanded<0x1111_1111_1111_1111, 0x3333_3333_3333_3333, 0x5555_5555_5555_5555, 0x7777_7777_7777_7777, 0x9999_9999_9999_9999, 0xbbbb_bbbb_bbbb_bbbb, 0xdddd_dddd_dddd_dddd, 0xffff_ffff_ffff_ffff, 160>;
    /// let txt = "TANGLING";
    /// ```
    /// 
    /// # Example 3 for SHA2_384
    /// ```
    /// use cryptocol::hash::SHA2_384;
    /// let mut hash = SHA2_384::new();
    /// let txt = "TANGLING";
    /// ```
    /// 
    /// # Example 4 for SHA2_384_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_384_Expanded;
    /// type MySHA2 = SHA2_384_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "TANGLING";
    /// ```
    /// 
    /// # Example 5 for SHA2_512_256
    /// ```
    /// use cryptocol::hash::SHA2_512_256;
    /// let mut hash = SHA2_512_256::new();
    /// let txt = "TANGLING";
    /// hash.digest_str(txt);
    /// ```
    /// 
    /// # Example 6 for SHA2_512_256_Expanded
    /// ```
    /// use cryptocol::hash::SHA2_512_256_Expanded;
    /// type MySHA2 = SHA2_512_256_Expanded<160>;
    /// let mut my_hash = MySHA2::new();
    /// let txt = "TANGLING";
    /// ```
    #[inline]
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
    /// Returns the desiable `L` according to the size of `T`.
    #[inline]
    pub fn get_desirable_l() -> usize
    {
        T::BITS.trailing_zeros() as usize
    }

    // pub fn get_desirable_rounds() -> usize
    /// Returns the desiable number of rounds according to the size of `T`.
    #[inline]
    pub fn get_desirable_rounds() -> usize
    {
        12 + 2 * Self::get_desirable_l() as usize
    }

    // pub fn get_desirable_b() -> usize
    /// Returns the desiable `B` according to the size of `T`. 
    /// The desiable `B` is expressed not in bits but in bytes here.
    #[inline]
    pub fn get_desirable_b() -> usize
    {
        25 * (1 << (Self::get_desirable_l() - 3))
    }

    // pub fn get_desirable_output_length() -> usize
    /// Returns the desiable `OUTPUT_LENGTH` of specific algorithm.
    /// The desiable `OUTPUT_LENGTH` is expressed not in bits but in bytes here.
    #[inline]
    pub fn get_desirable_output_length() -> usize
    {
        Self::OUPUT_LENGTH
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
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + Shl<Output = T>
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
