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
                    { a = (a << 1) ^ ($m as u8); }
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
            let a_2 = GF_mul!(a_1, a_1, $irreducible as u8);
            let a_4 = GF_mul!(a_2, a_2, $irreducible as u8);
            let a_8 = GF_mul!(a_4, a_4, $irreducible as u8);
            let a_16 = GF_mul!(a_8, a_8, $irreducible as u8);
            let a_32 = GF_mul!(a_16, a_16, $irreducible as u8);
            let a_64 = GF_mul!(a_32, a_32, $irreducible as u8);

            let a_3 = GF_mul!(a_1, a_2, $irreducible as u8);
            let mut ret = GF_mul!(a_3, a_4, $irreducible as u8);
            ret = GF_mul!(ret, a_8, $irreducible as u8);
            if ret == 1
            {
                false
            }
            else
            {
                ret = GF_mul!(a_3, a_16, $irreducible as u8);
                ret = GF_mul!(ret, a_32, $irreducible as u8);
                if ret == 1
                {
                    false
                }
                else
                {
                    ret = GF_mul!(a_1, a_4, $irreducible as u8);
                    ret = GF_mul!(ret, a_16, $irreducible as u8);
                    ret = GF_mul!(ret, a_32, $irreducible as u8);
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
                if GF_is_candidate!(j as u8, $irreducible as u8)
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
                if GF_mul!($a, inv, $irreducible as u8) == 1
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

            let generator = GF_generator!(IRREDUCIBLE as u8);
            let inverse = GF_inverse_of!(generator, IRREDUCIBLE as u8);
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
                    p = GF_mul!(p, generator, IRREDUCIBLE as u8);
                    q = GF_mul!(q, inverse, IRREDUCIBLE as u8);
                }
                j = 0;
                while j < 4
                {
                    aug[row][j] = GF_mul!(aug[row][j], q, IRREDUCIBLE as u8);
                    inv[row][j] = GF_mul!(inv[row][j], q, IRREDUCIBLE as u8);
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
                            inv[r][j] ^= GF_mul!(inv[row][j], factor, IRREDUCIBLE as u8);
                            aug[r][j] ^= GF_mul!(aug[row][j], factor, IRREDUCIBLE as u8);
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


#[allow(non_camel_case_types)]
pub type AES_128 = Rijndael_Generic;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct Rijndael_Generic<const ROUND: usize = 10, const NB: usize = 4, const NK: usize = 4,
            const IRREDUCIBLE: u16 = 0b1_0001_1011,
            const AFFINE_MUL: u64 = 0b_11111000_01111100_00111110_00011111_10001111_11000111_11100011_11110001,
            const AFFINE_ADD: u8 = 0b_01100011,
            const SR0: usize = 0, const SR1: usize = 1, const SR2: usize = 2, const SR3: usize = 3,
            const MC00: u8 = 2, const MC01: u8 = 3, const MC02: u8 = 1, const MC03: u8 = 1,
            const MC10: u8 = 1, const MC11: u8 = 2, const MC12: u8 = 3, const MC13: u8 = 1,
            const MC20: u8 = 1, const MC21: u8 = 1, const MC22: u8 = 2, const MC23: u8 = 3,
            const MC30: u8 = 3, const MC31: u8 = 1, const MC32: u8 = 1, const MC33: u8 = 2>
{
    key: [[u8; NK]; 4],
    block: [[u8; NB]; 4],
    round_key: [u128; ROUND],
    enc: fn (s: &mut Self, message: u128) -> u128,
    dec: fn (s: &mut Self, cipher: u128) -> u128,
}

impl <const ROUND: usize, const NB: usize, const NK: usize, const IRREDUCIBLE: u16, const AFFINE_MUL: u64,
        const AFFINE_ADD: u8, const SR0: usize, const SR1: usize, const SR2: usize, const SR3: usize,
        const MC00: u8, const MC01: u8, const MC02: u8, const MC03: u8,
        const MC10: u8, const MC11: u8, const MC12: u8, const MC13: u8,
        const MC20: u8, const MC21: u8, const MC22: u8, const MC23: u8,
        const MC30: u8, const MC31: u8, const MC32: u8, const MC33: u8>
Rijndael_Generic<ROUND, NB, NK, IRREDUCIBLE, AFFINE_MUL, AFFINE_ADD, SR0, SR1, SR2, SR3,
        MC00, MC01, MC02, MC03, MC10, MC11, MC12, MC13, MC20, MC21, MC22, MC23, MC30, MC31, MC32, MC33>
{
    const SUCCESS: u128 = !0;
    const FAILURE: u128 = 0;
    const SBOX: [u8; 256] = make_SBOX!(IRREDUCIBLE, AFFINE_MUL, AFFINE_ADD);
    const INV_SBOX: [u8; 256] = make_INV_SBOX!();
    const SR: [usize; 4] = [SR0, SR1, SR2, SR3];
    const MC: [[u8; 4]; 4] = [  [ MC00, MC01, MC02, MC03 ],
                                [ MC10, MC11, MC12, MC13 ],
                                [ MC20, MC21, MC22, MC23 ],
                                [ MC30, MC31, MC32, MC33 ] ];
    const INV_MC: [[u8; 4]; 4] = make_INV_MC!();

    #[inline]
    pub fn new() -> Self
    {
        Self::new_with_key([0_u8; 16])
    }

    pub fn new_with_key<const K: usize>(key: [u8; K]) -> Self
    {
        let mut rijndael = Self
        {
            key:        [[0_u8; NK]; 4],
            block:      [[0_u8; NB]; 4],
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
        self.sub_bytes();
        self.shift_rows();
        self.mix_columns();
        self.add_round_key();
    }

    fn sub_bytes(&mut self)
    {
        for i in 0..4
        {
            for j in 0..NB
                { self.block[i][j] = Self::SBOX[self.block[i][j] as usize]; }    
        }
    }

    fn shift_rows(&mut self)
    {
        let mut b = [0_u8; NB];
        for i in 0..4
        {
            for j in 0..Self::SR[i]
                { b[NB-i+j] = self.block[i][j]; }
            for j in 0..NB-Self::SR[i]
                { self.block[i][j] = self.block[i][j+Self::SR[i]]; }
            for j in NB-Self::SR[i]..NB
                { self.block[i][j] = b[j]; }
        }
    }

    fn mix_columns(&mut self)
    {
        let mut new_block = [[0_u8; NB]; 4];
        for col in 0..NB
        {
            for row in 0..4
            {
                for i in 0..4
                    { new_block[row][col] ^= GF_mul!(Self::MC[row][i], self.block[i][col], IRREDUCIBLE); }
            }
        }
        self.block = new_block;
    }

    fn add_round_key(&mut self)
    {

    }

    fn decrypt_block(&mut self)
    {
        self.add_round_key();
        self.inv_mix_columns();
        self.inv_shift_rows();
        self.inv_sub_bytes();
    }

    fn inv_sub_bytes(&mut self)
    {
        for i in 0..4
        {
            for j in 0..NB
                { self.block[i][j] = Self::INV_SBOX[self.block[i][j] as usize]; }    
        }
    }

    fn inv_shift_rows(&mut self)
    {
        let mut b = [0_u8; NB];
        for i in 0..4
        {
            for j in 0..Self::SR[i]
                { b[j] = self.block[i][NB-Self::SR[i]+j]; }
            for j in 0..NB-sr[i]
                { self.block[i][NB-j] = self.block[i][NB-j-Self::SR[i]]; }
            for j in 0..Self::SR[i]
                { self.block[i][j] = b[j]; }
        }
    }

    fn inv_mix_columns(&mut self)
    {
        let mut new_block = [[0_u8; NB]; 4];
        for col in 0..NB
        {
            for row in 0..4
            {
                for i in 0..4
                    { new_block[row][col] ^= GF_mul!(Self::INV_MC[row][i], self.block[i][col], IRREDUCIBLE); }
            }
        }
        self.block = new_block;
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
            { println!("{:02X} => {:02X}", i, Self::SBOX[i]); }
    }

    #[allow(non_snake_case)]
    pub fn show_InvSBox()
    {
        for i in 0..256
            { println!("{:02X} => {:02X}", i, Self::INV_SBOX[i]); }
    }

    #[allow(non_snake_case)]
    pub fn show_MC()
    {
        println!("MC is as follows:");
        for i in 0..4
        {
            for j in 0..4
                { print!("{:02x} ", Self::MC[i][j]); }
            println!();
        }
    }

    #[allow(non_snake_case)]
    pub fn show_InvMC()
    {
        println!("Inverse MC is as follows:");
        for i in 0..4
        {
            for j in 0..4
                { print!("{:02x} ", Self::INV_MC[i][j]); }
            println!();
        }
    }
}