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


use crate::number::{ SmallUInt, IntUnion, LongerUnion };


#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct Rijndael_Generic<const ROUND: usize = 16, const BSIZE: usize = 32, const KSIZE: usize = 32>
{
    key: LongerUnion,
    block: LongerUnion,
    round_key: [u64; ROUND],
    enc: fn (s: &mut Self, message: u128) -> u128,
    dec: fn (s: &mut Self, cipher: u128) -> u128,
}

impl <const ROUND: usize, const BSIZE: usize, const KSIZE: usize>
    Rijndael_Generic<ROUND, BSIZE, KSIZE>
{
    const SUCCESS: u128 = !0;
    const FAILURE: u128 = 0;

    #[inline]
    pub fn new() -> Self
    {
        Self::new_with_key([0_u8; 16])
    }

    pub fn new_with_key(key: [u8; 16]) -> Self
    {
        let mut rijndael = Self
        {
            key:        LongerUnion::new_with_ubytes(key),
            block:      LongerUnion::new(),
            round_key:  [0_u64; ROUND],
            enc:        Self::encrypt_u128,
            dec:        Self::decrypt_u128,
        };
        rijndael.make_round_keys();
        rijndael
    }

    pub fn new_with_key_u128(key: u128) -> Self
    {
        let mut rijndael = Self
        {
            key:        LongerUnion::new_with(key),
            block:      LongerUnion::new(),
            round_key:  [0_u64; ROUND],
            enc:        Self::encrypt_u128,
            dec:        Self::decrypt_u128,
        };
        rijndael.make_round_keys();
        rijndael
    }

    #[inline]
    pub fn encryptor_with_key(key: [u8; 16]) -> Self
    {
        Self::new_with_key(key)
    }

    #[inline]
    pub fn encryptor_with_key_u128(key: u128) -> Self
    {
        Self::new_with_key_u128(key)
    }

    pub fn decryptor_with_key(key: [u8; 16]) -> Self
    {
        let mut rijndael = Self
        {
            key:        LongerUnion::new_with_ubytes(key),
            block:      LongerUnion::new(),
            round_key:  [0_u64; ROUND],
            enc:        Self::decrypt_u128,
            dec:        Self::encrypt_u128,
        };
        rijndael.make_round_keys();
        rijndael
    }

    pub fn decryptor_with_key_u128(key: u128) -> Self
    {
        let mut rijndael = Self
        {
            key:        LongerUnion::new_with(key),
            block:      LongerUnion::new(),
            round_key:  [0_u64; ROUND],
            enc:        Self::decrypt_u128,
            dec:        Self::encrypt_u128,
        };
        rijndael.make_round_keys();
        rijndael
    }

    pub fn get_key(&mut self) -> [u8; 16]
    {
        let mut key = [0u8; 16];
        for i in 0..16
            { key[i] = self.key.get_ubyte_(i); }
        key
    }

    #[inline]
    pub fn get_key_u128(&self) -> u128
    {
        self.key.get_ulonger()
    }

    pub fn set_key(&mut self, key: [u8; 16])
    {
        let mut i = 0_usize;
        for val in key
        {
            self.key.set_ubyte_(i, val);
            i += 1;
        }
        self.make_round_keys();
    }

    pub fn set_key_u128(&mut self, key: u128)
    {
        self.key.set(key);
        self.make_round_keys();
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
        self.block.get() == Self::SUCCESS
    }

    #[inline]
    pub fn is_failed(&self) -> bool
    {
        self.block.get() == Self::FAILURE
    }

    #[inline]
    pub fn set_successful(&mut self)
    {
        self.block.set(Self::SUCCESS);
    }

    #[inline]
    pub fn set_failed(&mut self)
    {
        self.block.set(Self::FAILURE);
    }

    fn encrypt_block(&mut self)
    {

    }

    fn decrypt_block(&mut self)
    {

    }

    fn make_round_keys(&mut self)
    {

    }

    fn make_a_round_key(&mut self, round: usize, left: IntUnion, mut right: IntUnion)
    {

    }

    #[inline] fn get_block(&self) -> u128           { self.block.get() }
    #[inline] fn set_block(&mut self, block: u128)  { self.block.set(block); }
}