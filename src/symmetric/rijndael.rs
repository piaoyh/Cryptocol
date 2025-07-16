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
            if RC_AUTO
            {
                let mut i = 1;
                out[0] = 1_u32.to_le();
                while i < ROUND
                {
                    out[i] = (GF_mul!(out[i-1].to_le() as u8, 2) as u32).to_le();
                    i += 1;
                }
            }
            else
            {
                let round = [RC0.to_le(), RC1.to_le(), RC2.to_le(), RC3.to_le(), RC4.to_le(),
                             RC5.to_le(), RC6.to_le(), RC7.to_le(), RC8.to_le(), RC9.to_le()];
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
            }
            out
        }
    }
}


#[allow(non_camel_case_types)]
pub type AES_256 = Rijndael_Generic<14, 4, 8>;

#[allow(non_camel_case_types)]
pub type AES_192 = Rijndael_Generic<12, 4, 6>;

#[allow(non_camel_case_types)]
pub type AES_128 = Rijndael_Generic;
/// # Generic Parameters
/// - ROUND: You can determine how many times the Substitution-Permutation
///   Network will be repeated. Its minimum value is 0. Original Rijndael has
///   the number of rounds as follows:
///   |              | 128-bit key | 192-bit key | 256-bit key |
///   |--------------|-------------|-------------|-------------|
///   | 128-bit data | 10 rounds   | 12 rounds   | 14 rounds   |
///   | 192-bit data | 12 rounds   | 12 rounds   | 14 rounds   |
///   | 256-bit data | 14 rounds   | 14 rounds   | 14 rounds   |
/// 
/// - NB: You can determine the size of block.
/// - NK: You can determine the size of key.
/// - IRREDUCIBLE: You can determine the irreducible polynomial.
/// - AFFINE_MUL: You can determine the affine matrix.
/// - AFFINE_ADD: You can determine the non-linear part of the affine
///   transformation.
/// - SR0 ~ SR3: You can determine the amount of shifting rows for each rows
///   for the ShiftRows step.
/// - MC00 ~ MC33: You can determine the Mix-Column matrix used for the
///   MixColumn step.
/// - RC_AUTO: If `RC_AUTO` is set to be `true`, Round Constants will be
///   automatically determined and `RC0` ~ `RC9` will be ignored. If it is set
///   to be `false`, `RC0` ~ `RC9` will be used for Round Constants.
/// - RC0 ~ RC9: Round Constants. They are maaningful only when `RC_AUTO` is
///   `false`. If `Round` is greater than 10, the round constants after RC9 will
///   be determined by multiplying previous round constant with two on Galois
///   Field. So, So-called RC10 is double of `RC9` on Galois Field and RC11 is
///   double of `RC10` on Galois Field, and so on.
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
            const RC_AUTO: bool = true, const RC0: u32 = 0b_0000_0001, const RC1: u32 = 0b_0000_0010,
            const RC2: u32 = 0b_0000_0100, const RC3: u32 = 0b_0000_1000, const RC4: u32 = 0b_0001_0000,
            const RC5: u32 = 0b_0010_0000, const RC6: u32 = 0b_0100_0000, const RC7: u32 = 0b_1000_0000,
            const RC8: u32 = 0b_0001_1011, const RC9: u32 = 0b_001_10110, const ROT: u32 = 1>
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
        const RC_AUTO: bool, const RC0: u32, const RC1: u32, const RC2: u32, const RC3: u32, const RC4: u32,
        const RC5: u32, const RC6: u32, const RC7: u32, const RC8: u32, const RC9: u32, const ROT: u32>
Rijndael_Generic<ROUND, NB, NK, IRREDUCIBLE, AFFINE_MUL, AFFINE_ADD, SR0, SR1, SR2, SR3,
        MC00, MC01, MC02, MC03, MC10, MC11, MC12, MC13, MC20, MC21, MC22, MC23, MC30, MC31, MC32, MC33,
        RC_AUTO, RC0, RC1, RC2, RC3, RC4, RC5, RC6, RC7, RC8, RC9, ROT>
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
                                                    
    #[inline]
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

    pub fn get_key(&mut self) -> Vec<u8>
    {
        let mut key = vec![0u8; NK * 4];
        for i in 0..NK*4
            { key[i] = self.key[i / NK].get_ubyte_(i % NK); }
        key
    }

    pub fn get_key_u128(&self) -> u128
    {
        let ints = [self.key[0].get(), self.key[1].get(), self.key[2].get(), self.key[3].get()];
        let key = LongerUnion::new_with_uints(ints);
        key.get()
    }

    pub fn set_key<const K: usize>(&mut self, key: [u8; K])
    {
        let len = if K < NK * 4 { K } else { NK * 4 };
        unsafe { copy_nonoverlapping(key.as_ptr(), self.key.as_mut_ptr() as *mut u8, len); }
        unsafe { copy_nonoverlapping(self.key.as_ptr() as *const u8, self.round_key.as_mut_ptr() as *mut u8, NK * 4); }
        Self::method_make_round_keys(self);
    }

    pub fn set_key_u128(&mut self, key: u128)
    {
        let longer = LongerUnion::new_with(key);
        for i in 0..4
            { self.key[i].set_uint(longer.get_uint_(i)); }
        unsafe { copy_nonoverlapping(self.key.as_ptr() as *const u8, self.round_key.as_mut_ptr() as *mut u8, NK * 4); }
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

    pub fn encrypt_u128(&mut self, message: u128) -> u128
    {
        self.set_block(message);
        self.encrypt_block();
        self.get_block()
    }

    pub fn decrypt_u128(&mut self, cipher: u128) -> u128
    {
        self.set_block(cipher);
        self.decrypt_block();
        self.get_block()
    }

    #[inline]
    pub fn _encrypt(&mut self, message: u128) -> u128
    {
        (self.enc)(self, message)
    }

    #[inline]
    pub fn _decrypt(&mut self, cipher: u128) -> u128
    {
        (self.dec)(self, cipher)
    }

    pub fn encrypt_array_u128<const N: usize>(&mut self, message: &[u128; N], cipher: &mut [u128; N])
    {
        for i in 0..N
        {
            self.set_block(message[i]);
            self.encrypt_block();
            cipher[i] = self.get_block();
        }
    }

    pub fn decrypt_array_u128<const N: usize>(&mut self, cipher: &[u128; N], message: &mut [u128; N])
    {
        for i in 0..N
        {
            self.set_block(cipher[i]);
            self.decrypt_block();
            message[i] = self.get_block();
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
        for round in 1..ROUND-1
        {
            self.sub_bytes();
            Self::method_shift_rows(self);
            self.mix_columns();
            self.add_round_key(round);
        }
        self.sub_bytes();
        Self::method_shift_rows(self);
        self.add_round_key(ROUND-1);
    }

    fn sub_bytes(&mut self)
    {
        for i in 0..4
        {
            for j in 0..NB
                { self.block[i][j] = Self::SBOX[self.block[i][j] as usize]; }    
        }
    }

    fn optimal_shift_rows(&mut self)
    {
        let mut b = [0_u8; NB];
        for i in 1..4
        {
            for j in 0..i
                { b[NB-i+j] = self.block[i][j]; }
            for j in 0..NB-i
                { self.block[i][j] = self.block[i][j+i]; }
            for j in NB-i..NB
                { self.block[i][j] = b[j]; }
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
                    { new_block[row][col] ^= GF_mul!(Self::MC[row][i], self.block[i][col]); }
            }
        }
        self.block = new_block;
    }

    fn add_round_key(&mut self, round: usize)
    {
        // let mut idx = 0;
        for i in 0..4
        {
            for j in 0..NB
            {
                self.block[i][j] ^= self.round_key[round][i].get_ubyte_(j); }
        }
    }

    fn decrypt_block(&mut self)
    {
        self.add_round_key(ROUND-1);
        let mut round = ROUND-2;
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
                { self.block[i][j] = Self::INV_SBOX[self.block[i][j] as usize]; }    
        }
    }

    fn optimal_inv_shift_rows(&mut self)
    {
        let mut b = [0_u8; NB];
        for i in 1..4
        {
            for j in 0..i
                { b[j] = self.block[i][NB-i+j]; }
            for j in 0..NB-i
                { self.block[i][NB-j] = self.block[i][NB-j-i]; }
            for j in 0..i
                { self.block[i][j] = b[j]; }
        }
    }

    fn inv_shift_rows(&mut self)
    {
        let mut b = [0_u8; NB];
        for i in 0..4
        {
            for j in 0..Self::SR[i]
                { b[j] = self.block[i][NB-Self::SR[i]+j]; }
            for j in 0..NB-Self::SR[i]
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
                    { new_block[row][col] ^= GF_mul!(Self::INV_MC[row][i], self.block[i][col]); }
            }
        }
        self.block = new_block;
    }

    fn make_round_keys_nk_up_to_6_and_nk_equal_to_nb(&mut self)
    {
        for round in 1..ROUND
        {
            let mut tmp = self.round_key[round-1][NB-1];
            #[cfg(target_endian = "little")] { tmp = tmp.rotate_right(8 * ROT); }
            #[cfg(target_endian = "big")] { tmp = tmp.rotate_left(8 * ROT); }
            for j in 0..4
                { tmp.set_ubyte_(j, Self::SBOX[tmp.get_ubyte_(j) as usize]); }
            tmp.set(tmp.get() ^ Self::RC[round-1]);
            self.round_key[round][0] = tmp ^ self.round_key[round-1][0];
            for i in 1..NK
                { self.round_key[round][i] = self.round_key[round][i-1] ^ self.round_key[round-1][i]; }
        }
    }

    fn make_round_keys_nk_greater_than_6_and_nk_equal_to_nb(&mut self)
    {
        for round in 1..ROUND
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
            for i in 5..NK
                { self.round_key[round][i] = self.round_key[round][i-1] ^ self.round_key[round-1][i]; }
        }
    }

    fn make_round_keys_nk_up_to_6_and_nk_diff_from_nb(&mut self)
    {
        let mut round = NK / NB;
        let mut cc = NK % NB;
        let mut idx = NK;
        while round < ROUND
        {
            let mut tmp = if cc == 0 { self.round_key[round-1][NB-1] } else { self.round_key[round][cc-1] };
            if idx % NK == 0
            {
                    println!("tmp: {} - {:08x}", round, tmp.to_be());
                #[cfg(target_endian = "little")] { tmp = tmp.rotate_right(8 * ROT); }    println!("rot: {} - {:08x}", round, tmp.to_be());
                #[cfg(target_endian = "big")] { tmp = tmp.rotate_left(8 * ROT); }
                for j in 0..4
                    { tmp.set_ubyte_(j, Self::SBOX[tmp.get_ubyte_(j) as usize]); }    println!("sub: {} - {:08x}", round, tmp.to_be());
                tmp.set(tmp.get() ^ Self::RC[round-1]);    println!("xor: {} - {:08x}", round, tmp.to_be());
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
        let mut round = NK / NB;
        let mut cc = NK % NB;
        let mut idx = NK;
        while round < ROUND
        {
            let mut tmp = self.round_key[round][cc-1];
            if idx % NK == 0
            {
                #[cfg(target_endian = "little")] { tmp = tmp.rotate_right(8 * ROT); }
                #[cfg(target_endian = "big")] { tmp = tmp.rotate_left(8 * ROT); }
                for j in 0..4
                    { tmp.set_ubyte_(j, Self::SBOX[tmp.get_ubyte_(j) as usize]); }
                tmp.set(tmp.get() ^ Self::RC[round-1]);
            }
            else if idx % NK == 4
            {
                for j in 0..4
                    { tmp.set_ubyte_(j, Self::SBOX[tmp.get_ubyte_(j) as usize]); }
            }
            self.round_key[round][4] = tmp ^ self.round_key[round-1][4];
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

    fn get_block(&self) -> u128
    {
        let ubytes = [
                    self.block[0][0], self.block[0][1], self.block[0][2], self.block[0][3],
                    self.block[1][0], self.block[1][1], self.block[1][2], self.block[1][3],
                    self.block[2][0], self.block[2][1], self.block[2][2], self.block[2][3],
                    self.block[3][0], self.block[3][1], self.block[3][2], self.block[3][3]
        ];
        let block = LongerUnion::new_with_ubytes(ubytes);
        block.get()
    }

    fn set_block(&mut self, block: u128)
    {
        let longer = LongerUnion::new_with(block);
        for i in 0..16
            { self.block[i/4][i%4] = longer.get_ubyte_(i); }
    }

    fn GF_mul(mut a: u8, mut b: u8, m: u8) -> u8
    {
        let mut ret = 0_u8;
        while b != 0
        {
            if b & 1 == 1
                { ret ^= a; }
            if a & 0b1000_0000 != 0
                { a = (a << 1) ^ m; }
            else
                { a <<= 1; }
            b >>= 1;
        }
        ret
    }


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

    #[allow(non_snake_case)]
    pub fn show_RC()
    {
        for i in 0..ROUND
            { println!("{:02} => {:02X}", i, Self::RC[i]); }
    }
}