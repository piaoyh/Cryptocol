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

use crate::number::{ BigUInt, SmallUInt, IntUnion, LongUnion, LongerUnion, BigUInt_Prime, BigUInt_Modular };
use crate::random::Random;
use cryptocol::define_utypes_with;


struct RSA_Generic<const N: usize = 32, T = u32, const MR: usize = 5>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    number:         BigUInt<T, N>,
    key_public:     BigUInt<T, N>,
    key_private:    BigUInt<T, N>,
    block:          [T; N],
}

impl<const N: usize, T, const MR: usize> RSA<N, T>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Prime = BigUInt<T, {N / 2}>;
    type Number = BigUInt<T, N>;

    #[inline]
    pub fn new() -> Self
    {
        Self
        {
            number:         Number::new(),
            key_public:     Number::new(),
            key_private:    Number::new(),
            block:          [T; N],
        }
    }

    pub fn new_with_automatic_keys() -> Self
    {
        let mut rsa = Self
        {
            number:         Number::new(),
            key_public:     Number::new(),
            key_private:    Number::new(),
            block:          [T; N],
        };
        rsa.find_keys();
        rsa
    }

    #[inline]
    pub fn new_with_keys(key_public: BigUInt<T, N>, key_private: BigUInt<T, N>, number: BigUInt<T, N>) -> Self
    {
        Self
        {
            number,
            key_public,
            key_private,
            block: [T; N],
        }
    }

    #[inline]
    pub fn set_public_key(key_public: BigUInt<T, N>)
    {
        self.key_public = key_public;
    }

    #[inline]
    pub fn set_private_key(key_private: BigUInt<T, N>)
    {
        self.key_private = key_private;
    }

    #[inline]
    pub fn set_number(number: BigUInt<T, N>)
    {
        self.number = number;
    }
    
    fn choose_prime_numbers() -> (Prime, Prime)
    {
        let rand = Random::new();
        let prime_1: Prime = rand.random_prime_with_msb_set_using_miller_rabin_biguint(MR);
        let prime_2: Prime = rand.random_prime_with_msb_set_using_miller_rabin_biguint(MR);
        (prime_1, prime_2)
    }

    pub fn find_keys(&mut self)
    {
        let (prime_1, prime_2) = Self::choose_prime_numbers();
        self.number = prime_1.expanding_mul(&prime_2);
        let phi = prime_1.wrapping_sub_uint(1).expanding_mul(&prime_2.wrapping_sub_uint(1));
        self.key_public = Number::from_uint(2);
        let mut one: Number;
        (one, self.key_private, _) = self.key_public.extended_gcd(phi);
        while !one.is_one()
        {
            self.key_public.wrapping_add_uint(1);
            (one, self.key_private, _) = self.key_public.extended_gcd(phi);
        }
        if self.key_private.is_underflow()
            { self.key_private = self.number.wrapping_sub(Number::zero().wrapping_sub(&self.key_private)) }
        else
            { self.key_private.wrapping_rem_assign(&self.number); }
    }

    // pub fn encrypt_unit(&self, message: &BigUInt<T, N>) -> BigUInt<T, N>
    /// 
    #[inline]
    pub fn encrypt_unit(&self, message: &BigUInt<T, N>) -> BigUInt<T, N>
    {
        message.to_be().modular_pow(&self.key_public, &number).to_be()
    }

    // pub fn decrypt_unit(&self, cipher: &BigUInt<T, N>) -> BigUInt<T, N>
    ///
    #[inline]
    pub fn decrypt_unit(&self, cipher: &BigUInt<T, N>) -> BigUInt<T, N>
    {
        cipher.to_be().modular_pow(&self.key_private, &number).to_be()
    }

    // pub fn sign_unit(&self, message: &BigUInt<T, N>) -> BigUInt<T, N>
    /// 
    #[inline]
    pub fn sign_unit(&self, message: &BigUInt<T, N>) -> BigUInt<T, N>
    {
        message.to_be().modular_pow(&self.key_private, &number).to_be()
    }

    // pub fn unsign_unit(&self, cipher: &BigUInt<T, N>) -> BigUInt<T, N>
    ///
    #[inline]
    pub fn unsign_unit(&self, cipher: &BigUInt<T, N>) -> BigUInt<T, N>
    {
        cipher.to_be().modular_pow(&self.key_public, &number).to_be()
    }
}
