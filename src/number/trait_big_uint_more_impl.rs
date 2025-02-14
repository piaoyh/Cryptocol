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
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, 
                Add, AddAssign, Sub, SubAssign, Mul, MulAssign,
                Div, DivAssign, Rem, RemAssign };

use crate::number::{ SmallUInt, BigUInt };
use crate::number::trait_big_uint_more::BigUInt_More;



macro_rules! calc_assign_to_calc
{
    ($me:expr, $func:expr) => {
        let mut res = Self::from_array($me.get_number().clone());
        $func(&mut res);
        return res;
    };
    // calc_assign_to_calc!(self, Self::next_power_of_two_assign);
    //
    // let mut res = Self::from_array(self.get_number().clone());
    // res.next_power_of_two_assign();
    // res

    ($me:expr, $func:expr, $rhs:expr) => {
        let mut res = Self::from_array($me.get_number().clone());
        $func(&mut res, $rhs);
        return res;
    };
    // calc_assign_to_calc!(self, Self::saturating_add_assign_uint, rhs);
    //
    // let mut res = Self::from_array(self.get_number().clone());
    // res.saturating_add_assign_uint(rhs);
    // res

    ($me:expr, $func:expr, $rhs:expr, $modulo:expr) => {
        let mut res = Self::from_array($me.get_number().clone());
        $func(&mut res, $rhs, $modulo);
        return res;
    }
    // calc_assign_to_calc!(self, Self::modular_add_assign_uint, rhs, modulo);
    //
    // let mut res = Self::from_array(self.get_number().clone());
    // res.modular_add_assign_uint(rhs, modulo);
    // res
}

macro_rules! calc_assign_to_calc_div
{
    ($me:expr, $func:expr, $rhs:expr) => {
        let (quotient, _) = $func($me, $rhs);
        return quotient;
    }
    // calc_assign_to_calc_div!(self, Self::divide_fully_uint, rhs);
    //
    // let (quotient, _) = self.divide_fully_uint(rhs);
    // quotient
}

macro_rules! calc_assign_to_calc_rem
{
    ($me:expr, $func:expr, $rhs:expr) => {
        let (_, remainder) = $func($me, $rhs);
        return remainder;
    }
    // calc_assign_to_calc_rem!(self, Self::divide_fully_uint, rhs);
    //
    // let (_, remainder) = self.divide_fully_uint(rhs);
    // remainder
}

macro_rules! saturating_calc_assign
{
    ($me:expr, $func:expr, $rhs:expr) => {
        let overflow = $me.is_overflow();
        if $func($me, $rhs)
        {
            $me.set_max();
            if !overflow
                { $me.reset_overflow(); }
        }
    }
    // saturating_calc_assign!(self, Self::overflowing_add_assign_uint, rhs);
    //
    // let overflow = self.is_overflow();
    // if self.overflowing_add_assign_uint(rhs)
    // {
    //     self.set_max();
    //     if !overflow
    //         { self.reset_overflow(); }
    // }
}

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

macro_rules! checked_calc
{
    ($me:expr, $func:expr) => {
        return  if $me.is_zero()
                    { None }
                else
                    { Some($func($me)) };
    };
    // checked_calc!(self, Self::ilog2);
    //
    //  if self.is_zero()
    //      { None }
    //  else
    //      { Some(self.ilog2()) }

    ($me:expr, $func:expr, $rhs:expr) => {
        let (res, overflow) = $func($me, $rhs);
        return  if overflow
                    { None }
                else
                    { Some(res) };
    };
    // checked_calc!(self, Self::overflowing_add_uint, rhs);
    //
    // let (res, overflow) = self.overflowing_add_uint(rhs);
    // if overflow
    //     { None }
    // else
    //     { Some(res) }

    ($me:expr, $func:expr, $rhs:expr, $cond:expr) => {
        return  if $cond
                    { None }
                else
                    { Some($func($me, $rhs)) };
    };
    // checked_calc!(self, Self::ilog_uint, base, self.is_zero() || (base.is_zero_or_one()));
    //
    // if self.is_zero() || (base.is_zero_or_one())
    //     { None }
    // else
    //     { Some(self.ilog_uint(base)) }
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



impl<T, const N: usize> BigUInt_More<T, N> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /*** ADDITION UINT ***/

    fn checked_add_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if rhs.length_in_bytes() > T::size_in_bytes()
            { self.checked_add(&Self::from_uint(rhs)) }
        else    // if rhs.length_in_bytes() <= T::size_in_bytes()
            { checked_calc!(self, Self::overflowing_add_uint, rhs); }
    }

    #[inline]
    fn unchecked_add_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.checked_add_uint(rhs).unwrap()
    }

    fn saturating_add_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        calc_assign_to_calc!(self, Self::saturating_add_assign_uint, rhs);
    }

    fn saturating_add_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        saturating_calc_assign!(self, Self::overflowing_add_assign_uint, rhs);
    }

    fn safe_add_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        safe_calc!(self, Self::wrapping_add_uint, Self::unchecked_add_uint, rhs);
    }

    fn safe_add_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        safe_calc_assign!(self, Self::wrapping_add_assign_uint, Self::overflowing_add_assign_uint, rhs);
    }
    


    /*** SUBTRACTION UINT ***/

    fn checked_sub_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if rhs.length_in_bytes() > T::size_in_bytes()
            { self.checked_sub(&Self::from_uint(rhs)) }
        else
            { checked_calc!(self, Self::overflowing_sub_uint, rhs); }
    }

    #[inline]
    fn unchecked_sub_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.checked_sub_uint(rhs).unwrap()
    }

    fn saturating_sub_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        calc_assign_to_calc!(self, Self::saturating_sub_assign_uint, rhs);
    }

    fn saturating_sub_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        saturating_calc_sub_assign!(self, Self::overflowing_sub_assign_uint, rhs);
    }

    fn safe_sub_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        safe_calc!(self, Self::wrapping_sub_uint, Self::unchecked_sub_uint, rhs);
    }

    fn safe_sub_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        safe_calc_assign!(self, Self::wrapping_sub_assign_uint, Self::overflowing_sub_assign_uint, rhs);
    }



    /*** MULTIPLICATION UINT ***/

    fn checked_mul_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if rhs.length_in_bytes() > T::size_in_bytes()
            { self.checked_mul(&Self::from_uint(rhs)) }
        else
            { checked_calc!(self, Self::overflowing_mul_uint, rhs); }
    }
    
    #[inline]
    fn unchecked_mul_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.checked_mul_uint(rhs).unwrap()
    }

    fn saturating_mul_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        calc_assign_to_calc!(self, Self::saturating_mul_assign_uint, rhs);
    }

    fn saturating_mul_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        saturating_calc_assign!(self, Self::overflowing_mul_assign_uint, rhs);
    }

    fn safe_mul_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        safe_calc!(self, Self::wrapping_mul_uint, Self::unchecked_mul_uint, rhs);
    }

    fn safe_mul_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        safe_calc_assign!(self, Self::wrapping_mul_assign_uint, Self::overflowing_mul_assign_uint, rhs);
    }



    /*** DIVISION BIGUINT ***/

    fn checked_div_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        checked_calc!(self, Self::wrapping_div_uint, rhs, rhs.is_zero());
    }

    #[inline]
    fn unchecked_div_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.checked_div_uint(rhs).unwrap()
    }

    #[inline]
    fn saturating_div_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.wrapping_div_uint(rhs)
    }

    #[inline]
    fn saturating_div_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.wrapping_div_assign_uint(rhs)
    }

    fn checked_rem_uint<U>(&self, rhs: U) -> Option<U>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
            checked_calc!(self, Self::wrapping_rem_uint, rhs, rhs.is_zero());
    }

    #[inline]
    fn unchecked_rem_uint<U>(&self, rhs: U) -> U
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
            self.checked_rem_uint(rhs).unwrap()
    }

    #[inline]
    fn saturating_rem_uint<U>(&self, rhs: U) -> U
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.wrapping_rem_uint(rhs)
    }

    #[inline]
    fn saturating_rem_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        self.wrapping_rem_assign_uint(rhs)
    }



    /*** ADDITION BIGUINT ***/

    fn checked_add(&self, rhs: &Self) -> Option<Self>
    {
        checked_calc!(self, Self::overflowing_add, rhs);
    }

    #[inline]
    fn unchecked_add(&self, rhs: &Self) -> Self
    {
        self.checked_add(rhs).unwrap()
    }

    fn saturating_add(&self, rhs: &Self) -> Self
    {
        calc_assign_to_calc!(self, Self::saturating_add_assign, rhs);
    }

    fn saturating_add_assign(&mut self, rhs: &Self)
    {
        saturating_calc_assign!(self, Self::overflowing_add_assign, rhs);
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
        checked_calc!(self, Self::overflowing_sub, rhs);
    }

    #[inline]
    fn unchecked_sub(&self, rhs: &Self) -> Self
    {
        self.checked_sub(rhs).unwrap()
    }

    fn saturating_sub(&self, rhs: &Self) -> Self
    {
        calc_assign_to_calc!(self, Self::saturating_sub_assign, rhs);
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
        checked_calc!(self, Self::overflowing_mul, rhs);
    }

    fn saturating_mul(&self, rhs: &Self) -> Self
    {
        calc_assign_to_calc!(self, Self::saturating_mul_assign, rhs);
    }

    fn saturating_mul_assign(&mut self, rhs: &Self)
    {
        saturating_calc_assign!(self, Self::overflowing_mul_assign, rhs);
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
        checked_calc!(self, Self::wrapping_div, rhs, rhs.is_zero());
    }

    #[inline]
    fn unchecked_div(&self, rhs: &Self) -> Self
    {
        self.checked_div(rhs).unwrap()
    }

    fn saturating_div(&self, rhs: &Self) -> Self
    {
        calc_assign_to_calc_div!(self, Self::divide_fully, rhs);
    }

    fn saturating_div_assign(&mut self, rhs: &Self)
    {
        self.wrapping_div_assign(rhs)
    }

    fn checked_rem(&self, rhs: &Self) -> Option<Self>
    {
        checked_calc!(self, Self::wrapping_rem, rhs, rhs.is_zero());
    }

    #[inline]
    fn unchecked_rem(&self, rhs: &Self) -> Self
    {
        self.checked_rem(rhs).unwrap()
    }

    fn saturating_rem(&self, rhs: &Self) -> Self
    {
        calc_assign_to_calc_rem!(self, Self::divide_fully, rhs);
    }

    #[inline]
    fn saturating_rem_assign(&mut self, rhs: &Self)
    {
        self.wrapping_rem_assign(rhs)
    }
}