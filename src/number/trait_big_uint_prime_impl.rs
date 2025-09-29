// Copyright 2023, 2024, 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains a big unsigned integer struct
//! with user-defined fixed size and its methods.

// #![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]
// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]

use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
                BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign };

use crate::number::{ SmallUInt, BigUInt, BigUInt_Prime, BigUInt_Modular };
use crate::number::biguint_calc_to_calc_assign;



pub(super) fn common_gcd_uint<T, U, const N: usize>(me: &BigUInt<T, N>, other: U) -> BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
    U: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
        + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
        + Rem<Output=U> + RemAssign
        + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
        + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
        + BitXor<Output=U> + BitXorAssign + Not<Output=U>
        + PartialEq + PartialOrd
{
    let mut y = me.wrapping_rem_uint(other);
    let mut x = other;
    while !y.is_zero()
    {
        let t = y;
        y = x.wrapping_rem(y);
        x = t;
    }
    BigUInt::<T, N>::from_uint(x)
}

pub(super) fn common_gcd<T, const N: usize>(me: &BigUInt<T, N>, other: &BigUInt<T, N>) -> BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    let mut x = me.clone();
    let mut y = BigUInt::<T, N>::from_biguint(other);
    let mut t: BigUInt<T, N>;
    while !y.is_zero()
    {
        t = y;
        y = x.wrapping_rem(&t);
        x = t;
    }
    x
}

/// Performs Millar Rabin method with a number less than `self`.
pub(super) fn test_miller_rabin<T, const N: usize>(me: &BigUInt<T, N>, a: &BigUInt<T, N>) -> bool
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    let self_minus_one = me.wrapping_sub_uint(1_u8);
    let mut d = self_minus_one.clone();
    let mut s = 0_u64;
    while d.is_even()
    {
        d.shift_right_assign(1_u8);
        s += 1;
    }
    let mut x = a.modular_pow(&d, me);
    if x == self_minus_one || x.is_one()
        { return true; }
    for _ in 0..s-1
    {
        x.modular_pow_assign_uint(2_u8, me);
        if x.is_one()
            { return false; }
        if x == self_minus_one
            { return true; }
    }
    false
}

impl<T, const N: usize> BigUInt_Prime<T, N> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{    
    /*** METHODS FOR MISCELLANEOUS ARITHMETIC OPERATIONS ***/

    fn gcd_uint<U>(&self, other: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if self.is_zero() || other.is_zero()
            { panic!(); }
        common_gcd_uint(self, other)
    }

    fn gcd_assign_uint<U>(&mut self, other: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_to_calc_assign!(self, Self::gcd_uint, other);
    }

    fn lcm_uint<U>(&self, other: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if self.is_zero() || other.is_zero()
           { panic!(); }
        self.wrapping_div(&self.gcd_uint(other)).wrapping_mul_uint(other)
    }

    fn lcm_assign_uint<U>(&mut self, other: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_to_calc_assign!(self, Self::lcm_uint, other);
    }

    fn gcd(&self, other: &Self) -> Self
    {
        if self.is_zero() || other.is_zero()
            { panic!(); }
        common_gcd(self, other)
    }

    fn gcd_assign(&mut self, other: &Self)
    {
        biguint_calc_to_calc_assign!(self, Self::gcd, other);
    }

    fn extended_gcd(&self, other: &Self) -> (Self, Self, Self)
    {
        if self.is_zero() || other.is_zero()
            { panic!(); }

        let mut a = self.clone();
        let mut b = BigUInt::<T, N>::from_biguint(other);
        let mut x0 = BigUInt::<T, N>::one();
        let mut y0 = BigUInt::<T, N>::zero();
        let mut x1 = BigUInt::<T, N>::zero();
        let mut y1 = BigUInt::<T, N>::one();
        let mut t: BigUInt<T, N>;
        let mut q: BigUInt<T, N>;
        let mut x0_flags = 0_u8;
        let mut y0_flags = 0_u8;
        while !b.is_zero()
        {
            q = a.wrapping_div(&b);

            t = x1.clone();
            x1 = x0.wrapping_sub(&q.wrapping_mul(&x1));
            x0 = t;
            x0_flags |= x0.get_all_flags();

            t = y1.clone();
            y1 = y0.wrapping_sub(&q.wrapping_mul(&y1));
            y0 = t;
            y0_flags |= y0.get_all_flags();
            
            t = b;
            b = a.wrapping_rem(&t);
            a = t;
        }
        x0.set_all_flags(x0_flags);
        y0.set_all_flags(y0_flags);
        (a, x0, y0)
    }
    
    fn lcm(&self, other: &Self) -> Self
    {
        if self.is_zero() || other.is_zero()
           { panic!(); }
        self.wrapping_div(&self.gcd(&other)).wrapping_mul(&other)
    }

    fn lcm_assign(&mut self, other: &Self)
    {
        biguint_calc_to_calc_assign!(self, Self::lcm, other);
    }


    /*** METHODS FOR PRIME NUMBER TEST ***/

    fn is_prime_using_miller_rabin(&self, repetition: usize) -> bool
    {
        if self.is_uint(2_u8) ||  self.is_uint(3_u8)
            { return true; }

        if self.is_zero_or_one() || self.is_even()
            { return false; }
        
        if self.le_uint(u128::MAX)
        {
            let small_self = self.into_u128();
            return small_self.is_prime_using_miller_rabin(repetition);
        }

        let a_list = [2_u8, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71];
        let len = a_list.len();
        let common = if len < repetition { len } else { repetition };
        let mut i = 0;
        while i < common
        {
            if !test_miller_rabin(self, &Self::from_uint(a_list[i]))
                { return false; }
            i += 1;
        }

        let mut a = a_list[len-1] as u32 + 2;
        for _ in i..repetition
        {
            if !test_miller_rabin(self, &Self::from_uint(a))
                { return false; }
            a += 2;
        }
        true
    }
}
