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
// use crate::number::{ SmallUInt, IntUnion, LongerUnion };


// macro_rules! GF_rot_left {
//     ($r:expr, $n:expr) => {
//         $r.rotate_left_assign($n);
//     };
// }

macro_rules! GF_mul {
    ($a:expr, $b:expr, $m:expr) => {
        {
            let mut a = $a;
            let mut b = $b;
            let mut ret = 0_u8;
            while b != 0
            {
                if (b & 1 == 1)
                    { ret ^= a; }
                if (a & 0b1000_0000 != 0)
                    { a = (a << 1) ^ $m; }
                else
                    { a <<= 1; }
                b >>= 1;
            }
            ret
        }
    };
}

macro_rules! GF_is_candidate {
    ($a:expr, $irreducible:expr) => {
        {
            let a_1 = $a;
            let a_2 = GF_mul!(a_1, a_1, $irreducible);
            let a_4 = GF_mul!(a_2, a_2, $irreducible);
            let a_8 = GF_mul!(a_4, a_4, $irreducible);
            let a_16 = GF_mul!(a_8, a_8, $irreducible);
            let a_32 = GF_mul!(a_16, a_16, $irreducible);
            let a_64 = GF_mul!(a_32, a_32, $irreducible);

            let a_3 = GF_mul!(a_1, a_2, $irreducible);
            let mut ret = GF_mul!(a_3, a_4, $irreducible);
            ret = GF_mul!(ret, a_8, $irreducible);
            if ret == 1
            {
                false
            }
            else
            {
                ret = GF_mul!(a_3, a_16, $irreducible);
                ret = GF_mul!(ret, a_32, $irreducible);
                if ret == 1
                {
                    false
                }
                else
                {
                    ret = GF_mul!(a_1, a_4, $irreducible);
                    ret = GF_mul!(ret, a_16, $irreducible);
                    ret = GF_mul!(ret, a_32, $irreducible);
                    ret != 1
                }
            }
        }
    };
}

macro_rules! GF_generator {
    ($irreducible:expr) => {
        {
            let mut ret = 0_u8;
            let mut j = 2_u16;
            while j < 256
            {
                if GF_is_candidate!(j as u8, $irreducible)
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
    ($a:expr, $irreducible:expr) => {
        {
            let mut ret = 0_u8;
            let mut inv = 255_u8;
            while inv > 1
            {
                if GF_mul!($a, inv, $irreducible) == 1
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
    ($irreducible:expr, $affine_mul:expr, $affine_add:expr) => {
        {
            let mut out = [0_u8; 256];
            let mut p = 1_u8;
            let mut q = 1_u8;
            let generator = GF_generator!($irreducible as u8);
            let inverse = GF_inverse_of!(generator, $irreducible as u8);
            let mut j = 255_u16;
            while j > 0
            {
                p = GF_mul!(p, generator, $irreducible as u8);
                q = GF_mul!(q, inverse, $irreducible as u8);
                let mut transformed = 0_u8;
                let mut i = 0_u8;
                while i < 8
                {
                    let affine_u8 = ($affine_mul >> (i << 3)) as u8;
                    let bits = affine_u8 & q;
                    if bits.count_ones() & 1 == 1
                        { transformed |= 1 << i; }
                    i += 1;
                }
                out[p as usize] = transformed ^ $affine_add;
                if p == 1
                    { break; }
                j -= 1;
            }
            out[0] = $affine_add;
            out
        }
    };
}

macro_rules! make_INVSBOX {
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

#[allow(non_camel_case_types)]
pub type AES_128 = Rijndael_Generic;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct Rijndael_Generic<const ROUND: usize = 10, const NB: usize = 4, const NK: usize = 4,
            const IRREDUCIBLE: u16 = 0b1_0001_1011,
            const AFFINE_MUL: u64 = 0b_11111000_01111100_00111110_00011111_10001111_11000111_11100011_11110001,
            const AFFINE_ADD: u8 = 0b_01100011>
{
    key: [[u8; NK]; NK],
    block: [[u8; NB]; NB],
    round_key: [u128; ROUND],
    enc: fn (s: &mut Self, message: u128) -> u128,
    dec: fn (s: &mut Self, cipher: u128) -> u128,
}

impl <const ROUND: usize, const NB: usize, const NK: usize, const IRREDUCIBLE: u16,
    const AFFINE_MUL: u64, const AFFINE_ADD: u8>
Rijndael_Generic<ROUND, NB, NK, IRREDUCIBLE, AFFINE_MUL, AFFINE_ADD>
{
    const SUCCESS: u128 = !0;
    const FAILURE: u128 = 0;
    const SBOX: [u8; 256] = make_SBOX!(IRREDUCIBLE, AFFINE_MUL, AFFINE_ADD);
    const INVSBOX: [u8; 256] = make_INVSBOX!();

    #[inline]
    pub fn new() -> Self
    {
        Self::new_with_key([0_u8; 16])
    }

    pub fn new_with_key<const K: usize>(key: [u8; K]) -> Self
    {
        let mut rijndael = Self
        {
            key:        [[0_u8; NK]; NK],
            block:      [[0_u8; NB]; NB],
            round_key:  [0_u128; ROUND],
            enc:        Self::encrypt_u128,
            dec:        Self::decrypt_u128,
        };
        let len = if K < NK * NK { K } else { NK * NK };
        unsafe { copy_nonoverlapping(key.as_ptr(), rijndael.block.as_mut_ptr() as *mut u8, len); }

        // rijndael.make_round_keys();
        rijndael
    }

    // pub fn new_with_key_u128(key: u128) -> Self
    // {
    //     let mut rijndael = Self
    //     {
    //         key:        LongerUnion::new_with(key),
    //         block:      LongerUnion::new(),
    //         round_key:  [0_u64; ROUND],
    //         enc:        Self::encrypt_u128,
    //         dec:        Self::decrypt_u128,
    //     };
    //     rijndael.make_round_keys();
    //     rijndael
    // }

    // #[inline]
    // pub fn encryptor_with_key(key: [u8; 16]) -> Self
    // {
    //     Self::new_with_key(key)
    // }

    // #[inline]
    // pub fn encryptor_with_key_u128(key: u128) -> Self
    // {
    //     Self::new_with_key_u128(key)
    // }

    // pub fn decryptor_with_key(key: [u8; 16]) -> Self
    // {
    //     let mut rijndael = Self
    //     {
    //         key:        LongerUnion::new_with_ubytes(key),
    //         block:      LongerUnion::new(),
    //         round_key:  [0_u64; ROUND],
    //         enc:        Self::decrypt_u128,
    //         dec:        Self::encrypt_u128,
    //     };
    //     rijndael.make_round_keys();
    //     rijndael
    // }

    // pub fn decryptor_with_key_u128(key: u128) -> Self
    // {
    //     let mut rijndael = Self
    //     {
    //         key:        LongerUnion::new_with(key),
    //         block:      LongerUnion::new(),
    //         round_key:  [0_u64; ROUND],
    //         enc:        Self::decrypt_u128,
    //         dec:        Self::encrypt_u128,
    //     };
    //     rijndael.make_round_keys();
    //     rijndael
    // }

    // pub fn get_key(&mut self) -> [u8; 16]
    // {
    //     let mut key = [0u8; 16];
    //     for i in 0..16
    //         { key[i] = self.key.get_ubyte_(i); }
    //     key
    // }

    // #[inline]
    // pub fn get_key_u128(&self) -> u128
    // {
    //     self.key.get_ulonger()
    // }

    // pub fn set_key(&mut self, key: [u8; 16])
    // {
    //     let mut i = 0_usize;
    //     for val in key
    //     {
    //         self.key.set_ubyte_(i, val);
    //         i += 1;
    //     }
    //     self.make_round_keys();
    // }

    // pub fn set_key_u128(&mut self, key: u128)
    // {
    //     self.key.set(key);
    //     self.make_round_keys();
    // }

    // #[inline]
    // pub fn turn_inverse(&mut self)
    // {
    //     (self.enc, self.dec) = (self.dec, self.enc);
    // }

    // pub fn turn_encryptor(&mut self)
    // {
    //     self.enc = Self::encrypt_u128;
    //     self.dec = Self::decrypt_u128;
    // }

    // pub fn turn_decryptor(&mut self)
    // {
    //     self.enc = Self::decrypt_u128;
    //     self.dec = Self::encrypt_u128;
    // }

    pub fn encrypt_u128(&mut self, message: u128) -> u128
    {
        unimplemented!();
        // self.set_block(message);
        // self.encrypt_block();
        // self.get_block()
    }

    pub fn decrypt_u128(&mut self, cipher: u128) -> u128
    {
        unimplemented!();
        // self.set_block(cipher);
        // self.decrypt_block();
        // self.get_block()
    }

    // #[inline]
    // pub fn _encrypt(&mut self, message: u128) -> u128
    // {
    //     (self.enc)(self, message)
    // }

    // #[inline]
    // pub fn _decrypt(&mut self, cipher: u128) -> u128
    // {
    //     (self.dec)(self, cipher)
    // }

    // pub fn encrypt_array_u128<const N: usize>(&mut self, message: &[u128; N], cipher: &mut [u128; N])
    // {
    //     for i in 0..N
    //     {
    //         self.set_block(message[i]);
    //         self.encrypt_block();
    //         cipher[i] = self.get_block();
    //     }
    // }

    // pub fn decrypt_array_u128<const N: usize>(&mut self, cipher: &[u128; N], message: &mut [u128; N])
    // {
    //     for i in 0..N
    //     {
    //         self.set_block(cipher[i]);
    //         self.decrypt_block();
    //         message[i] = self.get_block();
    //     }
    // }

    // #[inline]
    // pub fn is_successful(&self) -> bool
    // {
    //     self.block.get() == Self::SUCCESS
    // }

    // #[inline]
    // pub fn is_failed(&self) -> bool
    // {
    //     self.block.get() == Self::FAILURE
    // }

    // #[inline]
    // pub fn set_successful(&mut self)
    // {
    //     self.block.set(Self::SUCCESS);
    // }

    // #[inline]
    // pub fn set_failed(&mut self)
    // {
    //     self.block.set(Self::FAILURE);
    // }

    fn encrypt_block(&mut self)
    {
        self.subBytes();
        self.shiftRows();
        self.mixColumn();
    }

    fn subBytes(&mut self)
    {
        for i in 0..NB
        {
            for j in 0..NB
            {
                self.block[i][j] = Self::SBOX[self.block[i][j] as usize];
            }    
        }
    }

    fn shiftRows(&mut self)
    {
        let mut b = [0_u8; NB];
        for i in 1..NB
        {
            for j in 0..i
                { b[NB-i+j] = self.block[i][j]; }
            for j in 0..NB-i
                { self.block[i][j] = self.block[i][j+i]; }
            for j in NB-i..NB
                { self.block[i][j] = b[j]; }
        }
    }

    fn mixColumn(&mut self)
    {
        
    }

    fn decrypt_block(&mut self)
    {
        self.subBytesInv();
    }

    fn subBytesInv(&mut self)
    {
        for i in 0..NB
        {
            for j in 0..NB
            {
                self.block[i][j] = Self::INVSBOX[self.block[i][j] as usize];
            }    
        }
    }

    fn shiftRowsInv(&mut self)
    {
        let mut b = [0_u8; NB];
        for i in 1..NB
        {
            for j in 0..i
                { b[j] = self.block[i][NB-i+j]; }
            for j in 0..NB-i
                { self.block[i][NB-j] = self.block[i][NB-j-i]; }
            for j in 0..i
                { self.block[i][j] = b[j]; }
        }
    }

    // fn make_round_keys(&mut self)
    // {

    // }

    // fn make_a_round_key(&mut self, round: usize, left: IntUnion, mut right: IntUnion)
    // {

    // }

    // #[inline] fn get_block(&self) -> u128           { self.block.get() }
    // #[inline] fn set_block(&mut self, block: u128)  { self.block.set(block); }



    /////// Testing Codes during Development //////
    #[allow(non_snake_case)]
    pub fn show_SBox()
    {
        for i in 0..256
        {
            println!("{:02X} => {:02X}", i, Self::SBOX[i]);
        }
    }

    #[allow(non_snake_case)]
    pub fn show_InvSBox()
    {
        for i in 0..256
        {
            println!("{:02X} => {:02X}", i, Self::INVSBOX[i]);
        }
    }
}