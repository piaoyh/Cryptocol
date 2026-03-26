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
#![allow(non_camel_case_types)]
// #![warn(rustdoc::missing_doc_code_examples)]

use crate::number::{ SmallUInt, BigUInt }; //, BigUInt_Modular, BigUInt_Prime };
// use crate::random::Random;


type ECC_25519_u128 = ECC_25519<u128, 2>;
type ECC_25519_u64 = ECC_25519<u64, 4>;
type ECC_25519_u32 = ECC_25519<u32, 8>;
type ECC_25519_u16 = ECC_25519<u16, 16>;
type ECC_25519_u8 = ECC_25519<u8, 32>;

/// This ECC_25519 algorithm is one of the ECC alorithms, which is considered
/// to be the best in both security and speed among many ECC algorithms. It is
/// used for SSH, Whatsapp, etc. It uses Montgomery form
/// `B * y^2 = x^3 + A * x^2 + x` rather than Weierstrass form
/// `y^2 = x^3 + a * x + b`.
/// In the Montgomery form of this algorithm, the coefficient `B` is `1` and
/// the coefficient `A` is `486662`. So, the elliptic curve used in this
/// algorithm ECC25519 is `y^2 = x^3 + 486662 * x^2 + x`. The coefficient `A`
/// was delicately chosen to be `486662`. It enables extremely fast calculations
/// using only the `x`-coordinate, without the `y`-coordinate, through an
/// algorithm called `Montgomery Ladder`. Its performance advantage disappears
/// when the Montgomery form is converted to Weierstrass form and the converted
/// Weierstrass form is used. `2^255 - 19` is used for the prime number p in
/// ECC25519. That is why its name is ECC25519.
/// (By the way, if the Montgomery form of this algorithm is transformed into
/// Weierstrass form, its coefficients `a` and `b` are
/// `19298681539552699237261830834781317975544997444273427339909597334652188435537`,
/// and
/// `55751746669818608904961251901811413253006430154035613303350130095898864705092`,
/// respectively.)
/// The standard generator `G` in ECC25519 is
/// (9, 4311442517106855203430367767572039564158563521142173164314164693445034241090).
/// The reason why the `x`-coordinate of the standard generator `G` is `9` is for
/// enhancing the transparency of design.
/// The order `n` in ECC25519 is `2^252 + 27742317777372353535851937790883648493`,
/// which means adding this many times makes coming to the original point back.
/// 
struct ECC_25519<T, const N: usize, const A: usize = 486662, const B: usize = 1>
where T: SmallUInt
{
    generatop: (BigUInt<T, N>, BigUInt<T, N>),
    key_public: (BigUInt<T, N>, BigUInt<T, N>),
    key_private: BigUInt<T, N>
}

impl<T, const N: usize> ECC_25519<T, N>
where T: SmallUInt
{
    pub fn new() -> Self
    {
        Self
        {
            generatop: (BigUInt::<T, N>::zero(), BigUInt::<T, N>::zero()),
            key_public: (BigUInt::<T, N>::zero(), BigUInt::<T, N>::zero()),
            key_private: BigUInt::<T, N>::zero()
        }
    }


}
