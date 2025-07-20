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
use crate::number::{ SmallUInt, IntUnion, LongerUnion, SharedArrays };


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

#[allow(non_camel_case_types)]
pub type Rijndael_256_256 = Rijndael_Generic<14, 8, 8>;

#[allow(non_camel_case_types)]
pub type Rijndael_256_192 = Rijndael_Generic<14, 8, 6>;

#[allow(non_camel_case_types)]
pub type Rijndael_256_128 = Rijndael_Generic<14, 8, 4>;

#[allow(non_camel_case_types)]
pub type Rijndael_192_256 = Rijndael_Generic<14, 6, 8>;

#[allow(non_camel_case_types)]
pub type Rijndael_192_192 = Rijndael_Generic<12, 6, 6>;

#[allow(non_camel_case_types)]
pub type Rijndael_192_128 = Rijndael_Generic<12, 6, 4>;

#[allow(non_camel_case_types)]
pub type Rijndael_128_256 = Rijndael_Generic<14, 4, 8>;

#[allow(non_camel_case_types)]
pub type Rijndael_128_192 = Rijndael_Generic<12, 4, 6>;

#[allow(non_camel_case_types)]
pub type Rijndael_128_128 = Rijndael_Generic;

#[allow(non_camel_case_types)]
pub type AES_256 = Rijndael_Generic<14, 4, 8>;

#[allow(non_camel_case_types)]
pub type AES_192 = Rijndael_Generic<12, 4, 6>;

#[allow(non_camel_case_types)]
pub type AES_128 = Rijndael_Generic;


/// A Rijndael or AES symmetric-key algorithm for the encryption of digital data
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
/// not known that the name Rijndael was made with that intention.
/// It is the symmetric key
/// encryption/decryption algorithm. It was originally developed based on
/// Lucifer encryption/decryption algorithm made by IBM. DES was approved as a 
/// federal standard in November 1976.
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
/// You have to import (use) one of the modules `AES_128`, `AES_192`, and
/// `AES_256` in order to use official AES as shown in Example 1.
/// 
/// # Example 1
/// ```
/// use cryptocol::symmetric::AES_128;
/// use cryptocol::symmetric::AES_192;
/// use cryptocol::symmetric::AES_256;
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
/// Also, you can instantiate the AES object with `[u8; 16]` key as shown in
/// Example 3. In this case, you don't have to take endianness into account.
/// The instantiated object should be mutable.
/// 
/// # Example 3
/// ```
/// use cryptocol::symmetric::AES_128;
/// let key = [0xEFu8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12];
/// let mut _a_aes = AES_128::new_with_key(key);
/// ```
/// 
/// You can instantiate the AES object without key and set a `u128` key later as
/// shown in Example 4 or a `[u8; 16]` key later as shown in Example 5.
/// 
/// # Example 4
/// ```
/// use cryptocol::symmetric::AES_128;
/// let mut a_des = AES_128::new();
/// let key = 0x_1234567890ABCDEF1234567890ABCDEF_u128;
/// a_des.set_key_u128(key);
/// ```
/// 
/// # Example 5
/// ```
/// use cryptocol::symmetric::AES_128;
/// let mut a_des = AES_128::new();
/// let key = [0xEFu8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12];
/// a_des.set_key(key);
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
/// let mut a_aes = DES::new_with_key([0xEFu8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
/// let message = "In the beginning God created the heavens and the earth.";
/// println!("M =\t{}", message);
/// let iv = 0x_FEDCBA0987654321FEDCBA0987654321_u128;
/// println!("IV =\t{}", iv);
/// let mut cipher = Vec::<u8>::new();
/// a_aes.encrypt_str_into_vec(iv, message, &mut cipher);
/// print!("C =\t");
/// for c in cipher.clone()
///     { print!("{:02X} ", c); }
/// println!();
/// let mut txt = String::new();
/// for c in cipher.clone()
///     { write!(txt, "{:02X} ", c); }
/// assert_eq!(txt, "4B B5 ED DC A0 58 7E 6D 6C 3B A2 00 38 C3 D4 29 42 B1 CF 0D E9 FA EA 11 11 6B C8 30 73 39 DD B7 3F 96 9B A3 76 05 34 7E 64 2F D4 CC B2 68 33 64 C5 9E EF 01 A9 4A FD 5B ");
/// 
/// let mut recovered = String::new();
/// a_aes.decrypt_vec_into_string(iv, &cipher, &mut recovered);
/// println!("B (16 rounds) =\t{}", recovered);
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
/// let mut a_rijndael = Rijndael_Generic::<22, 16, 16>::new_with_key_u128([0xEFu8, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]);
/// let message = "In the beginning God created the heavens and the earth.";
/// println!("M =\t{}", message);
/// let iv = [0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32, 0x_FEDCBA09_u32, 87654321_u32];
/// println!("IV =\t{}", iv);
/// let mut cipher = Vec::<u8>::new();
/// a_des.encrypt_str_into_vec(iv, message.as_ptr(), message.len() as u64, &mut cipher);
/// print!("C =\t");
/// for c in cipher.clone()
///     { print!("{:02X} ", c); }
/// println!();
/// let mut txt = String::new();
/// for c in cipher.clone()
///     { write!(txt, "{:02X} ", c); }
/// assert_eq!(txt, "0B EA 6B BC 68 F9 B0 3E 7D AF DE 71 9C 08 AA 16 42 40 1C C8 DC 40 51 C6 8D D4 E7 D2 0B A4 F2 09 02 02 C2 6E 99 BC 9E 2A F4 11 7E 48 A7 ED 76 70 C6 9D C6 BD A6 9B 58 8B ");
/// 
/// let mut recovered = String::new();
/// a_des.decrypt_vec_into_string(iv, &cipher, &mut recovered);
/// println!("B =\t{}", recovered);
/// assert_eq!(recovered, "In the beginning God created the heavens and the earth.");
/// assert_eq!(recovered, message);
/// ```
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
    enc:        fn (s: &mut Self, message: u128) -> u128,
    dec:        fn (s: &mut Self, cipher: u128) -> u128,
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

    pub fn new() -> Self
    {
        let mut rijndael = Self
        {
            key:        [IntUnion::new(); NK],
            block:      [[0_u8; NB]; 4],
            round_key:  vec![[IntUnion::new(); NB]; ROUND + 1],
            enc:        Self::encrypt_u128,
            dec:        Self::decrypt_u128,
        };
        Self::method_make_round_keys(&mut rijndael);
        rijndael
    }

    pub fn new_with_key<const K: usize>(key: [u8; K]) -> Self
    {
        let mut rijndael = Self
        {
            key:        [IntUnion::new(); NK],
            block:      [[0_u8; NB]; 4],
            round_key:  vec![[IntUnion::new(); NB]; ROUND + 1],
            enc:        Self::encrypt_u128,
            dec:        Self::decrypt_u128,
        };
        rijndael.set_key(key);
        rijndael
    }

    pub fn new_with_key_u128(key: u128) -> Self
    {
        let mut rijndael = Self
        {
            key:        [IntUnion::new(); NK],
            block:      [[0_u8; NB]; 4],
            round_key:  vec![[IntUnion::new(); NB]; ROUND + 1],
            enc:        Self::encrypt_u128,
            dec:        Self::decrypt_u128,
        };
        rijndael.set_key_u128(key);
        rijndael
    }

    #[inline]
    pub fn encryptor_with_key<const K: usize>(key: [u8; K]) -> Self
    {
        Self::new_with_key(key)
    }

    #[inline]
    pub fn encryptor_with_key_u128(key: u128) -> Self
    {
        Self::new_with_key_u128(key)
    }

    pub fn decryptor_with_key<const K: usize>(key: [u8; K]) -> Self
    {
        let mut rijndael = Self::new_with_key(key);
        rijndael.turn_inverse();
        rijndael
    }

    pub fn decryptor_with_key_u128(key: u128) -> Self
    {
        let mut rijndael = Self::new_with_key_u128(key);
        rijndael.turn_inverse();
        rijndael
    }

    pub fn get_key(&mut self) -> [u32; NK]
    {
        let mut key = [0_u32; NK];
        unsafe { copy_nonoverlapping(self.key.as_ptr() as *const u32,
                                 &mut key as *mut u32, NK); }
        key
    }

    pub fn get_key_u128(&self) -> u128
    {
        let mut key = 0_u128;
        unsafe { copy_nonoverlapping(self.key.as_ptr() as *const u8,
                                     &mut key as *mut u128 as *mut u8, 16); }
        key
    }

    pub fn set_key<const K: usize>(&mut self, key: [u8; K])
    {
        let len = if K < NK * 4 { K } else { NK * 4 };
        unsafe {
            copy_nonoverlapping(key.as_ptr(), self.key.as_mut_ptr() as *mut u8, len);
        }
        Self::method_make_round_keys(self);
    }

    pub fn set_key_u128(&mut self, key: u128)
    {
        unsafe {
            copy_nonoverlapping(&key as *const u128 as *const u8,
                                self.key.as_mut_ptr() as *mut u8, 16);
        }
        Self::method_make_round_keys(self);
    }

    #[inline]
    pub fn turn_inverse(&mut self)
    {
        (self.enc, self.dec) = (self.dec, self.enc);
    }

    pub fn turn_encryptor(&mut self)
    {
        self.enc = Self::encrypt_u128;
        self.dec = Self::decrypt_u128;
    }

    pub fn turn_decryptor(&mut self)
    {
        self.enc = Self::decrypt_u128;
        self.dec = Self::encrypt_u128;
    }

    pub fn encrypt_unit(&mut self, message: &[IntUnion; NB]) -> [IntUnion; NB]
    {
        self.set_block(message);
        self.encrypt_block();
        self.get_block()
    }

    pub fn encrypt_u128(&mut self, message: u128) -> u128
    {
        self.set_block_u128(message);
        self.encrypt_block();
        self.get_block_u128()
    }

    pub fn decrypt_unit(&mut self, cipher: &[IntUnion; NB]) -> [IntUnion; NB]
    {
        self.set_block(cipher);
        self.decrypt_block();
        self.get_block()
    }

    pub fn decrypt_u128(&mut self, cipher: u128) -> u128
    {
        self.set_block_u128(cipher);
        self.decrypt_block();
        self.get_block_u128()
    }

    #[inline]
    pub(super) fn _encrypt(&mut self, message: u128) -> u128
    {
        (self.enc)(self, message)
    }

    #[inline]
    pub(super) fn _decrypt(&mut self, cipher: u128) -> u128
    {
        (self.dec)(self, cipher)
    }

    pub fn encrypt_array_u128<const N: usize>(&mut self, message: &[u128; N], cipher: &mut [u128; N])
    {
        for i in 0..N
        {
            self.set_block_u128(message[i]);
            self.encrypt_block();
            cipher[i] = self.get_block_u128();
        }
    }

    pub fn decrypt_array_u128<const N: usize>(&mut self, cipher: &[u128; N], message: &mut [u128; N])
    {
        for i in 0..N
        {
            self.set_block_u128(cipher[i]);
            self.decrypt_block();
            message[i] = self.get_block_u128();
        }
    }

    #[inline]
    pub fn is_successful(&self) -> bool
    {
        self.block[0][0] == Self::SUCCESS
    }

    #[inline]
    pub fn is_failed(&self) -> bool
    {
        self.block[0][0] == Self::FAILURE
    }

    #[inline]
    pub fn set_successful(&mut self)
    {
        self.block[0][0] = Self::SUCCESS;
    }

    #[inline]
    pub fn set_failed(&mut self)
    {
        self.block[0][0] = Self::FAILURE;
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
                let ptr_block_0 = self.block[i].as_mut_ptr();
                let ptr_block_i = self.block[i].as_ptr().add(i);
                let ptr_block_nb_i = self.block[i].as_mut_ptr().add(NB - i);
                copy_nonoverlapping(ptr_block_0, tmp_ptr, i);
                copy(ptr_block_i, ptr_block_0, NB - i);
                copy_nonoverlapping(tmp_ptr, ptr_block_nb_i, i);
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
                let ptr_block_0 = self.block[i].as_mut_ptr();
                let ptr_block_i = self.block[i].as_ptr().add(Self::SR[i]);
                let ptr_block_nb_i = self.block[i].as_mut_ptr().add(NB - Self::SR[i]);
                copy_nonoverlapping(ptr_block_0, tmp_ptr, Self::SR[i]);
                copy(ptr_block_i, ptr_block_0, NB - Self::SR[i]);
                copy_nonoverlapping(tmp_ptr, ptr_block_nb_i, Self::SR[i]);
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
                let ptr_block_0 = self.block[i].as_mut_ptr();
                let ptr_block_i = self.block[i].as_mut_ptr().add(i);
                let ptr_block_nb_i = self.block[i].as_ptr().add(NB - i);
                copy_nonoverlapping(ptr_block_nb_i, tmp_ptr, i);
                copy(ptr_block_0, ptr_block_i, NB - i);
                copy_nonoverlapping(tmp_ptr, ptr_block_0, i);
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
                let ptr_block_0 = self.block[i].as_mut_ptr();
                let ptr_block_i = self.block[i].as_mut_ptr().add(Self::SR[i]);
                let ptr_block_nb_i = self.block[i].as_ptr().add(NB - Self::SR[i]);
                copy_nonoverlapping(ptr_block_nb_i, tmp_ptr, i);
                copy(ptr_block_0, ptr_block_i, NB - Self::SR[i]);
                copy_nonoverlapping(tmp_ptr, ptr_block_0, i);
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
        while round <= ROUND
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
        while round <= ROUND
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
        let mut block = LongerUnion::new();
        let mut idx = 0;
        for j in 0..NB
        {
            for i in 0..4
            {
                block.set_ubyte_(idx, self.block[i][j]);
                idx += 1;
            }
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
                self.block[i][j] = block[j].get_[i];
                idx += 1;
            }
        }
    }

    fn set_block_u128(&mut self, block: u128)
    {
        let block_union = LongerUnion::new_with(block);
        let mut idx = 0;
        for j in 0..NB
        {
            for i in 0..4
            {
                self.block[i][j] = block_union.get_ubyte_(idx);
                idx += 1;
            }
        }
    }

    #[inline]
    pub fn get_desirable_ROUND() -> usize
    {
        6 + if NB > NK { NB } else { NK }
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