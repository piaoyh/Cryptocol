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

use crate::number::{ SmallUInt, BigUInt, Arithmetic, NumberErr };



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



impl<T, const N: usize> Expanded for BigUInt<T, N>
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

    // pub fn overflowing_add_uint<U>(&self, rhs: U) -> (Self, bool)
    /// Calculates `self` + `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a tuple of the addition result `self` + `rhs` along with
    /// a boolean indicating whether an arithmetic overflow would occur.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a tuple of the addition `self` + `rhs` along with a boolean
    /// indicating whether an arithmetic overflow would occur. If an overflow
    /// would have occurred, then the wrapped (modular) value is returned.
    /// 
    /// # Features
    /// - Wrapping (modular) addition.
    /// - If overflow happens, the second element of the output tuple will
    ///   be true and the `OVERFLOW` flag of the return value will be set.
    /// - The second element of the output tuple reflects only
    ///   the current overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [overflowing_add()](struct@BigUInt#method.overflowing_add)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::expanded;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let (res, overflow) = a_biguint.overflowing_add_uint(1_u8);
    /// println!("{} + 1 = {}\noverflow = {}", a_biguint, res, overflow);
    /// assert_eq!(res, U512::max());
    /// assert_eq!(overflow, false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](../documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.overflowing_add_uint)
    fn overflowing_add_uint<U>(&self, rhs: U) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        overflowing_calc!(self, Self::overflowing_add_assign_uint, rhs);
    }

    // pub fn overflowing_add_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Calculates `self` + `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the addition result `self` + `rhs` to `self` back,
    /// and returns a boolean indicating whether an arithmetic overflow
    /// would occur.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns true if an arithmetic overflow would occur.
    /// Otherwise, it returns `false`.
    /// 
    /// # Features
    /// - Wrapping (modular) addition.
    /// - If overflow happened, the flag `OVERFLOW` of `self` will be set.
    /// - If overflow did not happen in the current operation, the output
    ///   will be false even if the `OVERFLOW` flag of `self` was already set
    ///   because of previous operation of `self`.
    /// - The output reflects only the current overflow.
    /// - All the flags are historical, which means, for example, if an overflow
    ///   occurred even once before this current operation or `OVERFLOW`
    ///   flag is already set before this current operation, the `OVERFLOW` flag
    ///   is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [overflowing_add_assign()](struct@BigUInt#method.overflowing_add_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = UU64::max().wrapping_sub_uint(1_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let overflow = a_biguint.overflowing_add_assign_uint(1_u8);
    /// println!("After a_biguint.overflowing_add_assign_uint(1_u8), a_biguint = {}\noverflow = {}", a_biguint, overflow);
    /// assert_eq!(a_biguint, UU64::max());
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](../documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.overflowing_add_assign_uint)
    fn overflowing_add_assign_uint<U>(&mut self, rhs: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        overflowing_calc_assign!(self, Self::wrapping_add_assign_uint, rhs);
    }

    // pub fn checked_add_uint<U>(&self, rhs: U) -> Option<Self>
    /// Calculates `self` + `rhs`,
    /// and returns an addition result `self` + `rhs`
    /// wrapped by `Some` of enum `Option`.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` wrapped by `Some` of enum `Option`
    /// if overflow did not occur at current operation.
    /// Otherwise, it returns `None` of enum `Option`.
    /// 
    /// # Features
    /// It does not wrap around at the boundary of the `Self` type.
    /// So, if overflow happened, it returns `None` of enum `Option`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [checked_add()](struct@BigUInt#method.checked_add)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let res = a_biguint.checked_add_uint(1_u8);
    /// match res
    /// {
    ///     Some(num) => {
    ///         println!("{} + 1 = {}", a_biguint, num);
    ///         assert_eq!(num, U512::max());
    ///         assert_eq!(num.is_overflow(), false);
    ///         assert_eq!(num.is_underflow(), false);
    ///         assert_eq!(num.is_divided_by_zero(), false);
    ///         assert_eq!(num.is_infinity(), false);
    ///         assert_eq!(num.is_undefined(), false);
    ///         assert_eq!(num.is_left_carry(), false);
    ///         assert_eq!(num.is_right_carry(), false);
    ///     },
    ///     None => {
    ///         println!("{} + 1 = overflow", a_biguint);
    ///     }
    /// }
    /// ```
    /// 
    /// # For more examples,
    /// click [here](../documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.checked_add_uint)
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

    // pub fn unchecked_add_uint<U>(&self, rhs: U) -> Self
    /// Calculates `self` + `rhs`, assuming overflow cannot occur,
    /// and returns an addition result `self` + `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If overflow occurred, it will panic. So, use this method
    ///   only when you are sure that overflow will not occur.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` if overflow did not occur at current
    /// operation. Otherwise, it will panic.
    /// 
    /// # Features
    /// It does not wrap around at the boundary of the `Self` type.
    /// So, if overflow happened, it will panic.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [unchecked_add()](struct@BigUInt#method.unchecked_add)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU64::max().wrapping_sub_uint(1_u8);
    /// let res = a_biguint.unchecked_add_uint(1_u8);
    /// println!("{} + 1 = {}", a_biguint, res);
    /// assert_eq!(res, UU64::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](../documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.unchecked_add_uint)
    #[inline]
    pub fn unchecked_add_uint<U>(&self, rhs: U) -> Self
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

    // pub fn saturating_add_uint<U>(&self, rhs: U) -> Self
    /// Calculates `self` + `rhs`,
    /// saturating at the numeric bounds instead of overflowing.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` if the result is less than or equal
    /// to the maximum value of `Self`. If the sum `self` + `rhs` is greater
    /// than the maximum value it returns the maximum value.
    /// 
    /// # Features
    /// - This method saturates when it reaches the maximum value of `Self`.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [saturating_add()](struct@BigUInt#method.saturating_add)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(2_u8);
    /// let res = a_biguint.saturating_add_uint(1_u8);
    /// println!("{} + 1 = {}", a_biguint, res);
    /// assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](../documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.saturating_add_uint)
    pub fn saturating_add_uint<U>(&self, rhs: U) -> Self
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

    // pub fn saturating_add_assign_uint<U>(&mut self, rhs: T)
    /// Calculates `self` + `rhs`,
    /// saturating at the numeric bounds instead of overflowing,
    /// and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - This method saturates when it reaches the maximum value of `Self`.
    /// - It does not set `OVERFLOW` flag of `self`.
    /// - All the flags are historical, which means, for example, if an overflow
    ///   occurred even once before this current operation or `OVERFLOW`
    ///   flag is already set before this current operation, the `OVERFLOW` flag
    ///   is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [saturating_add_assign()](struct@BigUInt#method.saturating_add_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU64::max().wrapping_sub_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084093");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_add_assign_uint(1_u8);
    /// println!("After a_biguint.saturating_add_assign_uint(1_u8), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](../documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.saturating_add_assign_uint)
    pub fn saturating_add_assign_uint<U>(&mut self, rhs: U)
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

    // pub fn modular_add_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    /// Calculates (`self` + `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the result of (`self` + `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either `zero` or `one`, this method will panic.
    /// 
    /// # Output
    /// It returns the modulo-sum (`self` + `rhs`) % `modulo` with wrapping
    /// (modular) addition at `modulo`.
    /// 
    /// # Features
    /// - It takes the addition (= `sum`) of `self` and `rhs`,
    ///   and then finally returns the remainder of `sum` divided by `modulo`.
    /// - Wrapping (modular) addition at `modulo`.
    /// - The differences between this method `modular_add_uint()` and the
    ///   method `wrapping_add_uint()` are, first, where wrapping around
    ///   happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_add_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_add_uint()` sets `OVERFLOW`
    ///   flag when wrapping around happens at `maximum value + 1`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [modular_add()](struct@BigUInt#method.modular_add)
    /// is proper rather than this method `modular_add_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8); // == 76801874298166903427690031858186486050853753882811946569946433649008
    /// let rhs = 1_u8;
    /// let res = a_biguint.modular_add_uint(rhs, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "76801874298166903427690031858186486050853753882811946569946433649007");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](../documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.modular_add_uint)
    pub fn modular_add_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd

    {
        calc_assign_to_calc!(self, Self::modular_add_assign_uint, rhs, modulo);
    }

    // pub fn modular_add_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    /// Calculates (`self` + `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type,
    /// and then, assigns the result back to `self`.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the result of (`self` + `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either `zero` or `one`, this method will panic.
    /// 
    /// # Features
    /// - It takes the addition (= `sum`) of `self` and `rhs`,
    ///   and then finally assigns the remainder of `sum` divided by `modulo`
    ///   to `self` back.
    /// - Wrapping (modular) addition at `modulo`.
    /// - The differences between this method `modular_add_assign_uint()` and
    ///   the method `wrapping_add_assign_uint()` are, first, where wrapping
    ///   around happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_add_assign_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_add_assign_uint()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation, the
    ///   `OVERFLOW` flag is not changed even if this current operation does
    ///    not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [modular_add_assign()](struct@BigUInt#method.modular_add_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 1_u8;
    /// a_biguint.modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490061");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](../documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.modular_add_assign_uint)
    pub fn modular_add_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        modular_calc_assign!(self, Self::common_modular_add_assign_uint, rhs, modulo);
    }

    // pub fn panic_free_modular_add_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    /// Calculates (`self` + `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the result of (`self` + `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the modulo-sum (`self` + `rhs`) % `modulo` with wrapping
    /// (modular) addition at `modulo`.
    /// 
    /// # Features
    /// - It takes the addition (= `sum`) of `self` and `rhs`,
    ///   and then finally returns the remainder of `sum` divided by `modulo`.
    /// - Wrapping (modular) addition at `modulo`.
    /// - The differences between this method `panic_free_modular_add_uint()`
    ///   and the method `wrapping_add_uint()` are, first, where
    ///   wrapping around happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_add_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_add_uint()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulo` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `modulo` | return value | flags       |
    /// |----------|--------------|-------------|
    /// | 0 or 1   | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_modular_add()](struct@BigUInt#method.panic_free_modular_add)
    /// is proper rather than this method `panic_free_modular_add_uint()`.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8);
    /// let rhs = 1_u8;
    /// let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    /// println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "76801874298166903427690031858186486050853753882811946569946433649007");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](../documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.panic_free_modular_add_uint)
    pub fn panic_free_modular_add_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd

    {
        calc_assign_to_calc!(self, Self::panic_free_modular_add_assign_uint, rhs, modulo);
    }

    // pub fn panic_free_modular_add_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    /// Calculates (`self` + `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type,
    /// and then, assigns the result back to `self`.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the result of (`self` + `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - It takes the addition (= `sum`) of `self` and `rhs`,
    ///   and then finally assigns the remainder of `sum` divided by `modulo`
    ///   to `self` back.
    /// - Wrapping (modular) addition at `modulo`.
    /// - The differences between this method
    ///   `panic_free_modular_add_assign_uint()` and the method
    ///   `wrapping_add_assign_uint()` are, first, where wrapping
    ///   around happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_add_assign_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_add_assign_uint()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulo` is either `zero` or `one`, the `UNDEFINED` flag of `self`
    ///   will be set and `self` will have the value `0`.
    /// - In summary, `self` and its flags will be set as follows:
    /// 
    /// | `modulo` | result value (self) | flags       |
    /// |----------|---------------------|-------------|
    /// | 0 or 1   | 0                   | `UNDEFINED` |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [panic_free_modular_add_assign_uint()](struct@BigUInt#method.panic_free_modular_add_assign_uint)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490060");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// 
    /// let rhs = 1_u8;
    /// a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490061");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](../documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.panic_free_modular_add_assign_uint)
    pub fn panic_free_modular_add_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
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
}