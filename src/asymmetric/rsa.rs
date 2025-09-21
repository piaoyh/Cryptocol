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


struct RSA_Generic<const N: usize = 128, T = u8>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    number: BigUInt<T, N>,
    key:    [BigUInt<T, N>; 2],
    block:  [T; N],
    enc:    fn (s: &mut Self, message: &[T; N]) -> [T; N],
    dec:    fn (s: &mut Self, cipher: &[T; N]) -> [T; N],
}

impl<const N: usize, T> RSA<N, T>
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
    
    fn choose_prime_numbers() -> (Prime, Prime)
    {
        let rand = Random::new();
        let prime_1: Prime = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        let prime_2: Prime = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
        (prime_1, prime_2)
    }

    fn find_keys(&mut self)
    {
        let (prime_1, prime_2) = Self::choose_prime_numbers();
        self.number = prime_1.expanding_mul(&prime_2);
        let phi = prime_1.wrapping_sub_uint(1).expanding_mul(&prime_2.wrapping_sub_uint(1));
        self.key[0] = Number::from_uint(2);
        while !self.key[0].gcd(phi).is_one()
            { self.key[0].wrapping_add_uint(1); }
        self.key[1] = phi.wrapping_sub_uint(1);
        // Extended Euclidan Algorithm will be substituted.
        while !self.key[0].modular_mul(&self.key[1], &phi)
            { self.key[1].wrapping_sub_uint(1); }
    }
}