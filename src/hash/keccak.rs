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
            let mut union_A = A::<$T, $ROUNDS> { U128: [0_u128; ROUNDS] };
            let WIDTH = <$T>::BITS as usize;
            let mut RC = [<$T>::MIN; ROUNDS];
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


#[allow(non_camel_case_types)]
pub type BIG_KECCAK = Keccak_Generic<72, 2, 26, u128>;

#[allow(non_camel_case_types)]
pub type KECCAK_224 = Keccak_Generic<144, 2>;

#[allow(non_camel_case_types)]
pub type KECCAK_256 = Keccak_Generic<136, 2>;

#[allow(non_camel_case_types)]
pub type KECCAK_384 = Keccak_Generic<104, 2>;

#[allow(non_camel_case_types)]
pub type KECCAK_512 = Keccak_Generic<72, 2>;

#[allow(non_camel_case_types)]
pub type SHAKE_128 = Keccak_Generic<168, 1>;

#[allow(non_camel_case_types)]
pub type SHAKE_256 = Keccak_Generic<136, 1>;

#[allow(non_camel_case_types)]
pub type SHA3_224 = Keccak_Generic<144>;

#[allow(non_camel_case_types)]
pub type SHA3_256 = Keccak_Generic<136>;

#[allow(non_camel_case_types)]
pub type SHA3_384 = Keccak_Generic<104>;

#[allow(non_camel_case_types)]
pub type SHA3_512 = Keccak_Generic;


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
/// - PADDING: The parameter `PADDING` determines padding way. Only `0`, `1`,
///   `2` are available for the parameter `PADDING`.
///   If `PADDING` is `0`, domain separator bits `01` are appended, and the
///   start bits of padding `1` is appended, and the necessary `0`s are added,
///   and then `1` is appended in order to make the length of the message be a
///   multiple of `RATE` for SHA-3 standard.
///   If `PADDING` is `1`, domain separator bits `1111` are appended, and
///   the start bits of padding `1` is appended, and the necessary `0`s are
///   added, and then `1` is appended in order to make the length of the
///   message be a multiple of `RATE` for SHAKE standard.
///   If `PADDING` is `2`, the start bits of padding `1` is appended without
///   domain separator bits, and the necessary `0`s are appended, and then `1`
///   is appended in order to make the length of the message be a multiple of
///   `RATE` for Keccak standard.
///   The default value is `0` for for SHA-3 standard.
/// - ROUNDS : The parameter `ROUNDS` determines how many rounds the digesting
///   steps are repeated. Usually, for the official SHA3 and SHAKE,
///   `ROUNDS` is 24. So, the default value is `24` for SHA3 and SHAKE.
/// - T : The parameter `T` is the datatype of the unit block to process. It
///   is one of `u8`, `u16`, `u32`, `u64`, and `u128`.
///   Usually, for the official SHA3 and SHAKE, `T` is `u64`.
///   So, the default value is `24` for SHA3 and SHAKE.
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
///   The default value of `LFSR` is 0b_0111_0001 for official SHA3 and SHAKE.
/// - THETA_SUB : The default value of `THETA_SUB` is 1.
/// - THETA_ADD : The default value of `THETA_ADD` is 1.
/// - THETA_ROT : The default value of `THETA_ROT` is 1.
/// - RHO_MUL_X : The default value of `RHO_MUL_X` is 2.
/// - RHO_MUL_Y : The default value of `RHO_MUL_Y` is 3.
/// - RHO_T : The default value of `RHO_T` is 24,
/// - PI_MUL_X : The default value of `PI_MUL_X` is 1.
/// - PI_MUL_Y : The default value of `PI_MUL_Y` is 3.
/// - CHI_ADD_1 : The default value of `THETA_ROT` is 1.
/// - CHI_ADD_2 : The default value of `CHI_ADD_2` is 2.
/// 
/// Watch [this video](https://www.youtube.com/watch?v=JWskjzgiIa4)
/// to learn SHA-3 more in detail.
/// 
/// # Security of your own expanded version
/// Your own algrorithm based on Keccak may be stronger or weaker than official
/// SHA3-224, SHA3-256, SHA3-384, SHA3-512, SHAKE-128, SHAKE-256, and other
/// Keccak variants. Unless you seriously checked the cryptographic security of
/// your own algorithms, it is hard to assume that your own alogrithms are
/// stronger than the official SHA3-224, SHA3-256, SHA3-384, SHA3-512,
/// SHAKE-128, SHAKE-256, and other Keccak variants.
/// 
/// # Reference
/// Read [more](https://en.wikipedia.org/wiki/SHA-3) about SHA-3 in detail.
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
                                0 => { 0b_0000_0110 },
                                1 => { 0b_0001_1111 },
                                _ => { 0b_0000_0001 },
                            };
    const TAIL: u8 = 0b_1000_0000;


    // pub fn new() -> Self
    /// Creates the new object of `Self`.
    #[inline]
    pub fn new() -> Self
    {
        Self { state: [[T::MIN; 5]; 5] }
    }

    #[inline]
    pub fn digest(&mut self, message: *const u8, length_in_bytes: u64)
    {
        self.absorb(message, length_in_bytes);
    }
    
    #[inline]
    pub fn digest_str(&mut self, message: &str)
    {
        self.absorb(message.as_ptr(), message.len() as u64);
    }

    #[inline]
    pub fn digest_string(&mut self, message: &String)
    {
        self.absorb(message.as_ptr(), message.len() as u64);
    }

    #[inline]
    pub fn digest_array<U, const M: usize>(&mut self, message: &[U; M])
    where U: SmallUInt + Copy + Clone
    {
        self.absorb(message.as_ptr() as *const u8, (M as u32 * U::size_in_bytes()) as u64);
    }

    #[inline]
    pub fn digest_vec<U>(&mut self, message: &Vec<T>)
    where U: SmallUInt + Copy + Clone
    {
        self.absorb(message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64);
    }

    pub fn get_hash_value_in_array<const M: usize>(&mut self) -> [u8; M]
    {
        let mut out = [0_u8; M];
        self.push_hash_value_in_array(&mut out);
        out
    }

    pub fn get_hash_value_in_vec<const M: usize>(&mut self) -> Vec<u8>
    {
        let mut out = Vec::<u8>::new();
        let arr: [u8; M] = self.get_hash_value_in_array();
        for it in arr
            { out.push(it); }
        out
    }

    pub fn get_hash_value_in_string(&mut self, length_in_bytes: usize) -> String
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

    pub fn push_hash_value_in_array<const M: usize>(&mut self, hash_value: &mut [u8; M])
    {
        self.get_hash_value(hash_value.as_mut_ptr(), M);
    }

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

    pub fn read_hash_value_in_hexadecimal<const M: usize>(hash: &[u8; M]) -> String
    {
        let mut txt = String::new();
        for i in 0..M
        {
            let byte = hash[i];
            txt.push(Self::to_char(byte >> 4));
            txt.push(Self::to_char(byte & 0b1111));
        }
        txt
    }

    // pub fn squeeze(&mut self) -> [u8; RATE]
    /// Returns the resulting hash code.
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
    pub fn absorb_array<U, const M: usize>(&mut self, message: &[U; M])
    where U: SmallUInt + Copy + Clone
    {
        self.absorb(message.as_ptr() as *const u8, (M as u32 * U::size_in_bytes()) as u64);
    }

    #[inline]
    pub fn absorb_vec<U>(&mut self, message: &Vec<T>)
    where U: SmallUInt + Copy + Clone
    {
        self.absorb(message.as_ptr() as *const u8, (message.len() as u32 * U::size_in_bytes()) as u64);
    }

    // pub fn absorb(&mut self, message: *const u8, length_in_bytes: usize)
    /// Digests the message.
    pub fn absorb(&mut self, message: *const u8, length_in_bytes: u64)
    {
        let chunk_num = length_in_bytes as usize / RATE;
        self.initialize_state();
        for i in 0..chunk_num
            { self.absorb_block(unsafe { message.add(i * RATE) }); } 

        let rest = length_in_bytes as usize % RATE;
        let mut padded = [0_u8; RATE];
        unsafe { copy_nonoverlapping(message.add(RATE * chunk_num), padded.as_mut_ptr(), rest); }
        padded[rest] = Self::SEPARATOR;
        padded[RATE - 1] |= Self::TAIL;
        self.absorb_block(padded.as_ptr());
    }

    // pub fn tangle(&mut self, tangling: u64)
    /// Tangles the hash value
    /// 
    /// # Argument
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
        m[8] = tangling;
        self.absorb_block(rate.as_ptr());
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
            self.theta();
            self.rho();
            self.pi();
            self.chi();
            self.iota(round);
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
    // #[inline] pub fn _show_state(&self)
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
}




/*
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
        write!(f, "{}", self.get_hash_value_in_string())
    }
}
*/