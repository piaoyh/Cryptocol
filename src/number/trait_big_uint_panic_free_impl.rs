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

use std::mem::size_of;
use std::convert::From;
use std::str::FromStr;
use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
                BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, RangeBounds };

use crate::number::{ SmallUInt, BigUInt, BigUInt_Panic_Free,
                     LongerUnion, SharedValues, SharedArrays, NumberErr };
use crate::number::trait_big_uint_more_impl::{ common_next_multiple_of_assign_uint,
                                               common_modular_next_multiple_of_assign_uint,
                                               common_next_multiple_of_assign,
                                               common_modular_next_multiple_of_assign };



/*** Macro Fuctions ***/

macro_rules! carrying_calc
{
    ($me:expr, $func:expr, $rhs:expr, $carry:expr) => {
        let mut res = Self::from_array($me.get_number().clone());
        let c = $func(&mut res, $rhs, $carry);
        return (res, c);
    }
    // carrying_calc!(self, Self::carrying_add_assign_uint, rhs, carry);
    //
    // let mut res = Self::from_array(self.get_number().clone());
    // let c = res.carrying_add_assign_uint(rhs, carry);
    // (res, c)
}

macro_rules! overflowing_calc
{
    ($me:expr, $func:expr, $rhs:expr) => {
        let mut res = Self::from_array($me.get_number().clone());
        let current_overflow = $func(&mut res, $rhs);
        return (res, current_overflow);
    }
    // overflowing_calc!(self, Self::overflowing_add_assign_uint, rhs);
    //
    // let mut res = Self::from_array(self.get_number().clone());
    // let current_overflow = res.overflowing_add_assign_uint(rhs);
    // (res, current_overflow)
}

macro_rules! overflowing_calc_div
{
    ($me:expr, $func:expr, $rhs:expr) => {
        let (quotient, _) = $func($me, $rhs);
        return (quotient, false);
    }
    // overflowing_calc_div!(self, Self::divide_fully_uint, rhs);
    //
    // let (quotient, _) = self.divide_fully_uint(rhs);
    // let overflow = quotient.is_overflow();
    // (quotient, overflow)
}

macro_rules! overflowing_calc_rem
{
    ($me:expr, $func:expr, $rhs:expr) => {
        let (_, remainder) = $func($me, $rhs);
        return (remainder, false);
    }
    // overflowing_calc_rem!(self, Self::divide_fully_uint, rhs);
    //
    // let (_, remainder) = self.divide_fully_uint(rhs);
    // (remainder, false)
}

macro_rules! overflowing_calc_assign
{
    ($me:expr, $func:expr, $rhs:expr) => {
        let flags = $me.get_all_flags();
        $me.reset_all_flags();
        $func($me, $rhs);
        let current_overflow = $me.is_overflow();
        $me.set_flag_bit(flags);
        return current_overflow;
    }
    // overflowing_calc_assign!(self, Self::wrapping_add_assign_uint, rhs);
    //
    // let flags = self.get_all_flags();
    // self.reset_all_flags();
    // self.wrapping_add_assign_uint(rhs);
    // let current_overflow = self.is_overflow();
    // self.set_flag_bit(flags);
    // current_overflow
}

macro_rules! underflowing_calc_assign
{
    ($me:expr, $func:expr, $rhs:expr) => {
        let flags = $me.get_all_flags();
        $me.reset_all_flags();
        $func($me, $rhs);
        let current_underflow = $me.is_underflow();
        $me.set_flag_bit(flags);
        return current_underflow;
    }
    // underflowing_calc_assign!(self, Self::wrapping_sub_assign_uint, rhs);
    //
    // let flags = self.get_all_flags();
    // self.reset_all_flags();
    // self.wrapping_sub_assign_uint(rhs);
    // let current_underflow = self.is_underflow();
    // self.set_flag_bit(flags);
    // current_underflow
}

macro_rules! biguint_calc_assign_to_calc
{
    ($me:expr, $func:expr) => {
        let mut res = Self::from_array($me.get_number().clone());
        $func(&mut res);
        return res;
    };
    // biguint_calc_assign_to_calc!(self, Self::next_power_of_two_assign);
    //
    // let mut res = Self::from_array(self.get_number().clone());
    // res.next_power_of_two_assign();
    // res

    ($me:expr, $func:expr, $rhs:expr) => {
        let mut res = Self::from_array($me.get_number().clone());
        $func(&mut res, $rhs);
        return res;
    };
    // biguint_calc_assign_to_calc!(self, Self::saturating_add_assign_uint, rhs);
    //
    // let mut res = Self::from_array(self.get_number().clone());
    // res.saturating_add_assign_uint(rhs);
    // res

    ($me:expr, $func:expr, $rhs:expr, $modulo:expr) => {
        let mut res = Self::from_array($me.get_number().clone());
        $func(&mut res, $rhs, $modulo);
        return res;
    }
    // biguint_calc_assign_to_calc!(self, Self::modular_add_assign_uint, rhs, modulo);
    //
    // let mut res = Self::from_array(self.get_number().clone());
    // res.modular_add_assign_uint(rhs, modulo);
    // res
}
pub(super) use biguint_calc_assign_to_calc;

macro_rules! biguint_calc_assign_to_calc_div
{
    ($me:expr, $func:expr, $rhs:expr) => {
        let (quotient, _) = $func($me, $rhs);
        return quotient;
    }
    // biguint_calc_assign_to_calc_div!(self, Self::divide_fully_uint, rhs);
    //
    // let (quotient, _) = self.divide_fully_uint(rhs);
    // quotient
}
pub(super) use biguint_calc_assign_to_calc_div;

macro_rules! biguint_calc_assign_to_calc_rem
{
    ($me:expr, $func:expr, $rhs:expr) => {
        let (_, remainder) = $func($me, $rhs);
        return remainder;
    }
    // biguint_calc_assign_to_calc_rem!(self, Self::divide_fully_uint, rhs);
    //
    // let (_, remainder) = self.divide_fully_uint(rhs);
    // remainder
}
pub(super) use biguint_calc_assign_to_calc_rem;

macro_rules! calc_to_calc_assign
{
    ($me:expr, $func:expr) => {
        let res = $func($me);
        $me.set_number(res.get_number());
        $me.set_all_flags(res.get_all_flags());
    };
    // calc_to_calc_assign!(self, Self::ilog2);
    //
    // let res = self.ilog2();
    // self.set_number(res.get_number());
    // self.set_all_flags(res.get_all_flags());

    ($me:expr, $func:expr, $rhs:expr) => {
        let res = $func($me, $rhs);
        $me.set_number(res.get_number());
        $me.set_all_flags(res.get_all_flags());
    }
    // calc_to_calc_assign!(self, Self::wrapping_div_uint, rhs);
    //
    // let res = self.wrapping_div_uint(rhs);
    // self.set_number(res.get_number());
    // self.set_all_flags(res.get_all_flags());
}

macro_rules! biguint_saturating_calc_assign
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
    // biguint_saturating_calc_assign!(self, Self::overflowing_add_assign_uint, rhs);
    //
    // let overflow = self.is_overflow();
    // if self.overflowing_add_assign_uint(rhs)
    // {
    //     self.set_max();
    //     if !overflow
    //         { self.reset_overflow(); }
    // }
}
pub(super) use biguint_saturating_calc_assign;

macro_rules! biguint_checked_calc
{
    ($me:expr, $func:expr) => {
        return  if $me.is_zero()
                    { None }
                else
                    { Some($func($me)) };
    };
    // biguint_checked_calc!(self, Self::ilog2);
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
    // biguint_checked_calc!(self, Self::overflowing_add_uint, rhs);
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
    // biguint_checked_calc!(self, Self::ilog_uint, base, self.is_zero() || (base.is_zero_or_one()));
    //
    // if self.is_zero() || (base.is_zero_or_one())
    //     { None }
    // else
    //     { Some(self.ilog_uint(base)) }
}
pub(super) use biguint_checked_calc;

macro_rules! biguint_modular_calc_assign
{
    ($me:expr, $func:expr, $rhs:expr, $modulo:expr) => {
        if $modulo.is_zero_or_one()
            { panic!(); }
        $func($me, $rhs, $modulo);
    }
    // biguint_modular_calc_assign!(self, Self::common_modular_add_assign_uint, rhs, modulo);
    //
    // if modulo.is_zero_or_one()
    //     { panic!(); }
    // self.common_modular_add_assign_uint(rhs, modulo);
}
pub(super) use biguint_modular_calc_assign;

macro_rules! panic_free_modular_calc_assign
{
    ($me:expr, $func:expr, $rhs:expr, $modulo:expr) => {
        if $modulo.is_zero_or_one()
        {
            $me.set_zero();
            $me.set_undefined();
        }
        else
        {
            $func($me, $rhs, $modulo);
        }
    }
    // panic_free_modular_calc_assign!(self, Self::common_modular_add_assign_uint, rhs, modulo);
    //
    // if modulo.is_zero_or_one()
    // {
    //     self.set_undefined();
    //     self.set_zero();
    // }
    // else
    // {
    //     self.common_modular_add_assign(rhs, modulo);
    // }
}

macro_rules! panic_free_calc_div_rem_assign
{
    ($me:expr, $func:expr, $rhs:expr) => {
        let res = $func($me, $rhs);
        $me.set_number(res.get_number());
        $me.set_flag_bit(res.get_all_flags());
    }
    // panic_free_calc_div_rem_assign!(self, Self::panic_free_div_uint, rhs);
    //
    // let res = self.panic_free_div_uint(rhs);
    // self.set_number(res.get_number());
    // self.set_flag_bit(res.get_all_flags());
}

macro_rules! if_rhs_is_zero
{
    ($me:expr, $rhs:expr) => {
        let mut q: Self;
        let mut r = Self::zero();
        r.set_all_flags(Self::DIVIDED_BY_ZERO);
        if Self::is_zero($me)
        {
            q = Self::zero();
            q.set_all_flags(Self::UNDEFINED | Self::DIVIDED_BY_ZERO);
        }
        else
        {
            q = Self::max();
            q.set_all_flags(Self::INFINITY | Self::DIVIDED_BY_ZERO);
        }
        return (q, r);
    }
    // if_rhs_is_zero!(self, rhs);
    //
    // let mut q: Self;
    // let mut r = Self::zero();
    // r.set_all_flags(Self::DIVIDED_BY_ZERO);
    // if self.is_zero()
    // {
    //     q = Self::zero();
    //     q.set_all_flags(Self::UNDEFINED | Self::DIVIDED_BY_ZERO);
    // }
    // else
    // {
    //     q = Self::max();
    //     q.set_all_flags(Self::INFINITY | Self::DIVIDED_BY_ZERO);
    // }
    // return (q, r);
}

macro_rules! general_pow_assign
{
    ($me:expr, $func:expr, $exp:expr) => {
        if $me.is_zero() && $exp.is_zero()
            { panic!(); }
        $func($me, $exp);
    }
    // general_pow_assign!(self, Self::common_pow_assign, exp);
    //
    // if self.is_zero() && exp.is_zero()
    //     { panic!(); }
    // self.common_pow_assign(exp);
}

macro_rules! panic_free_calc_pow_assign
{
    ($me:expr, $func:expr, $exp:expr) => {
        if $me.is_zero() && $exp.is_zero()
        {
            $me.set_zero();
            $me.set_undefined();
            return;
        }
        $func($me, $exp);
    }
    // panic_free_calc_pow_assign!(self, Self::common_pow_assign_uint, exp);
    //
    // if self.is_zero() && exp.is_zero()
    // {
    //     self.set_zero();
    //     self.set_undefined();
    //     return;
    // }
    // self.common_pow_assign_uint(exp);
}

macro_rules! general_modular_calc_pow_assign
{
    ($me:expr, $one:expr, $exp:expr, $modulo:expr) => {
        let mut res = Self::one();
        let mut mexp = $exp.clone();
        while !mexp.is_zero()
        {
            if mexp.is_odd()
                { res.modular_mul_assign($me, $modulo); }
            $me.modular_mul_assign(&$me.clone(), $modulo);
            mexp >>= $one;
        }
        $me.set_number(res.get_number());
        $me.set_flag_bit(res.get_all_flags());
    }
    // general_modular_calc_pow_assign!(self, 1, exp, modulo);
    //
    // let mut res = Self::one();
    // let mut mexp = exp.clone();
    // while !mexp.is_zero()
    // {
    //     if mexp.is_odd()
    //         { res.modular_mul_assign(self, modulo); }
    //     self.modular_mul_assign(&self.clone(), modulo);
    //     mexp >>= 1;
    // }
    // self.set_number(res.get_number());
    // self.set_flag_bit(res.get_all_flags());
}

macro_rules! general_calc_iroot
{
    ($me:expr, $func:expr, $exp:expr) => {
        if $exp.is_zero()
        {
            if $me.is_zero_or_one()
            {
                panic!();
            }
            else
            {
                let mut res = Self::max();
                res.set_all_flags(Self::UNDEFINED | Self::INFINITY);
                return res;
            }
        }
        else if $me.is_zero()
        {
            return Self::zero();
        }
        else if $me.is_one()
        {
            return Self::one();
        }
        else
        {
            return $func($me, $exp);
        }
    }
    // general_calc_iroot!(self, Self::common_iroot, exp);
    //
    // if exp.is_zero()
    // {
    //     if self.is_zero_or_one()
    //     {
    //         panic!();
    //     }
    //     else
    //     {
    //         let mut res = Self::max();
    //         res.set_all_flags(Self::UNDEFINED | Self::INFINITY);
    //         res
    //     }
    // }
    // else if self.is_zero()
    // {
    //     Self::zero()
    // }
    // else if self.is_one()
    // {
    //     Self::one()
    // }
    // else
    // {
    //     self.common_iroot(exp)
    // }
}

macro_rules! general_panic_free_calc_iroot
{
    ($me:expr, $func:expr, $exp:expr) => {
        if $exp.is_zero()
        {
            let mut res;
            if $me.is_zero_or_one()
            {
                res = Self::zero();
                res.set_undefined();
            }
            else
            {
                res = Self::max();
                res.set_all_flags(Self::UNDEFINED | Self::INFINITY);
            }
            return res;
        }
        else if $me.is_zero()
        {
            return Self::zero();
        }
        else if $me.is_one()
        {
            return Self::one();
        }
        else
        {
            return $func($me, $exp);
        }
    }
    // general_panic_free_calc_iroot!(self, Self::common_iroot, exp);
    //
    // if exp.is_zero()
    // {
    //     let mut res;
    //     if self.is_zero_or_one()
    //     {
    //         res = Self::zero();
    //         res.set_undefined();
    //     }
    //     else
    //     {
    //         res = Self::max();
    //         res.set_all_flags(Self::UNDEFINED | Self::INFINITY);
    //     }
    //     res
    // }
    // else if self.is_zero()
    // {
    //     Self::zero()
    // }
    // else if self.is_one()
    // {
    //     Self::one()
    // }
    // else
    // {
    //     self.common_iroot(exp)
    // }
}

macro_rules! general_calc_common_ilog
{
    ($me:expr, $func:expr, $base:expr) => {
        if $me.is_one() && !$base.is_zero_or_one()
            { return Self::zero(); }
        let mut count = 0_u128;
        let mut quotient = $me.clone();
        $func(&mut quotient, $base);
        while !quotient.is_zero()
        {
            count += 1;
            $func(&mut quotient, $base);
        }
        return Self::from_uint(count)
    }
    // general_calc_common_ilog!(self, Self::wrapping_div_assign_uint, base);
    //
    // if self.is_one() && !base.is_zero_or_one()
    //     { return Self::zero(); }
    // let mut count = 0_u128;
    // let mut quotient = self.clone();
    // quotient.wrapping_div_assign_uint(base);
    // while !quotient.is_zero()
    // {
    //     count += 1;
    //     quotient.wrapping_div_assign_uint(base);
    // }
    // Self::from_uint(count)
}

macro_rules! general_calc_ilog
{
    ($me:expr, $func:expr, $base:expr) => {
        if $me.is_zero() ||  $base.is_zero_or_one()
            { panic!(); }
        return $func($me, $base);
    }
    // general_calc_ilog!(self, Self::common_ilog_uint, base);
    //
    // if self.is_zero() || base.is_zero_or_one()
    //     { panic!(); }
    // self.common_ilog_uint(base)
}

macro_rules! general_panic_free_calc_ilog
{
    ($me:expr, $func:expr, $base:expr) => {
        if $me.is_zero()
        {
            let mut res = Self::zero();
            res.set_undefined();
            return res;
        }
        else if $me.is_one()
        {
            if $base.is_zero_or_one()
            {
                let mut res = Self::zero();
                res.set_undefined();
                return res;
            }
        }
        else if $base.is_zero_or_one()
        {
            let mut res = Self::max();
            res.set_flag_bit(Self::INFINITY | Self::UNDEFINED);
            return res;
        }
        return $func($me, $base);
    }
    // general_panic_free_calc_ilog!(self, Self::common_ilog_uint, base);
    //
    // if self.is_zero()
    // {
    //     let mut res = Self::zero();
    //     res.set_undefined();
    //     return res;
    // }
    // else if sekf.is_one()
    // {
    //     if $base.is_zero_or_one()
    //     {
    //         let mut res = Self::zero();
    //         res.set_undefined();
    //         return res;
    //     }
    // }
    // else if $base.is_zero_or_one()
    // {
    //     let mut res = Self::max();
    //     res.set_infinity();
    //     res.set_undefined();
    //     return res;
    // }
    // self.common_ilog_uint(base)
}

macro_rules! bitcalc
{
    ($me:expr, $op:tt, $rhs:expr) => {
        let mut n: T;
        for idx in 0..N
        {
            n = $me.get_num_(idx) $op $rhs.get_num_(idx);
            $me.set_num_(idx, n);
        }
    }
    // bitcalc!(self, &, rhs);
    //
    // let mut n: T;
    // for idx in 0..N
    // {
    //     n = self.get_num_(idx) & rhs.get_num_(idx);
    //     self.set_num_(idx, n);
    // }
}

macro_rules! calc_rotate_assign
{
    ($me:expr, $func_main:expr, $func_aux:expr, $n:expr) => {
        let len = $me.length_in_bits().into_u128();
        let m = $n.into_u128().wrapping_rem(len);
        let k = len - m;
        let flags = $me.get_all_flags();
        let right = $func_aux($me, k);
        $func_main($me, m);
        $me.or_assign(&right);
        $me.set_all_flags(flags);
    }
    // calc_rotate_assign!(self, Self::shift_left_assign, Self::shift_right, n);
    //
    // let len = self.length_in_bits().into_u128();
    // let m = n.into_u128().wrapping_rem(len);
    // let k = len - m;
    // let flags = self.get_all_flags();
    // let right = self.shift_right(k);
    // self.shift_left_assign(m);
    // self.or_assign(&right);
    // self.set_all_flags(flags);
}



impl<T, const N: usize> BigUInt_Panic_Free<T, N> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd

{
    /*** ADDITION ***/

    fn panic_free_modular_add_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd

    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_modular_add_assign_uint, rhs, modulo);
    }
    
    fn panic_free_modular_add_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        panic_free_modular_calc_assign!(self, Self::common_modular_add_assign_uint, rhs, modulo);
    }

    fn panic_free_modular_add(&self, rhs: &Self, modulo: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_modular_add_assign, rhs, modulo);
    }

    fn panic_free_modular_add_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        panic_free_modular_calc_assign!(self, Self::common_modular_add_assign, rhs, modulo);
    }


    /*** SUBTRACTION ***/
    fn panic_free_modular_sub_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_modular_sub_assign_uint, rhs, modulo);
    }

    fn panic_free_modular_sub_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        panic_free_modular_calc_assign!(self, Self::common_modular_sub_assign_uint, rhs, modulo);
    }

    fn panic_free_modular_sub(&self, rhs: &Self, modulo: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_modular_sub_assign, rhs, modulo);
    }

    fn panic_free_modular_sub_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        panic_free_modular_calc_assign!(self, Self::common_modular_sub_assign, rhs, modulo);
    }



    /*** MULTIPLICATION ***/

    fn panic_free_modular_mul_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_modular_mul_assign_uint, rhs, modulo);
    }

    fn panic_free_modular_mul_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        panic_free_modular_calc_assign!(self, Self::common_modular_mul_assign_uint, rhs, modulo);
    }

    fn panic_free_modular_mul(&self, rhs: &Self, modulo: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_modular_mul_assign, rhs, modulo);
    }

    fn panic_free_modular_mul_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        panic_free_modular_calc_assign!(self, Self::common_modular_mul_assign, rhs, modulo);
    }


    /*** DIVISION ***/

    fn panic_free_divide_fully_uint<U>(&self, rhs: U) -> (Self, Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if rhs.is_zero()
        {
            if_rhs_is_zero!(self, rhs);
        }
        else
        {
            let (q, rem) = self.common_divide_fully_uint(rhs);
            let r = Self::from_uint(rem);
            (q, r)
        }
    }

    fn panic_free_div_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_assign_to_calc_div!(self, Self::panic_free_divide_fully_uint, rhs);
    }

    fn panic_free_div_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        panic_free_calc_div_rem_assign!(self, Self::panic_free_div_uint, rhs);
    }

    fn panic_free_modular_div_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_modular_div_assign_uint, rhs, modulo);
    }

    fn panic_free_modular_div_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut terminated = false;
        let mut mrhs = rhs;
        if !modulo.is_zero_or_one()
        {
            if *self >= *modulo
            {
                self.wrapping_rem_assign(modulo);
                self.reset_all_flags();
            }
            if modulo.le_uint(rhs)
            {
                let modu = modulo.into_uint::<U>();
                mrhs = rhs.wrapping_rem(modu);
            }
        }

        if mrhs.is_zero()
        {
            if self.is_zero()
            {
                self.set_zero();
                self.set_flag_bit(Self::UNDEFINED | Self::DIVIDED_BY_ZERO);
            }
            else
            {
                self.set_max();
                self.set_flag_bit(Self::INFINITY | Self::DIVIDED_BY_ZERO);
            }
            terminated = true;
        }
        if modulo.is_zero_or_one()
        {
            self.set_zero();
            self.set_undefined();
            return;
        }
        if terminated
            { return; }

        let flags = self.get_all_flags();
        if *self >= *modulo
        {
            self.wrapping_rem_assign(modulo);
            self.reset_all_flags();
        }

        if mrhs.length_in_bytes() > T::size_in_bytes()
        {
            self.modular_div_assign(&Self::from_uint(rhs), modulo);
        }
        else if modulo.gt_uint(rhs)
        {
            self.wrapping_div_assign_uint(mrhs);
        }
        else
        {
            let modu = modulo.into_uint::<U>();
            let mself = self.into_uint::<U>().wrapping_rem(modu);
            self.set_uint(mself.wrapping_div(mrhs));
        }
        self.set_flag_bit(flags);
    }

    fn panic_free_divide_fully(&self, rhs: &Self) -> (Self, Self)
    {
        if rhs.is_zero()
            { if_rhs_is_zero!(self, rhs); }
        else
            { self.common_divide_fully(rhs) }
    }

    fn panic_free_div(&self, rhs: &Self) -> Self
    {
        biguint_calc_assign_to_calc_div!(self, Self::panic_free_divide_fully, rhs);
    }

    fn panic_free_div_assign(&mut self, rhs: &Self)
    {
        panic_free_calc_div_rem_assign!(self, Self::panic_free_div, rhs);
    }

    fn panic_free_modular_div(&self, rhs: &Self, modulo: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_modular_div_assign, rhs, modulo);
    }

    fn panic_free_modular_div_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        let mut terminated = false;
        let mut mrhs = rhs.clone();
        let flags = self.get_all_flags();
        if !modulo.is_zero_or_one()
        {
            if *self >= *modulo
            {
                self.wrapping_rem_assign(modulo);
                self.reset_all_flags();
            }
            if modulo.le(rhs)
                { mrhs.wrapping_rem_assign(modulo); }
        }

        if mrhs.is_zero()
        {
            if self.is_zero()
            {
                self.set_zero();
                self.set_flag_bit(Self::UNDEFINED | Self::DIVIDED_BY_ZERO);
            }
            else
            {
                self.set_max();
                self.set_flag_bit(Self::INFINITY | Self::DIVIDED_BY_ZERO);
            }
            terminated = true;
        }
        if modulo.is_zero_or_one()
        {
            self.set_zero();
            self.set_undefined();
            return;
        }
        if terminated
            { return; }

        self.wrapping_div_assign(&mrhs);
        self.set_flag_bit(flags);
    }

    fn panic_free_rem_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_assign_to_calc_rem!(self, Self::panic_free_divide_fully_uint, rhs);
    }

    fn panic_free_rem_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        panic_free_calc_div_rem_assign!(self, Self::panic_free_rem_uint, rhs);
    }

    fn panic_free_modular_rem_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_modular_rem_assign_uint, rhs, modulo);
    }

    fn panic_free_modular_rem_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        let mut terminated = false;
        let mut mrhs = rhs;
        let flags = self.get_all_flags();
        if !modulo.is_zero_or_one()
        {
            if *self >= *modulo
            {
                self.wrapping_rem_assign(modulo);
                self.reset_all_flags();
            }
            if modulo.le_uint(rhs)
            {
                let modu = modulo.into_uint::<U>();
                mrhs = rhs.wrapping_rem(modu);
            }
        }

        if mrhs.is_zero()
        {
            self.set_zero();
            self.set_divided_by_zero();
            terminated = true;
        }
        if modulo.is_zero_or_one()
        {
            if !terminated
                { self.set_zero(); }
            self.set_undefined();
            terminated = true;
        }
        if terminated
        {
            self.set_flag_bit(flags);
            return;
        }

        if rhs.length_in_bytes() > T::size_in_bytes()
        {
            self.panic_free_modular_rem_assign(&Self::from_uint(rhs), modulo);
        }
        else if modulo.gt_uint(rhs)
        {
            self.wrapping_rem_assign_uint(rhs);
        }
        else    // if (U::size_in_bytes() <= T::size_in_bytes()) && (*self < modulo <= rhs)
        {
            let modu = modulo.into_uint::<U>();
            let mself = self.into_uint::<U>().wrapping_rem(modu);
            self.set_uint(mself.wrapping_rem(mrhs));
        }
        self.set_flag_bit(flags);
    }

    fn panic_free_rem(&self, rhs: &Self) -> Self
    {
        biguint_calc_assign_to_calc_rem!(self, Self::panic_free_divide_fully, rhs);
    }

    fn panic_free_rem_assign(&mut self, rhs: &Self)
    {
        panic_free_calc_div_rem_assign!(self, Self::panic_free_rem, rhs);
    }

    fn panic_free_modular_rem(&self, rhs: &Self, modulo: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_modular_rem_assign, rhs, modulo);
    }
    
    fn panic_free_modular_rem_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        let mut terminated = false;
        let mut mrhs = rhs.clone();
        let flags = self.get_all_flags();
        if !modulo.is_zero_or_one()
        {
            if *self >= *modulo
            {
                self.wrapping_rem_assign(modulo);
                self.reset_all_flags();
            }
            if *rhs >= *modulo
                { mrhs = rhs.wrapping_rem(modulo); }
        }

        if mrhs.is_zero()
        {
            self.set_zero();
            self.set_divided_by_zero();
            terminated = true;
        }
        if modulo.is_zero_or_one()
        {
            if !terminated
                { self.set_zero(); }
            self.set_undefined();
            terminated = true;
        }
        if terminated
        {
            self.set_flag_bit(flags);
            return;
        }

        self.wrapping_rem_assign(&mrhs);
        self.set_flag_bit(flags);
    }


    /*** METHODS FOR EXPONENTIATION AND LOGARITHM WITH UINT ***/

    fn panic_free_pow_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_pow_assign_uint, exp);
    }

    fn panic_free_pow_assign_uint<U>(&mut self, exp: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        panic_free_calc_pow_assign!(self, Self::common_pow_assign_uint, exp);
    }

    fn panic_free_modular_pow_uint<U>(&self, exp: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_modular_pow_assign_uint, exp, modulo);
    }

    fn panic_free_modular_pow_assign_uint<U>(&mut self, exp: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if modulo.is_zero_or_one() || (self.is_zero() && exp.is_zero())
        {
            self.set_zero();
            self.set_undefined();
            return;
        }
        if *self >= *modulo
            { self.wrapping_rem_assign(modulo); }
        let mut mexp = exp;
        if modulo.le_uint(exp)
        {
            let modu = modulo.into_uint::<U>();
            mexp = exp.wrapping_rem(modu);
        }
        if self.is_zero() && mexp.is_zero()
        {
            self.set_zero();
            self.set_undefined();
            return;
        }
        self.common_modular_pow_assign_uint(mexp, modulo);
    }

    fn panic_free_pow(&self, exp: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_pow_assign, exp);
    }

    fn panic_free_pow_assign(&mut self, exp: &Self)
    {
        panic_free_calc_pow_assign!(self, Self::common_pow_assign, exp);
    }

    fn panic_free_modular_pow(&self, exp: &Self, modulo: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_modular_pow_assign, exp, modulo);
    }

    fn panic_free_modular_pow_assign(&mut self, exp: &Self, modulo: &Self)
    {
        if modulo.is_zero_or_one() || (self.is_zero() && exp.is_zero())
        {
            self.set_zero();
            self.set_undefined();
            return;
        }
        if *self >= *modulo
            { self.wrapping_rem_assign(modulo); }
        let mut mexp = exp.clone();
        if *modulo <= *exp
            { mexp.wrapping_rem_assign(&modulo); }
        if self.is_zero() && mexp.is_zero()
        {
            self.set_zero();
            self.set_undefined();
            return;
        }
        self.common_modular_pow_assign(&mexp, modulo);
    }

    fn panic_free_iroot_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        general_panic_free_calc_iroot!(self, Self::common_iroot_uint, exp);
    }

    fn panic_free_iroot_assign_uint<U>(&mut self, exp: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        calc_to_calc_assign!(self, Self::panic_free_iroot_uint, exp);
    }

    fn panic_free_iroot(&self, exp: &Self) -> Self
    {
        general_panic_free_calc_iroot!(self, Self::common_iroot, exp);
    }

    fn panic_free_iroot_assign(&mut self, exp: &Self)
    {
        calc_to_calc_assign!(self, Self::panic_free_iroot, exp);
    }

    fn panic_free_ilog_uint<U>(&self, base: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        general_panic_free_calc_ilog!(self, Self::common_ilog_uint, base);
    }

    fn panic_free_ilog_assign_uint<U>(&mut self, base: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        calc_to_calc_assign!(self, Self::panic_free_ilog_uint, base);
    }

    fn panic_free_ilog(&self, base: &Self) -> Self
    {
        general_panic_free_calc_ilog!(self, Self::common_ilog, base);
    }

    fn panic_free_ilog_assign(&mut self, base: &Self)
    {
        calc_to_calc_assign!(self, Self::panic_free_ilog, base);
    }

    fn panic_free_ilog2(&self) -> Self
    {
        if self.is_zero()
        {
            let mut res = Self::zero();
            res.set_undefined();
            return res;
        }
        Self::from_uint(self.length_in_bits() as u64 - self.leading_zeros() as u64 - 1_u64)
    }
    
    fn panic_free_ilog2_assign(&mut self)
    {
        calc_to_calc_assign!(self, Self::panic_free_ilog2);
    }

    #[inline]
    fn panic_free_ilog10(&self) -> Self
    {
        self.panic_free_ilog_uint(10_u8)
    }

    #[inline]
    fn panic_free_ilog10_assign(&mut self)
    {
        self.panic_free_ilog_assign_uint(10_u8);
        // For the future upgrade, the following code is remained.
        // let flag = self.get_all_flags();
        // let log10 = self.panic_free_ilog10();
        // self.set_number(log10.get_number());
        // self.set_flag_bit(flag);
    }
    

    /*** METHODS FOR MISCELLANEOUS ARITHMETIC OPERATIONS ***/

    fn panic_free_gcd_uint<U>(&self, other: U) -> Self
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
        {
            let mut res = Self::zero();
            res.set_undefined();
            res
        }
        else
        {
            self.common_gcd_uint(other)
        }
    }

    fn panic_free_gcd_assign_uint<U>(&mut self, other: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        calc_to_calc_assign!(self, Self::panic_free_gcd_uint, other);
    }

    fn panic_free_gcd(&self, other: &Self) -> Self
    {
        if self.is_zero() || other.is_zero()
        {
            let mut res = Self::zero();
            res.set_undefined();
            res
        }
        else
        {
            self.common_gcd(other)
        }
    }

    fn panic_free_gcd_assign(&mut self, other: &Self)
    {
        calc_to_calc_assign!(self, Self::panic_free_gcd, other);
    }

    fn panic_free_lcm_uint<U>(&self, other: U) -> Self
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
        {
            let mut res = Self::zero();
            res.set_undefined();
            res
        }
        else
        {
            self.wrapping_div(&self.gcd_uint(other)).wrapping_mul_uint(other)
        }
    }

    fn panic_free_lcm_assign_uint<U>(&mut self, other: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        calc_to_calc_assign!(self, Self::panic_free_lcm_uint, other);
    }

    fn panic_free_lcm(&self, other: &Self) -> Self
    {
        if self.is_zero() || other.is_zero()
        {
            let mut res = Self::zero();
            res.set_undefined();
            res
        }
        else
        {
            self.wrapping_div(&self.gcd(&other)).wrapping_mul(&other)
        }
    }

    fn panic_free_lcm_assign(&mut self, other: &Self)
    {
        calc_to_calc_assign!(self, Self::panic_free_lcm, other);
    }

    fn panic_free_next_multiple_of_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_next_multiple_of_assign_uint, rhs);
    }

    fn panic_free_next_multiple_of_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if rhs == U::zero()
        {
            self.set_zero();
            self.set_undefined();
        }
        else
        {
            common_next_multiple_of_assign_uint(self, rhs);
        }
    }

    fn panic_free_modular_next_multiple_of_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_modular_next_multiple_of_assign_uint, rhs, modulo);
    }

    fn panic_free_modular_next_multiple_of_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        if modulo.is_zero_or_one() || rhs.is_zero()
        {
            self.set_zero();
            self.set_undefined();
            return;
        }
        else if modulo.le_uint(rhs)
        {
            let modu = modulo.into_uint::<U>();
            if rhs.wrapping_rem(modu).is_zero()
            {
                self.set_zero();
                self.set_undefined();
                return;
            }
        }
        common_modular_next_multiple_of_assign_uint(self, rhs, modulo);
    }

    fn panic_free_next_multiple_of(&self, rhs: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_next_multiple_of_assign, rhs);
    }

    fn panic_free_next_multiple_of_assign(&mut self, rhs: &Self)
    {
        if rhs.is_zero()
        {
            self.set_zero();
            self.set_undefined();
            return;
        }
        common_next_multiple_of_assign(self, rhs);
    }

    fn panic_free_modular_next_multiple_of(&self, rhs: &Self, modulo: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::panic_free_modular_next_multiple_of_assign, rhs, modulo);
    }

    fn panic_free_modular_next_multiple_of_assign(&mut self, rhs: &Self, modulo: &Self)
    {
        if modulo.is_zero_or_one() || rhs.is_zero() || rhs.wrapping_rem(modulo).is_zero()
        {
            self.set_zero();
            self.set_undefined();
            return;
        }
        common_modular_next_multiple_of_assign(self, rhs, modulo);
    }
}
