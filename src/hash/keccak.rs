// Copyright 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]
// #![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]


use std::ptr::{ copy_nonoverlapping, write_bytes };
use std::ptr::addr_of_mut;
use std::slice::from_raw_parts;
use std::fmt::{ self, Debug, Display, Formatter };
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, 
                Add, AddAssign, Sub, SubAssign, Mul, MulAssign,
                Div, DivAssign, Rem, RemAssign };

use crate::number::{ SmallUInt, IntUnion, LongUnion };


macro_rules! run_register {
    ($STATE:expr, $LFSR:expr) => {
        (($STATE >> 1) | ((($STATE & $LFSR).count_ones() as u8) << 7), $STATE & 1)
    };
}

macro_rules! make_RC {
    ($T:ty, $ROUNDS:expr, $LFSR:expr) => {
        {
            let mut union_A = A::<$T, $ROUNDS> { U128: [0_u128; ROUNDS] };
            let WIDTH = <$T>::BYTES;
            let mut RC = [<$T>::MIN; ROUNDS];
            let mut bit = [0_usize; 7];
            let mut j = 0_usize;
            while j < 7_usize
            {
                bit[j] = ((1_usize << j) - 1) % WIDTH;
                j += 1;
            }
            let mut state = 1_u8;
            let mut output: u8;
            let mut i = 0_usize;
            while i < $ROUNDS
            {
                j = 0_usize;
                while j < 7_usize
                {
                    (state, output) = run_register!(state, $LFSR);
                    if output != 0
                    {
                        unsafe {
                            match WIDTH
                            {
                                16 =>   { union_A.U128[i] |= 1_u128 << bit[j]; },
                                8 =>    { union_A.U64[i] |= 1_u64 << bit[j]; },
                                4 =>    { union_A.U32[i] |= 1_u32 << bit[j]; },
                                2 =>    { union_A.U16[i] |= 1_u16 << bit[j]; },
                                1 =>    { union_A.U8[i] |= 1_u8 << bit[j]; },
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


union A<T, const ROUNDS: usize>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + Shl<Output = T>
{
    #[allow(non_snake_case)] RC:    [T; ROUNDS],
    #[allow(non_snake_case)] U128:  [u128; ROUNDS],
    #[allow(non_snake_case)] U64:   [u64; ROUNDS],
    #[allow(non_snake_case)] U32:   [u32; ROUNDS],
    #[allow(non_snake_case)] U16:   [u16; ROUNDS],
    #[allow(non_snake_case)] U8:    [u8; ROUNDS],
}

// #[allow(non_snake_case)]
// const fn make_RC<T, const ROUNDS: usize, const LFSR: u8>() -> [T; ROUNDS]
// where T: SmallUInt + Copy + Clone + Display + Debug + ToString
//         + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
//         + BitXor<Output=T> + BitXorAssign + Not<Output=T>
//         + Shl<Output = T>
// {
//     let mut union_A = A::<T, ROUNDS> { U128: [0_u128; ROUNDS] };
//     let WIDTH = T::BYTES;
//     let mut RC = [T::MIN; ROUNDS];
//     let mut bit = [0_usize; 7];
//     let mut j = 0_usize;
//     while j < 7_usize
//     {
//         bit[j] = ((1_usize << j) - 1) % WIDTH;
//         j += 1;
//     }
//     let mut state = 1_u8;
//     let mut output: u8;
//     let mut i = 0_usize;
//     while i < ROUNDS
//     {
//         j = 0_usize;
//         while j < 7_usize
//         {
//             (state, output) = run_register!(state, LFSR);
//             if output != 0
//             {
//                 unsafe {
//                     match WIDTH
//                     {
//                         16 =>   { union_A.U128[i] |= 1_u128 << bit[j]; },
//                         8 =>    { union_A.U64[i] |= 1_u64 << bit[j]; },
//                         4 =>    { union_A.U32[i] |= 1_u32 << bit[j]; },
//                         2 =>    { union_A.U16[i] |= 1_u16 << bit[j]; },
//                         1 =>    { union_A.U8[i] |= 1_u8 << bit[j]; },
//                         _ =>    {},
//                     }
//                 }
//             }
//             j += 1;
//         }
//         unsafe { RC[i] = union_A.RC[i]; }
//         i += 1;
//     }
//     RC
// }

// macro_rules! new_Keccak_Generic {
//     ($N:expr, $T:ty,
//         $THETA_LEFT:expr, $THETA_RIGHT:expr, $THETA_RR1:expr,
//         $RHO_NEXT1:expr, $RHO_NEXT2:expr, $ROUND:expr,
//         $RATE:expr, $LFSR:expr) => {
//         match ($T)
//         {
//             (u128) => Keccak_Generic::<$N, $T, $THETA_LEFT$THETA_RIGHT, $THETA_RR1,
//                                 $RHO_NEXT1, $RHO_NEXT2, $ROUND, $RATE, 7,
//                                 $LFSR>::new(),
//             (u64) => Keccak_Generic::<$N, $T, $THETA_LEFT$THETA_RIGHT, $THETA_RR1,
//                                 $RHO_NEXT1, $RHO_NEXT2, $ROUND, $RATE, 6,
//                                 $LFSR>::new(),
//             (u32) => Keccak_Generic::<$N, $T, $THETA_LEFT$THETA_RIGHT, $THETA_RR1,
//                                 $RHO_NEXT1, $RHO_NEXT2, $ROUND, $RATE, 5,
//                                 $LFSR>::new(),
//             (u16) => Keccak_Generic::<$N, $T, $THETA_LEFT$THETA_RIGHT, $THETA_RR1,
//                                 $RHO_NEXT1, $RHO_NEXT2, $ROUND, $RATE, 4,
//                                 $LFSR>::new(),
//             (u8) => Keccak_Generic::<$N, $T, $THETA_LEFT$THETA_RIGHT, $THETA_RR1,
//                                 $RHO_NEXT1, $RHO_NEXT2, $ROUND, $RATE, 3,
//                                 $LFSR>::new(),
//         };
//     }
// }

pub type SHA3 = Keccak_Generic;


/// A Keccak message-digest algorithm that lossily compresses data of arbitrary
/// length into a 128-bit hash value, and its flexible variants that allows
/// you to develop your own `Keccak`-based hash algorithms
/// 
/// # Introduction
/// 
/// # Vulnerability
/// There have been several attacks against Keccak
/// but they are all incomplete attacks.
/// Read [more](https://en.wikipedia.org/wiki/SHA-2#Cryptanalysis_and_validation)
/// 
/// # Use of SHA3 and their variants
/// You can use Keccak and its variants for cryptograpic purposes such as:
/// - Generating IDs
/// - Integrity test
/// - Storing passwords
/// - Digital Signature
/// - Key generation
/// - Implementing proof of work for block chain.
/// - Study of hash algorithms
/// - Cryptanalysis Research to find the weakness of SHA-3 and Keccak
/// construction which SHA3 family uses
/// 
/// # Generic Parameters
/// You can create your own expanded version of Keccak by changing the generic
/// parameters.
/// - T : The parameter `T` is the datatype of the unit block to process. It
///   is one of `u8`, `u16`, `u32`, `u64`, and `u128`.
/// - RATE : The parameter `RATE` is in bytes though it is usually written in
///   bits in most of the document. `RATE` means how many bytes the message
///   will be absorbed at once.
/// 
/// 
/// , where w is the number of bits for one
///   block. If w is 32 for 32-bit width, L is 5. If w is 64 for 64-bit width,
///   L is 6. If w is 128 for 128-bit width, L is 7. L can be up to 7 because
///   Rust support up to 128-bit integer for primitive data type, but normally
///   L is chosen to be 6 for 64-bit width. SHA-3 chose 6 for L.
/// - N : the size of output. N cannot be 0 or greater than 4. If N is 4, the
/// output is 128 bits, while if N is 1, the output is 32 bits.
/// - H0 ~ H3 : the initial hash values, four u32 values.
/// The default values of H0, H1, H2, and H3 are 0x67452301, 0xefcdab89,
/// 0x98badcfe, and 0x10325476, respectively (in little endian representation).
/// - ROUND : the number of rounds. The default value of it is `48` (= 16 * 3).
/// - K0 ~ K2 : the added values in hashing process, three u32 values.
/// The default values of K0, K1, and K2 are 0x00000000, 0x5A827999, and
/// 0x6ED9EBA1, respectively (in little endian representation).
/// 0x5A827999 is a 32-bit constant of the square root of 2 in little endian
/// prepresentation.
/// 0x6ED9EBA1 is a 32-bit constant of the square root of 3 in little endian
/// prepresentation.
/// - R00 ~ R03, R10 ~ R13, and R20 ~ R23 : the amounts of rotate left in the
/// hashing process.
/// The default values of R00, R01, R02, and R03 are 3, 7, 11, and 19, respectively.
/// The default values of R10, R11, R12, and R13 are 3, 5, 9, and 13, respectively.
/// The default values of R20, R11, R12, and R23 are 3, 9, 11, and 15, respecively.
/// 
/// About the parameters and their default values,
/// read [more](https://datatracker.ietf.org/doc/html/rfc1320)
/// and/or watch [this video](https://www.youtube.com/watch?v=JIhZWgJA-9o)
/// to learn SHA-1 more in detail.
/// 
/// # Security of your own expanded version
/// Your own algrorithm based on MD4 may be stronger or weaker than official
/// MD4. Unless you seriously checked the cryptographic security of your own
/// algorithms, it is hard to assume that your own alogrithms are stronger
/// than the official Keccak.
/// 
/// 
/// # Reference
/// Read [more](https://en.wikipedia.org/wiki/MD4) about MD4 in detail.
/// 
/// # Quick Start
/// In order to use the module Keccak, you don't have to import (or use)
/// cryptocol::hash::keccak::* directly because the module cryptocol::hash::keccak
/// is re-exported. All you have to do is only import Keccak, Keccak_Expanded,
/// Keccak_Generic_HR_fixed and/or Keccak_Generic in the module cryptocol::hash.
/// Example 1 shows how to import structs Keccak, Keccak_Expanded,
/// Keccak_Generic_HR_fixed and/or Keccak_Generic. Plus, what you have to know is
/// these. All the types (or structs) are the specific versions of Keccak_Generic.
/// Actually, Keccak is a specific version of Keccak_Expanded. Keccak_Expanded and
/// Keccak_Generic_HR_fixed are specific versions of Keccak_Generic.
/// 
/// ## Example 1
/// ```
/// use cryptocol::hash::Keccak;
/// use cryptocol::hash::Keccak;
/// use cryptocol::hash::Keccak_Generic_HR_fixed;
/// use cryptocol::hash::Keccak_Generic;
/// ```
/// Then, you create Keccak object by the method Keccak::new(). Now, you are ready to
/// use all provided methods to hash any data. If you want to hash a string,
/// for example, you can use the method absorb_str(). Then, the Keccak object that
/// you created will contain its hash value. You can use the macro println!(),
/// for instance, to print on a commandline screen by `println!("{}", hash)`
/// where hash is the Keccak object. Example 2 shows how to use MD4 struct quickly.
/// 
/// ## Example 2
/// ```
/// ```
/// 
/// # Big-endian issue
/// It is just experimental for Big Endian CPUs. So, you are not encouraged
/// to use it for Big Endian CPUs for serious purpose. Only use this crate
/// for Big-endian CPUs with your own full responsibility.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct Keccak_Generic<const N: usize = 8, T = u64, const ROUNDS: usize = 24,
        const RATE: usize = 136, const LFSR: u8 = 0b_0111_0001,
        const THETA_LEFT: usize = 1, const THETA_RIGHT: usize = 1, const THETA_RR1: u32 = 1,
        const RHO_NEXT1: usize = 1, const RHO_NEXT2: usize = 2,
        const RHO_RC_00: u8 = 00, const RHO_RC_01: u8 = 36, const RHO_RC_02: u8 = 03,
        const RHO_RC_03: u8 = 41, const RHO_RC_04: u8 = 18,
        const RHO_RC_10: u8 = 01, const RHO_RC_11: u8 = 44, const RHO_RC_12: u8 = 10,
        const RHO_RC_13: u8 = 45, const RHO_RC_14: u8 = 02,
        const RHO_RC_20: u8 = 62, const RHO_RC_21: u8 = 06, const RHO_RC_22: u8 = 43,
        const RHO_RC_23: u8 = 15, const RHO_RC_24: u8 = 61,
        const RHO_RC_30: u8 = 28, const RHO_RC_31: u8 = 55, const RHO_RC_32: u8 = 25,
        const RHO_RC_33: u8 = 21, const RHO_RC_34: u8 = 56,
        const RHO_RC_40: u8 = 27, const RHO_RC_41: u8 = 20, const RHO_RC_42: u8 = 39,
        const RHO_RC_43: u8 = 08, const RHO_RC_44: u8 = 14
        >
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + Shl<Output = T>
{
    state: [[T; 5]; 5],
}

impl<const N: usize, T, const ROUNDS: usize, const RATE: usize, const LFSR: u8,
        const THETA_LEFT: usize, const THETA_RIGHT: usize, const THETA_RR1: u32,
        const RHO_NEXT1: usize, const RHO_NEXT2: usize,
        const RHO_RC_00: u8, const RHO_RC_01: u8, const RHO_RC_02: u8,
        const RHO_RC_03: u8, const RHO_RC_04: u8,
        const RHO_RC_10: u8, const RHO_RC_11: u8, const RHO_RC_12: u8,
        const RHO_RC_13: u8, const RHO_RC_14: u8,
        const RHO_RC_20: u8, const RHO_RC_21: u8, const RHO_RC_22: u8,
        const RHO_RC_23: u8, const RHO_RC_24: u8,
        const RHO_RC_30: u8, const RHO_RC_31: u8, const RHO_RC_32: u8,
        const RHO_RC_33: u8, const RHO_RC_34: u8,
        const RHO_RC_40: u8, const RHO_RC_41: u8, const RHO_RC_42: u8,
        const RHO_RC_43: u8, const RHO_RC_44: u8
    >
Keccak_Generic<N, T, ROUNDS, RATE, LFSR,
                THETA_LEFT, THETA_RIGHT, THETA_RR1,
                RHO_NEXT1, RHO_NEXT2,
                RHO_RC_00, RHO_RC_01, RHO_RC_02, RHO_RC_03, RHO_RC_04,
                RHO_RC_10, RHO_RC_11, RHO_RC_12, RHO_RC_13, RHO_RC_14,
                RHO_RC_20, RHO_RC_21, RHO_RC_22, RHO_RC_23, RHO_RC_24,
                RHO_RC_30, RHO_RC_31, RHO_RC_32, RHO_RC_33, RHO_RC_34,
                RHO_RC_40, RHO_RC_41, RHO_RC_42, RHO_RC_43, RHO_RC_44
                >
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + Shl<Output = T> 
{
    const RC: [T; ROUNDS] = make_RC!(T, ROUNDS, LFSR);
    // make_RC::<T, ROUNDS, LFSR>();
                // [ 0x00000000000000000000000000000001, 0x00000000000000000000000000008082,
                //   0x8000000000000000000000000000808A, 0x80000000000000000000000080008000,
                //   0x0000000000000000000000000000808B, 0x00000000000000000000000080000001, 
                //   0x80000000000000000000000080008081, 0x80000000000000000000000000008009,
                //   0x0000000000000000000000000000008A, 0x00000000000000000000000000000088,
                //   0x00000000000000000000000080008009, 0x0000000000000000000000008000000A, 
                //   0x0000000000000000000000008000808B, 0x8000000000000000000000000000008B,
                //   0x80000000000000000000000000008089, 0x80000000000000000000000000008003,
                //   0x80000000000000000000000000008002, 0x80000000000000000000000000000080, 
                //   0x0000000000000000000000000000800A, 0x8000000000000000000000008000000A,
                //   0x80000000000000000000000080008081, 0x80000000000000000000000000008080,
                //   0x00000000000000000000000080000001, 0x80000000000000000000000080008008,
                //   0x00000000000000000000000080008000, 0x8000000000000000000000000000800a  ];
    
    // const RC: [u64; 24] = [ 0x0000000000000001, 0x0000000000008082, 0x800000000000808A, 
    //                         0x8000000080008000, 0x000000000000808B, 0x0000000080000001, 
    //                         0x8000000080008081, 0x8000000000008009, 0x000000000000008A, 
    //                         0x0000000000000088, 0x0000000080008009, 0x000000008000000A, 
    //                         0x000000008000808B, 0x800000000000008B, 0x8000000000008089, 
    //                         0x8000000000008003, 0x8000000000008002, 0x8000000000000080, 
    //                         0x000000000000800A, 0x800000008000000A, 0x8000000080008081, 
    //                         0x8000000000008080, 0x0000000080000001, 0x8000000080008008 ];

    const R: [[u8; 5]; 5] = [
            [ RHO_RC_00, RHO_RC_01, RHO_RC_02, RHO_RC_03, RHO_RC_04 ],
            [ RHO_RC_10, RHO_RC_11, RHO_RC_12, RHO_RC_13, RHO_RC_14 ],
            [ RHO_RC_20, RHO_RC_21, RHO_RC_22, RHO_RC_23, RHO_RC_24 ],
            [ RHO_RC_30, RHO_RC_31, RHO_RC_32, RHO_RC_33, RHO_RC_34 ],
            [ RHO_RC_40, RHO_RC_41, RHO_RC_42, RHO_RC_43, RHO_RC_44 ]
    ];


    // pub fn new() -> Self
    /// Creates the new object of `Self`.
    #[inline]
    pub fn new() -> Self
    {
        Self { state: [[T::zero(); 5]; 5] }
    }

    // pub fn get_desirable_l() -> usize
    /// Returns the desiable `L` according to the size of `T`.
    #[inline]
    pub fn get_desirable_l() -> usize
    {
        T::size_in_bytes().trailing_zeros() as usize + 3
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

    // pub fn initialize_state(&mut self)
    /// Initialize state array to be all zeros.
    pub fn initialize_state(&mut self)
    {
        unsafe { write_bytes(self.state.as_mut_ptr(), 0, 5 * 5); }
        // for y in 0..5
        // {
        //     for x in 0..5
        //         { self.state[x][y] = T::zero(); }
        // }
    }

    pub fn absorb(&mut self, message: *const u8, length_in_bytes: isize)
    {
        let shift_num = T::size_in_bits().ilog(2);
        const CHUNK_NUM: usize = 16;
        self.initialize_state();
        let length_done = (length_in_bytes >> shift_num) as usize;
        // for i in 0..length_done
        //     { self.update(unsafe { from_raw_parts(message.add(i << shift_num) as *const T, 5 * 5) } ); }
        // self.finalize(unsafe { message.add(length_done << shift_num) }, length_in_bytes);

    }

    // fn feed_message_to_state(&mut self, message: *const u8)
    /// The message will be absorbed `RATE` bytes by `RATE` bytes.
    #[inline]
    fn feed_message_to_state(&mut self, message: *const u8)
    {
        self.initialize_state();
        let mut state = [[T::zero(); 5]; 5];
        unsafe { copy_nonoverlapping(message, state.as_mut_ptr() as *mut u8, RATE); }
        let limit_y = RATE / 5;
        for y in 0..limit_y
        {
            for x in 0..5_usize
                { self.state[y][x] ^= state[y][x]; }
        }
        let limit_x = RATE % 5;
        for x in 0..limit_x
            { self.state[limit_y][x] ^= state[limit_y][x]; }
    }

    fn convert_state_to_message(&mut self) -> Vec<T>
    {
        let mut message = Vec::<T>::new();
        for y in 0..5
        {
            for x in 0..5
                { message.push(self.state[x][y]); }
        }
        message
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
            d[i] = c[i.modular_sub(THETA_LEFT, 5)]
                 ^ c[i.modular_add(THETA_RIGHT, 5)].rotate_right(THETA_RR1);
        }

        for y in 0..5_usize
        {
            for x in 0..5_usize
                { self.state[y][x] ^= d[x]; }
        }
    }

    fn rho_pi_chi(&mut self)
    {
        let mut array= [[T::zero(); 5]; 5];
        for y in 0..5_usize
        {
            for x in 0..5_usize
            {
                let x2 = x.modular_mul(2, 5);
                let y3 = y.modular_mul(3, 5);
                let row = x2.modular_add(y3, 5);
                // pi step                      // rho step
                array[row][y] = self.state[y][x].rotate_right(Self::R[x][y] as u32);
            }
        }

        // chi step
        for y in 0..5_usize
        {
            for x in 0..5_usize
            {
                let a = array[y][x];
                let b = !array[y][x.modular_add(1, 5)];
                let c = array[y][x.modular_add(2, 5)];
                self.state[y][x] = a ^ (b & c);
            }
        }
    }

    #[inline]
    fn iota(&mut self, round: usize)
    {
        self.state[0][0] ^= Self::RC[round];
    }


    #[inline] fn get_RC(&mut self, idx: usize) -> T     { Self::RC[idx] }
}