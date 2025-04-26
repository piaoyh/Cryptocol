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
/// # Use of Keccak or SHA3 and their variants
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
/// - PADDING: The parameter `PADDING` is whether or not the domain separate
///   bits is `01` for SHA-3 style. So, if `PADDING` is `true`, the domain
///   separate bits is `01` for SHA-3. If `PADDING` is `false`, the domain
///   bits is `1111` for SHAKE.45
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
        self.feed_block_to_state(block);  print!("After feed "); self._show_state();
        for round in 0..ROUNDS
        {
            self.theta();   print!("After theta {} ", round); self._show_state();
            self.rho();     print!("After rho {} ", round); self._show_state();
            self.pi();      print!("After pi {} ", round); self._show_state();
            self.chi();     print!("After chi {} ", round); self._show_state();
            self.iota(round);   print!("After iota {} ", round); self._show_state();
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



    // For test
    #[inline] pub fn _initialize_state(&mut self)   { self.initialize_state(); }
    #[inline] pub fn _feed_block_to_state(&mut self, block: *const u8)  { self.feed_block_to_state(block); }
    #[inline] pub fn _absorb_block(&mut self, block: *const u8)         { self.absorb_block(block); }
    #[inline] pub fn _theta(&mut self)  { self.theta(); }
    #[inline] pub fn _rho(&mut self)    { self.rho(); }
    #[inline] pub fn _pi(&mut self)     { self.pi(); }
    #[inline] pub fn _chi(&mut self)    { self.chi(); }
    #[inline] pub fn _iota(&mut self, round: usize) { self.iota(round); }
    #[inline] pub fn _show_state(&self)
    {
        println!("State: ");
        for y in 0..5
        {
            for x in 0..5
            {
                let u = LongUnion::new_with(self.state[y][x].into_u64());
                for i in 0..8
                {
                    print!("{:02X} ", u.get_ubyte_(i));
                }
                print!("  ");
                if (x + y * 5)  & 1 == 1
                    { println!(); }
            }
        }
        println!();println!();
    } 
}