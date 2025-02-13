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

use std::str::FromStr;
use std::convert::From;
use std::mem::size_of_val;
use std::fmt::{ Alignment, Error, Formatter, Display, Debug, Pointer,
                Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, 
                Add, AddAssign, Sub, SubAssign, Mul, MulAssign,
                Div, DivAssign, Rem, RemAssign };

use crate::number::{ SmallUInt, BigUInt, BigUInt_More, NumberErr };



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

macro_rules! modular_calc_assign
{
    ($me:expr, $func:expr, $rhs:expr, $modulo:expr) => {
        if $modulo.is_zero_or_one()
            { panic!(); }
        $func($me, $rhs, $modulo);
    }
    // modular_calc_assign!(self, Self::common_modular_add_assign_uint, rhs, modulo);
    //
    // if modulo.is_zero_or_one()
    //     { panic!(); }
    // self.common_modular_add_assign_uint(rhs, modulo);
}

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



impl<T, const N: usize> BigUInt_More for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{

}