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

use std::ptr::{ copy_nonoverlapping, copy };
use crate::number::{ IntUnion, LongUnion, LongerUnion };


// macro_rules! rijndael_pre_encrypt_into_vec {
//     ($to:expr, $length_in_bytes:expr, $type:ty) => {
//         let len = ($length_in_bytes + 1).next_multiple_of(4 * NB as u64) as usize / <$type>::size_in_bytes() as usize;
//         $to.truncate(len);
//         $to.resize(len + 1, <$type>::zero());
//     };
// }
// pub(super) use rijndael_pre_encrypt_into_vec;


// macro_rules! GF_rot_left {
//     ($r:expr, $n:expr) => {
//         $r.rotate_left_assign($n);
//     };
// }

macro_rules! GF_mul {
    ($a:expr, $b:expr) => {
        {
            let mut a = $a;
            let mut b = $b;
            let mut ret = 0_u8;
            while b != 0
            {
                if (b & 1 == 1)
                    { ret ^= a; }
                if (a & 0b1000_0000 != 0)
                    { a = (a << 1) ^ (IRREDUCIBLE as u8); }
                else
                    { a <<= 1; }
                b >>= 1;
            }
            ret
        }
    };
}

macro_rules! GF_is_candidate {
    ($a:expr) => {
        {
            let a_1 = $a;
            let a_2 = GF_mul!(a_1, a_1);
            let a_4 = GF_mul!(a_2, a_2);
            let a_8 = GF_mul!(a_4, a_4);
            let a_16 = GF_mul!(a_8, a_8);
            let a_32 = GF_mul!(a_16, a_16);
            let a_64 = GF_mul!(a_32, a_32);

            let a_3 = GF_mul!(a_1, a_2);
            let mut ret = GF_mul!(a_3, a_4);
            ret = GF_mul!(ret, a_8);
            if ret == 1
            {
                false
            }
            else
            {
                ret = GF_mul!(a_3, a_16);
                ret = GF_mul!(ret, a_32);
                if ret == 1
                {
                    false
                }
                else
                {
                    ret = GF_mul!(a_1, a_4);
                    ret = GF_mul!(ret, a_16);
                    ret = GF_mul!(ret, a_32);
                    ret != 1
                }
            }
        }
    };
}

macro_rules! GF_generator {
    () => {
        {
            let mut ret = 0_u8;
            let mut j = 2_u16;
            while j < 256
            {
                if GF_is_candidate!(j as u8)
                {
                    ret = j as u8;
                    break;
                }
                j += 1;
            }
            ret
        }
    };
}

macro_rules! GF_inverse_of {
    ($a:expr) => {
        {
            let mut ret = 0_u8;
            let mut inv = 255_u8;
            while inv > 1
            {
                if GF_mul!($a, inv) == 1
                {
                    ret = inv;
                    break;
                }
                inv -= 1;
            }
            ret
        }
    }
}

macro_rules! make_SBOX {
    () => {
        {
            let mut out = [0_u8; 256];
            let mut p = 1_u8;
            let mut q = 1_u8;
            let generator = GF_generator!();
            let inverse = GF_inverse_of!(generator);
            let mut j = 255_u16;
            while j > 0
            {
                p = GF_mul!(p, generator);
                q = GF_mul!(q, inverse);
                let mut transformed = 0_u8;
                let mut i = 0_u8;
                while i < 8
                {
                    let affine_u8 = (AFFINE_MUL >> (i << 3)) as u8;
                    let bits = affine_u8 & q;
                    if bits.count_ones() & 1 == 1
                        { transformed |= 1 << i; }
                    i += 1;
                }
                out[p as usize] = transformed ^ AFFINE_ADD;
                if p == 1
                    { break; }
                j -= 1;
            }
            out[0] = AFFINE_ADD;
            out
        }
    };
}

macro_rules! make_INV_SBOX {
    () => {
        {
            let mut out = [0_u8; 256];
            let mut i = 0_u16;
            while i < 256
            {
                out[Self::SBOX[i as usize] as usize] = i as u8;
                i += 1;
            }
            out
        }
    }
}

macro_rules! make_INV_MC {
    () => {
        {
            let mut aug = [[0_u8; 4]; 4];
            let mut inv = [[0_u8; 4]; 4];
            let mut i = 0_usize;
            let mut j: usize;
            while i < 4
            {
                j = 0;
                while j < 4
                {
                    aug[i][j] = Self::MC[i][j];
                    j += 1;
                }
                inv[i][i] = 1;
                i += 1;
            }

            let generator = GF_generator!();
            let inverse = GF_inverse_of!(generator);
            let mut row = 0_usize;
            while row < 4
            {
                let mut pivot = aug[row][row];
                if pivot == 0
                {
                    i = row + 1;
                    while i < 4
                    {
                        if aug[i][row] != 0
                        {
                            j = 0;
                            while j < 4
                            {
                                (aug[row][j], aug[i][j]) = (aug[i][j], aug[row][j]);
                                (inv[row][j], inv[i][j]) = (inv[i][j], inv[row][j]);
                                j += 1;
                            }
                            pivot = aug[row][row];
                            break;
                        }
                        i += 1;
                    }
                    if pivot == 0
                        { panic!("Singular Matrix"); }
                }

                let mut p = 1_u8;
                let mut q = 1_u8;
                while p != pivot
                {
                    p = GF_mul!(p, generator);
                    q = GF_mul!(q, inverse);
                }
                j = 0;
                while j < 4
                {
                    aug[row][j] = GF_mul!(aug[row][j], q);
                    inv[row][j] = GF_mul!(inv[row][j], q);
                    j += 1;
                }
                let mut r = 0;
                while r < 4
                {
                    if r != row
                    {
                        let factor = aug[r][row];
                        j = 0;
                        while j < 4
                        {
                            inv[r][j] ^= GF_mul!(inv[row][j], factor);
                            aug[r][j] ^= GF_mul!(aug[row][j], factor);
                            j += 1;
                        }
                    }
                    r += 1;
                }
                row += 1;
            }
            inv
        }
    }
}

macro_rules! make_RC_ARRAY {
    () => {
        {
            let mut out = [0u32; ROUND];
            let round = [ RC0.to_le(), RC1.to_le(), RC2.to_le(), RC3.to_le(), RC4.to_le(),
                          RC5.to_le(), RC6.to_le(), RC7.to_le(), RC8.to_le(), RC9.to_le() ];
            let nr = if ROUND <= 10 { ROUND } else { 10 };
            let mut i = 0;
            while i < nr
            {
                out[i] = round[i];
                i += 1;
            }
            while i < ROUND
            {
                out[i] = (GF_mul!(out[i-1].to_le() as u8, 2) as u32).to_le();
                i += 1;
            }
            out
        }
    }
}


/// Rijndael_32_32 is not really practical but only educational to understand
/// AES or Rijndael cryptographic algorithm. Its key is 32-bit and its
/// encryption block is 32-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_32_32 = Rijndael_Generic<7, 1, 1>;

/// Rijndael_64_64 is not really practical too, but it can be used with DES
/// cryptographic algorithm along. Its key is 64-bit and its encryption block
/// is 64-bit. So, it can work with DES.
#[allow(non_camel_case_types)]
pub type Rijndael_64_64 = Rijndael_Generic<8, 2, 2>;

/// Rijndael_512_512 can be used as one of post-Quantum algorithms for a
/// while because even Grover algorithm takes long enough to break
/// Rijndael_512_512. Its key is 512-bit and its encryption block is 512-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_512_512 = Rijndael_Generic<22, 16, 16>;

/// Rijndael_512_384 is one of Rijndael series. Its key is 384-bit and its
/// encryption block is 512-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_512_384 = Rijndael_Generic<18, 16, 12>;

/// Rijndael_384_256 is one of Rijndael series. Its key is 256-bit and its
/// encryption block is 512-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_512_256 = Rijndael_Generic<14, 16, 8>;

/// Rijndael_384_192 is one of Rijndael series. Its key is 192-bit and its
/// encryption block is 512-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_512_192 = Rijndael_Generic<14, 16, 6>;

/// Rijndael_512_128 is one of Rijndael series. Its key is 128-bit and its
/// encryption block is 512-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_512_128 = Rijndael_Generic<14, 16, 4>;

/// Rijndael_384_512 can be used as one of post-Quantum algorithms for a
/// while because even Grover algorithm takes long enough to break
/// Rijndael_384_512. Its key is 512-bit and its encryption block is 384-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_384_512 = Rijndael_Generic<22, 12, 16>;

/// Rijndael_384_384 is one of Rijndael series. Its key is 384-bit and its
/// encryption block is 384-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_384_384 = Rijndael_Generic<18, 12, 12>;

/// Rijndael_384_256 is one of Rijndael series. Its key is 256-bit and its
/// encryption block is 384-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_384_256 = Rijndael_Generic<14, 12, 8>;

/// Rijndael_384_192 is one of Rijndael series. Its key is 192-bit and its
/// encryption block is 384-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_384_192 = Rijndael_Generic<14, 12, 6>;

/// Rijndael_384_128 is one of Rijndael series. Its key is 128-bit and its
/// encryption block is 384-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_384_128 = Rijndael_Generic<14, 12, 4>;

/// Rijndael_256_512 can be used as one of post-Quantum algorithms for a
/// while because even Grover algorithm takes long enough to break
/// Rijndael_256_512. Its key is 512-bit and its encryption block is 256-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_256_512 = Rijndael_Generic<22, 8, 16>;

/// Rijndael_256_384 is one of Rijndael series. Its key is 384-bit and its
/// encryption block is 256-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_256_384 = Rijndael_Generic<18, 8, 12>;

/// Rijndael_256_256 is one of Rijndael series. Its key is 256-bit and its
/// encryption block is 256-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_256_256 = Rijndael_Generic<14, 8, 8>;

/// Rijndael_256_192 is one of Rijndael series. Its key is 192-bit and its
/// encryption block is 256-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_256_192 = Rijndael_Generic<14, 8, 6>;

/// Rijndael_256_128 is one of Rijndael series. Its key is 128-bit and its
/// encryption block is 256-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_256_128 = Rijndael_Generic<14, 8, 4>;

/// Rijndael_192_512 can be used as one of post-Quantum algorithms for a
/// while because even Grover algorithm takes long enough to break
/// Rijndael_192_512. Its key is 512-bit and its encryption block is 192-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_192_512 = Rijndael_Generic<22, 6, 16>;

/// Rijndael_192_384 is one of Rijndael series. Its key is 384-bit and its
/// encryption block is 192-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_192_384 = Rijndael_Generic<18, 6, 12>;

/// Rijndael_192_256 is one of Rijndael series. Its key is 256-bit and its
/// encryption block is 192-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_192_256 = Rijndael_Generic<14, 6, 8>;

/// Rijndael_192_192 is one of Rijndael series. Its key is 192-bit and its
/// encryption block is 192-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_192_192 = Rijndael_Generic<12, 6, 6>;

/// Rijndael_256_128 is one of Rijndael series. Its key is 128-bit and its
/// encryption block is 192-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_192_128 = Rijndael_Generic<12, 6, 4>;

/// Rijndael_128_512 can be used as one of post-Quantum algorithms for a
/// while because even Grover algorithm takes long enough to break
/// Rijndael_128_512. Its key is 512-bit and its encryption block is 128-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_128_512 = Rijndael_Generic<22, 4, 16>;

/// Rijndael_128_384 is one of Rijndael series. Its key is 384-bit and its
/// encryption block is 128-bit.
#[allow(non_camel_case_types)]
pub type Rijndael_128_384 = Rijndael_Generic<18, 4, 12>;

/// Rijndael_128_256 is one of Rijndael series. Its key is 256-bit and its
/// encryption block is 128-bit. It is the same as the AES_256.
#[allow(non_camel_case_types)]
pub type Rijndael_128_256 = Rijndael_Generic<14, 4, 8>;

/// Rijndael_128_192 is one of Rijndael series. Its key is 192-bit and its
/// encryption block is 128-bit. It is the same as the AES_192.
#[allow(non_camel_case_types)]
pub type Rijndael_128_192 = Rijndael_Generic<12, 4, 6>;

/// Rijndael_128_128 is one of Rijndael series. Its key is 128-bit and its
/// encryption block is 128-bit. It is the same as the AES_128.
#[allow(non_camel_case_types)]
pub type Rijndael_128_128 = Rijndael_Generic;

/// AES_256 is one of AES series. Its key is 256-bit and its encryption
/// block is 128-bit. It is the same as the Rijndael_128_256.
#[allow(non_camel_case_types)]
pub type AES_256 = Rijndael_Generic<14, 4, 8>;

/// AES_192 is one of AES series. Its key is 192-bit and its encryption
/// block is 128-bit. It is the same as the Rijndael_128_192.
#[allow(non_camel_case_types)]
pub type AES_192 = Rijndael_Generic<12, 4, 6>;

/// AES_128 is one of AES series. Its key is 128-bit and its encryption
/// block is 128-bit. It is the same as the Rijndael_128_128.
#[allow(non_camel_case_types)]
pub type AES_128 = Rijndael_Generic;


/// A Rijndael or AES (Advanced Encryption Standard) symmetric-key algorithm
/// for the encryption of digital data
/// 
/// # Note
/// ** This descryption about Rijndael is according to little endianness. **
/// MSB (Most Significant Bit) is the 64th bit and LSB (Least Significant Bit)
/// is the first bit in this descryption unless otherwise mentioned.
/// 
/// # Introduction
/// The name, AES, is the acronym of Advanced Encryption Standard. The name,
/// Rijndael, is created out of the names of the Belgian cryptographers, Joan
/// Daemen and Vincent Rijmen. Rijndael means Rhine valley in Dutch though it is
/// not known whether the name Rijndael was made with that intention. AES is the
/// subset of Rijndael. AES has three versions: AES-128, AES-192 and AES-256.
/// All of them have the 128-bit data block. AES-128 has the 128-bit key length.
/// AES-192 has the 192-bit key length. AES-256 has the 256-bit key length.
/// On the other hand, Rijndael has nine versions: Rijndael-128-128,
/// Rijndael-128-192, Rijndael-128-256, Rijndael-192-128, Rijndael-192-192,
/// Rijndael-192-256, Rijndael-256-128, Rijndael-256-192, and Rijndael-256-256.
/// Rijndael-128-*, Rijndael-192-*, and Rijndael-256-* have 128-bit data block,
/// 192-bit data block, and 256-bit data block, respectively. Rijndael-*-128,
/// Rijndael-*-192, and Rijndael-*-256 have 128-bit key length, 192-bit key
/// length, and 256-bit key length, respectively. However, this module,
/// Rijndael_Generic provides more expanded versions such as 32-bit and 64-bit
/// data blocks and key lengths and more than 256-bit data blocks and key
/// lengths.
/// 
/// # Short History of birth of AES
/// On 2nd, January, 1997, NIST announced that they wished to choose the
/// encryption algorithm standard in replacement of DES, which is called AES.   
/// The contest held three rounds. At the first round, fifteen were submitted:
/// CAST-256, CRYPTON, DEAL, DFC, E2, FROG, HPC, LOKI97, MAGENTA, MARS, RC6,
/// Rijndael, SAFER+, Serpent, and Twofish. At the second round, five remained:
/// MARS, RC6, Rijndael, Serpent, and Twofish. At the third round, Rijndael was
/// chosen out of the remained five. However, all the five algorithms are
/// commonly referred to as "AES finalists", and they are also considered
/// well-known and respected in the community. So, the other four algorithms
/// are also being actively used as well as Rijndael, today.
/// 
/// # Use of AES or Rijndael and its variants
/// This algorithm is implemented generic way. Most of the constants are
/// implemented as generic constants. So, without changing any line of code, you
/// can change the algorithm by changing the generic parameter. You can design
/// and implement your own algorithm based on Rijndael which uses SPN
/// (Substitution–Permutation Network). You can also use the the source code of
/// Rijndael_Generic in order to find the weakness of SPN.
/// 
/// # Generic Parameters
///   You can change the generic parameters if you want to make your own
///   encryption/decryption algorithm based on Rijndael. However, your own
///   algorithm may or may not be securer than AES. It is a high chance that
///   your own algorithm is less secure than AES unless you have performed the
///   security test for your own algorithm and proved that your algorithm is
///   at least as secure as AES.
/// - ROUND: Indicates the number of rounds. You can change how many times the
///   Substitution-Permutation Network will be repeated. Its minimum value is 0.
///   Original Rijndael has the number of rounds as follows:
///   |  data \ key  | 128-bit key | 192-bit key | 256-bit key |
///   |--------------|-------------|-------------|-------------|
///   | 128-bit data | 10 rounds   | 12 rounds   | 14 rounds   |
///   | 192-bit data | 12 rounds   | 12 rounds   | 14 rounds   |
///   | 256-bit data | 14 rounds   | 14 rounds   | 14 rounds   |
/// 
///   The default of `ROUND` is 10. The default value `10` is for AES-128
///   which is most frequently used.
/// - NB: Indicates the size of block. The block size is (`NB` * 4) bytes.
///   You can change the size of block by changing the value `NB`.
///   The default value of `NB` is `4`. The default value `4` is for AES
///   which is most frequently used.
/// - NK: Indicates the size of key. The key size is (`NK` * 4) bytes.
///   You can change the size of Key by changing the value `NK`.
///   The default value of `NK` is `4`. The default value `4` is for AES-128
///   which is most frequently used.
/// - IRREDUCIBLE: Indicates the irreducible polynomial. You can change the
///   irreducible polynomial by changing `IRREDUCIBLE`. The default value of
///   `IRREDUCIBLE` is `0b_0001_1011` which means x^8 + x^4 + x^3 + x + 1. In
///   `IRREDUCIBLE`, the term `x^8` is always omitted. The default value
///   `0b_0001_1011` is for Rijndael or AES.
/// - AFFINE_MUL: Indicates the two-dimensional matrix that is used to make
///   S-Box. `AFFINE_MUL` is the linear part of the affine transformation.
///   You can change the S-Box by changing `AFFINE_MUL` with `AFFINE_ADD`
///   along. The default value of `AFFINE_MUL` is
///   `0b_11111000_01111100_00111110_00011111_10001111_11000111_11100011_11110001`
///   which means the matrix as follows.
///   | Matrix          |
///   |-----------------|
///   | 1 1 1 1 1 0 0 0 |
///   | 0 1 1 1 1 1 0 0 |
///   | 0 0 1 1 1 1 1 0 |
///   | 0 0 0 1 1 1 1 1 |
///   | 1 0 0 0 1 1 1 1 |
///   | 1 1 0 0 0 1 1 1 |
///   | 1 1 1 0 0 0 1 1 |
///   | 1 1 1 1 0 0 0 1 |
/// 
///   The default value of `AFFINE_MUL` is for Rijndael or AES.
/// - AFFINE_ADD: Indicates the one-dimensional matrix that is used to make
///   S-Box. `AFFINE_ADD` is the non-linear part of the affine transformation.
///   You can change the S-Box by changing `AFFINE_ADD` with `AFFINE_MUL` along.
///   The default value of `AFFINE_ADD` is `0b_01100011` which means the matrix
///   as follows.
///   | Matrix |
///   |--------|
///   | 0      |
///   | 1      |
///   | 1      |
///   | 0      |
///   | 0      |
///   | 0      |
///   | 1      |
///   | 1      |
/// 
///   The default value of `AFFINE_ADD` is for Rijndael or AES.
/// - SR0 ~ SR3: Indicates how many bytes to shift (rotate) in each row in
///   ShiftRows step. You can change the amounts of shifting (rotating) in each
///   row by changing `SR0`, `SR1`, `SR2`, and `SR3`. The default values of
///   `SR0`, `SR1`, `SR2`, and `SR3` are `0`, `1`, `2`, and `3`, respectively.
///   The default values `0`, `1`, `2`, and `3` are for Rijndael or AES.
/// - MC00 ~ MC33: Indicates the two-dimensional matrix that is used as
///   MixColumns matrix in MixColumn step. `MC00`, `MC01`, `MC02`, `MC03`,
///   `MC10`, `MC11`, `MC12`, `MC13`, `MC20`, `MC21`, `MC22`, `MC23`, `MC30`,
///   `MC31`, `MC32`, and `MC33` are the elements of the two-dimensional
///   MixColumns matrix. You can change the MixColumns matrix to change the
///   MixColumn step by changing `MC00` ~ `MC33`. The default values of `MC00`,
///   `MC01`, `MC02`, `MC03`, `MC10`, `MC11`, `MC12`, `MC13`, `MC20`, `MC21`,
///   `MC22`, `MC23`, `MC30`, `MC31`, `MC32`, and `MC33` are `2`, `3`, `1`,
///   `1`, `1`, `2`, `3`, `1`, `1`, `1`, `2`, `3`, `3`, `1`, `1`, and `2`,
///   respectively, which means the matrix as follows.
///   | Matrix  |
///   |---------|
///   | 2 3 1 1 |
///   | 1 2 3 1 |
///   | 1 1 2 3 |
///   | 3 1 1 2 |
/// 
///   The default values of `MC00` ~ `MC33` are for Rijndael or AES.
/// - RC0 ~ RC9: Indicates Round Constants that are used to make round keys.
///   The round keys are used in the AddRoundKey step. You can change the
///   round keys by changing `RC0` ~ `RC9`. The default values of `RC0`, `RC1`,
///   `RC2`, `RC3`, `RC4`, `RC5`, `RC6`, `RC7`, `RC8`, and `RC9` are
///   `0b_0000_0001`, `0b_0000_0010`, `0b_0000_0100`, `0b_0000_1000`,
///   `0b_0001_0000`, `0b_0010_0000`,`0b_0100_0000`, `0b_1000_0000`,
///   `0b_0001_1011`, and `0b_001_10110`, respectively. The default values are
///   for Rijndael or AES. If `Round` is greater than 10, the round constants
///   after RC9 will be determined by multiplying previous round constant by `2`
///   on Galois Field. So, So-called RC10 is double of `RC9` on Galois Field and
///   RC11 is double of `RC10` on Galois Field, and so on.
/// - ROT: Indicates how may bytes to shift (rotate) in making round keys. You
///   can change round keys by changing `ROT`. The default value of `ROT` is
///   `1`. The default value `1` is for for Rijndael or AES.
/// 
/// # Reference
/// [Read more](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard)
/// about AES in brief.
/// Watch [this video](https://www.youtube.com/watch?v=x1v2tX4_dkQ) and then
/// [this video](https://www.youtube.com/watch?v=NHuibtoL_qk) in series
/// for more (or deeper or full) understanding of AES.
/// 
/// # Quick Start
/// You have to import (use) one of the modules: `AES_128`, `AES_192`, and
/// `AES_256` in order to use official AES as shown in Example 1. And, in the
/// same way you can import (use) the modules:
/// `Rijndael_128_128`, `Rijndael_128_192`, `Rijndael_128_256`,
/// `Rijndael_128_384`, `Rijndael_128_512`,
/// `Rijndael_192_128`, `Rijndael_192_192`, `Rijndael_192_256`,
/// `Rijndael_192_384`, `Rijndael_192_512`,
/// `Rijndael_256_128`, `Rijndael_256_192`, `Rijndael_256_256`,
/// `Rijndael_256_384`, `Rijndael_256_512`,
/// `Rijndael_384_128`, `Rijndael_384_192`, `Rijndael_384_256`,
/// `Rijndael_384_384`, `Rijndael_384_512`,
/// `Rijndael_512_128`, `Rijndael_512_192`, `Rijndael_512_256`,
/// `Rijndael_512_384`, and `Rijndael_512_512`.
/// Of course, you can also expand Rijndael by changing the generic parameters.
/// Also, there are some predefined modules:
/// `Rijndael_32_32` for 32 bit key and 32-bit encryption data, and
/// `Rijndael_64_64` for 64 bit key and 64-bit encryption data for educational
/// purpose, though they are not very practical.
/// 
/// # Example 1
/// ```
/// use cryptocol::symmetric::AES_128;
/// use cryptocol::symmetric::AES_192;
/// use cryptocol::symmetric::AES_256;
/// 
/// use cryptocol::symmetric::Rijndael_128_128;
/// use cryptocol::symmetric::Rijndael_128_192;
/// use cryptocol::symmetric::Rijndael_128_256;
/// use cryptocol::symmetric::Rijndael_128_384;
/// use cryptocol::symmetric::Rijndael_128_512;
/// 
/// use cryptocol::symmetric::Rijndael_192_128;
/// use cryptocol::symmetric::Rijndael_192_192;
/// use cryptocol::symmetric::Rijndael_192_256;
/// use cryptocol::symmetric::Rijndael_192_384;
/// use cryptocol::symmetric::Rijndael_192_512;
/// 
/// use cryptocol::symmetric::Rijndael_256_128;
/// use cryptocol::symmetric::Rijndael_256_192;
/// use cryptocol::symmetric::Rijndael_256_256;
/// use cryptocol::symmetric::Rijndael_256_384;
/// use cryptocol::symmetric::Rijndael_256_512;
/// 
/// use cryptocol::symmetric::Rijndael_384_128;
/// use cryptocol::symmetric::Rijndael_384_192;
/// use cryptocol::symmetric::Rijndael_384_256;
/// use cryptocol::symmetric::Rijndael_384_384;
/// use cryptocol::symmetric::Rijndael_384_512;
/// 
/// use cryptocol::symmetric::Rijndael_512_128;
/// use cryptocol::symmetric::Rijndael_512_192;
/// use cryptocol::symmetric::Rijndael_512_256;
/// use cryptocol::symmetric::Rijndael_512_384;
/// use cryptocol::symmetric::Rijndael_512_512;
/// 
/// use cryptocol::symmetric::Rijndael_64_64;
/// use cryptocol::symmetric::Rijndael_32_32;
/// ```
/// 
/// You can instantiate the AES object with `u128` key as Example 2.
/// In this case, you have to take endianness into account.
/// In little-endianness, 0x_1234567890ABCDEF1234567890ABCDEF_u128 is [0xEFu8,
/// 0xCDu8, 0xABu8, 0x90u8, 0x78u8, 0x56u8, 0x34u8, 0x12u8, 0xEFu8, 0xCDu8,
/// 0xABu8, 0x90u8, 0x78u8, 0x56u8, 0x34u8, 0x12u8] while the same 
/// 0x_1234567890ABCDEF1234567890ABCDEF_u128 is [0x12u8, 0x34u8, 0x56u8, 0x78u8,
/// 0x90u8, 0xABu8, 0xCDu8, 0xEF_u64, 0x12u8, 0x34u8, 0x56u8, 0x78u8, 0x90u8,
/// 0xABu8, 0xCDu8, 0xEF_u64] in big-endianness.
/// The instantiated object should be mutable.
/// 
/// # Example 2
/// ```
/// use cryptocol::symmetric::AES_128;
/// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
/// let mut _a_aes = AES_128::new_with_key_u128(key);
/// ```
/// 
/// Note that the object should be intantiated as mutable object.
/// Also, you can instantiate the AES object with `[u8; 16]` key as shown in
/// Example 3. In this case, you don't have to take endianness into account.
/// The instantiated object should be mutable.
/// 
/// # Example 3
/// ```
/// use cryptocol::symmetric::AES_128;
/// let key = [0xEFu8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12];
/// let mut _a_aes = AES_128::new_with_key(&key);
/// ```
/// 
/// You can instantiate the AES object without key and set a `u128` key later as
/// shown in Example 4 or a `[u8; 16]` key later as shown in Example 5.
/// 
/// # Example 4
/// ```
/// use cryptocol::symmetric::AES_128;
/// let mut a_aes = AES_128::new();
/// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
/// a_aes.set_key_u128(key);
/// ```
/// 
/// # Example 5
/// ```
/// use cryptocol::symmetric::AES_128;
/// let mut a_aes = AES_128::new();
/// let key = [0xEFu8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12];
/// a_aes.set_key(&key);
/// ```
/// 
/// Now, you can freely use any operation mode. This crate provide
/// ECB (Electronic CodeBook), CBC(Cipher Block Chaining), PCBC (Propagation
/// Cipher Block Chaining), CFB (Cipher FeedBack) OFB (Output FeedBack), and
/// CTR (CounTeR). You can choose the way of padding bits according to either
/// [PKCS #7](https://node-security.com/posts/cryptography-pkcs-7-padding/) or
/// [ISO 7816-4](https://en.wikipedia.org/wiki/Padding_(cryptography)#ISO/IEC_7816-4).
/// So, you can import (use) one of the following traits: ECB_PKCS7, ECB_ISO,
/// CBC_PKCS7, CBC_ISO, PCBC_PKCS7, PCBC_ISO, CFB, OFB, and CTR. The following
/// example 6 shows the case that you choose CBC operation mode and padding bits
/// according to PKCS #7.
/// 
/// # Example 6
/// ```
/// use std::io::Write;
/// use std::fmt::Write as _;
/// use cryptocol::symmetric::{ AES_128, CBC_PKCS7 };
///
/// let mut a_aes = AES_128::new_with_key(&[0xEFu8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
/// let message = "In the beginning God created the heavens and the earth.";
/// println!("M =\t{}", message);
/// let iv = [0x87654321_u32, 0xFEDCBA09_u32, 0x87654321_u32, 0xFEDCBA09_u32];
/// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
/// let mut cipher = Vec::<u8>::new();
/// a_aes.encrypt_str_into_vec(iv, message, &mut cipher);
/// print!("C =\t");
/// for c in cipher.clone()
///     { print!("{:02X} ", c); }
/// println!();
/// let mut txt = String::new();
/// for c in cipher.clone()
///     { write!(txt, "{:02X} ", c); }
/// assert_eq!(txt, "C9 1C 27 CE 83 92 A1 CF 7D A4 64 35 16 48 01 72 CC E3 6D CD BB 19 FC D0 80 22 09 9F 23 32 73 27 58 37 F9 9B 3C 44 7B 03 B3 80 7E 99 DF 97 4E E9 A3 89 67 0C 21 29 3E 4D DC AD B6 44 09 D4 3B 02 ");
///
/// let mut recovered = String::new();
/// a_aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
/// println!("B =\t{}", recovered);
/// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
/// assert_eq!(recovered, message);
/// println!();
/// ```
/// 
/// You can modify the Rijndael encryption/decryption algorithm as you want. All
/// the constants are implemented as generic parameters. For instance, you can
/// change S-box, the number of rounds of Substitution - Permutation network,
/// the irreducible polynormial, key length, etc. The following Example 7 shows
/// the variation of Rijndael which has 512-bit data block and 512-bit key.
/// 
/// # Example 7
/// ```
/// use std::io::Write;
/// use std::fmt::Write as _;
/// use cryptocol::symmetric::{ Rijndael_Generic, CBC_PKCS7 };
///
/// let mut a_rijndael = Rijndael_Generic::<22, 16, 16>::new_with_key(&[0xEFu8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
/// let message = "In the beginning God created the heavens and the earth.";
/// println!("M =\t{}", message);
/// let iv = [0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32];
/// println!("IV =\t{:08X}{:08X}{:08X}{:08X}", iv[0].to_be(), iv[1].to_be(), iv[2].to_be(), iv[3].to_be());
///
/// let mut cipher = Vec::<u8>::new();
/// a_rijndael.encrypt_str_into_vec(iv, message, &mut cipher);
/// print!("C =\t");
/// for c in cipher.clone()
///     { print!("{:02X} ", c); }
/// println!();
/// let mut txt = String::new();
/// for c in cipher.clone()
///     { write!(txt, "{:02X} ", c); }
/// assert_eq!(txt, "B1 C0 1F 84 17 46 35 12 D9 16 52 44 5F 40 A1 7F 3B 55 F7 E6 42 E5 1F 42 57 43 AD E4 00 19 54 1D B6 F3 1B 20 C8 D3 08 92 B7 C4 0C E2 77 73 A5 E4 0D E7 0F F4 B0 38 FE 78 30 56 E4 A7 9E CE 0E 50 ");
///
/// let mut recovered = String::new();
/// a_rijndael.decrypt_vec_into_string(iv, &cipher, &mut recovered);
/// println!("B =\t{}", recovered);
/// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
/// assert_eq!(recovered, message);
/// ```
/// 
/// You can use Rijndael_512_512 as one of post-Quantum algorithms for a
/// while because even Grover algorithm takes long enough to break
/// Rijndael_512_512. Its key is 512-bit and its encryption block is 512-bit.
/// 
/// # Example 8 for Post-Quantum
/// ```
/// use std::io::Write;
/// use std::fmt::Write as _;
/// use cryptocol::number::SharedArrays;
/// use cryptocol::hash::SHA3_512;
/// use cryptocol::symmetric::{ Rijndael_512_512, CBC_ISO };
/// 
/// let mut sha3 = SHA3_512::new();
/// sha3.absorb_str("Post-Quantum");
/// let key: [u8; 64] = sha3.get_hash_value_in_array();
/// print!("K =\t");
/// for i in 0..64
///     { print!("{:02X}", key[i]); }
/// println!();
/// let mut a_rijndael = Rijndael_512_512::new_with_key(&key);
/// sha3.absorb_str("Initialize");
/// let mut iv = SharedArrays::<u32, 16, u8, 64>::new();
/// iv.src = sha3.get_hash_value_in_array();
/// let iv = unsafe { iv.des };
/// print!("IV =\t");
/// for i in 0..16
///     { print!("{:08X}", iv[i].to_be()); }
/// println!();
/// let message = "In the beginning God created the heavens and the earth.";
/// println!("M =\t{}", message);
/// let mut cipher = [0_u8; 64];
/// a_rijndael.encrypt(iv.clone(), message.as_ptr(), message.len() as u64, cipher.as_mut_ptr());
/// print!("C =\t");
/// for c in cipher.clone()
///     { print!("{:02X} ", c); }
/// println!();
/// let mut txt = String::new();
/// for c in cipher.clone()
///     { write!(txt, "{:02X} ", c); }
/// assert_eq!(txt, "C2 C4 1C 91 EE 50 F0 04 B6 73 00 B2 81 05 01 25 C1 87 24 27 7E CE 01 65 C5 CB 87 38 99 9F 5B 0C D1 DF 8D 52 C4 C4 C8 D8 F5 D5 AD F3 FD DA 35 C2 33 F6 5D 83 02 85 F1 20 8C 56 0B 72 9C 91 84 42 ");
/// 
/// let mut recovered = vec![0; 55];
/// a_rijndael.decrypt(iv, cipher.as_ptr(), cipher.len() as u64, recovered.as_mut_ptr());
/// print!("Ba =\t");
/// for b in recovered.clone()
///     { print!("{:02X} ", b); }
/// println!();
/// let mut txt = String::new();
/// for c in recovered.clone()
///     { write!(txt, "{:02X} ", c); }
/// assert_eq!(txt, "49 6E 20 74 68 65 20 62 65 67 69 6E 6E 69 6E 67 20 47 6F 64 20 63 72 65 61 74 65 64 20 74 68 65 20 68 65 61 76 65 6E 73 20 61 6E 64 20 74 68 65 20 65 61 72 74 68 2E ");
/// 
/// let mut converted = String::new();
/// unsafe { converted.as_mut_vec() }.append(&mut recovered);
/// 
/// println!("Bb =\t{}", converted);
/// assert_eq!(converted, "In the beginning God created the heavens and the earth.");
/// assert_eq!(converted, message);
/// ```
/// 
/// # Notice for Practical Use
/// Now, you can freely use any methods with any paddings
/// in any operation modes.
/// - This crate provides six operation modes:
///   ECB, CBC, PCBC, CFB, OFB, and CTR.
/// - This crate provides two padding ways: ISO 7816-4 and PKCS #7.
/// - The operation modes ECB, CBC and PCBC requires padding bytes.
/// - You can combine three operation modes and two padding ways.
/// - The operation modes
///   [`CFB`](./trait.CFB.html#trait.CFB),
///   [`OFB`](./trait.OFB.html#trait.OFB), and
///   [`CTR`](./trait.CTR.html#trait.CTR)
///   does not require padding bytes.
/// - The traits that implements combination of operation modes and padding
///   ways are provided such as
///   [`ECB_PKCS7`](./trait.ECB_PKCS7.html#trait.ECB_PKCS7),
///   [`ECB_ISO`](./trait.ECB_ISO.html#trait.ECB_ISO),
///   [`CBC_PKCS7`](./trait.CBC_PKCS7.html#trait.ECB_PKCS7),
///   [`CBC_ISO`](./trait.CBC_ISO.html#trait.CBC_ISO),
///   [`PCBC_PKCS7`](./trait.PCBC_PKCS7.html#trait.PCBC_PKCS7), and
///   [`PCBC_ISO`](./trait.PCBC_ISO.html#trait.PCBC_ISO).
/// - You can find detaild instructions and their helpful examples
///   if you go through those links.
///
/// In summary,
///
/// |      | padding PKCS7                                          | padding ISO                                      | no padding                        |
/// |------|--------------------------------------------------------|--------------------------------------------------|-----------------------------------|
/// | ECB  | [ECB_PKCS7](./trait.ECB_PKCS7.html#trait.ECB_PKCS7)    | [ECB_ISO](./trait.ECB_ISO.html#trait.ECB_ISO)    |                                   |
/// | CBC  | [CBC_PKCS7](./trait.CBC_PKCS7.html#trait.ECB_PKCS7)    | [CBC_ISO](./trait.CBC_ISO.html#trait.CBC_ISO)    |                                   |
/// | PCBC | [PCBC_PKCS7](./trait.PCBC_PKCS7.html#trait.PCBC_PKCS7) | [PCBC_ISO](./trait.PCBC_ISO.html#trait.PCBC_ISO) |                                   |
/// | CFB  |                                                        |                                                  | [CFB](./trait.CFB.html#trait.CFB) |
/// | OFB  |                                                        |                                                  | [OFB](./trait.OFB.html#trait.OFB) |
/// | CTR  |                                                        |                                                  | [CTR](./trait.CTR.html#trait.CTR) |
/// 
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct Rijndael_Generic<const ROUND: usize = 10, const NB: usize = 4, const NK: usize = 4,
            const IRREDUCIBLE: u8 = 0b_0001_1011,
            const AFFINE_MUL: u64 = 0b_11111000_01111100_00111110_00011111_10001111_11000111_11100011_11110001,
            const AFFINE_ADD: u8 = 0b_01100011,
            const SR0: usize = 0, const SR1: usize = 1, const SR2: usize = 2, const SR3: usize = 3,
            const MC00: u8 = 2, const MC01: u8 = 3, const MC02: u8 = 1, const MC03: u8 = 1,
            const MC10: u8 = 1, const MC11: u8 = 2, const MC12: u8 = 3, const MC13: u8 = 1,
            const MC20: u8 = 1, const MC21: u8 = 1, const MC22: u8 = 2, const MC23: u8 = 3,
            const MC30: u8 = 3, const MC31: u8 = 1, const MC32: u8 = 1, const MC33: u8 = 2,
            const RC0: u32 = 0b_0000_0001, const RC1: u32 = 0b_0000_0010, const RC2: u32 = 0b_0000_0100,
            const RC3: u32 = 0b_0000_1000, const RC4: u32 = 0b_0001_0000, const RC5: u32 = 0b_0010_0000,
            const RC6: u32 = 0b_0100_0000, const RC7: u32 = 0b_1000_0000, const RC8: u32 = 0b_0001_1011,
            const RC9: u32 = 0b_001_10110, const ROT: u32 = 1>
{
    key:        [IntUnion; NK],
    block:      [[u8; NB]; 4],
    round_key:  Vec<[IntUnion; NB]>,
    enc:        fn (s: &mut Self, message: &[IntUnion; NB]) -> [IntUnion; NB],
    dec:        fn (s: &mut Self, cipher: &[IntUnion; NB]) -> [IntUnion; NB],
}

impl <const ROUND: usize, const NB: usize, const NK: usize, const IRREDUCIBLE: u8, const AFFINE_MUL: u64,
        const AFFINE_ADD: u8, const SR0: usize, const SR1: usize, const SR2: usize, const SR3: usize,
        const MC00: u8, const MC01: u8, const MC02: u8, const MC03: u8,
        const MC10: u8, const MC11: u8, const MC12: u8, const MC13: u8,
        const MC20: u8, const MC21: u8, const MC22: u8, const MC23: u8,
        const MC30: u8, const MC31: u8, const MC32: u8, const MC33: u8,
        const RC0: u32, const RC1: u32, const RC2: u32, const RC3: u32, const RC4: u32,
        const RC5: u32, const RC6: u32, const RC7: u32, const RC8: u32, const RC9: u32, const ROT: u32>
Rijndael_Generic<ROUND, NB, NK, IRREDUCIBLE, AFFINE_MUL, AFFINE_ADD, SR0, SR1, SR2, SR3,
        MC00, MC01, MC02, MC03, MC10, MC11, MC12, MC13, MC20, MC21, MC22, MC23, MC30, MC31, MC32, MC33,
        RC0, RC1, RC2, RC3, RC4, RC5, RC6, RC7, RC8, RC9, ROT>
{
    pub(super) const BLOCK_SIZE: usize = 4 * NB;

    const SUCCESS: u8 = !0;
    const FAILURE: u8 = 0;
    const SBOX: [u8; 256] = make_SBOX!();
    const INV_SBOX: [u8; 256] = make_INV_SBOX!();
    const SR: [usize; 4] = [SR0, SR1, SR2, SR3];
    const MC: [[u8; 4]; 4] = [  [ MC00, MC01, MC02, MC03 ],
                                [ MC10, MC11, MC12, MC13 ],
                                [ MC20, MC21, MC22, MC23 ],
                                [ MC30, MC31, MC32, MC33 ] ];
    const INV_MC: [[u8; 4]; 4] = make_INV_MC!();
    const RC: [u32; ROUND] = make_RC_ARRAY!();

    #[allow(non_upper_case_globals)]
    const method_shift_rows: fn (&mut Self) =   if (SR0 == 0) && (SR1 == 1) && (SR2 == 2) && (SR3 == 3)
                                                    { Self::optimal_shift_rows }
                                                else
                                                    { Self::shift_rows };

    #[allow(non_upper_case_globals)]
    const method_inv_shift_rows: fn (&mut Self) =   if (SR0 == 0) && (SR1 == 1) && (SR2 == 2) && (SR3 == 3)
                                                        { Self::optimal_inv_shift_rows }
                                                    else
                                                        { Self::inv_shift_rows };

    #[allow(non_upper_case_globals)]
    const method_make_round_keys: fn (&mut Self) =  if NK > 6
                                                    {
                                                        if NK == NB
                                                            { Self::make_round_keys_nk_greater_than_6_and_nk_equal_to_nb }
                                                        else
                                                            { Self::make_round_keys_nk_greater_than_6_and_nk_diff_from_nb }
                                                    }
                                                    else
                                                    {
                                                        if NK == NB
                                                            { Self::make_round_keys_nk_up_to_6_and_nk_equal_to_nb }
                                                        else
                                                            { Self::make_round_keys_nk_up_to_6_and_nk_diff_from_nb }
                                                    };

    #[allow(non_upper_case_globals)]
    const method_mix_columns: fn (&mut Self) =  if (MC00 == 2) && (MC01 == 3) && (MC02 == 1) && (MC03 == 1)
                                                && (MC10 == 1) && (MC11 == 2) && (MC12 == 3) && (MC13 == 1)
                                                && (MC20 == 1) && (MC21 == 1) && (MC22 == 2) && (MC23 == 3)
                                                && (MC30 == 3) && (MC31 == 1) && (MC32 == 1) && (MC33 == 2)
                                                    { Self::optimal_mix_columns }
                                                else
                                                    { Self::mix_columns };

    // pub fn new() -> Self
    /// Constructs a new object Rijndael_Generic.
    ///
    /// # Features
    /// - In order to encrypt data, object should be instantiated mutable.
    /// - This method sets the key to have all bits zeros.
    /// - The default key which has all bits zeros is not a weak key unlike DES.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::AES_128;
    /// let mut _a_aes = AES_128::new();
    /// let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    /// let ciphertext = _a_aes.encrypt_u128(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#032X}", plaintext);
    /// println!("Ciphertext:\t\t{:#032X}", ciphertext);
    /// assert_eq!(ciphertext, 0xE2C8CD3BFD4D72366A4806B100659867);
    ///
    /// let recovered_cipher_text = _a_aes.decrypt_u128(ciphertext);
    /// println!("Recovered-ciphertext:\t{:#032X}", recovered_cipher_text);
    /// assert_eq!(recovered_cipher_text, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(recovered_cipher_text, plaintext);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.new)
    pub fn new() -> Self
    {
        let mut rijndael = Self
        {
            key:        [IntUnion::new(); NK],
            block:      [[0_u8; NB]; 4],
            round_key:  vec![[IntUnion::new(); NB]; ROUND + 1],
            enc:        Self::encrypt_unit,
            dec:        Self::decrypt_unit,
        };
        Self::method_make_round_keys(&mut rijndael);
        rijndael
    }

    // pub fn new_with_key<const K: usize>(key: &[u8; K]) -> Self
    /// Constructs a new object Rijndael_Generic.
    ///
    /// # Arguments
    /// - The argument `key` is the array of u8 that has `K` elements.
    ///
    /// # Features
    /// - This method sets the key to be the given argument `key`.
    /// - If `K` is less than `4 * NK`, the rest bits of
    ///   the key to be set after `K` bits will be set to be `zero`.
    /// - If `K` is greater than `4 * NK`, the rest bits after `4 * NK` bits
    ///   of the key given as the argument will be ignored.
    ///
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    /// let ciphertext = aes.encrypt_u128(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    ///
    /// let cipher_cipher_text = aes.encrypt_u128(ciphertext);
    /// println!("Recovered-ciphertext:\t{:#034X}", recovered_cipher_text);
    /// assert_eq!(recovered_cipher_text, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(recovered_cipher_text, plaintext);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.new_with_key)
    pub fn new_with_key<const K: usize>(key: &[u8; K]) -> Self
    {
        let mut rijndael = Self
        {
            key:        [IntUnion::new(); NK],
            block:      [[0_u8; NB]; 4],
            round_key:  vec![[IntUnion::new(); NB]; ROUND + 1],
            enc:        Self::encrypt_unit,
            dec:        Self::decrypt_unit,
        };
        rijndael.set_key(key);
        rijndael
    }

    // pub fn new_with_key_u128(key: u128) -> Self
    /// Constructs a new object Rijndael_Generic.
    ///
    /// # Arguments
    /// - The argument `key` is of `u128`.
    /// - It should be in the same endianness of machine. For example,
    ///   if the intended key is [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD,
    ///   0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF], the key in
    ///   `u128` for this argument is 0x_1234567890ABCDEF1234567890ABCDEF_u128
    ///   for big-endian machine, and the key in `u128` for this argument is
    ///   0x_EFCDAB9078563412EFCDAB9078563412_u128 for little-endian machine.
    ///
    /// # Features
    /// - This method sets the key to be the given argument `key`.
    /// - If `NK` is less than `4`, the rest bits at the address higher than
    ///   `4 * NK` of the key given as the argument will be ignored.
    /// - If `NK` is greater than `4`, the rest bits of the key to be set after
    ///   128 bits will be set to be `zero`.
    ///
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new_with_key_u128(0xEFCDAB9078563412EFCDAB9078563412);
    /// let plaintext = 0x1234567890ABCDEF1234567890ABCDEF_u128;
    /// let ciphertext = aes.encrypt_u128(plaintext);
    ///
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    ///
    /// let recovered_cipher_text = aes.decrypt_u128(ciphertext);
    /// println!("Recovered-ciphertext:\t{:#034X}\n", recovered_cipher_text);
    /// assert_eq!(recovered_cipher_text, 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(recovered_cipher_text, plaintext);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.new_with_key_u128)
    pub fn new_with_key_u128(key: u128) -> Self
    {
        let mut rijndael = Self
        {
            key:        [IntUnion::new(); NK],
            block:      [[0_u8; NB]; 4],
            round_key:  vec![[IntUnion::new(); NB]; ROUND + 1],
            enc:        Self::encrypt_unit,
            dec:        Self::decrypt_unit,
        };
        rijndael.set_key_u128(key);
        rijndael
    }

    // pub fn encryptor_with_key<const K: usize>(key: &[u8; K]) -> Self
    /// Constructs a new object Rijndael_Generic as a positive encryptor (or
    /// an encryptor) for the component of BigCryptor128 and NAES.
    ///
    /// # Arguments
    /// - The argument `key` is the array of u8 that has `K` elements.
    ///
    /// # Features
    /// - You won't use this method unless you use BigCryptor128 and NAES
    ///   for such as Triple AES.
    /// - This method constructs the encryptor component of BigCryptor128
    ///   and NAES.
    /// - This method sets the key to be the given argument `key`.
    /// - If `K` is less than `4 * NK`, the rest bits of
    ///   the key to be set after `K` bits will be set to be `zero`.
    /// - If `K` is greater than `4 * NK`, the rest bits after `4 * NB` bits
    ///   of the key given as the argument will be ignored.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let keys: [Box<dyn SmallCryptor<u128, 16>>; 3]
    ///         = [ Box::new(AES_128::encryptor_with_key(&[0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])),
    ///             Box::new(AES_128::decryptor_with_key(&[0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])),
    ///             Box::new(AES_128::encryptor_with_key(&[0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])) ];
    /// let mut taes = BigCryptor128::new_with_small_cryptor_array(keys);
    /// let plaintext = 0x_1234567890ABCDEFFEDCA0987654321_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_E301940B5A5DE1600D78C375BF58F232_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_1234567890ABCDEFFEDCA0987654321_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.encryptor_with_key)
    #[inline]
    pub fn encryptor_with_key<const K: usize>(key: &[u8; K]) -> Self
    {
        Self::new_with_key(key)
    }

    // pub fn encryptor_with_key_u128(key: u128) -> Self
    /// Constructs a new object Rijndael_Generic as a positive encryptor (or
    /// an encryptor) for the component of BigCryptor128 and NAES.
    ///
    /// # Arguments
    /// - The argument `key` is of `u128`.
    /// - It should be in the same endianness of machine. For example,
    ///   if the intended key is [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD,
    ///   0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF], the key in
    ///   `u128` for this argument is 0x_1234567890ABCDEF1234567890ABCDEF_u128
    ///   for big-endian machine, and the key in `u128` for this argument is
    ///   0x_EFCDAB9078563412EFCDAB9078563412_u128 for little-endian machine.
    ///
    /// # Features
    /// - You won't use this method unless you use BigCryptor128 and NAES
    ///   for such as Triple AES.
    /// - This method constructs the encryptor component of BigCryptor128
    ///   and NAES.
    /// - This method sets the key to be the given argument `key`.
    /// - If `NK` is less than `4`, the rest bits at the address higher than
    ///   `4 * NK` of the key given as the argument will be ignored.
    /// - If `NK` is greater than `4`, the rest bits of the key to be set after
    ///   128 bits will be set to be `zero`.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let mut taes = BigCryptor128::new_with_small_cryptor_array(
    ///             [Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)),
    ///             Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///             Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128))]
    /// );
    /// let plaintext = 0x_11223344556677889900AABBCCDDEEFF_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_DB56B1B7D320D7481BF40A1964E9C7C4_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.encryptor_with_key_u128)
    #[inline]
    pub fn encryptor_with_key_u128(key: u128) -> Self
    {
        Self::new_with_key_u128(key)
    }

    // pub fn decryptor_with_key<const K: usize>(key: &[u8; K]) -> Self
    /// Constructs a new object Rijndael_Generic as a negative encryptor (or
    /// a decryptor) for the component of BigCryptor128 and NAES.
    ///
    /// # Arguments
    /// - The argument `key` is the array of u8 that has `K` elements.
    ///
    /// # Features
    /// - You won't use this method unless you use BigCryptor128 and NAES
    ///   for such as Triple AES.
    /// - This method constructs the encryptor component of BigCryptor128
    ///   and NAES.
    /// - This method sets the key to be the given argument `key`.
    /// - If `K` is less than `4 * NK`, the rest bits of
    ///   the key to be set after `K` bits will be set to be `zero`.
    /// - If `K` is greater than `4 * NK`, the rest bits after `4 * NB` bits
    ///   of the key given as the argument will be ignored.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let keys: [Box<dyn SmallCryptor<u128, 16>>; 3]
    ///         = [ Box::new(AES_128::encryptor_with_key(&[0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])),
    ///             Box::new(AES_128::decryptor_with_key(&[0x21_u8, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12])),
    ///             Box::new(AES_128::encryptor_with_key(&[0xEF_u8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0x21, 0x43, 0x65, 0x87, 0x09, 0xBA, 0xDC, 0xFE])) ];
    /// let mut taes = BigCryptor128::new_with_small_cryptor_array(keys);
    /// let plaintext = 0x_1234567890ABCDEFFEDCA0987654321_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_E301940B5A5DE1600D78C375BF58F232_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_1234567890ABCDEFFEDCA0987654321_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.decryptor_with_key)
    pub fn decryptor_with_key<const K: usize>(key: &[u8; K]) -> Self
    {
        let mut rijndael = Self::new_with_key(key);
        rijndael.turn_inverse();
        rijndael
    }

    // pub fn decryptor_with_key_u128(key: u128) -> Self
    /// Constructs a new object Rijndael_Generic as a negative encryptor (or
    /// a decryptor) for the component of BigCryptor128 and NAES.
    ///
    /// # Arguments
    /// - The argument `key` is of `u128`.
    /// - It should be in the same endianness of machine. For example,
    ///   if the intended key is [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD,
    ///   0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF], the key in
    ///   `u128` for this argument is 0x_1234567890ABCDEF1234567890ABCDEF_u128
    ///   for big-endian machine, and the key in `u128` for this argument is
    ///   0x_EFCDAB9078563412EFCDAB9078563412_u128 for little-endian machine.
    ///
    /// # Features
    /// - You won't use this method unless you use BigCryptor128 and NAES
    ///   for such as Triple AES.
    /// - This method constructs the encryptor component of BigCryptor128
    ///   and NAES.
    /// - This method sets the key to be the given argument `key`.
    /// - If `NK` is less than `4`, the rest bits at the address higher than
    ///   `4 * NK` of the key given as the argument will be ignored.
    /// - If `NK` is greater than `4`, the rest bits of the key to be set after
    ///   128 bits will be set to be `zero`.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let mut taes = BigCryptor128::new_with_small_cryptor_array(
    ///             [Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)),
    ///             Box::new(AES_128::decryptor_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///             Box::new(AES_128::encryptor_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128))]
    /// );
    /// let plaintext = 0x_11223344556677889900AABBCCDDEEFF_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_DB56B1B7D320D7481BF40A1964E9C7C4_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.decryptor_with_key_u128)
    pub fn decryptor_with_key_u128(key: u128) -> Self
    {
        let mut rijndael = Self::new_with_key_u128(key);
        rijndael.turn_inverse();
        rijndael
    }

    // pub fn get_key(&mut self) -> [u32; NK]
    /// Gets the key.
    ///
    /// # Output
    /// This method returns the key in the form of array of `u32`.
    ///
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut _aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let key = _aes.get_key();
    /// print!("K = ");
    /// for k in key
    ///     { print!("{:#010X} ", k); }
    /// println!();
    /// assert_eq!(key, [0x78563412, 0xEFCDAB90, 0x78563412, 0xEFCDAB90]);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.get_key)
    pub fn get_key(&mut self) -> [u32; NK]
    {
        let mut key = [0_u32; NK];
        unsafe { copy_nonoverlapping(self.key.as_ptr() as *const u32,
                                 &mut key as *mut u32, NK); }
        key
    }

    // pub fn get_key_u128(&self) -> u128
    /// Gets the key.
    ///
    /// # Output
    /// This method returns the key in the form of `u128`.
    ///
    /// # Features
    /// - If `NK` is less than `4`, the rest bits at the address higher than
    ///   `4 * NK` of the returned key will be set to be `zero`.
    /// - If `NK` is greater than `4`, the rest bits of the key to be returned
    ///   after 128 bits will be ignored.
    ///
    /// # Example 1 for 128-bit key
    /// ```
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut _aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let key = _aes.get_key_u128();
    /// println!("Key = {:#034X}", key);
    /// assert_eq!(key, 0xEFCDAB9078563412EFCDAB9078563412_u128);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.get_key_u128)
    pub fn get_key_u128(&self) -> u128
    {
        let len = if 16 < NK * 4 { 16 } else { NK * 4 };
        let mut key = 0_u128;
        unsafe { copy_nonoverlapping(self.key.as_ptr() as *const u8,
                                     &mut key as *mut u128 as *mut u8, len); }
        key
    }

    // pub fn set_key<const K: usize>(&mut self, key: &[u8; K])
    /// Sets the key.
    ///
    /// # Arguments
    /// - The argument `key` is the array of `u8` that has `K` elements.
    ///
    /// # Features
    /// - This method sets the key to be the given argument `key`.
    /// - If `K` is less than `4 * NK`, the rest bits of
    ///   the key to be set after `K` bits will be set to be `zero`.
    /// - If `K` is greater than `4 * NK`, the rest bits after `4 * NK` bits
    ///   of the key given as the argument will be ignored.
    ///
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new();
    /// aes.set_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let key = aes.get_key();
    /// print!("K = ");
    /// for k in key
    ///     { print!("{:#010X} ", k); }
    /// println!();
    /// assert_eq!(key, [0x78563412, 0xEFCDAB90, 0x78563412, 0xEFCDAB90]);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.set_key)
    pub fn set_key<const K: usize>(&mut self, key: &[u8; K])
    {
        let len = if K < NK * 4 { K } else { NK * 4 };
        unsafe {
            copy_nonoverlapping(key.as_ptr(), self.key.as_mut_ptr() as *mut u8, len);
        }
        Self::method_make_round_keys(self);
    }

    //  pub fn set_key_u128(&mut self, key: u128)
    /// Constructs a new object Rijndael_Generic.
    ///
    /// # Arguments
    /// - The argument `key` is of `u128`.
    /// - It should be in the same endianness of machine. For example,
    ///   if the intended key is [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD,
    ///   0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF], the key in
    ///   `u128` for this argument is 0x_1234567890ABCDEF1234567890ABCDEF_u128
    ///   for big-endian machine, and the key in `u128` for this argument is
    ///   0x_EFCDAB9078563412EFCDAB9078563412_u128 for little-endian machine.
    ///
    /// # Features
    /// - This method sets the key to be the given argument `key`.
    /// - If `NK` is less than `4`, the rest bits at the address higher than
    ///   `4 * NK` of the key given as the argument will be ignored.
    /// - If `NK` is greater than `4`, the rest bits of the key to be set after
    ///   128 bits will be set to be `zero`.
    ///
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new();
    /// aes.set_key_u128(0xEFCDAB9078563412EFCDAB9078563412);
    /// let key = aes.get_key();
    /// print!("K = ");
    /// for k in key
    ///     { print!("{:#010X} ", k); }
    /// println!();
    /// assert_eq!(key, [0x78563412, 0xEFCDAB90, 0x78563412, 0xEFCDAB90]);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.set_key_u128)
    pub fn set_key_u128(&mut self, key: u128)
    {
        let len = if 16 < NK * 4 { 16 } else { NK * 4 };
        unsafe {
            copy_nonoverlapping(&key as *const u128 as *const u8,
                                self.key.as_mut_ptr() as *mut u8, len);
        }
        Self::method_make_round_keys(self);
    }

    // pub fn turn_inverse(&mut self)
    /// Flips its role in BigCryptor128 or NAES.
    ///
    /// # Features
    /// - You won't use this method unless you use BigCryptor128 or NAES
    ///   for such as Triple DES.
    /// - Even if you are writing codes in the context of using BigCryptor128
    ///   or NAES, you will hardly use this method because it is high chance
    ///   that you will have constructed components with the methods,
    ///   encryptor_with_key(struct@Rijndael_Generic#method.encryptor_with_key),
    ///   encryptor_with_key_u64(struct@Rijndael_Generic#method.encryptor_with_key_u128),
    ///   decryptor_with_key(struct@Rijndael_Generic#method.decryptor_with_key), and
    ///   decryptor_with_key_u64(struct@Rijndael_Generic#method.decryptor_with_key_u128).
    /// - If it is constructed as encryptor for BigCryptor128 or NAES,
    ///   it will be changed into decryptor.
    /// - If it is constructed as decryptor for BigCryptor128 or NAES,
    ///   it will be changed into encryptor.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let mut keys: [Box<dyn SmallCryptor<u128, 16>>; 3]
    ///             = [ Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)),
    ///                 Box::new(AES_128::new_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///                 Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)) ];
    /// keys[1].turn_inverse();
    /// 
    /// let mut taes = BigCryptor128::new_with_small_cryptor_array(keys);
    /// let plaintext = 0x_11223344556677889900AABBCCDDEEFF_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_DB56B1B7D320D7481BF40A1964E9C7C4_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.turn_inverse)
    #[inline]
    pub fn turn_inverse(&mut self)
    {
        (self.enc, self.dec) = (self.dec, self.enc);
    }

    // pub fn turn_encryptor(&mut self)
    /// Changes its role in BigCryptor128 or NAES to encryptor.
    ///
    /// # Features
    /// - You won't use this method unless you use BigCryptor128 or NAES
    ///   for such as Triple AES.
    /// - Even if you are writing codes in the context of using BigCryptor128
    ///   or NAES, you will hardly use this method because it is high chance
    ///   that you will have constructed components with the methods,
    ///   [encryptor_with_key](struct@Rijndael_Generic#method.encryptor_with_key),
    ///   [encryptor_with_key_u128](struct@Rijndael_Generic#method.encryptor_with_key_u128),
    ///   [decryptor_with_key](struct@Rijndael_Generic#method.decryptor_with_key), and
    ///   [decryptor_with_key_u128](struct@Rijndael_Generic#method.decryptor_with_key_u128).
    /// - If it is constructed as encryptor for BigCryptor128 or NAES,
    ///   it will not be changed at all.
    /// - If it is constructed as decryptor for BigCryptor128 or NAES,
    ///   it will be changed into encryptor.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let mut keys: [Box<dyn SmallCryptor<u128, 16>>; 3]
    ///         = [ Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)),
    ///             Box::new(AES_128::new_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///             Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)) ];
    /// keys[0].turn_encryptor();
    /// 
    /// let mut taes = BigCryptor128::new_with_small_cryptor_array(keys);
    /// let plaintext = 0x_11223344556677889900AABBCCDDEEFF_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_5C842CA9ECB742B2F3164BC33E0BDCB6_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.turn_encryptor)
    pub fn turn_encryptor(&mut self)
    {
        self.enc = Self::encrypt_unit;
        self.dec = Self::decrypt_unit;
    }

    // pub fn turn_decryptor(&mut self)
    /// Changes its role in BigCryptor128 or NAES to decryptor.
    ///
    /// # Features
    /// - You won't use this method unless you use BBigCryptor128 or NAES
    ///   for such as Triple DES.
    /// - Even if you are writing codes in the context of using BigCryptor128
    ///   or NAES, you will hardly use this method because it is high chance
    ///   that you will have constructed components with the methods,
    ///   [encryptor_with_key](struct@Rijndael_Generic#method.encryptor_with_key),
    ///   [encryptor_with_key_u128](struct@Rijndael_Generic#method.encryptor_with_key_u128),
    ///   [decryptor_with_key](struct@Rijndael_Generic#method.decryptor_with_key), and
    ///   [decryptor_with_key_u128](struct@Rijndael_Generic#method.decryptor_with_key_u128).
    /// - If it is constructed as encryptor for BigCryptor128 or NAES,
    ///   it will be changed into decryptor.
    /// - If it is constructed as decryptor for BigCryptor128 or NAES,
    ///   it will not be changed at all.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::symmetric::{ AES_128, BigCryptor128, SmallCryptor };
    /// 
    /// let mut keys: [Box<dyn SmallCryptor<u128, 16>>; 3]
    ///             = [ Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)),
    ///                 Box::new(AES_128::new_with_key_u128(0x_FEDCBA09876543211234567890ABCDEF_u128)),
    ///                 Box::new(AES_128::new_with_key_u128(0x_1234567890ABCDEFFEDCA0987654321_u128)) ];
    /// keys[1].turn_decryptor();
    /// 
    /// let mut taes = BigCryptor128::new_with_small_cryptor_array(keys);
    /// let plaintext = 0x_11223344556677889900AABBCCDDEEFF_u128;
    /// let ciphertext = taes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x_DB56B1B7D320D7481BF40A1964E9C7C4_u128);
    /// 
    /// let recovered_text = taes.decrypt_u128(ciphertext);
    /// println!("Recovered text:\t{:#034X}", recovered_text);
    /// assert_eq!(recovered_text, 0x_11223344556677889900AABBCCDDEEFF_u128);
    /// assert_eq!(recovered_text, plaintext);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.turn_decryptor)
    pub fn turn_decryptor(&mut self)
    {
        self.enc = Self::decrypt_unit;
        self.dec = Self::encrypt_unit;
    }

    //  pub fn encrypt_unit(&mut self, message: &[IntUnion; NB]) -> [IntUnion; NB]
    /// Encrypts data in the form of an array of IntUnion with `NB` elements.
    ///
    /// # Arguments
    /// `message` is of the type `&[IntUnion; NB]`
    /// and the plaintext to be encrypted.
    ///
    /// # Output
    /// This method returns the encrypted data in the array of `IntUnion`
    /// with `NB` elements.
    ///
    /// # Counterpart methods
    /// For each trait
    /// [`ECB_PKCS7`](symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
    /// [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
    /// [`CBC_PKCS7`](symmetric/trait.CBC_PKCS7.html#trait.ECB_PKCS7),
    /// [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
    /// [`PCBC_PKCS7`](symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
    /// [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO).
    /// [`CFB`](symmetric/trait.CFB.html#trait.CFB),
    /// [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
    /// [`CTR`](symmetric/trait.CTR.html#trait.CTR),
    /// there are provided useful counterpart methods:
    /// encrypt(), encrypt_into_vec(), encrypt_into_array(),
    /// encrypt_str(), encrypt_str_into_vec(), encrypt_str_into_array(),
    /// encrypt_string(), encrypt_string_into_vec(), encrypt_string_into_array(),
    /// encrypt_vec(), encrypt_vec_into_vec(), encrypt_vec_into_array(),
    /// encrypt_array(), encrypt_array_into_vec(), and encrypt_array_into_array().
    ///
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = [IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678), IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678)];
    /// let ciphertext = aes.encrypt_unit(&plaintext);
    ///
    /// println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}", plaintext[0].get(), plaintext[1].get(), plaintext[2].get(), plaintext[3].get());
    /// println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}", ciphertext[0].get(), ciphertext[1].get(), ciphertext[2].get(), ciphertext[3].get());
    /// assert_eq!(ciphertext[0].get(), 0x27584D87);
    /// assert_eq!(ciphertext[1].get(), 0x44E2BAE9);
    /// assert_eq!(ciphertext[2].get(), 0x4AECB5D6);
    /// assert_eq!(ciphertext[3].get(), 0x01CCF826);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.encrypt_unit)
    pub fn encrypt_unit(&mut self, message: &[IntUnion; NB]) -> [IntUnion; NB]
    {
        self.set_block(message);
        self.encrypt_block();
        self.get_block()
    }

    // pub fn encrypt_u128(&mut self, message: u128) -> u128
    /// Encrypts a 128-bit data.
    ///
    /// # Arguments
    /// `message` is of `u128`-type and the plaintext to be encrypted.
    ///
    /// # Output
    /// This method returns the encrypted data of `u128`-type from `message`.
    /// 
    /// # Caution
    /// - This method is meaningful only when `NB` is `4`. 
    /// - If `NB` is other than `4`, this method may panic.
    /// - Even if this method does not panic, its behaviour is not defined.
    /// 
    /// # Counterpart Methods
    /// For each trait
    /// [`ECB_PKCS7`](symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
    /// [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
    /// [`CBC_PKCS7`](symmetric/trait.CBC_PKCS7.html#trait.ECB_PKCS7),
    /// [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
    /// [`PCBC_PKCS7`](symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
    /// [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO).
    /// [`CFB`](symmetric/trait.CFB.html#trait.CFB),
    /// [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
    /// [`CTR`](symmetric/trait.CTR.html#trait.CTR),
    /// there are provided useful counterpart methods:
    /// encrypt(), encrypt_into_vec(), encrypt_into_array(),
    /// encrypt_str(), encrypt_str_into_vec(), encrypt_str_into_array(),
    /// encrypt_string(), encrypt_string_into_vec(), encrypt_string_into_array(),
    /// encrypt_vec(), encrypt_vec_into_vec(), encrypt_vec_into_array(),
    /// encrypt_array(), encrypt_array_into_vec(), and encrypt_array_into_array().
    ///
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = 0x1234567890ABCDEF1234567890ABCDEF;
    /// let ciphertext = aes.encrypt_u128(plaintext);
    /// 
    /// println!("Plaintext:\t{:#034X}", plaintext);
    /// println!("Ciphertext:\t{:#034X}", ciphertext);
    /// assert_eq!(ciphertext, 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.encrypt_u128)
    pub fn encrypt_u128(&mut self, message: u128) -> u128
    {
        self.set_block_u128(message);
        self.encrypt_block();
        self.get_block_u128()
    }

    // pub fn encrypt_u64(&mut self, message: u64) -> u64
    /// Encrypts a 64-bit data.
    ///
    /// # Arguments
    /// `message` is of `u64`-type and the plaintext to be encrypted.
    ///
    /// # Output
    /// This method returns the encrypted data of `u64`-type from `message`.
    /// 
    /// # Caution
    /// - This method is meaningful only when `NB` is `2`. 
    /// - If `NB` is other than `2`, this method may panic.
    /// - Even if this method does not panic, its behaviour is not defined.
    /// 
    /// # Counterpart Methods
    /// For each trait
    /// [`ECB_PKCS7`](symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
    /// [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
    /// [`CBC_PKCS7`](symmetric/trait.CBC_PKCS7.html#trait.ECB_PKCS7),
    /// [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
    /// [`PCBC_PKCS7`](symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
    /// [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO).
    /// [`CFB`](symmetric/trait.CFB.html#trait.CFB),
    /// [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
    /// [`CTR`](symmetric/trait.CTR.html#trait.CTR),
    /// there are provided useful counterpart methods:
    /// encrypt(), encrypt_into_vec(), encrypt_into_array(),
    /// encrypt_str(), encrypt_str_into_vec(), encrypt_str_into_array(),
    /// encrypt_string(), encrypt_string_into_vec(), encrypt_string_into_array(),
    /// encrypt_vec(), encrypt_vec_into_vec(), encrypt_vec_into_array(),
    /// encrypt_array(), encrypt_array_into_vec(), and encrypt_array_into_array().
    ///
    /// # Example 1 for Rijndael_64_64
    /// ```
    /// use cryptocol::symmetric::Rijndael_64_64;
    /// 
    /// let mut rijndael = Rijndael_64_64::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = 0x1234567890ABCDEF;
    /// println!("Plaintext:\t{:#018X}", plaintext);
    /// let ciphertext = rijndael.encrypt_u64(plaintext);
    /// println!("Ciphertext:\t{:#018X}", ciphertext);
    /// assert_eq!(ciphertext, 0x4FAA3F0E49CC4DCF_u64);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.encrypt_u64)
    pub fn encrypt_u64(&mut self, message: u64) -> u64
    {
        self.set_block_u64(message);
        self.encrypt_block();
        self.get_block_u64()
    }

    // pub fn encrypt_u32(&mut self, message: u32) -> u32
    /// Encrypts a 32-bit data.
    ///
    /// # Arguments
    /// `message` is of `u32`-type and the plaintext to be encrypted.
    ///
    /// # Output
    /// This method returns the encrypted data of `u32`-type from `message`.
    /// 
    /// # Caution
    /// - This method is meaningful only when `NB` is `1`. 
    /// - If `NB` is other than `1`, this method may panic.
    /// - Even if this method does not panic, its behaviour is not defined.
    /// 
    /// # Counterpart Methods
    /// For each trait
    /// [`ECB_PKCS7`](symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
    /// [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
    /// [`CBC_PKCS7`](symmetric/trait.CBC_PKCS7.html#trait.ECB_PKCS7),
    /// [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
    /// [`PCBC_PKCS7`](symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
    /// [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO).
    /// [`CFB`](symmetric/trait.CFB.html#trait.CFB),
    /// [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
    /// [`CTR`](symmetric/trait.CTR.html#trait.CTR),
    /// there are provided useful counterpart methods:
    /// encrypt(), encrypt_into_vec(), encrypt_into_array(),
    /// encrypt_str(), encrypt_str_into_vec(), encrypt_str_into_array(),
    /// encrypt_string(), encrypt_string_into_vec(), encrypt_string_into_array(),
    /// encrypt_vec(), encrypt_vec_into_vec(), encrypt_vec_into_array(),
    /// encrypt_array(), encrypt_array_into_vec(), and encrypt_array_into_array().
    ///
    /// # Example 1 for Rijndael_32_32
    /// ```
    /// use cryptocol::symmetric::Rijndael_32_32;
    /// 
    /// let mut rijndael = Rijndael_32_32::new_with_key(&[0x12, 0x34, 0x56, 0x78]);
    /// let plaintext = 0x1234567;
    /// println!("Plaintext:\t{:#010X}", plaintext);
    /// let ciphertext = rijndael.encrypt_u32(plaintext);
    /// println!("Ciphertext:\t{:#010X}", ciphertext);
    /// assert_eq!(ciphertext, 0xB25E4E09_u32);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.encrypt_u32)
    pub fn encrypt_u32(&mut self, message: u32) -> u32
    {
        self.set_block_u32(message);
        self.encrypt_block();
        self.get_block_u32()
    }

    // pub fn decrypt_unit(&mut self, cipher: &[IntUnion; NB]) -> [IntUnion; NB]
    /// Decrypts data in the form of an array of IntUnion with `NB` elements.
    ///
    /// # Arguments
    /// `cipher` is of the type `&[IntUnion; NB]`
    /// and the ciphertext to be decrypted.
    ///
    /// # Output
    /// This method returns the decrypted data in the array of `IntUnion`
    /// with `NB` elements.
    /// 
    /// # Counterpart Methods
    /// For each trait
    /// [`ECB_PKCS7`](symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
    /// [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
    /// [`CBC_PKCS7`](symmetric/trait.CBC_PKCS7.html#trait.ECB_PKCS7),
    /// [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
    /// [`PCBC_PKCS7`](symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
    /// [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO).
    /// [`CFB`](symmetric/trait.CFB.html#trait.CFB),
    /// [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
    /// [`CTR`](symmetric/trait.CTR.html#trait.CTR),
    /// there are provided useful counterpart methods:
    /// decrypt(), decrypt_into_vec(), decrypt_into_array(),
    /// decrypt_into_string(),
    /// decrypt_vec(), decrypt_vec_into_vec(), decrypt_vec_into_array(),
    /// decrypt_vec_into_string(),
    /// decrypt_array(), decrypt_array_into_vec(), decrypt_array_into_array(),
    /// and decrypt_array_into_string().
    ///
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let ciphertext = [IntUnion::new_with(0x27584D87), IntUnion::new_with(0x44E2BAE9), IntUnion::new_with(0x4AECB5D6), IntUnion::new_with(0x01CCF826)];
    /// let plaintext = aes.decrypt_unit(&ciphertext);
    ///
    /// println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}", ciphertext[0].get(), ciphertext[1].get(), ciphertext[2].get(), ciphertext[3].get());
    /// println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}", plaintext[0].get(), plaintext[1].get(), plaintext[2].get(), plaintext[3].get());
    /// assert_eq!(plaintext[0].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[1].get(), 0x12345678);
    /// assert_eq!(plaintext[2].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[3].get(), 0x12345678);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.decrypt_unit)
    pub fn decrypt_unit(&mut self, cipher: &[IntUnion; NB]) -> [IntUnion; NB]
    {
        self.set_block(cipher);
        self.decrypt_block();
        self.get_block()
    }

    // pub fn decrypt_u128(&mut self, cipher: u128) -> u128
    /// Decrypts a 128-bit data.
    ///
    /// # Arguments
    /// `cioher` is of `u128`-type and the ciphertext to be decrypted.
    ///
    /// # Output
    /// This method returns the decrypted data of `u128`-type from `cipher`.
    /// 
    /// # Caution
    /// - This method is meaningful only when `NB` is `4`. 
    /// - If `NB` is other than `4`, this method may panic.
    /// - Even if this method does not panic, its behaviour is not defined.
    /// 
    /// # Counterpart Methods
    /// For each trait
    /// [`ECB_PKCS7`](symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
    /// [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
    /// [`CBC_PKCS7`](symmetric/trait.CBC_PKCS7.html#trait.ECB_PKCS7),
    /// [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
    /// [`PCBC_PKCS7`](symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
    /// [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO).
    /// [`CFB`](symmetric/trait.CFB.html#trait.CFB),
    /// [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
    /// [`CTR`](symmetric/trait.CTR.html#trait.CTR),
    /// there are provided useful counterpart methods:
    /// decrypt(), decrypt_into_vec(), decrypt_into_array(),
    /// decrypt_into_string(),
    /// decrypt_vec(), decrypt_vec_into_vec(), decrypt_vec_into_array(),
    /// decrypt_vec_into_string(),
    /// decrypt_array(), decrypt_array_into_vec(), decrypt_array_into_array(),
    /// and decrypt_array_into_string().
    ///
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let ciphertext = 0x01CCF8264AECB5D644E2BAE927584D87_u128;
    /// let plaintext = aes.decrypt_u128(ciphertext);
    ///
    /// println!("Ciphertext:\t{:#034X}", ciphertext);
    /// println!("Plaintext:\t{:#034X}", plaintext);
    /// assert_eq!(plaintext, 0x1234567890ABCDEF1234567890ABCDEF);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.decrypt_u128)
    pub fn decrypt_u128(&mut self, cipher: u128) -> u128
    {
        self.set_block_u128(cipher);
        self.decrypt_block();
        self.get_block_u128()
    }

    // pub fn decrypt_u64(&mut self, cipher: u64) -> u64
    /// Decrypts a 64-bit data.
    ///
    /// # Arguments
    /// `cioher` is of `u64`-type and the ciphertext to be decrypted.
    ///
    /// # Output
    /// This method returns the decrypted data of `u64`-type from `cipher`.
    /// 
    /// # Caution
    /// - This method is meaningful only when `NB` is `2`. 
    /// - If `NB` is other than `2`, this method may panic.
    /// - Even if this method does not panic, its behaviour is not defined.
    /// 
    /// # Counterpart Methods
    /// For each trait
    /// [`ECB_PKCS7`](symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
    /// [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
    /// [`CBC_PKCS7`](symmetric/trait.CBC_PKCS7.html#trait.ECB_PKCS7),
    /// [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
    /// [`PCBC_PKCS7`](symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
    /// [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO).
    /// [`CFB`](symmetric/trait.CFB.html#trait.CFB),
    /// [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
    /// [`CTR`](symmetric/trait.CTR.html#trait.CTR),
    /// there are provided useful counterpart methods:
    /// decrypt(), decrypt_into_vec(), decrypt_into_array(),
    /// decrypt_into_string(),
    /// decrypt_vec(), decrypt_vec_into_vec(), decrypt_vec_into_array(),
    /// decrypt_vec_into_string(),
    /// decrypt_array(), decrypt_array_into_vec(), decrypt_array_into_array(),
    /// and decrypt_array_into_string().
    ///
    /// # Example 1 for Rijndael_64_64
    /// ```
    /// use cryptocol::symmetric::Rijndael_64_64;
    /// 
    /// let mut rijndael = Rijndael_64_64::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let ciphertext = 0x4FAA3F0E49CC4DCF_u64;
    /// println!("Ciphertext:\t{:#018X}", ciphertext);
    /// let plaintext = rijndael.decrypt_u64(ciphertext);
    /// println!("Plaintext:\t{:#018X}", plaintext);
    /// assert_eq!(plaintext, 0x1234567890ABCDEF_u64);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.decrypt_u64)
    pub fn decrypt_u64(&mut self, cipher: u64) -> u64
    {
        self.set_block_u64(cipher);
        self.decrypt_block();
        self.get_block_u64()
    }

    // pub fn decrypt_u32(&mut self, cipher: u32) -> u32
    /// Decrypts a 32-bit data.
    ///
    /// # Arguments
    /// `cioher` is of `u32`-type and the ciphertext to be decrypted.
    ///
    /// # Output
    /// This method returns the decrypted data of `u32`-type from `cipher`.
    /// 
    /// # Caution
    /// - This method is meaningful only when `NB` is `1`. 
    /// - If `NB` is other than `1`, this method may panic.
    /// - Even if this method does not panic, its behaviour is not defined.
    /// 
    /// # Counterpart Methods
    /// For each trait
    /// [`ECB_PKCS7`](symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
    /// [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
    /// [`CBC_PKCS7`](symmetric/trait.CBC_PKCS7.html#trait.ECB_PKCS7),
    /// [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
    /// [`PCBC_PKCS7`](symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
    /// [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO).
    /// [`CFB`](symmetric/trait.CFB.html#trait.CFB),
    /// [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
    /// [`CTR`](symmetric/trait.CTR.html#trait.CTR),
    /// there are provided useful counterpart methods:
    /// decrypt(), decrypt_into_vec(), decrypt_into_array(),
    /// decrypt_into_string(),
    /// decrypt_vec(), decrypt_vec_into_vec(), decrypt_vec_into_array(),
    /// decrypt_vec_into_string(),
    /// decrypt_array(), decrypt_array_into_vec(), decrypt_array_into_array(),
    /// and decrypt_array_into_string().
    ///
    /// # Example 1 for Rijndael_32_32
    /// ```
    /// use cryptocol::symmetric::Rijndael_32_32;
    /// 
    /// let mut rijndael = Rijndael_32_32::new_with_key(&[0x12, 0x34, 0x56, 0x78]);
    /// let ciphertext = 0xB25E4E09_u32;
    /// println!("Ciphertext:\t{:#010X}", ciphertext);
    /// let plaintext = rijndael.decrypt_u32(ciphertext);
    /// println!("Plaintext:\t{:#010X}", plaintext);
    /// assert_eq!(plaintext, 0x1234567_u32);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.decrypt_u32)
    pub fn decrypt_u32(&mut self, cipher: u32) -> u32
    {
        self.set_block_u32(cipher);
        self.decrypt_block();
        self.get_block_u32()
    }

    #[inline]
    pub(super) fn _encrypt(&mut self, message: &[IntUnion; NB]) -> [IntUnion; NB]
    {
        (self.enc)(self, message)
    }

    #[inline]
    pub(super) fn _decrypt(&mut self, cipher: &[IntUnion; NB]) -> [IntUnion; NB]
    {
        (self.dec)(self, cipher)
    }

    // pub fn encrypt_array_unit<const N: usize>(&mut self, message: &[[IntUnion; NB]; N], cipher: &mut [[IntUnion; NB]; N])
    /// Encrypts an array of unit data, `[[IntUnion; NB]; N]`.
    ///
    /// # Arguments
    /// - `message` is of an array of `[IntUnion; NB]`-type and the plaintext
    ///   to be encrypted.
    /// - `cipher` is of an array of `[IntUnion; NB]`-type and the ciphertext
    ///   to be stored.
    ///
    /// # Features
    /// This method encrypts multiple of 64-bit data without padding anything
    /// in ECB (Electronic CodeBook) mode.
    /// 
    /// # Counterpart Methods
    /// For each trait
    /// [`ECB_PKCS7`](symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
    /// [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
    /// [`CBC_PKCS7`](symmetric/trait.CBC_PKCS7.html#trait.ECB_PKCS7),
    /// [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
    /// [`PCBC_PKCS7`](symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
    /// [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO).
    /// [`CFB`](symmetric/trait.CFB.html#trait.CFB),
    /// [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
    /// [`CTR`](symmetric/trait.CTR.html#trait.CTR),
    /// there are provided useful counterpart methods:
    /// encrypt(), encrypt_into_vec(), encrypt_into_array(),
    /// encrypt_str(), encrypt_str_into_vec(), encrypt_str_into_array(),
    /// encrypt_string(), encrypt_string_into_vec(), encrypt_string_into_array(),
    /// encrypt_vec(), encrypt_vec_into_vec(), encrypt_vec_into_array(),
    /// encrypt_array(), encrypt_array_into_vec(), and encrypt_array_into_array().
    /// 
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_128;
    ///
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = [[IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678), IntUnion::new_with(0x90ABCDEF), IntUnion::new_with(0x12345678)]; 3];
    /// let mut ciphertext = [[IntUnion::new(); 4]; 3];
    /// aes.encrypt_array_unit(&plaintext, &mut ciphertext);
    /// 
    /// println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
    ///         plaintext[0][0].get(), plaintext[0][1].get(), plaintext[0][2].get(), plaintext[0][3].get(),
    ///         plaintext[1][0].get(), plaintext[1][1].get(), plaintext[1][2].get(), plaintext[1][3].get(),
    ///         plaintext[2][0].get(), plaintext[2][1].get(), plaintext[2][2].get(), plaintext[2][3].get());
    /// println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
    ///         ciphertext[0][0].get(), ciphertext[0][1].get(), ciphertext[0][2].get(), ciphertext[0][3].get(),
    ///         ciphertext[1][0].get(), ciphertext[1][1].get(), ciphertext[1][2].get(), ciphertext[1][3].get(),
    ///         ciphertext[2][0].get(), ciphertext[2][1].get(), ciphertext[2][2].get(), ciphertext[2][3].get());
    /// assert_eq!(ciphertext[0][0].get(), 0x27584D87);
    /// assert_eq!(ciphertext[0][1].get(), 0x44E2BAE9);
    /// assert_eq!(ciphertext[0][2].get(), 0x4AECB5D6);
    /// assert_eq!(ciphertext[0][3].get(), 0x01CCF826);
    /// assert_eq!(ciphertext[1][0].get(), 0x27584D87);
    /// assert_eq!(ciphertext[1][1].get(), 0x44E2BAE9);
    /// assert_eq!(ciphertext[1][2].get(), 0x4AECB5D6);
    /// assert_eq!(ciphertext[1][3].get(), 0x01CCF826);
    /// assert_eq!(ciphertext[2][0].get(), 0x27584D87);
    /// assert_eq!(ciphertext[2][1].get(), 0x44E2BAE9);
    /// assert_eq!(ciphertext[2][2].get(), 0x4AECB5D6);
    /// assert_eq!(ciphertext[2][3].get(), 0x01CCF826);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.encrypt_array_unit)
    pub fn encrypt_array_unit<const N: usize>(&mut self, message: &[[IntUnion; NB]; N], cipher: &mut [[IntUnion; NB]; N])
    {
        for i in 0..N
        {
            self.set_block(&message[i]);
            self.encrypt_block();
            cipher[i] = self.get_block();
        }
    }

    // pub fn encrypt_array_u128<const N: usize>(&mut self, message: &[u128; N], cipher: &mut [u128; N])
    /// Encrypts an array of 128-bit data.
    ///
    /// # Arguments
    /// - `message` is of an array of `u128`-type and the plaintext
    ///   to be encrypted.
    /// - `cipher` is of an array of `u128`-type and the ciphertext
    ///   to be stored.
    ///
    /// # Features
    /// This method encrypts multiple of 128-bit data without padding anything
    /// in ECB (Electronic CodeBook) mode.
    /// 
    /// # Counterpart Methods
    /// For each trait
    /// [`ECB_PKCS7`](symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
    /// [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
    /// [`CBC_PKCS7`](symmetric/trait.CBC_PKCS7.html#trait.ECB_PKCS7),
    /// [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
    /// [`PCBC_PKCS7`](symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
    /// [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO).
    /// [`CFB`](symmetric/trait.CFB.html#trait.CFB),
    /// [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
    /// [`CTR`](symmetric/trait.CTR.html#trait.CTR),
    /// there are provided useful counterpart methods:
    /// encrypt(), encrypt_into_vec(), encrypt_into_array(),
    /// encrypt_str(), encrypt_str_into_vec(), encrypt_str_into_array(),
    /// encrypt_string(), encrypt_string_into_vec(), encrypt_string_into_array(),
    /// encrypt_vec(), encrypt_vec_into_vec(), encrypt_vec_into_array(),
    /// encrypt_array(), encrypt_array_into_vec(), and encrypt_array_into_array().
    /// 
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_128;
    /// 
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let plaintext = [0x1234567890ABCDEF1234567890ABCDEF_u128, 0x1234567890ABCDEF1234567890ABCDEF, 0x1234567890ABCDEF1234567890ABCDEF];
    /// let mut ciphertext = [0_u128; 3];
    /// aes.encrypt_array_u128(&plaintext, &mut ciphertext);
    /// 
    /// println!("Plaintext:\t{:#034X} {:#034X} {:#034X}", plaintext[0], plaintext[1], plaintext[2]);
    /// println!("Ciphertext:\t{:#034X} {:#034X} {:#034X}", ciphertext[0], ciphertext[1], ciphertext[2]);
    /// assert_eq!(ciphertext[0], 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    /// assert_eq!(ciphertext[1], 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    /// assert_eq!(ciphertext[2], 0x01CCF8264AECB5D644E2BAE927584D87_u128);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.encrypt_array_u128)
    pub fn encrypt_array_u128<const N: usize>(&mut self, message: &[u128; N], cipher: &mut [u128; N])
    {
        for i in 0..N
        {
            self.set_block_u128(message[i]);
            self.encrypt_block();
            cipher[i] = self.get_block_u128();
        }
    }

    // pub fn decrypt_array_unit<const N: usize>(&mut self, cipher: &[[IntUnion; NB]; N], message: &mut [[IntUnion; NB]; N])
    /// Decrypts an array of unit data, `[[IntUnion; NB]; N]`.
    ///
    /// # Arguments
    /// - `cipher` is of an array of `[IntUnion; NB]`-type and the ciphertext
    ///   to be encrypted.
    /// - `message` is of an array of `[IntUnion; NB]`-type and the plaintext
    ///   to be stored.
    ///
    /// # Features
    /// This method decrypts multiple of 64-bit data without padding anything
    /// in ECB (Electronic CodeBook) mode.
    /// 
    /// # Counterpart Methods
    /// For each trait
    /// [`ECB_PKCS7`](symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
    /// [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
    /// [`CBC_PKCS7`](symmetric/trait.CBC_PKCS7.html#trait.ECB_PKCS7),
    /// [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
    /// [`PCBC_PKCS7`](symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
    /// [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO).
    /// [`CFB`](symmetric/trait.CFB.html#trait.CFB),
    /// [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
    /// [`CTR`](symmetric/trait.CTR.html#trait.CTR),
    /// there are provided useful counterpart methods:
    /// decrypt(), decrypt_into_vec(), decrypt_into_array(),
    /// decrypt_into_string(),
    /// decrypt_vec(), decrypt_vec_into_vec(), decrypt_vec_into_array(),
    /// decrypt_vec_into_string(),
    /// decrypt_array(), decrypt_array_into_vec(), decrypt_array_into_array(),
    /// and decrypt_array_into_string().
    /// 
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_128;
    /// 
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let ciphertext = [[IntUnion::new_with(0x27584D87), IntUnion::new_with(0x44E2BAE9), IntUnion::new_with(0x4AECB5D6), IntUnion::new_with(0x01CCF826)]; 3];
    /// let mut plaintext = [[IntUnion::new(); 4]; 3];
    /// aes.decrypt_array_unit(&ciphertext, &mut plaintext);
    /// 
    /// println!("Ciphertext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
    ///         ciphertext[0][0].get(), ciphertext[0][1].get(), ciphertext[0][2].get(), ciphertext[0][3].get(),
    ///         ciphertext[1][0].get(), ciphertext[1][1].get(), ciphertext[1][2].get(), ciphertext[1][3].get(),
    ///         ciphertext[2][0].get(), ciphertext[2][1].get(), ciphertext[2][2].get(), ciphertext[2][3].get());
    /// println!("Plaintext:\t{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
    ///         plaintext[0][0].get(), plaintext[0][1].get(), plaintext[0][2].get(), plaintext[0][3].get(),
    ///         plaintext[1][0].get(), plaintext[1][1].get(), plaintext[1][2].get(), plaintext[1][3].get(),
    ///         plaintext[2][0].get(), plaintext[2][1].get(), plaintext[2][2].get(), plaintext[2][3].get());
    /// assert_eq!(plaintext[0][0].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[0][1].get(), 0x12345678);
    /// assert_eq!(plaintext[0][2].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[0][3].get(), 0x12345678);
    /// assert_eq!(plaintext[1][0].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[1][1].get(), 0x12345678);
    /// assert_eq!(plaintext[1][2].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[1][3].get(), 0x12345678);
    /// assert_eq!(plaintext[2][0].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[2][1].get(), 0x12345678);
    /// assert_eq!(plaintext[2][2].get(), 0x90ABCDEF);
    /// assert_eq!(plaintext[2][3].get(), 0x12345678);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.decrypt_array_unit)
    pub fn decrypt_array_unit<const N: usize>(&mut self, cipher: &[[IntUnion; NB]; N], message: &mut [[IntUnion; NB]; N])
    {
        for i in 0..N
        {
            self.set_block(&cipher[i]);
            self.decrypt_block();
            message[i] = self.get_block();
        }
    }

    // pub fn decrypt_array_u128<const N: usize>(&mut self, cipher: &[u128; N], message: &mut [u128; N])
    /// Decrypts an array of 128-bit data.
    ///
    /// # Arguments
    /// - `cipher` is of an array of `u128`-type and the ciphertext to be
    ///   decrypted.
    /// - `message` is of an array of `u128`-type and the plaintext to be stored.
    ///
    /// # Features
    /// This method decrypts multiple of 64-bit data without padding anything
    /// in ECB (Electronic CodeBook) mode.
    /// 
    /// # Counterpart Methods
    /// For each trait
    /// [`ECB_PKCS7`](symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
    /// [`ECB_ISO`](symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
    /// [`CBC_PKCS7`](symmetric/trait.CBC_PKCS7.html#trait.ECB_PKCS7),
    /// [`CBC_ISO`](symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
    /// [`PCBC_PKCS7`](symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
    /// [`PCBC_ISO`](symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO).
    /// [`CFB`](symmetric/trait.CFB.html#trait.CFB),
    /// [`OFB`](symmetric/trait.OFB.html#trait.OFB), and
    /// [`CTR`](symmetric/trait.CTR.html#trait.CTR),
    /// there are provided useful counterpart methods:
    /// decrypt(), decrypt_into_vec(), decrypt_into_array(),
    /// decrypt_into_string(),
    /// decrypt_vec(), decrypt_vec_into_vec(), decrypt_vec_into_array(),
    /// decrypt_vec_into_string(),
    /// decrypt_array(), decrypt_array_into_vec(), decrypt_array_into_array(),
    /// and decrypt_array_into_string().
    /// 
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::number::IntUnion;
    /// use cryptocol::symmetric::AES_128;
    /// 
    /// let mut aes = AES_128::new_with_key(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    /// let ciphertext = [0x01CCF8264AECB5D644E2BAE927584D87_u128, 0x01CCF8264AECB5D644E2BAE927584D87_u128, 0x01CCF8264AECB5D644E2BAE927584D87_u128];
    /// let mut plaintext = [0_u128; 3];
    /// aes.decrypt_array_u128(&ciphertext, &mut plaintext);
    /// 
    /// println!("Ciphertext:\t{:#034X} {:#034X} {:#034X}", ciphertext[0], ciphertext[1], ciphertext[2]);
    /// println!("Plaintext:\t{:#034X} {:#034X} {:#034X}", plaintext[0], plaintext[1], plaintext[2]);
    /// assert_eq!(plaintext[0], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(plaintext[1], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// assert_eq!(plaintext[2], 0x1234567890ABCDEF1234567890ABCDEF_u128);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.decrypt_array_u128)
    pub fn decrypt_array_u128<const N: usize>(&mut self, cipher: &[u128; N], message: &mut [u128; N])
    {
        for i in 0..N
        {
            self.set_block_u128(cipher[i]);
            self.decrypt_block();
            message[i] = self.get_block_u128();
        }
    }

    // pub fn is_succeful(&self) -> bool
    /// Checks whether the previous encryption or decryption was successful.
    ///
    /// # Output
    /// If the previous encryption or decryption was successful, this method
    /// returns true. Otherwise, it returns false.
    ///
    /// # Features
    /// - Usually, you don't have to use this method because the encryption
    ///   methods returns the length of ciphertext and the decryption methods
    ///   returns the length of plaintext but they returns `0` when they failed.
    /// - If the ciphertext is 8 bytes for decryption with the padding either
    ///   pkcs7 or iso, the return value `0` of the decryption methods is not
    ///   discriminatory. You don't know whether the previous decryption was
    ///   failed or the original plaintext was just null string or "". In this
    ///   case you can check its success with this method.
    ///
    /// # Example 1 for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
    /// println!("K =\t{:#034X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// let len = a_aes.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 16);
    /// let success = a_aes.is_successful();
    /// assert_eq!(success, true);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "26 F2 F8 B7 B7 FD 46 9A 97 97 F3 24 E7 51 99 47 ");
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.is_successful)
    #[inline]
    pub fn is_successful(&self) -> bool
    {
        self.block[0][0] == Self::SUCCESS
    }

    // pub fn is_failed(&self) -> bool
    /// Checks whether the previous encryption or decryption was failed.
    ///
    /// # Output
    /// If the previous encryption or decryption was failed, this method
    /// returns true. Otherwise, it returns false.
    ///
    /// # Features
    /// - Usually, you don't have to use this method because the encryption
    ///   methods returns the length of ciphertext and the decryption methods
    ///   returns the length of plaintext but they returns `0` when they failed.
    /// - If the ciphertext is 8 bytes for decryption with the padding either
    ///   pkcs7 or iso, the return value `0` of the decryption methods is not
    ///   discriminatory. You don't know whether the previous decryption was
    ///   failed or the original plaintext was just null string or "". In this
    ///   case you can check its success with this method.
    ///
    /// # Example 1 for the message of 0 bytes
    /// ```
    /// use std::io::Write;
    /// use std::fmt::Write as _;
    /// use cryptocol::symmetric::{ AES_128, ECB_PKCS7 };
    /// 
    /// let key = 0xEFCDAB9078563412EFCDAB9078563412_u128;
    /// println!("K =\t{:#034X}", key);
    /// let mut a_aes = AES_128::new_with_key_u128(key);
    /// let message = "";
    /// println!("M =\t{}", message);
    /// let mut cipher = [0_u8; 16];
    /// let len = a_aes.encrypt_into_array(message.as_ptr(), message.len() as u64, &mut cipher);
    /// println!("The length of ciphertext = {}", len);
    /// assert_eq!(len, 16);
    /// let failure = a_aes.is_failed();
    /// assert_eq!(failure, false);
    /// print!("C =\t");
    /// for c in cipher.clone()
    ///     { print!("{:02X} ", c); }
    /// println!();
    /// let mut txt = String::new();
    /// for c in cipher.clone()
    ///     { write!(txt, "{:02X} ", c); }
    /// assert_eq!(txt, "26 F2 F8 B7 B7 FD 46 9A 97 97 F3 24 E7 51 99 47 ");
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.is_failed)
    #[inline]
    pub fn is_failed(&self) -> bool
    {
        self.block[0][0] == Self::FAILURE
    }

    // pub(super) fn set_successful(&mut self)
    /// Sets the flag to mean that the previous encryption or decryption
    /// was successful.
    ///
    /// # Features
    /// You won't use this method unless you write codes for implementation
    /// of a trait for BigCryptor128.
    #[inline]
    pub(super) fn set_successful(&mut self)
    {
        self.block[0][0] = Self::SUCCESS;
    }

    // pub(super) fn set_failed(&mut self)
    /// Sets the flag to mean that the previous encryption or decryption
    /// was failed.
    ///
    /// # Features
    /// You won't use this method unless you write codes for implementation
    /// of a trait for BigCryptor64 or NDES.
    #[inline]
    pub(super) fn set_failed(&mut self)
    {
        self.block[0][0] = Self::FAILURE;
    }

    // pub fn get_desirable_round() -> usize
    /// Returns the desirable number of rounds
    /// according to the Rijndael documents
    /// 
    /// # Example 1 for AES_128
    /// ```
    /// use cryptocol::symmetric::AES_128;
    /// let rounds = AES_128::get_desirable_round();
    /// println!("The desirable number of rounds of AES_128 is {}", rounds);
    /// assert_eq!(rounds, 10);
    /// ```
    ///
    /// # For more examples,
    /// click [here](./documentation/rijndael_basic/struct.Rijndael_Generic.html#method.get_desirable_round)
    #[inline]
    pub fn get_desirable_round() -> usize
    {
        6 + if NB > NK { NB } else { NK }
    }

    fn encrypt_block(&mut self)
    {
        self.add_round_key(0);
        for round in 1..ROUND
        {
            self.sub_bytes();
            Self::method_shift_rows(self);
            Self::method_mix_columns(self);
            self.add_round_key(round);
        }
        self.sub_bytes();
        Self::method_shift_rows(self);
        self.add_round_key(ROUND);
    }

    fn sub_bytes(&mut self)
    {
        for i in 0..4
        {
            for j in 0..NB
            {
                self.block[i][j] = Self::SBOX[self.block[i][j] as usize];
            }
        }
    }

    fn optimal_shift_rows(&mut self)
    {
        let mut tmp = [0_u8; 3];
        let tmp_ptr = tmp.as_mut_ptr();
        for i in 1..4
        {
            unsafe {
                let j = i % NB;
                let ptr_block_0 = self.block[i].as_mut_ptr();
                let ptr_block_i = self.block[i].as_ptr().add(j);
                let ptr_block_nb_i = self.block[i].as_mut_ptr().add(NB - j);
                copy_nonoverlapping(ptr_block_0, tmp_ptr, j);
                copy(ptr_block_i, ptr_block_0, NB - j);
                copy_nonoverlapping(tmp_ptr, ptr_block_nb_i, j);
            }
        }
    }

    fn shift_rows(&mut self)
    {
        let mut tmp = [0_u8; NB];
        let tmp_ptr = tmp.as_mut_ptr();
        for i in 0..4
        {
            unsafe {
                let j = Self::SR[i] % NB;
                let ptr_block_0 = self.block[i].as_mut_ptr();
                let ptr_block_i = self.block[i].as_ptr().add(j);
                let ptr_block_nb_i = self.block[i].as_mut_ptr().add(NB - j);
                copy_nonoverlapping(ptr_block_0, tmp_ptr, j);
                copy(ptr_block_i, ptr_block_0, NB - j);
                copy_nonoverlapping(tmp_ptr, ptr_block_nb_i, j);
            }
        }
    }

    fn optimal_mix_columns(&mut self)
    {
        let mut new_block = [[0_u8; NB]; 4];
        for col in 0..NB
        {
            new_block[0][col] = GF_mul!(2, self.block[0][col]) ^ GF_mul!(3, self.block[1][col])
                                ^ self.block[2][col] ^ self.block[3][col];
            new_block[1][col] = self.block[0][col] ^ GF_mul!(2, self.block[1][col])
                                ^ GF_mul!(3, self.block[2][col]) ^ self.block[3][col];
            new_block[2][col] = self.block[0][col] ^ self.block[1][col]
                                ^ GF_mul!(2, self.block[2][col]) ^ GF_mul!(3, self.block[3][col]);
            new_block[3][col] = GF_mul!(3, self.block[0][col]) ^ self.block[1][col]
                                ^ self.block[2][col] ^ GF_mul!(2, self.block[3][col]);
        }
        self.block = new_block;
    }

    fn mix_columns(&mut self)
    {
        let mut new_block = [[0_u8; NB]; 4];
        for row in 0..4
        {
            for col in 0..NB
            {
                for i in 0..4
                {
                    new_block[row][col] ^= GF_mul!(Self::MC[row][i], self.block[i][col]);
                }
            }
        }
        self.block = new_block;
    }

    fn add_round_key(&mut self, round: usize)
    {
        for i in 0..4
        {
            for j in 0..NB
            {
                self.block[i][j] ^= self.round_key[round][j].get_ubyte_(i);
            }
        }
    }

    fn decrypt_block(&mut self)
    {
        self.add_round_key(ROUND);
        let mut round = ROUND-1;
        while round > 0
        {
            Self::method_inv_shift_rows(self);
            self.inv_sub_bytes();
            self.add_round_key(round);
            self.inv_mix_columns();
            round -= 1;
        }
        Self::method_inv_shift_rows(self);
        self.inv_sub_bytes();
        self.add_round_key(round);
    }

    fn inv_sub_bytes(&mut self)
    {
        for i in 0..4
        {
            for j in 0..NB
            {
                self.block[i][j] = Self::INV_SBOX[self.block[i][j] as usize];
            }
        }
    }

    fn optimal_inv_shift_rows(&mut self)
    {
        let mut tmp = [0_u8; 3];
        let tmp_ptr = tmp.as_mut_ptr();
        for i in 1..4
        {
            unsafe {
                let j = i % NB;
                let ptr_block_0 = self.block[i].as_mut_ptr();
                let ptr_block_i = self.block[i].as_mut_ptr().add(j);
                let ptr_block_nb_i = self.block[i].as_ptr().add(NB - j);
                copy_nonoverlapping(ptr_block_nb_i, tmp_ptr, j);
                copy(ptr_block_0, ptr_block_i, NB - j);
                copy_nonoverlapping(tmp_ptr, ptr_block_0, j);
            }
        }
    }

    fn inv_shift_rows(&mut self)
    {
        let mut tmp = [0_u8; NB];
        let tmp_ptr = tmp.as_mut_ptr();
        for i in 0..4
        {
            unsafe {
                let j = Self::SR[i] % NB;
                let ptr_block_0 = self.block[i].as_mut_ptr();
                let ptr_block_i = self.block[i].as_mut_ptr().add(j);
                let ptr_block_nb_i = self.block[i].as_ptr().add(NB - j);
                copy_nonoverlapping(ptr_block_nb_i, tmp_ptr, j);
                copy(ptr_block_0, ptr_block_i, NB - j);
                copy_nonoverlapping(tmp_ptr, ptr_block_0, j);
            }
        }
    }

    fn inv_mix_columns(&mut self)
    {
        let mut new_block = [[0_u8; NB]; 4];
        for row in 0..4
        {
            for col in 0..NB
            {
                for i in 0..4
                {
                    new_block[row][col] ^= GF_mul!(Self::INV_MC[row][i], self.block[i][col]);
                }
            }
        }
        self.block = new_block;
    }

    fn make_round_keys_nk_up_to_6_and_nk_equal_to_nb(&mut self)
    {
        self.set_zeroth_round_key();
        for round in 1..=ROUND
        {
            let mut tmp = self.round_key[round-1][NB-1];
            #[cfg(target_endian = "little")] { tmp = tmp.rotate_right(8 * ROT); }
            #[cfg(target_endian = "big")] { tmp = tmp.rotate_left(8 * ROT); }
            for j in 0..4
                { tmp.set_ubyte_(j, Self::SBOX[tmp.get_ubyte_(j) as usize]); }
            tmp.set(tmp.get() ^ Self::RC[round-1]);
            self.round_key[round][0] = tmp ^ self.round_key[round-1][0];
            for i in 1..NB
                { self.round_key[round][i] = self.round_key[round][i-1] ^ self.round_key[round-1][i]; }
        }
    }

    fn make_round_keys_nk_greater_than_6_and_nk_equal_to_nb(&mut self)
    {
        self.set_zeroth_round_key();
        for round in 1..=ROUND
        {
            let mut tmp = self.round_key[round-1][NB-1];
            #[cfg(target_endian = "little")] { tmp = tmp.rotate_right(8 * ROT); }
            #[cfg(target_endian = "big")] { tmp = tmp.rotate_left(8 * ROT); }
            for j in 0..4
                { tmp.set_ubyte_(j, Self::SBOX[tmp.get_ubyte_(j) as usize]); }
            tmp.set(tmp.get() ^ Self::RC[round-1]);
            self.round_key[round][0] = tmp ^ self.round_key[round-1][0];
            for i in 1..3
                { self.round_key[round][0] = self.round_key[round][i-1] ^ self.round_key[round-1][i]; }
            tmp = self.round_key[round][3];
            for j in 0..4
                { tmp.set_ubyte_(j, Self::SBOX[tmp.get_ubyte_(j) as usize]); }
            self.round_key[round][4] = tmp ^ self.round_key[round-1][4];
            for i in 5..NB
                { self.round_key[round][i] = self.round_key[round][i-1] ^ self.round_key[round-1][i]; }
        }
    }

    fn make_round_keys_nk_up_to_6_and_nk_diff_from_nb(&mut self)
    {
        self.set_zeroth_round_key();
        let mut round = NK / NB;
        let mut cc = NK % NB;
        let mut idx = NK;
        let mut rc_round = 0;
        while (round <= ROUND) && (rc_round < ROUND)
        {
            let mut tmp = if cc == 0 { self.round_key[round-1][NB-1] } else { self.round_key[round][cc-1] };
            if idx % NK == 0
            {
                #[cfg(target_endian = "little")] { tmp = tmp.rotate_right(8 * ROT); }
                #[cfg(target_endian = "big")] { tmp = tmp.rotate_left(8 * ROT); }
                for j in 0..4
                    { tmp.set_ubyte_(j, Self::SBOX[tmp.get_ubyte_(j) as usize]); }
                tmp.set(tmp.get() ^ Self::RC[rc_round]);
                rc_round += 1;
            }
            if cc < NK
            {
                let rrr = (idx - NK) / NB;
                let ccc = (idx - NK) % NB;
                self.round_key[round][cc] = tmp ^ self.round_key[rrr][ccc];
            }
            else
            {
                self.round_key[round][cc] = tmp ^ self.round_key[round][cc-NK];
            }
            if cc == NB - 1
            {
                cc = 0;
                round += 1;
            }
            else
            {
                cc += 1;
            }
            idx += 1;
        }
    }

    fn make_round_keys_nk_greater_than_6_and_nk_diff_from_nb(&mut self)
    {
        self.set_zeroth_round_key();
        let mut round = NK / NB;
        let mut cc = NK % NB;
        let mut idx = NK;
        let mut rc_round = 0;
        while (round <= ROUND) && (rc_round < ROUND)
        {
            let mut tmp = if cc == 0 { self.round_key[round-1][NB-1] } else { self.round_key[round][cc-1] };
            if idx % NK == 0
            {
                #[cfg(target_endian = "little")] { tmp = tmp.rotate_right(8 * ROT); }
                #[cfg(target_endian = "big")] { tmp = tmp.rotate_left(8 * ROT); }
                for j in 0..4
                    { tmp.set_ubyte_(j, Self::SBOX[tmp.get_ubyte_(j) as usize]); }
                tmp.set(tmp.get() ^ Self::RC[rc_round]);
                rc_round += 1;
            }
            else if idx % NK == 4
            {
                let rrr = (idx - NK) / NB;
                let ccc = (idx - NK) % NB;
                for j in 0..4
                    { tmp.set_ubyte_(j, Self::SBOX[tmp.get_ubyte_(j) as usize]); }
                self.round_key[round][cc] = tmp ^ self.round_key[rrr][ccc];
            }
            if cc < NK
            {
                let rrr = (idx - NK) / NB;
                let ccc = (idx - NK) % NB;
                self.round_key[round][cc] = tmp ^ self.round_key[rrr][ccc];
            }
            else
            {
                self.round_key[round][cc] = tmp ^ self.round_key[round][cc-NK];
            }
            if cc < NB - 1
            {
                cc += 1;
            }
            else
            {
                cc = 0;
                round += 1;
            }
            idx += 1;
        }
    }

    #[inline]
    fn set_zeroth_round_key(&mut self)
    {
        unsafe {
            copy_nonoverlapping(self.key.as_ptr() as *const u8,
                                self.round_key.as_mut_ptr() as *mut u8, NK * 4);
        }
    }

    fn get_block(&self) -> [IntUnion; NB]
    {
        let mut block = [IntUnion::new(); NB];
        for j in 0..NB
        {
            for i in 0..4
            {
                block[j].set_ubyte_(i, self.block[i][j]);
            }
        }
        block
    }

    fn get_block_u128(&self) -> u128
    {
        let nb = if 4 < NB {4} else {NB};
        let mut block = LongerUnion::new();
        let mut idx = 0;
        for j in 0..nb
        {
            for i in 0..4
            {
                block.set_ubyte_(idx, self.block[i][j]);
                idx += 1;
            }
        }
        block.get()
    }

    fn get_block_u64(&self) -> u64
    {
        let nb = if 2 < NB {2} else {NB};
        let mut block = LongUnion::new();
        let mut idx = 0;
        for j in 0..nb
        {
            for i in 0..4
            {
                block.set_ubyte_(idx, self.block[i][j]);
                idx += 1;
            }
        }
        block.get()
    }

    fn get_block_u32(&self) -> u32
    {
        let mut block = IntUnion::new();
        let mut idx = 0;
        for i in 0..4
        {
            block.set_ubyte_(idx, self.block[i][0]);
            idx += 1;
        }
        block.get()
    }

    fn set_block(&mut self, block: &[IntUnion; NB])
    {
        let mut idx = 0;
        for j in 0..NB
        {
            for i in 0..4
            {
                self.block[i][j] = block[j].get_ubyte_(i);
                idx += 1;
            }
        }
    }

    fn set_block_u128(&mut self, block: u128)
    {
        let nb = if 4 < NB {4} else {NB};
        let block_union = LongerUnion::new_with(block);
        let mut idx = 0;
        for j in 0..nb
        {
            for i in 0..4
            {
                self.block[i][j] = block_union.get_ubyte_(idx);
                idx += 1;
            }
        }
    }

    fn set_block_u64(&mut self, block: u64)
    {
        let nb = if 2 < NB {2} else {NB};
        let block_union = LongUnion::new_with(block);
        let mut idx = 0;
        for j in 0..nb
        {
            for i in 0..4
            {
                self.block[i][j] = block_union.get_ubyte_(idx);
                idx += 1;
            }
        }
    }

    fn set_block_u32(&mut self, block: u32)
    {
        let block_union = IntUnion::new_with(block);
        let mut idx = 0;
        for i in 0..4
        {
            self.block[i][0] = block_union.get_ubyte_(idx);
            idx += 1;
        }
    }

    // fn GF_mul(mut a: u8, mut b: u8, m: u8) -> u8
    // {
    //     let mut ret = 0_u8;
    //     while b != 0
    //     {
    //         if b & 1 == 1
    //             { ret ^= a; }
    //         if a & 0b1000_0000 != 0
    //             { a = (a << 1) ^ m; }
    //         else
    //             { a <<= 1; }
    //         b >>= 1;
    //     }
    //     ret
    // }


    /////// Testing Codes during Development //////
    // #[allow(non_snake_case)]
    // pub fn show_SBox()
    // {
    //     for i in 0..256
    //         { println!("{:02X} => {:02X}", i, Self::SBOX[i]); }
    // }

    // #[allow(non_snake_case)]
    // pub fn show_InvSBox()
    // {
    //     for i in 0..256
    //         { println!("{:02X} => {:02X}", i, Self::INV_SBOX[i]); }
    // }

    // #[allow(non_snake_case)]
    // pub fn show_MC()
    // {
    //     println!("MC is as follows:");
    //     for i in 0..4
    //     {
    //         for j in 0..4
    //             { print!("{:02x} ", Self::MC[i][j]); }
    //         println!();
    //     }
    // }

    // #[allow(non_snake_case)]
    // pub fn show_InvMC()
    // {
    //     println!("Inverse MC is as follows:");
    //     for i in 0..4
    //     {
    //         for j in 0..4
    //             { print!("{:02x} ", Self::INV_MC[i][j]); }
    //         println!();
    //     }
    // }

    // #[allow(non_snake_case)]
    // pub fn show_RC()
    // {
    //     for i in 0..ROUND
    //         { println!("{:02} => {:02X}", i, Self::RC[i]); }
    // }
}