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

use crate::number::{ BigUInt, SmallUInt, IntUnion, LongUnion, LongerUnion, BigUInt_Prime };
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
    prime:  [BigUInt<T, N>; 2],
    number: BigUInt<T, {N * 2}>,
    key:    BigUInt<T, N>,
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
    type Prime = BigUInt<T, N>;
    type Number = BigUInt<T, {N * 2}>;
    
    fn choose_prime_numbers(&mut self)
    {
        let rand = Random::new();
        self.prime[0] = rand.random_prime_using_miller_rabin_biguint(5);
        self.prime[1] = rand.random_prime_using_miller_rabin_biguint(5);
        self.number = self.prime[0].expanding_mul(&self.prime[1]);
    }
}