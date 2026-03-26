// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

// #![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]
// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]

use std::fmt::{ Display, Debug };
use std::str::FromStr;
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, 
                Add, AddAssign, Sub, SubAssign, Mul, MulAssign,
                Div, DivAssign, Rem, RemAssign };

use crate::number::{ SmallUInt, BigUInt, BigUInt_More };
use crate::number::{ biguint_calc_assign_to_calc, biguint_checked_calc,
                     biguint_calc_assign_to_calc_div, biguint_calc_assign_to_calc_rem,
                     biguint_saturating_calc_assign };


macro_rules! saturating_calc_sub_assign
{
    ($me:expr, $func:expr, $rhs:expr) => {
        let underflow = $me.is_underflow();
        if $func($me, $rhs)
        {
            $me.set_zero();
            if !underflow
                { $me.reset_underflow(); }
        }
    }
    // saturating_calc_sub_assign!(self, Self::overflowing_sub_assign_uint, rhs);
    //
    // let underflow = self.is_underflow();
    // if self.overflowing_sub_assign_uint(rhs)
    // {
    //     self.set_zero();
    //     if !underflow
    //         { self.reset_underflow(); }
    // }
}

macro_rules! safe_calc {
    ($me:expr, $func_release:expr, $func_debug:expr, $rhs:expr) => {
        #[cfg(debug_assertions)]        return $func_debug($me, $rhs);
        #[cfg(not(debug_assertions))]   return $func_release($me, $rhs);
    };
    // safe_calc!(self, Self::wrapping_add, Self::unchecked_add, rhs);
    // 
    // #[cfg(debug_assertions)]        return self.unchecked_add(rhs);
    // #[cfg(not(debug_assertions))]   return self.wrapping_add(rhs);
}

macro_rules! safe_calc_assign {
    ($me:expr, $func_release:expr, $func_debug:expr, $rhs:expr) => {
        #[cfg(debug_assertions)]
        {
            if $func_debug($me, $rhs)
                { panic!(); }
        }
        #[cfg(not(debug_assertions))]   $func_release($me, $rhs);
    };
    // safe_calc_assign!(self, Self::wrapping_add_assign_uint, Self::overflowing_add_assign_uint, rhs);
    // 
    // #[cfg(debug_assertions)]
    // {
    //     if self.overflowing_add_assign_uint(rhs)
    //         { panic!(); }
    // }
    // #[cfg(not(debug_assertions))]   self.wrapping_add_assign_uint(rhs);
}



impl<T, const N: usize> BigUInt<T, N>
where T: SmallUInt,
    Self: Sized + Clone + Display + Debug + ToString
        + Add<Output = Self> + AddAssign
        + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign
        + Div<Output = Self> + DivAssign
        + Rem<Output = Self> + RemAssign
        + Shl<i32, Output = Self> + ShlAssign<i32>
        + Shr<i32, Output = Self> + ShrAssign<i32>
        + BitAnd<Self, Output = Self> + BitAndAssign
        + BitOr<Self, Output = Self> + BitOrAssign
        + BitXor<Self, Output = Self> + BitXorAssign
        + Not<Output = Self>
        + From<T> + FromStr + From<[T; N]> + From<u32>
{
    pub(super) fn common_next_multiple_of_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt
    {
        if U::size_in_bytes() > T::size_in_bytes()
        {
            self.next_multiple_of_assign(&Self::from_uint(rhs));
        }
        else    // if U::size_in_bytes() <= T::size_in_bytes()
        {
            let trhs = T::num(rhs);
            let r = self.wrapping_rem_uint(trhs);
            if !r.is_zero()
                { self.wrapping_add_assign_uint(trhs - r); }
        }
    }

    pub(super) fn common_next_multiple_of_assign(&mut self, rhs: &Self)
    {
        let r = self.wrapping_rem(rhs);
        if !r.is_zero()
            { self.wrapping_add_assign(&rhs.wrapping_sub(&r)); }
    }
}



impl<T, const N: usize> BigUInt_More<T, N> for BigUInt<T, N>
where T: SmallUInt
{
    /*** ADDITION UINT ***/

    fn checked_add_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt
    {
        if rhs.length_in_bytes() > T::size_in_bytes()
            { self.checked_add(&Self::from_uint(rhs)) }
        else    // if rhs.length_in_bytes() <= T::size_in_bytes()
            { biguint_checked_calc!(self, Self::overflowing_add_uint, rhs); }
    }

    #[inline]
    fn unchecked_add_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt
    {
        self.checked_add_uint(rhs).unwrap()
    }

    fn saturating_add_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt
    {
        biguint_calc_assign_to_calc!(self, Self::saturating_add_assign_uint, rhs);
    }

    fn saturating_add_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt
    {
        biguint_saturating_calc_assign!(self, Self::overflowing_add_assign_uint, rhs);
    }

    fn safe_add_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt
    {
        safe_calc!(self, Self::wrapping_add_uint, Self::unchecked_add_uint, rhs);
    }

    fn safe_add_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt
    {
        safe_calc_assign!(self, Self::wrapping_add_assign_uint, Self::overflowing_add_assign_uint, rhs);
    }
    


    /*** SUBTRACTION UINT ***/

    fn checked_sub_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt
    {
        if rhs.length_in_bytes() > T::size_in_bytes()
            { self.checked_sub(&Self::from_uint(rhs)) }
        else
            { biguint_checked_calc!(self, Self::overflowing_sub_uint, rhs); }
    }

    #[inline]
    fn unchecked_sub_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt
    {
        self.checked_sub_uint(rhs).unwrap()
    }

    fn saturating_sub_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt
    {
        biguint_calc_assign_to_calc!(self, Self::saturating_sub_assign_uint, rhs);
    }

    fn saturating_sub_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt
    {
        saturating_calc_sub_assign!(self, Self::overflowing_sub_assign_uint, rhs);
    }

    fn safe_sub_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt
    {
        safe_calc!(self, Self::wrapping_sub_uint, Self::unchecked_sub_uint, rhs);
    }

    fn safe_sub_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt
    {
        safe_calc_assign!(self, Self::wrapping_sub_assign_uint, Self::overflowing_sub_assign_uint, rhs);
    }



    /*** MULTIPLICATION UINT ***/

    fn checked_mul_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt
    {
        if rhs.length_in_bytes() > T::size_in_bytes()
            { self.checked_mul(&Self::from_uint(rhs)) }
        else
            { biguint_checked_calc!(self, Self::overflowing_mul_uint, rhs); }
    }
    
    #[inline]
    fn unchecked_mul_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt
    {
        self.checked_mul_uint(rhs).unwrap()
    }

    fn saturating_mul_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt
    {
        biguint_calc_assign_to_calc!(self, Self::saturating_mul_assign_uint, rhs);
    }

    fn saturating_mul_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt
    {
        biguint_saturating_calc_assign!(self, Self::overflowing_mul_assign_uint, rhs);
    }

    fn safe_mul_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt
    {
        safe_calc!(self, Self::wrapping_mul_uint, Self::unchecked_mul_uint, rhs);
    }

    fn safe_mul_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt
    {
        safe_calc_assign!(self, Self::wrapping_mul_assign_uint, Self::overflowing_mul_assign_uint, rhs);
    }



    /*** DIVISION BIGUINT ***/

    fn checked_div_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt
    {
        biguint_checked_calc!(self, Self::wrapping_div_uint, rhs, rhs.is_zero());
    }

    #[inline]
    fn unchecked_div_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt
    {
        self.checked_div_uint(rhs).unwrap()
    }

    #[inline]
    fn saturating_div_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt
    {
        self.wrapping_div_uint(rhs)
    }

    #[inline]
    fn saturating_div_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt
    {
        self.wrapping_div_assign_uint(rhs)
    }

    fn checked_rem_uint<U>(&self, rhs: U) -> Option<U>
    where U: SmallUInt
    {
            biguint_checked_calc!(self, Self::wrapping_rem_uint, rhs, rhs.is_zero());
    }

    #[inline]
    fn unchecked_rem_uint<U>(&self, rhs: U) -> U
    where U: SmallUInt
    {
            self.checked_rem_uint(rhs).unwrap()
    }

    #[inline]
    fn saturating_rem_uint<U>(&self, rhs: U) -> U
    where U: SmallUInt
    {
        self.wrapping_rem_uint(rhs)
    }

    #[inline]
    fn saturating_rem_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt
    {
        self.wrapping_rem_assign_uint(rhs)
    }



    /*** ADDITION BIGUINT ***/

    fn checked_add(&self, rhs: &Self) -> Option<Self>
    {
        biguint_checked_calc!(self, Self::overflowing_add, rhs);
    }

    #[inline]
    fn unchecked_add(&self, rhs: &Self) -> Self
    {
        self.checked_add(rhs).unwrap()
    }

    fn saturating_add(&self, rhs: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::saturating_add_assign, rhs);
    }

    fn saturating_add_assign(&mut self, rhs: &Self)
    {
        biguint_saturating_calc_assign!(self, Self::overflowing_add_assign, rhs);
    }

    fn safe_add(&self, rhs: &Self) -> Self
    {
        safe_calc!(self, Self::wrapping_add, Self::unchecked_add, rhs);
    }

    fn safe_add_assign(&mut self, rhs: &Self)
    {
        safe_calc_assign!(self, Self::wrapping_add_assign, Self::overflowing_add_assign, rhs);
    }



    /*** SUBTRACTION BIGUINT ***/

    fn checked_sub(&self, rhs: &Self) -> Option<Self>
    {
        biguint_checked_calc!(self, Self::overflowing_sub, rhs);
    }

    #[inline]
    fn unchecked_sub(&self, rhs: &Self) -> Self
    {
        self.checked_sub(rhs).unwrap()
    }

    fn saturating_sub(&self, rhs: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::saturating_sub_assign, rhs);
    }

    fn saturating_sub_assign(&mut self, rhs: &Self)
    {
        saturating_calc_sub_assign!(self, Self::overflowing_sub_assign, rhs);
    }

    fn safe_sub(&self, rhs: &Self) -> Self
    {
        safe_calc!(self, Self::wrapping_sub, Self::unchecked_sub, rhs);
    }

    fn safe_sub_assign(&mut self, rhs: &Self)
    {
        safe_calc_assign!(self, Self::wrapping_sub_assign, Self::overflowing_sub_assign, rhs);
    }



    /*** MULTIPLICATION BIGUINT ***/

    fn checked_mul(&self, rhs: &Self) -> Option<Self>
    {
        biguint_checked_calc!(self, Self::overflowing_mul, rhs);
    }

    fn saturating_mul(&self, rhs: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::saturating_mul_assign, rhs);
    }

    fn saturating_mul_assign(&mut self, rhs: &Self)
    {
        biguint_saturating_calc_assign!(self, Self::overflowing_mul_assign, rhs);
    }

    fn safe_mul(&self, rhs: &Self) -> Self
    {
        safe_calc!(self, Self::wrapping_mul, Self::unchecked_mul, rhs);
    }

    fn safe_mul_assign(&mut self, rhs: &Self)
    {
        safe_calc_assign!(self, Self::wrapping_mul_assign, Self::overflowing_mul_assign, rhs);
    }



    /*** DIVISION BIGUINT ***/
        
    fn checked_div(&self, rhs: &Self) -> Option<Self>
    {
        biguint_checked_calc!(self, Self::wrapping_div, rhs, rhs.is_zero());
    }

    #[inline]
    fn unchecked_div(&self, rhs: &Self) -> Self
    {
        self.checked_div(rhs).unwrap()
    }

    fn saturating_div(&self, rhs: &Self) -> Self
    {
        biguint_calc_assign_to_calc_div!(self, Self::divide_fully, rhs);
    }

    fn saturating_div_assign(&mut self, rhs: &Self)
    {
        self.wrapping_div_assign(rhs)
    }

    fn checked_rem(&self, rhs: &Self) -> Option<Self>
    {
        biguint_checked_calc!(self, Self::wrapping_rem, rhs, rhs.is_zero());
    }

    #[inline]
    fn unchecked_rem(&self, rhs: &Self) -> Self
    {
        self.checked_rem(rhs).unwrap()
    }

    fn saturating_rem(&self, rhs: &Self) -> Self
    {
        biguint_calc_assign_to_calc_rem!(self, Self::divide_fully, rhs);
    }

    #[inline]
    fn saturating_rem_assign(&mut self, rhs: &Self)
    {
        self.wrapping_rem_assign(rhs)
    }



    /*** MULTIPLE UINT ***/
    
    fn next_multiple_of_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt
    {
        biguint_calc_assign_to_calc!(self, Self::next_multiple_of_assign_uint, rhs);
    }

    fn next_multiple_of_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt
    {
        if rhs == U::zero()
            { panic!(); }
        self.common_next_multiple_of_assign_uint(rhs);
    }

    fn is_multiple_of_uint<U>(&self, rhs: U) -> bool
    where U: SmallUInt
    {
        if rhs.is_zero()
            { self.is_zero() }
        else
            { self.wrapping_rem_uint(rhs).is_zero() }
    }

    fn next_multiple_of(&self, rhs: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::next_multiple_of_assign, rhs);
    }

    fn next_multiple_of_assign(&mut self, rhs: &Self)
    {
        if rhs.is_zero()
            { panic!(); }
        self.common_next_multiple_of_assign(rhs);
    }

    fn is_multiple_of(&self, rhs: &Self) -> bool
    {
        if rhs.is_zero()
            { self.is_zero() }
        else
            { self.wrapping_rem(rhs).is_zero() }
    }

    fn midpoint_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt
    {
        biguint_calc_assign_to_calc!(self, Self::midpoint_assign_uint, rhs);
    }
    
    fn midpoint_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt
    {
        if self.is_uint(rhs)
            { return; }
        let b = self.is_odd() && rhs.is_odd();
        let right_carry = self.is_right_carry();
        let overflow = self.is_overflow();
        self.shift_right_assign(1_u8);
        self.wrapping_add_assign_uint(rhs >> U::one());
        if b
            { self.wrapping_add_assign_uint(1_u8); }
        if !right_carry  { self.reset_right_carry(); }
        if !overflow     { self.reset_overflow(); }
    }

    fn midpoint(&self, rhs: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::midpoint_assign, rhs);
    }

    fn midpoint_assign(&mut self, rhs: &Self)
    {
        if *self == *rhs
            { return; }
        let b = self.is_odd() && rhs.is_odd();
        let right_carry = self.is_right_carry();
        let overflow = self.is_overflow();
        self.shift_right_assign(1_u8);
        self.wrapping_add_assign(&(rhs.shift_right(1_u8)));
        if b
            { self.wrapping_add_assign_uint(1_u8); }
        if !right_carry  { self.reset_right_carry(); }
        if !overflow     { self.reset_overflow(); }
    }

    fn checked_pow_uint<U>(&self, exp: U) -> Option<Self>
    where U: SmallUInt
    {
        if self.is_zero() && exp.is_zero()
            { return None; }
        biguint_checked_calc!(self, Self::overflowing_pow_uint, exp);
    }

    fn unchecked_pow_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt
    {
        self.checked_pow_uint(exp).unwrap()
    }

    fn saturating_pow_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt
    {
        biguint_calc_assign_to_calc!(self, Self::saturating_pow_assign_uint, exp);
    }

    fn saturating_pow_assign_uint<U>(&mut self, exp: U)
    where U: SmallUInt
    {
        biguint_saturating_calc_assign!(self, Self::overflowing_pow_assign_uint, exp);
    }

    fn checked_iroot_uint<U>(&self, exp: U) -> Option<Self>
    where U: SmallUInt
    {
        biguint_checked_calc!(self, Self::iroot_uint, exp, exp.is_zero());
    }

    #[inline]
    fn unchecked_iroot_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt
    {
        self.iroot_uint(exp)
    }

    fn checked_ilog_uint<U>(&self, base: U) -> Option<Self>
    where U: SmallUInt
    {
        biguint_checked_calc!(self, Self::ilog_uint, base, self.is_zero() || (base.is_zero_or_one()));
    }

    #[inline]
    fn unchecked_ilog_uint<U>(&self, base: U) -> Self
    where U: SmallUInt
    {
        self.ilog_uint(base)
    }

    fn checked_pow(&self, exp: &Self) -> Option<Self>
    {
        if self.is_zero() && exp.is_zero()
            { return None; }
        biguint_checked_calc!(self, Self::overflowing_pow, exp);
    }

    #[inline]
    fn unchecked_pow(&self, exp: &Self) -> Self
    {
        self.checked_pow(&exp).unwrap()
    }

    fn saturating_pow(&self, exp: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::saturating_pow_assign, exp);
    }

    fn saturating_pow_assign(&mut self, exp: &Self)
    {
        biguint_saturating_calc_assign!(self, Self::overflowing_pow_assign, exp);
    }

    fn checked_iroot(&self, exp: &Self) -> Option<Self>
    {
        biguint_checked_calc!(self, Self::iroot, exp, exp.is_zero());
    }

    #[inline]
    fn unchecked_iroot(&self, exp: &Self) -> Self
    {
        self.iroot(exp)
    }

    fn checked_ilog(&self, base: &Self) -> Option<Self>
    {
        biguint_checked_calc!(self, Self::ilog, base, self.is_zero() || (base.is_zero_or_one()));
    }

    #[inline]
    fn unchecked_ilog(&self, base: &Self) -> Self
    {
        self.ilog(base)
    }

    fn checked_ilog2(&self) -> Option<Self>
    {
        biguint_checked_calc!(self, Self::ilog2);
    }

    #[inline]
    fn unchecked_ilog2(&self) -> Self
    {
        self.ilog2()
    }

    fn checked_ilog10(&self) -> Option<Self>
    {
        self.checked_ilog_uint(10_u8)
    }

    #[inline]
    fn unchecked_ilog10(&self) -> Self
    {
        self.unchecked_ilog_uint(10_u8)
    }

    fn checked_shift_left<U>(&self, n: U) -> Option<Self>
    where U: SmallUInt
    {
        biguint_checked_calc!(self, Self::shift_left, n, Self::size_in_bits().into_u128() <= n.into_u128());
    }

    #[inline]
    fn unchecked_shift_left<U>(&self, n: U) -> Self
    where U: SmallUInt
    {
        self.checked_shift_left(n).unwrap()
    }

    fn checked_shift_right<U>(&self, n: U) -> Option<Self>
    where U: SmallUInt
    {
        biguint_checked_calc!(self, Self::shift_right, n, Self::size_in_bits().into_u128() <= n.into_u128());
    }

    #[inline]
    fn unchecked_shift_right<U>(&self, n: U) -> Self
    where U: SmallUInt
    {
        self.checked_shift_right(n).unwrap()
    }
}